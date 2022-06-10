use scylla::macros::{IntoUserType,FromUserType,FromRow};
use scylla::cql_to_rust::FromCqlVal;
use scylla::frame::value::Timestamp;

/// Represents a video queried from the database
#[derive(Debug,IntoUserType, FromUserType,FromRow)]
pub struct ChannelVideo{
    channel_id: String,// Primary Key -> partition key
    video_id: String, // Primary Key -> clustering key
    updated_at: Timestamp, 
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
    premiere_timestamp: Timestamp,
    live: bool,
}