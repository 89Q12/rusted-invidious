use std::{sync::Arc, collections::HashMap};
use askama::langid;
use reqwest::ClientBuilder;
use tokio::sync::RwLock;
use axum::{Extension,extract::Query, response::Response, http::Request, body::Body};
askama::localization!(LOCALES);
use askama::Template;

use crate::{config::State, api::piped::SearchFilter};
use crate::handlers::templates::TemplateContext;
use super::{templates::search::Search, utils::render};
use crate::api::piped::PipedApi;
use crate::handlers::utils::render_error;


/// Handler for the /search path.
pub async fn search_and_render(Extension(state): Extension<Arc<RwLock<State>>>,Query(params): Query<HashMap<String, String>>,request: Request<Body>) -> Response {
    let query = params.get("q").unwrap();
    let filter = match params.get("filter"){
        Some(filter) => filter.to_owned(),
        None => String::default(),
    };
    let client = ClientBuilder::new().gzip(true).build().unwrap();
    let config = &state.read().await.config;
    let piped = PipedApi::new(client).api_host(config.piped_api_domain.to_owned()).build();
    let context = TemplateContext::new(&request, None, config);
    let items = match piped.get_search_results(query.to_string(), SearchFilter::from(filter)).await{
          Ok(items) => items,
          Err(err) => return render_error(err, context) ,
    };
    let template = Search{
        loc: askama::Locale::new(langid!("en-US"), &LOCALES),
        context,
        items: items.get_items(),
    };
    render(template.render())
}