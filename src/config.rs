use crate::database::db_manger::DbManager;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};
use youtubei_rs::types::{client::ClientConfig, query_results::BrowseResult};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub db_host: String,
    pub db_port: u16,
    pub db_keyspace: String,
    pub log_level: String,
    pub output: String,
    pub modified_source_code_url: String,
    pub banner: String,
    pub hmac_key: String,
    pub domain: String,
    pub use_pubsub_feeds: bool,
    pub popular_enabled: bool,
    pub captcha_enabled: bool,
    pub login_enabled: bool,
    pub registration_enabled: bool,
    pub statistics_enabled: bool,
    pub admins: Vec<String>,
    pub external_port: String,
    pub default_user_preferences: Preferences,
    pub dmca_content: bool,
    pub check_tables: String,
    pub hsts: bool,
    pub disable_proxy: bool,
    pub port: String,
    pub host_binding: String,
}

impl Config {
    /// Loads the configuration and parses it into a Config object, panics if the configuration file is invalid or missing.
    pub fn new() -> Config {
        let loaded_config = fs::read_to_string("config.yaml").unwrap();
        serde_yaml::from_str(&serde_yaml::to_string(&loaded_config).unwrap()).unwrap()
    }
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Preferences {
    pub autoplay: bool,
    pub captions: Vec<String>,
    pub comments: Vec<String>,
    pub load_next: bool,
    pub continue_autoplay: bool,
    pub dark_mode: String,
    pub latest_only: bool,
    pub listen: bool,
    pub local: bool,
    pub locale: String,
    pub watch_history: bool,
    pub max_results: i32,
    pub notifications_only: bool,
    pub player_style: String,
    pub quality: String,
    pub quality_dash: String,
    pub default_home: String,
    pub feed_menu: Vec<String>,
    pub automatic_instance_redirect: bool,
    pub region: String,
    pub related_videos: bool,
    pub sort: String,
    pub speed: f32,
    pub thin_mode: bool,
    pub unseen_only: bool,
    pub video_loop: bool,
    pub extend_desc: bool,
    pub volume: i32,
    pub vr_mode: bool,
    pub show_nick: bool,
    pub save_player_pos: bool,
}

/// The state that is shared with all handlers
pub struct State {
    pub yt_client_config: ClientConfig,
    pub db_manager: DbManager,
}
