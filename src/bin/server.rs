use envconfig::Envconfig;
use rusted_invidious::{routes::get_router, config::Config, logging::filters::construct_logging_filter};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::trace::TraceLayer;
use std::net::SocketAddr;

/// Main entry point for the web server:
/// it setups logging, loads the configuration, inits the database api and starts the server
#[tokio::main]
pub async fn main() {
    // TODO: implement configuration via env file
    //let config = Config::init_from_env().ok();

    tracing_subscriber::registry()
    .with(construct_logging_filter())
    .with(tracing_subscriber::fmt::layer())
    .init();
    let app = get_router().layer(TraceLayer::new_for_http());
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}
