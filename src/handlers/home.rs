use std::sync::Arc;
use askama::langid;
use reqwest::ClientBuilder;
use tokio::sync::RwLock;
use axum::{Extension, response::Response, http::Request, body::Body};
use crate::config::State;
use crate::handlers::templates::TemplateContext;
use askama::Template;
askama::localization!(LOCALES);
use super::{templates::base::Base, utils::{render, render_error}};
use crate::api::piped::PipedApi;
/// Handler for the root path.
/// Ideally this should be redirect the user to the configured home path
/// e.g. /feed/popular or serve search page I guess but this could be changed
pub async fn index(Extension(state): Extension<Arc<RwLock<State>>>,request: Request<Body>)-> Response {
    let lock = state.read().await;
    // TODO: implement
    let base = Base{
        loc: askama::Locale::new(langid!("en-US"), &LOCALES),
        context: TemplateContext::new(&request, None, &lock.config),
    };
    render(base.render())
}

/// Handler for the /feed/popular path.
pub async fn popular(Extension(_state): Extension<Arc<RwLock<State>>>) -> Response {
    todo!()
}

/// Handler for the /feed/trending path.
pub async fn trending(Extension(state): Extension<Arc<RwLock<State>>>,request: Request<Body>) -> Response {
    let client = ClientBuilder::new().gzip(true).build().unwrap();
    let piped = PipedApi::new(client).api_host("http://localhost:8080".to_owned()).build();
    let config = &state.read().await.config;
    let context = TemplateContext::new(&request, None, config);
    // let videos = match piped.get_trending("en".to_string()).await{
    //       Ok(chan) => Box::new(chan.get_videos()),
    //       Err(err) => return render_error(err, context) ,
    // };
    todo!()
    //let template = super::templates::Trending{
      //  loc: askama::Locale::new(langid!("en-US"), &LOCALES),
        //videos,
      //context,
    //};
    //render(template.render())
}
