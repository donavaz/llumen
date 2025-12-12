mod test_connection;
mod list_models;

use axum::{Router, routing::{post, get}};
use crate::AppState;
use std::sync::Arc;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/test", post(test_connection::handle))
        .route("/models", post(list_models::handle))
}
