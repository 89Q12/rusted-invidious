use axum::{Extension, http::HeaderValue};
use reqwest::Method;
use rusted_invidious_types::database::db_manager::DbManager;
use rusted_invidious::{routes::get_router, config::{Config, State, Preferences}, logging::filters::construct_logging_filter};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::{trace::TraceLayer, cors::Any};
use std::net::SocketAddr;
use tokio::sync::RwLock;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
/// Main entry point for the web server:
/// it setups logging, loads the configuration, inits the database api and starts the server
#[tokio::main]
async fn main() {
    // TODO: implement configuration via env file
    let config = Config::new();
    tracing_subscriber::registry()
    .with(construct_logging_filter())
    .with(tracing_subscriber::fmt::layer())
    .init();

    let mut db_manager = DbManager::new("172.17.0.1:19042", None).await.unwrap();
    
    //db_manager.drop_database().await.unwrap();
    db_manager.init_database().await.unwrap();
    db_manager.use_keyspace().await;
    db_manager.init_prepared_statements().await;
    // Needs to be wrapped in a mutex since we edit values is the state.
    let shared_state = Arc::new(RwLock::new(State{
        db_manager,
        config,
        preferences: Preferences::new()
    }));


    let app = get_router().layer(CorsLayer::new()
    .allow_origin("http://localhost".parse::<HeaderValue>().unwrap()).allow_origin(Any).allow_methods(Any).allow_headers(Any)).layer(TraceLayer::new_for_http()).layer(Extension(shared_state));
    
    let addr = SocketAddr::from(([172, 17, 0, 1], 3000));

    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}
