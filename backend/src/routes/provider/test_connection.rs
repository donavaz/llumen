use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{providers::ProviderConfig, AppState};

#[derive(Debug, Deserialize)]
pub struct TestConnectionReq {
    pub provider_config: ProviderConfig,
}

#[derive(Debug, Serialize)]
pub struct TestConnectionResp {
    pub success: bool,
    pub error: Option<String>,
}

pub async fn handle(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<TestConnectionReq>,
) -> Result<Json<TestConnectionResp>, StatusCode> {
    match req.provider_config.test_connection().await {
        Ok(success) => Ok(Json(TestConnectionResp {
            success,
            error: None,
        })),
        Err(e) => Ok(Json(TestConnectionResp {
            success: false,
            error: Some(e.to_string()),
        })),
    }
}
