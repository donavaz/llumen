pub mod openai;
pub mod anthropic;
pub mod google;

pub use openai::{OpenAIClient, OpenAIError};
pub use anthropic::{AnthropicClient, AnthropicError};
pub use google::{GoogleClient, GoogleError};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
    OpenAI,
    Anthropic,
    Google,
    OpenRouter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider_type: ProviderType,
    pub api_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub display_name: String,
    pub provider: ProviderType,
    pub capabilities: ModelCapabilities,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing: Option<ModelPricing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_window: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCapabilities {
    pub text: bool,
    pub vision: bool,
    pub image_gen: bool,
    pub files: bool,
    pub audio: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPricing {
    pub input_cost_per_1m: f32,
    pub output_cost_per_1m: f32,
}

impl ProviderConfig {
    pub async fn test_connection(&self) -> anyhow::Result<bool> {
        match self.provider_type {
            ProviderType::OpenAI => {
                let client = OpenAIClient::new(self.api_key.clone());
                Ok(client.test_connection().await?)
            }
            ProviderType::Anthropic => {
                let client = AnthropicClient::new(self.api_key.clone());
                Ok(client.test_connection().await?)
            }
            ProviderType::Google => {
                let client = GoogleClient::new(self.api_key.clone());
                Ok(client.test_connection().await?)
            }
            ProviderType::OpenRouter => {
                Ok(true)
            }
        }
    }

    pub fn get_default_models(&self) -> Vec<ModelInfo> {
        match self.provider_type {
            ProviderType::OpenAI => vec![
                ModelInfo {
                    id: "gpt-4o".to_string(),
                    display_name: "GPT-4o".to_string(),
                    provider: ProviderType::OpenAI,
                    capabilities: ModelCapabilities {
                        text: true,
                        vision: true,
                        image_gen: false,
                        files: true,
                        audio: false,
                    },
                    pricing: Some(ModelPricing {
                        input_cost_per_1m: 2.5,
                        output_cost_per_1m: 10.0,
                    }),
                    context_window: Some(128000),
                },
                ModelInfo {
                    id: "gpt-4o-mini".to_string(),
                    display_name: "GPT-4o Mini".to_string(),
                    provider: ProviderType::OpenAI,
                    capabilities: ModelCapabilities {
                        text: true,
                        vision: true,
                        image_gen: false,
                        files: true,
                        audio: false,
                    },
                    pricing: Some(ModelPricing {
                        input_cost_per_1m: 0.15,
                        output_cost_per_1m: 0.6,
                    }),
                    context_window: Some(128000),
                },
                ModelInfo {
                    id: "gpt-4-turbo".to_string(),
                    display_name: "GPT-4 Turbo".to_string(),
                    provider: ProviderType::OpenAI,
                    capabilities: ModelCapabilities {
                        text: true,
                        vision: true,
                        image_gen: false,
                        files: true,
                        audio: false,
                    },
                    pricing: Some(ModelPricing {
                        input_cost_per_1m: 10.0,
                        output_cost_per_1m: 30.0,
                    }),
                    context_window: Some(128000),
                },
                ModelInfo {
                    id: "dall-e-3".to_string(),
                    display_name: "DALL-E 3".to_string(),
                    provider: ProviderType::OpenAI,
                    capabilities: ModelCapabilities {
                        text: false,
                        vision: false,
                        image_gen: true,
                        files: false,
                        audio: false,
                    },
                    pricing: None,
                    context_window: None,
                },
            ],
            ProviderType::Anthropic => vec![
                ModelInfo {
                    id: "claude-sonnet-4-20250514".to_string(),
                    display_name: "Claude Sonnet 4".to_string(),
                    provider: ProviderType::Anthropic,
                    capabilities: ModelCapabilities {
                        text: true,
                        vision: true,
                        image_gen: false,
                        files: true,
                        audio: false,
                    },
                    pricing: Some(ModelPricing {
                        input_cost_per_1m: 3.0,
                        output_cost_per_1m: 15.0,
                    }),
                    context_window: Some(200000),
                },
                ModelInfo {
                    id: "claude-opus-4-20250514".to_string(),
                    display_name: "Claude Opus 4".to_string(),
                    provider: ProviderType::Anthropic,
                    capabilities: ModelCapabilities {
                        text: true,
                        vision: true,
                        image_gen: false,
                        files: true,
                        audio: false,
                    },
                    pricing: Some(ModelPricing {
                        input_cost_per_1m: 15.0,
                        output_cost_per_1m: 75.0,
                    }),
                    context_window: Some(200000),
                },
                ModelInfo {
                    id: "claude-haiku-4-20250514".to_string(),
                    display_name: "Claude Haiku 4".to_string(),
                    provider: ProviderType::Anthropic,
                    capabilities: ModelCapabilities {
                        text: true,
                        vision: true,
                        image_gen: false,
                        files: true,
                        audio: false,
                    },
                    pricing: Some(ModelPricing {
                        input_cost_per_1m: 0.4,
                        output_cost_per_1m: 2.0,
                    }),
                    context_window: Some(200000),
                },
            ],
            ProviderType::Google => vec![
                ModelInfo {
                    id: "gemini-1.5-pro".to_string(),
                    display_name: "Gemini 1.5 Pro".to_string(),
                    provider: ProviderType::Google,
                    capabilities: ModelCapabilities {
                        text: true,
                        vision: true,
                        image_gen: false,
                        files: true,
                        audio: true,
                    },
                    pricing: Some(ModelPricing {
                        input_cost_per_1m: 1.25,
                        output_cost_per_1m: 5.0,
                    }),
                    context_window: Some(2097152),
                },
                ModelInfo {
                    id: "gemini-1.5-flash".to_string(),
                    display_name: "Gemini 1.5 Flash".to_string(),
                    provider: ProviderType::Google,
                    capabilities: ModelCapabilities {
                        text: true,
                        vision: true,
                        image_gen: false,
                        files: true,
                        audio: true,
                    },
                    pricing: Some(ModelPricing {
                        input_cost_per_1m: 0.075,
                        output_cost_per_1m: 0.3,
                    }),
                    context_window: Some(1048576),
                },
                ModelInfo {
                    id: "imagen-3".to_string(),
                    display_name: "Imagen 3".to_string(),
                    provider: ProviderType::Google,
                    capabilities: ModelCapabilities {
                        text: false,
                        vision: false,
                        image_gen: true,
                        files: false,
                        audio: false,
                    },
                    pricing: None,
                    context_window: None,
                },
            ],
            ProviderType::OpenRouter => vec![],
        }
    }
}
