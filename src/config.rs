use crate::database::db_manger::DbManager;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use youtubei_rs::types::{client::ClientConfig, query_results::BrowseResult};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub db_host: String,
    pub db_port: u16,
    pub log_level: String,
    pub output: String,
    pub modified_source_code_url: String,
    pub banner: String,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Preferences {
    pub annotations: bool,
    pub autoplay: bool,
    pub captions: Vec<String>,
    pub comments: Vec<String>,
    pub continue_what_needs_work: bool,
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
    pub channel_map: HashMap<String, BrowseResult>,
}
