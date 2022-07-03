use std::{sync::Arc, collections::HashMap, fmt::Debug};
use axum::{response::{Response, Redirect, IntoResponse, Html}, Extension,extract::{path::Path, Query}, http::StatusCode};
use tokio::sync::Mutex;
use youtubei_rs::query::{player, next_video_id, resolve};
use crate::config::State;

/// Handler for the /watch?v=id path and renders the watch page
pub async fn watch_v(Extension(state): Extension<Arc<Mutex<State>>>,Query(params): Query<HashMap<String, String>>) -> String {
    todo!()
}

/// Handler for the paths /w/:id, /v/:id,/e/:id, /shorts/:id and /watch/:id it redirects to the /watch/v=id path
pub async fn redirect(Path(id): Path<String>) -> Redirect{
    Redirect::temporary(&("/watch?v=".to_owned()+ &id))
}

/// Handler for the path /watch_ajax which is used to mark a video as watched if the user is logged in
/// Protected route
pub async fn watch_ajax(Extension(state): Extension<Arc<Mutex<State>>>,body: String){
    todo!()
}

/// Handler for the paths /clip/:id it extracts the videoId from the clip id and redirects to the /watch?v=id path
pub async fn clip(Extension(state): Extension<Arc<Mutex<State>>>,Path(id): Path<String>) -> Redirect{
    let lock = state.lock().await;
    let video_id = resolve("https://www.youtube.com/clip/".to_owned()+ &id, &lock.yt_client_config).await;
    match video_id{
        Ok(vid) => Redirect::temporary(&("/watch?v=".to_owned() +&vid.endpoint.watch_endpoint.unwrap().video_id)),
        Err(_) => Redirect::temporary(&("/watch?v=".to_owned() + &id)),
    }
   
}
