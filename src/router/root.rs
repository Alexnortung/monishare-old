use crate::AppState;
use axum::Router;

use super::auth;
// Router for all the other sub routers
// It just nests the sub routers into itself.
pub fn root_router() -> axum::Router<AppState> {
    let router = Router::new().nest("/auth", super::auth::auth_router());

    router
}
