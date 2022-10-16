use crate::handlers::templates::TemplateContext;
use crate::{config::State, api::piped::Channel};
use axum::{
    extract::path::Path,
    response::{Redirect, Response},
    Extension, http::{StatusCode, Request}, body::Body,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use askama::{langid, Template};
use super::utils::{string_to_body, proxyfi_url, render};
askama::localization!(LOCALES);
/// Handler for the path /channel/:id, /c/:id, /user/:id
pub async fn index(
    Extension(state): Extension<Arc<Mutex<State>>>,
    Path(id): Path<String>,
    request: Request<Body>
) -> Response {
    let context = TemplateContext::new(&request, None, &state.lock().await.config);
    let template = super::templates::channel::ChannelTemplate{
        loc: askama::Locale::new(langid!("en-US"), &LOCALES),
        channel: todo!(),
        context,
    };
    render(template.render())
}

/// Handler for the path /channel/:id/videos, /c/:id/videos, /user/:id/videos
pub async fn videos(
    Extension(state): Extension<Arc<Mutex<State>>>,
    Path(id): Path<String>,
) -> Response {
    todo!()
}

/// Handler for the path /channel/:id/playlists, /c/:id/playlists, /user/:id/playlists
pub async fn playlists(
    Extension(state): Extension<Arc<Mutex<State>>>,
    Path(id): Path<String>,
    request: Request<Body>
) -> Response {
    todo!()
}

/// Handler for the path /channel/:id/community, /c/:id/community, /user/:id/community
pub async fn community(
    Extension(state): Extension<Arc<Mutex<State>>>,
    Path(id): Path<String>,
    request: Request<Body>
) -> Response {
    todo!()
}
/// Handler for the path /channel/:id/live, /c/:id/, /user/:id/live
pub async fn live(
    Extension(_state): Extension<Arc<Mutex<State>>>,
    Path(_id): Path<String>,
) -> Redirect {
    todo!()
}

/// Handler for the path /attribution_link?a=something&u=/channel/:id/ it resolves the url and redirects to /channel/:id/
pub async fn attribution_link(
    Extension(_state): Extension<Arc<Mutex<State>>>,
    Path(_id): Path<String>,
) -> Redirect {
    todo!()
}