use scylla::macros::{IntoUserType,FromUserType};
use scylla::cql_to_rust::FromCqlVal;
use scylla::frame::value::Timestamp;

/// Represents a video queried from the database
#[derive(Debug,IntoUserType, FromUserType)]
pub struct Video{
    video_id: String, // Primary Key -> partition key
    updated_at: Timestamp, // Primary Key -> clustering key
    channel_id: String,
    title: String,
    likes: String,
    view_count: i64,
    length_in_seconds: i64,
    description: String,
    genre: String,
    gener_url: String,
    license: String,
    author_verified: bool,
    subcriber_count: String,
    author_name: String,
    author_thumbnail_url: String,
    is_famliy_safe: bool,
    publish_date: Timestamp,
    formats:String, // This string contains json that holds all formats for the video. Could be stored in own table I think
    storyboard_spec_url: String,
}