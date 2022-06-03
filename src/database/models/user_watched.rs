use scylla::macros::{IntoUserType,FromUserType};
use scylla::cql_to_rust::FromCqlVal;
use scylla::frame::value::Timestamp;

/// Represents a video that a user watched
#[derive(Debug,IntoUserType, FromUserType)]
pub struct UserWatched{
    uuid: String, // partition key I guess
    video_id: String,// clustering key
}