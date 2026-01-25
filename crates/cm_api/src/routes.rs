//! API routes.

use axum::{routing::get, Router};

use crate::handlers;

/// Create API router.
pub fn create_router() -> Router {
    Router::new()
        .route("/api/clubs/:id", get(handlers::get_club))
        .route("/api/players/:id", get(handlers::get_player))
        .route("/api/state", get(handlers::get_state))
}
