use std::sync::Arc;
use axum::{response::Redirect, Extension,extract::path::Path, response::Response};
use crate::config::State;
/// Handler for the root path.
/// Ideally this should be redirect the user to the configured home path
/// e.g. /feed/popular or serve search page I guess but this could be changed
pub async fn index(Extension(state): Extension<Arc<State>>)-> Response {
    // TODO: implement
    todo!()
}

/// Handler for the /feed/popular path.
pub async fn popular(Extension(state): Extension<Arc<State>>) -> Response {
    todo!()
}

/// Handler for the /feed/trending path.
pub async fn trending(Extension(state): Extension<Arc<State>>) -> Response {
    todo!()
}