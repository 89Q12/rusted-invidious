use std::{sync::{Arc, Mutex}, collections::HashMap};
use axum::{response::{Response, Redirect}, Extension,extract::{path::Path, Query}};
use crate::config::State;

/// Handler for the /watch?v=id path and renders the watch page
pub async fn watch_v(Extension(state): Extension<Arc<Mutex<State>>>,Query(params): Query<HashMap<String, String>>){
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

/// Handler for the paths /clip/:id it extracts the videoId from the clip id and redirects to the /watch/v=id path
pub async fn clip(Extension(state): Extension<Arc<Mutex<State>>>,Path(id): Path<String>) -> Redirect{
    todo!()
}
