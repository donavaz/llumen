use super::error::GoogleError;
use super::types::*;
use super::stream::GoogleStream;

pub struct GoogleClient {
    api_key: String,
    base_url: String,
    http_client: reqwest::Client,
}

impl GoogleClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://generativelanguage.googleapis.com/v1beta".to_string(),
            http_client: reqwest::Client::new(),
        }
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }

    pub async fn test_connection(&self) -> Result<bool, GoogleError> {
        let url = format!("{}/models?key={}", self.base_url, self.api_key);
        let response = self.http_client.get(&url).send().await?;

        Ok(response.status().is_success())
    }

    pub async fn list_models(&self) -> Result<Vec<Model>, GoogleError> {
        let url = format!("{}/models?key={}", self.base_url, self.api_key);
        let response = self.http_client.get(&url).send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(GoogleError::InvalidResponse(error_text));
        }

        let model_list: ModelListResponse = response.json().await?;
        Ok(model_list.models)
    }

    pub async fn generate_content(
        &self,
        model: &str,
        request: GenerateContentRequest,
    ) -> Result<GenerateContentResponse, GoogleError> {
        let url = format!(
            "{}/models/{}:generateContent?key={}",
            self.base_url, model, self.api_key
        );

        let response = self
            .http_client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(GoogleError::InvalidResponse(error_text));
        }

        let content_response: GenerateContentResponse = response.json().await?;
        Ok(content_response)
    }

    pub async fn generate_content_stream(
        &self,
        model: &str,
        request: GenerateContentRequest,
    ) -> Result<GoogleStream, GoogleError> {
        let url = format!(
            "{}/models/{}:streamGenerateContent?key={}",
            self.base_url, model, self.api_key
        );

        let response = self
            .http_client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(GoogleError::InvalidResponse(error_text));
        }

        Ok(GoogleStream::new(response))
    }
}
