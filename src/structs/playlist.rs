use youtubei_rs::types::{enums::PlaylistVideoListContent, misc::{SimpleText, Runs, RunsOption, RunsSimpleTextAccessibility}, endpoints::{WatchEndpoint, BrowseEndpoint}};

pub struct Playlist {
    pub contents: Vec<PlaylistVideoListContent>,
    pub playlist_id: String,
    pub title: SimpleText,
    pub play_endpoint: WatchEndpoint,
    pub num_videos_text: Runs,
    pub description_text: RunsOption,
    pub owner_text: Runs,
    pub view_count_text: SimpleText,
    pub privacy: String,
    pub owner_endpoint: BrowseEndpoint,
    pub stats: Vec<RunsSimpleTextAccessibility>,
    pub brief_stats: Vec<Runs>,
    
}