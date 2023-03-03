use axum::{
    extract::path::Path,
    response::{Redirect, Response},
    Extension, http::Request, body::Body,
};
use reqwest::ClientBuilder;
use std::sync::Arc;
use tokio::sync::RwLock;
use askama::{langid, Template};
use super::utils::{render, render_error, render_not_implemented};
use crate::config::State;
use crate::{handlers::templates::TemplateContext, api::piped::PipedApi};
askama::localization!(LOCALES);

/// Handler for the path /channel/:id, /c/:id, /user/:id
pub async fn index(
    Extension(state): Extension<Arc<RwLock<State>>>,
    Path(id): Path<String>,
    request: Request<Body>
) -> Response {
  let client = ClientBuilder::new().gzip(true).build().unwrap();
  let config = &state.read().await.config;
  let piped = PipedApi::new(client).api_host(config.piped_api_domain.to_owned()).build();
  let context = TemplateContext::new(&request, None, config);
  let channel = match piped.get_channel(id).await{
        Ok(chan) => chan,
        Err(err) => return render_error(err, context) ,
  };
  let template = super::templates::channel::ChannelTemplate{
      loc: askama::Locale::new(langid!("en-US"), &LOCALES),
      channel,
      context,
  };
  render(template.render())
}

/// Handler for the path /channel/:id/videos, /c/:id/videos, /user/:id/videos
pub async fn videos(
    Path(id): Path<String>,
) -> Redirect {
    Redirect::temporary(&format!("/channel/{}/",id))
}

/// Handler for the path /channel/:id/playlists, /c/:id/playlists, /user/:id/playlists
pub async fn playlists(
    Extension(state): Extension<Arc<RwLock<State>>>,
    request: Request<Body>
) -> Response {
    let config = &state.read().await.config;
    let context = TemplateContext::new(&request, None, config);
    render_not_implemented(context)
}

/// Handler for the path /channel/:id/community, /c/:id/community, /user/:id/community
pub async fn community(
    Extension(state): Extension<Arc<RwLock<State>>>,
    request: Request<Body>
) -> Response {
    let config = &state.read().await.config;
    let context = TemplateContext::new(&request, None, config);
    render_not_implemented(context)
}
/// Handler for the path /channel/:id/live, /c/:id/, /user/:id/live
pub async fn live(
    Extension(state): Extension<Arc<RwLock<State>>>,
    request: Request<Body>
) -> Response {
    let config = &state.read().await.config;
    let context = TemplateContext::new(&request, None, config);
    render_not_implemented(context)
}

/// Handler for the path /attribution_link?a=something&u=/channel/:id/ it resolves the url and redirects to /channel/:id/
pub async fn attribution_link(
    Extension(state): Extension<Arc<RwLock<State>>>,
    request: Request<Body>
) -> Response {
    let config = &state.read().await.config;
    let context = TemplateContext::new(&request, None, config);
    render_not_implemented(context)
}