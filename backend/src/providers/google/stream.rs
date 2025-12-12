use super::error::GoogleError;
use super::types::GenerateContentResponse;
use bytes::Bytes;
use futures_util::stream::Stream;
use futures_util::StreamExt;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct GoogleStream {
    inner: Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>,
    buffer: String,
}

impl GoogleStream {
    pub fn new(response: reqwest::Response) -> Self {
        Self {
            inner: Box::pin(response.bytes_stream()),
            buffer: String::new(),
        }
    }

    fn parse_chunk(&mut self, chunk: &str) -> Option<Result<GenerateContentResponse, GoogleError>> {
        if chunk.is_empty() || chunk.trim() == "[" || chunk.trim() == "]" {
            return None;
        }

        let cleaned = chunk.trim().trim_end_matches(',');

        match serde_json::from_str::<GenerateContentResponse>(cleaned) {
            Ok(response) => Some(Ok(response)),
            Err(e) => Some(Err(GoogleError::SerdeJson(e))),
        }
    }
}

impl Stream for GoogleStream {
    type Item = Result<GenerateContentResponse, GoogleError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            if let Some(newline_pos) = self.buffer.find('\n') {
                let line = self.buffer[..newline_pos].to_string();
                self.buffer = self.buffer[newline_pos + 1..].to_string();

                if let Some(result) = self.parse_chunk(&line) {
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
                    return Poll::Ready(Some(Err(GoogleError::Reqwest(e))));
                }
                Poll::Ready(None) => {
                    if !self.buffer.is_empty() {
                        let remaining = self.buffer.clone();
                        self.buffer.clear();
                        if let Some(result) = self.parse_chunk(&remaining) {
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
