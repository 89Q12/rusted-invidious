use std::sync::Arc;
use axum::{response::Response, Extension,extract::path::Path};
use youtubei_rs::types::{channel::Tab, query_results::ChannelQuery};
use crate::config::State;

/// Handler for the path /channel/:id, /c/:id, /user/:id
pub async fn index(Extension(state): Extension<Arc<State>>,Path(id): Path<String>){
    todo!()
}

/// Handler for the path /channel/:id/videos, /c/:id/videos, /user/:id/videos
pub async fn videos(Extension(state): Extension<Arc<State>>,Path(id): Path<String>){
    todo!()
}

/// Handler for the path /channel/:id/playlists, /c/:id/playlists, /user/:id/playlists
pub async fn playlists(Extension(state): Extension<Arc<State>>,Path(id): Path<String>){
    todo!()
}

/// Handler for the path /channel/:id/community, /c/:id/community, /user/:id/community
pub async fn community(Extension(state): Extension<Arc<State>>,Path(id): Path<String>){
    todo!()
}

fn fetch_channel(ucid: String, tab: Tab,state: Extension<Arc<State>>) ->ChannelQuery {
    todo!()
}