use axum::{
    routing::{get, post},
    Router,
};
// Imports of the handlers
use crate::handlers::{home,feed,channel, video};

/// Short hand for nested /channel routes
fn channel_routes() -> Router{
    Router::new()
    .route("/:ucid", get(channel::index))
    .route("/:ucid/home", get(channel::index))
    .route("/:ucid/videos", get(channel::videos))
    .route("/:ucid/playlists", get(channel::playlists))
    .route("/:ucid/community", get(channel::community))
    .route("/:ucid/about", get(channel::index))
    .route("/:ucid/live", get(channel::live))
}
/// Short hand for nested /c and /user routes
fn user_and_c_routes() -> Router{
    Router::new()
    .route("/:user",get(channel::index))
    .route("/:user/home",get(channel::index))
    .route("/:user/videos",get(channel::videos))
    .route("/:user/playlists",get(channel::playlists))
    .route("/:user/community",get(channel::community))
    .route("/:user/about",get(channel::index))
    .route("/:user/live",get(channel::live))
}
/// Create the main router struct used for all routes
pub fn get_router() -> Router{
    Router::new()
    .route("/", get(home::index))
    .nest("/channel", channel_routes())

    .nest("/c", user_and_c_routes())
    .nest("/user", user_and_c_routes())

    .route("/attribution_link", get(channel::attribution_link))
    
    .route("/watch", get(video::watch_v))
    .route("/watch_ajax", post(video::watch_v))
    .route("/watch/:id", get(video::redirect))
    .route("/shorts/:id", get(video::redirect))
    .route("/clip/:clip", get(video::clip))
    .route("/w/:id", get(video::redirect))
    .route("/v/:id", get(video::redirect))
    .route("/e/:id", get(video::redirect))
    
    .route("/search", get(|| async {}))
    .route("/results", get(|| async {}))


}