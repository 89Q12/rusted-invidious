use tracing_subscriber::EnvFilter;
/// Constructs the filter to log specific events/spans 
/// https://docs.rs/tracing-subscriber/0.3.11/tracing_subscriber/struct.EnvFilter.html
/// TODO: implement env variables instead of predefined target levels
pub fn construct_logging_filter() -> EnvFilter{
    EnvFilter::new( "warn,tokio::net=debug,tower_http=debug,db=info,server=trace,youtubei_rs=trace")
}