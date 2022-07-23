use std::{sync::Arc, collections::HashMap};
use askama::langid;
use tokio::sync::Mutex;
use axum::{Extension,extract::Query, response::Response, http::Request, body::Body};
use youtubei_rs::query::search;
use crate::config::{State};
use askama::Template;
askama::localization!(LOCALES);
use super::{templates::{base::Base, search::Search}, utils::{render, build_params}};


/// Handler for the /search path.
pub async fn search_and_render(Extension(state): Extension<Arc<Mutex<State>>>,Query(params): Query<HashMap<String, String>>,request: Request<Body>) -> Response {
    let lock = state.lock().await;
    let search_result = search(params.get("q").unwrap().to_string(), "".to_string(), &lock.yt_client_config).await.unwrap();
    // TODO: implement
    let base = Base{
        title: "invidious".to_string(),
        config: &lock.config,
        user: None,
        preferences: &lock.preferences, 
        loc: askama::Locale::new(langid!("en-US"), &LOCALES),
        params: build_params(&request)
    };
    let search = Search{
        loc: askama::Locale::new(langid!("en-US"), &LOCALES),
        _parent: &base,
        videos: match search_result.contents{
            youtubei_rs::types::enums::TwoColumnTypes::TwoColumnSearchResultsRenderer(res) =>  match  res.primary_contents.section_list_renderer.contents.get(0).unwrap(){
                youtubei_rs::types::enums::ItemSectionRendererContents::ItemSectionRenderer(renderer) => renderer.contents.clone(),
                _ => unreachable!()
            },
            _ => unreachable!()
        },
    };
    render(search.render())
}