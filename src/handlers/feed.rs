use std::sync::Arc;
use tokio::sync::Mutex;
use axum::{response::Response, Extension};
use crate::config::State;

/// Handler for the /feed/subscriptions path.
pub async fn subscription(Extension(_state): Extension<Arc<Mutex<State>>>) -> Response {
    todo!()
}

/// Handler for the /feed/playlists path.
pub async fn playlists(Extension(_state): Extension<Arc<Mutex<State>>>) -> Response {
    todo!()
}

/// Handler for the /feed/history path.
pub async fn history(Extension(_state): Extension<Arc<Mutex<State>>>) -> Response {
    todo!()
}

