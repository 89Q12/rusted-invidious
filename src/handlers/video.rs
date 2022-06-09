use std::{sync::Arc, collections::HashMap, fmt::Debug};
use axum::{response::{Response, Redirect, IntoResponse}, Extension,extract::{path::Path, Query}, http::StatusCode};
use tokio::sync::Mutex;
use youtubei_rs::query::{player, next_video_id, resolve};
use crate::config::State;

/// Handler for the /watch?v=id path and renders the watch page
pub async fn watch_v(Extension(state): Extension<Arc<Mutex<State>>>,Query(params): Query<HashMap<String, String>>) -> Result<String, String> {
    let video_id = params.get("v").unwrap();
    let lock = state.lock().await;
    let (player, next) = tokio::join!(player(video_id.to_owned(), "".to_string(), &lock.yt_client_config), next_video_id(video_id.to_string(), "".to_string(), &lock.yt_client_config));
    match player{
        Ok(pl) => Ok(pl.video_details.title),
        Err(e) => match e{
            youtubei_rs::types::error::Errors::RequestError(err) =>Err(err.message) ,
            youtubei_rs::types::error::Errors::ParseError(err) => Err(err.message),
        },
    }
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

/// Handler for the paths /clip/:id it extracts the videoId from the clip id and redirects to the /watch/v=id path
pub async fn clip(Extension(state): Extension<Arc<Mutex<State>>>,Path(id): Path<String>) -> Redirect{
    let lock = state.lock().await;
    let video_id = resolve("https://www.youtube.com/clip/".to_owned()+ &id, &lock.yt_client_config).await;
    match video_id{
        Ok(vid) => Redirect::temporary(&("/watch?v=".to_owned() +&vid.endpoint.watch_endpoint.unwrap().video_id)),
        Err(_) => Redirect::temporary(&("/watch?v=".to_owned() + &id)),
    }
   
}
