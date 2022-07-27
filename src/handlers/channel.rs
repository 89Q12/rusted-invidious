use crate::{config::State, structs::{channel::Channel, template_context}};
use axum::{
    extract::path::Path,
    response::{Redirect, Response},
    Extension, http::{StatusCode, Request}, body::Body,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use youtubei_rs::query::browse_id;
use youtubei_rs::types::{error::Errors, query_results::BrowseResult};
use askama::{langid, Template};
use super::{utils::{string_to_body, build_params, proxyfi_url, render}, templates::base::Base};
askama::localization!(LOCALES);
/// Handler for the path /channel/:id, /c/:id, /user/:id
pub async fn index(
    Extension(state): Extension<Arc<Mutex<State>>>,
    Path(id): Path<String>,
    request: Request<Body>
) -> Response {
    let ch = fetch_channel(id.to_owned(), Tab::Videos, &state);
    let about_call = fetch_channel(id.to_owned(), Tab::About, &state);
    let (about, result) = tokio::join!(ch, about_call);
    let channel = match result {
        Ok(ch) => ch,
        Err(e) => match e {
            youtubei_rs::types::error::Errors::RequestError(err) => return string_to_body(StatusCode::NOT_FOUND.to_string()),
            youtubei_rs::types::error::Errors::ParseError(err) => return string_to_body(StatusCode::NOT_FOUND.to_string()),
        },
    };
    let about = match about {
        Ok(ch) => ch,
        Err(e) => match e {
            youtubei_rs::types::error::Errors::RequestError(err) => return string_to_body(StatusCode::NOT_FOUND.to_string()),
            youtubei_rs::types::error::Errors::ParseError(err) => return string_to_body(StatusCode::NOT_FOUND.to_string()),
        },
    };
    let about = match about.contents.unwrap(){
        youtubei_rs::types::enums::TwoColumnTypes::TwoColumnBrowseResultsRenderer(column_renderer) => match column_renderer.tabs.get(column_renderer.tabs.len() - 1).unwrap().tab_renderer.as_ref().unwrap().content.as_ref().unwrap(){
            youtubei_rs::types::enums::TabRendererContent::SectionListRenderer(list_renderer) => match list_renderer.contents.get(0).unwrap(){
                youtubei_rs::types::enums::ItemSectionRendererContents::ChannelAboutFullMetadataRenderer(about) => about.to_owned(),
                _ => unreachable!()
            },
            _ => unreachable!()
        },
        _ => unreachable!(),
    };
    let header = match channel.header.unwrap(){
        youtubei_rs::types::enums::HeaderContents::C4TabbedHeaderRenderer(c4_tabbed_header) => c4_tabbed_header,
        _ => unreachable!()
    };
    let metadata = match channel.metadata.unwrap(){
        youtubei_rs::types::enums::MetadataRenderers::ChannelMetadataRenderer(data) => data,
        _ => unreachable!()
    };
    let lock = state.lock().await;
    let banner = match &header.banner {
        Some(banner) => Some(proxyfi_url(banner.thumbnails.last().unwrap().url.to_string(), &lock.config)),
        None => None,
    };
    let thumbnail = proxyfi_url(metadata.avatar.thumbnails.last().unwrap().url.to_string(),&lock.config);
    let base = Base{
        title: "invidious".to_string(),
        config: &lock.config,
        user: None,
        preferences: &lock.preferences,
        loc: askama::Locale::new(langid!("en-US"), &LOCALES),
        params: build_params(&request)
    };

    let channel = Channel{
        about,
        banner,
        header,
        metadata,
        thumbnail,
        sort_options: vec!["newest".to_string(), "oldest".to_string(), "popular".to_string()],
        sorted_by: "newest".to_string(),
        auto_generated: false,
        items: match channel.contents.unwrap(){
            youtubei_rs::types::enums::TwoColumnTypes::TwoColumnBrowseResultsRenderer(column_renderer) => match column_renderer.tabs.get(column_renderer.tabs.len() - 1).unwrap().tab_renderer.as_ref().unwrap().content.as_ref().unwrap(){
                youtubei_rs::types::enums::TabRendererContent::SectionListRenderer(list_renderer) =>list_renderer.contents.clone(),
                _ => unreachable!()
            },
            _ => unreachable!()
        },
        has_community_enabled: true,
    };
    let template = super::templates::channel::ChannelTemplate{
        loc: askama::Locale::new(langid!("en-US"), &LOCALES),
        _parent: &base,
        channel,
    };
    render(template.render())
}

/// Handler for the path /channel/:id/videos, /c/:id/videos, /user/:id/videos
pub async fn videos(
    Extension(state): Extension<Arc<Mutex<State>>>,
    Path(id): Path<String>,
) -> Response {
    let result = fetch_channel(id, Tab::Videos, &state).await;
    match result {
        Ok(pl) => {
            todo!()
        }
        Err(e) => match e {
            youtubei_rs::types::error::Errors::RequestError(err) => todo!(),
            youtubei_rs::types::error::Errors::ParseError(err) => todo!(),
        },
    }
}

/// Handler for the path /channel/:id/playlists, /c/:id/playlists, /user/:id/playlists
pub async fn playlists(
    Extension(state): Extension<Arc<Mutex<State>>>,
    Path(id): Path<String>,
) -> Response {
    let result = fetch_channel(id, Tab::Playlists, &state).await;
    match result {
        Ok(pl) => {
            todo!()
        }
        Err(e) => match e {
            youtubei_rs::types::error::Errors::RequestError(err) => todo!(),
            youtubei_rs::types::error::Errors::ParseError(err) => todo!(),
        },
    }
}

/// Handler for the path /channel/:id/community, /c/:id/community, /user/:id/community
pub async fn community(
    Extension(state): Extension<Arc<Mutex<State>>>,
    Path(id): Path<String>,
) -> Response {
    let result = fetch_channel(id, Tab::Community, &state).await;
    match result {
        Ok(pl) => {
            todo!()
        }
        Err(e) => match e {
            youtubei_rs::types::error::Errors::RequestError(err) => todo!(),
            youtubei_rs::types::error::Errors::ParseError(err) => todo!(),
        },
    }
}
/// Handler for the path /channel/:id/live, /c/:id/, /user/:id/live
pub async fn live(
    Extension(state): Extension<Arc<Mutex<State>>>,
    Path(id): Path<String>,
) -> Redirect {
    todo!()
}

/// Handler for the path /attribution_link?a=something&u=/channel/:id/ it resolves the url and redirects to /channel/:id/
pub async fn attribution_link(
    Extension(state): Extension<Arc<Mutex<State>>>,
    Path(id): Path<String>,
) -> Redirect {
    todo!()
}

/// fetches a channel with the given tab e.g. videos
async fn fetch_channel(
    ucid: String,
    tab: Tab,
    state: &Arc<Mutex<State>>,
) -> Result<BrowseResult, Errors> {
    let lock = state.lock().await;
    match tab {
        Tab::Videos => {
            browse_id(
                ucid,
                "EgZ2aWRlb3O4AQDyBgQKAjoA".to_string(),
                &lock.yt_client_config,
            )
            .await
        }
        Tab::Playlists => {
            browse_id(
                ucid,
                "EglwbGF5bGlzdHO4AQDyBgQKAkIA".to_string(),
                &lock.yt_client_config,
            )
            .await
        }
        Tab::Community => {
            browse_id(
                ucid,
                "Egljb21tdW5pdHm4AQDyBgQKAkoA".to_string(),
                &lock.yt_client_config,
            )
            .await
        }
        Tab::About =>  browse_id(
            ucid,
            "EgVhYm91dLgBAPIGBAoCEgA%3D".to_string(),
            &lock.yt_client_config,
        )
        .await,
        // TODO add about tab, params: "EgVhYm91dLgBAPIGBAoCEgA%3D"
    }
}

/// For associations between protobufs and tab name
pub enum Tab{
    Videos,
    Playlists,
    Community,
    About
}

