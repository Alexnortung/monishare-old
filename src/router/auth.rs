use axum::{Router, extract::State, routing::get};

use crate::AppState;

pub fn auth_router() -> Router<AppState> {
    let router = Router::new();
    router
}
