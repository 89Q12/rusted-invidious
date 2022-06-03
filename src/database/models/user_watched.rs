use scylla::macros::{IntoUserType,FromUserType};
use scylla::cql_to_rust::FromCqlVal;
/// Represents a video that a user watched
#[derive(Debug,IntoUserType, FromUserType)]
pub struct UserWatched{
    uuid: String, // partition key I guess
    video_id: String,// clustering key
}