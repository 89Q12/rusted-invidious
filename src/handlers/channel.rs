use crate::config::State;
use axum::{
    extract::path::Path,
    response::{Redirect, Response},
    Extension,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use youtubei_rs::query::{browse_id, resolve};
use youtubei_rs::types::{error::Errors, query_results::BrowseResult};

/// Handler for the path /channel/:id, /c/:id, /user/:id
pub async fn index(
    Extension(state): Extension<Arc<Mutex<State>>>,
    Path(id): Path<String>,
) -> Response {
    let result = fetch_channel(id.to_owned(), Tab::Videos, &state).await;
    let lock = state.lock().await;
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
        // TODO add about tab in youtubei_rs params: "EgVhYm91dLgBAPIGBAoCEgA%3D"
    }
}

/// For associations between protobufs and tab name
pub enum Tab{
    Videos,
    Playlists,
    Community
}

