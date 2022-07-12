use axum::Extension;
use rusted_invidious_types::database::db_manager::DbManager;
use rusted_invidious::{routes::get_router, config::{Config, State, Preferences}, logging::filters::construct_logging_filter};
use youtubei_rs::utils::default_client_config;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::trace::TraceLayer;
use opentelemetry::global;
use std::net::SocketAddr;
use tokio::sync::Mutex;
use std::sync::Arc;
/// Main entry point for the web server:
/// it setups logging, loads the configuration, inits the database api and starts the server
#[tokio::main]
async fn main() {
    // TODO: implement configuration via env file
    let config = Config::new();
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = opentelemetry_jaeger::new_pipeline().install_simple().unwrap();

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
    .with(construct_logging_filter())
    .with(telemetry)
    .with(tracing_subscriber::fmt::layer())
    .init();

    let mut db_manager = DbManager::new("172.17.0.1:19042", None).await.unwrap();
    
    //db_manager.drop_database().await.unwrap();
    db_manager.init_database().await.unwrap();
    db_manager.use_keyspace().await;
    db_manager.init_prepared_statements().await;
    // Needs to be wrapped in a mutex since we edit values is the state.
    let shared_state = Arc::new(Mutex::new(State{
        yt_client_config:default_client_config(),
        db_manager,
        config,
        preferences: Preferences::new()
    }));


    let app = get_router().layer(TraceLayer::new_for_http()).layer(Extension(shared_state));
    
    let addr = SocketAddr::from(([172, 17, 0, 1], 3000));

    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
     global::shutdown_tracer_provider();
}
