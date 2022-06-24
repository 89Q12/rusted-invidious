use scylla::macros::{IntoUserType,FromUserType,FromRow};
use scylla::cql_to_rust::FromCqlVal;
use scylla::frame::value::Timestamp;

/// Represents a video queried from the database
#[derive(Debug,IntoUserType, FromUserType,FromRow)]
pub struct Video{
    pub video_id: String, // Primary Key -> partition key
    pub updated_at: Timestamp,
    pub channel_id: String,
    pub title: String,
    pub likes: String,
    pub view_count: i64,
    pub length_in_seconds: i64,
    pub description: String,
    pub genre: String,
    pub genre_url: String,
    pub license: String,
    pub author_verified: bool,
    pub subcriber_count: String,
    pub author_name: String,
    pub author_thumbnail_url: String,
    pub is_famliy_safe: bool,
    pub publish_date: String,
    pub formats:String, // This string contains json that holds all formats for the video. Could be stored in own table I think
    pub storyboard_spec_url: String,
    pub continuation_comments: Option<String>,
    pub continuation_related: Option<String>,
}