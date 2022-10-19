use std::{sync::Arc, collections::HashMap};
use askama::{langid, Template};
use axum::{response::{Response, Redirect}, Extension,extract::{path::Path, Query}, http::{StatusCode, Request}, body::Body};
use tokio::sync::RwLock;
use tracing::Level;
use crate::config::State;
use super::{utils::{string_to_body, render,proxyfi_url}, templates::watch::Watch};
use crate::handlers::templates::TemplateContext;
askama::localization!(LOCALES);
/// Handler for the /watch?v=id path and renders the watch page
/// TODO clean up, reduce smell
pub async fn watch_v(Extension(state): Extension<Arc<RwLock<State>>>,Query(params): Query<HashMap<String, String>>,request: Request<Body>) -> Response {
    todo!()
}

/// Handler for the paths /w/:id, /v/:id,/e/:id, /shorts/:id and /watch/:id it redirects to the /watch/v=id path
pub async fn redirect(Path(id): Path<String>) -> Redirect{
    Redirect::temporary(&("/watch?v=".to_owned()+ &id))
}

/// Handler for the path /watch_ajax which is used to mark a video as watched if the user is logged in
/// Protected route
pub async fn watch_ajax(Extension(_state): Extension<Arc<RwLock<State>>>,_body: String){
    todo!()
}

/// Handler for the paths /clip/:id it extracts the videoId from the clip id and redirects to the /watch?v=id path
pub async fn clip(Extension(state): Extension<Arc<RwLock<State>>>,Path(id): Path<String>) -> Redirect{
    todo!()
}
