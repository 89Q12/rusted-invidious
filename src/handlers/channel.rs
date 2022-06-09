use std::sync::Arc;
use tokio::sync::Mutex;
use axum::{response::{Response, Redirect}, Extension,extract::path::Path};
use youtubei_rs::types::{channel::Tab, query_results::ChannelQuery};
use crate::config::State;

/// Handler for the path /channel/:id, /c/:id, /user/:id
pub async fn index(Extension(state): Extension<Arc<Mutex<State>>>,Path(id): Path<String>) -> Response{
    todo!()
}

/// Handler for the path /channel/:id/videos, /c/:id/videos, /user/:id/videos
pub async fn videos(Extension(state): Extension<Arc<Mutex<State>>>,Path(id): Path<String>) -> Response{
    todo!()
}

/// Handler for the path /channel/:id/playlists, /c/:id/playlists, /user/:id/playlists
pub async fn playlists(Extension(state): Extension<Arc<Mutex<State>>>,Path(id): Path<String>) -> Response{
    todo!()
}

/// Handler for the path /channel/:id/community, /c/:id/community, /user/:id/community
pub async fn community(Extension(state): Extension<Arc<Mutex<State>>>,Path(id): Path<String>) -> Response{
    todo!()
}
/// Handler for the path /channel/:id/live, /c/:id/Mutex<State>, /user/:id/live
pub async fn live(Extension(state): Extension<Arc<State>>,Path(id): Path<String>) -> Redirect{
    todo!()
}

/// Handler for the path /attribution_link?a=something&u=/channel/:id/ it resolves the url and redirects to /channel/:id/
pub async fn attribution_link(Extension(state): Extension<Arc<Mutex<State>>>,Path(id): Path<String>) -> Redirect{
    todo!()
}

/// fetches a channel with the given tab e.g. videos
fn fetch_channel(ucid: String, tab: Tab,state: Extension<Arc<Mutex<State>>>) -> ChannelQuery{
    todo!()
}