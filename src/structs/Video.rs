use youtubei_rs::types::{video::CompactVideoRenderer, misc::{Format, StreamingData}};

pub struct Video{
    pub thumbnail: String,
    pub id: String,
    pub keywords: Vec<String>,
    pub short_description: String,
    pub title: String,
    pub is_listed: bool,
    pub reason: Option<String>,
    pub premiere_timestamp: Option<String>,
    pub live_now: bool,
    pub views: String,
    pub likes: String,
    pub genre_url: Option<String>,
    pub genre: Option<String>,
    pub license: Option<String>,
    pub ucid: String,
    pub related_videos: Vec<CompactVideoRenderer>,
    pub allowed_regions: Vec<String>,
    pub author_thumbnail: String,
    pub author_id: String,
    pub author: String,
    pub author_verified: bool,
    pub sub_count_text: String,
    pub description: String,
    pub published: String,
    pub length_seconds: String,
    pub engagement: String,
    pub wilson_score: String,
    pub rating: String,
    pub is_family_friendly: bool,
    pub projection_type: String,
    pub is_vr: bool,
    pub comments_count: String,
}