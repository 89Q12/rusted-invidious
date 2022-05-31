use axum::{
    routing::{get, post},
    Router,
};
fn channel_routes() -> Router{
    Router::new()
    .route("/:ucid", get(|| async {}))
    .route("/:ucid/home", get(|| async {}))
    .route("/:ucid/videos", get(|| async {}))
    .route("/:ucid/playlists", get(|| async {}))
    .route("/:ucid/community", get(|| async {}))
    .route("/:ucid/about", get(|| async {}))
    .route("/:ucid/live", get(|| async{}))
}
fn user_and_c_routes() -> Router{
    Router::new()
    .route("/:user/videos", get(|| async {}))
    .route("/:user/playlists", get(|| async {}))
    .route("/:user/community", get(|| async {}))
    .route("/:user/about", get(|| async {}))
}

pub fn get_router() -> Router{
    Router::new()
    .route("/", get(|| async {}))
    .nest("/channel", channel_routes())
    .nest("/c", user_and_c_routes())

    .nest("/user", user_and_c_routes())

    .route("/attribution_link", get(|| async {}))

    .route("/watch", get(|| async {}))
    .route("/watch/:id", get(|| async {}))
    .route("/watch/:id", get(|| async {}))
    .route("/shorts/:id", get(|| async {}))
    .route("/clip/:clip", get(|| async {}))
    .route("/w/:id", get(|| async {}))
    .route("/v/:id", get(|| async {}))
    .route("/e/:id", get(|| async {}))

    .route("search", get(|| async {}))
    .route("results", get(|| async {}))


}