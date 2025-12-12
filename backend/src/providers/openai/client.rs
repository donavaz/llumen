use super::error::OpenAIError;
use super::types::*;
use super::stream::OpenAIStream;

pub struct OpenAIClient {
    api_key: String,
    base_url: String,
    http_client: reqwest::Client,
}

impl OpenAIClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://api.openai.com/v1".to_string(),
            http_client: reqwest::Client::new(),
        }
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }

    pub async fn test_connection(&self) -> Result<bool, OpenAIError> {
        let url = format!("{}/models", self.base_url);
        let response = self
            .http_client
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        Ok(response.status().is_success())
    }

    pub async fn list_models(&self) -> Result<Vec<Model>, OpenAIError> {
        let url = format!("{}/models", self.base_url);
        let response = self
            .http_client
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(OpenAIError::InvalidResponse(error_text));
        }

        let model_list: ModelListResponse = response.json().await?;
        Ok(model_list.data)
    }

    pub async fn chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, OpenAIError> {
        let url = format!("{}/chat/completions", self.base_url);
        let response = self
            .http_client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(OpenAIError::InvalidResponse(error_text));
        }

        let completion: ChatCompletionResponse = response.json().await?;
        Ok(completion)
    }

    pub async fn chat_completion_stream(
        &self,
        mut request: ChatCompletionRequest,
    ) -> Result<OpenAIStream, OpenAIError> {
        request.stream = Some(true);
        let url = format!("{}/chat/completions", self.base_url);

        let response = self
            .http_client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(OpenAIError::InvalidResponse(error_text));
        }

        Ok(OpenAIStream::new(response))
    }

    pub async fn generate_image(
        &self,
        request: ImageGenerationRequest,
    ) -> Result<ImageGenerationResponse, OpenAIError> {
        let url = format!("{}/images/generations", self.base_url);
        let response = self
            .http_client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(OpenAIError::InvalidResponse(error_text));
        }

        let image_response: ImageGenerationResponse = response.json().await?;
        Ok(image_response)
    }
}
