use std::sync::Arc;
use axum::{response::Response, Extension,extract::path::Path};
use crate::config::State;

/// Handler for the /feed/subscriptions path.
pub async fn subscription(Extension(state): Extension<Arc<State>>) -> Response {
    todo!()
}

/// Handler for the /feed/playlists path.
pub async fn playlists(Extension(state): Extension<Arc<State>>) -> Response {
    todo!()
}

/// Handler for the /feed/history path.
pub async fn history(Extension(state): Extension<Arc<State>>) -> Response {
    todo!()
}

