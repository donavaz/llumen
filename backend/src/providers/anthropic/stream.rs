use super::error::AnthropicError;
use bytes::Bytes;
use futures_util::stream::Stream;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StreamEvent {
    MessageStart { message: MessageStart },
    ContentBlockStart { index: i32, content_block: ContentBlockStart },
    ContentBlockDelta { index: i32, delta: ContentDelta },
    ContentBlockStop { index: i32 },
    MessageDelta { delta: MessageDelta, usage: DeltaUsage },
    MessageStop,
    Ping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageStart {
    pub id: String,
    #[serde(rename = "type")]
    pub message_type: String,
    pub role: String,
    pub content: Vec<serde_json::Value>,
    pub model: String,
    pub stop_reason: Option<String>,
    pub stop_sequence: Option<String>,
    pub usage: Usage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub input_tokens: i32,
    pub output_tokens: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlockStart {
    Text { text: String },
    ToolUse { id: String, name: String, input: serde_json::Value },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentDelta {
    TextDelta { text: String },
    InputJsonDelta { partial_json: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDelta {
    pub stop_reason: Option<String>,
    pub stop_sequence: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaUsage {
    pub output_tokens: i32,
}

pub struct AnthropicStream {
    inner: Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>,
    buffer: String,
}

impl AnthropicStream {
    pub fn new(response: reqwest::Response) -> Self {
        Self {
            inner: Box::pin(response.bytes_stream()),
            buffer: String::new(),
        }
    }

    fn parse_line(&mut self, line: &str) -> Option<Result<StreamEvent, AnthropicError>> {
        if line.is_empty() {
            return None;
        }

        if let Some(data) = line.strip_prefix("data: ") {
            match serde_json::from_str::<StreamEvent>(data) {
                Ok(event) => Some(Ok(event)),
                Err(e) => Some(Err(AnthropicError::SerdeJson(e))),
            }
        } else if line.starts_with("event: ") {
            None
        } else {
            None
        }
    }
}

impl Stream for AnthropicStream {
    type Item = Result<StreamEvent, AnthropicError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            if let Some(newline_pos) = self.buffer.find('\n') {
                let line = self.buffer[..newline_pos].trim().to_string();
                self.buffer = self.buffer[newline_pos + 1..].to_string();

                if let Some(result) = self.parse_line(&line) {
                    return Poll::Ready(Some(result));
                }
                continue;
            }

            match self.inner.as_mut().poll_next(cx) {
                Poll::Ready(Some(Ok(bytes))) => {
                    let text = String::from_utf8_lossy(&bytes);
                    self.buffer.push_str(&text);
                }
                Poll::Ready(Some(Err(e))) => {
                    return Poll::Ready(Some(Err(AnthropicError::Reqwest(e))));
                }
                Poll::Ready(None) => {
                    if !self.buffer.is_empty() {
                        let remaining = self.buffer.clone();
                        self.buffer.clear();
                        if let Some(result) = self.parse_line(&remaining) {
                            return Poll::Ready(Some(result));
                        }
                    }
                    return Poll::Ready(None);
                }
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}
