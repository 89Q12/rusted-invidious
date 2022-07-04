use std::sync::Arc;
use askama::langid;
use tokio::sync::Mutex;
use axum::{response::Redirect, Extension,extract::path::Path, response::Response};
use crate::config::{State, Preferences};
use askama::Template;
askama::localization!(LOCALES);
use super::{templates::base::Base, utils::render};
/// Handler for the root path.
/// Ideally this should be redirect the user to the configured home path
/// e.g. /feed/popular or serve search page I guess but this could be changed
pub async fn index(Extension(state): Extension<Arc<Mutex<State>>>)-> Response {
    let lock = state.lock().await;
    // TODO: implement
    let base = Base{
        title: "invidious".to_string(),
        config: &lock.config,
        user: None,
        preferences: &lock.preferences,
        search_bar: None,
        loc: askama::Locale::new(langid!("en-US"), &LOCALES),
        params: todo!(),
    };
    render(base.render())
}

/// Handler for the /feed/popular path.
pub async fn popular(Extension(state): Extension<Arc<Mutex<State>>>) -> Response {
    todo!()
}

/// Handler for the /feed/trending path.
pub async fn trending(Extension(state): Extension<Arc<Mutex<State>>>) -> Response {
    todo!()
}