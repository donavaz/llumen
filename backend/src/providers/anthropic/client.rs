use super::error::AnthropicError;
use super::types::*;
use super::stream::AnthropicStream;

pub struct AnthropicClient {
    api_key: String,
    base_url: String,
    http_client: reqwest::Client,
    api_version: String,
}

impl AnthropicClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://api.anthropic.com/v1".to_string(),
            http_client: reqwest::Client::new(),
            api_version: "2023-06-01".to_string(),
        }
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }

    pub async fn test_connection(&self) -> Result<bool, AnthropicError> {
        let test_request = MessagesRequest {
            model: "claude-sonnet-4-20250514".to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: MessageContent::Text("Hi".to_string()),
            }],
            max_tokens: 10,
            temperature: None,
            system: None,
            stream: None,
            tools: None,
        };

        let url = format!("{}/messages", self.base_url);
        let response = self
            .http_client
            .post(&url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", &self.api_version)
            .header("content-type", "application/json")
            .json(&test_request)
            .send()
            .await?;

        Ok(response.status().is_success())
    }

    pub async fn create_message(
        &self,
        request: MessagesRequest,
    ) -> Result<MessagesResponse, AnthropicError> {
        let url = format!("{}/messages", self.base_url);
        let response = self
            .http_client
            .post(&url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", &self.api_version)
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AnthropicError::InvalidResponse(error_text));
        }

        let message_response: MessagesResponse = response.json().await?;
        Ok(message_response)
    }

    pub async fn create_message_stream(
        &self,
        mut request: MessagesRequest,
    ) -> Result<AnthropicStream, AnthropicError> {
        request.stream = Some(true);
        let url = format!("{}/messages", self.base_url);

        let response = self
            .http_client
            .post(&url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", &self.api_version)
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AnthropicError::InvalidResponse(error_text));
        }

        Ok(AnthropicStream::new(response))
    }
}
