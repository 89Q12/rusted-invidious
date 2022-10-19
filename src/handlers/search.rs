use std::{sync::Arc, collections::HashMap};
use askama::langid;
use tokio::sync::RwLock;
use axum::{Extension,extract::Query, response::Response, http::Request, body::Body};
use crate::config::State;
use crate::handlers::templates::TemplateContext;
use askama::Template;
askama::localization!(LOCALES);
use super::{templates::search::Search, utils::render};


/// Handler for the /search path.
pub async fn search_and_render(Extension(state): Extension<Arc<RwLock<State>>>,Query(params): Query<HashMap<String, String>>,request: Request<Body>) -> Response {
    todo!()
}