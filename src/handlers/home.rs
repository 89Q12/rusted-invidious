use axum::response::Redirect;
/// Handler for the root path.
/// Ideally this should be redirect the user to the configured home path
/// e.g. /feed/popular
pub async fn index()-> Redirect {
    // TODO: implement
Redirect::temporary("/feed/popular")
}