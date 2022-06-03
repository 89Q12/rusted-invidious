use envconfig::Envconfig;
use youtubei_rs::types::client::ClientConfig;
#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "DB_HOST")]
    pub db_host: String,
    #[envconfig(from = "DB_PORT", default = "5432")]
    pub db_port: u16,
    #[envconfig(from = "LOG_LEVEL", default = "debug")]
    pub log_level: String,
    #[envconfig(from = "OUTPUT_FILE", default = "stdout")]
    pub output: String
}

/// The state that is shared with all handlers
pub struct  State{
    pub yt_client_config: ClientConfig,
    
}