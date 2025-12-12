use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{providers::{ProviderConfig, ModelInfo}, AppState};

#[derive(Debug, Deserialize)]
pub struct ListModelsReq {
    pub provider_config: ProviderConfig,
}

#[derive(Debug, Serialize)]
pub struct ListModelsResp {
    pub models: Vec<ModelInfo>,
    pub error: Option<String>,
}

pub async fn handle(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<ListModelsReq>,
) -> Result<Json<ListModelsResp>, StatusCode> {
    let models = req.provider_config.get_default_models();

    Ok(Json(ListModelsResp {
        models,
        error: None,
    }))
}
