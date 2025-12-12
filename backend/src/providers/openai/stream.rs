use super::error::OpenAIError;
use bytes::Bytes;
use futures_util::stream::Stream;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChunk {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<StreamChoice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChoice {
    pub index: i32,
    pub delta: Delta,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCallDelta>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallDelta {
    pub index: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<FunctionDelta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDelta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
}

pub struct OpenAIStream {
    inner: Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>,
    buffer: String,
}

impl OpenAIStream {
    pub fn new(response: reqwest::Response) -> Self {
        Self {
            inner: Box::pin(response.bytes_stream()),
            buffer: String::new(),
        }
    }

    fn parse_line(&mut self, line: &str) -> Option<Result<StreamChunk, OpenAIError>> {
        if line.is_empty() || !line.starts_with("data: ") {
            return None;
        }

        let data = &line[6..];
        if data == "[DONE]" {
            return None;
        }

        match serde_json::from_str::<StreamChunk>(data) {
            Ok(chunk) => Some(Ok(chunk)),
            Err(e) => Some(Err(OpenAIError::SerdeJson(e))),
        }
    }
}

impl Stream for OpenAIStream {
    type Item = Result<StreamChunk, OpenAIError>;

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
                    return Poll::Ready(Some(Err(OpenAIError::Reqwest(e))));
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
