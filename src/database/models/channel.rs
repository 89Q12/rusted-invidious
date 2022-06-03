use scylla::macros::{IntoUserType,FromUserType};
use scylla::cql_to_rust::FromCqlVal;
use scylla::frame::value::Timestamp;

/// Represents a channel queried from the database
#[derive(Debug,IntoUserType, FromUserType)]
pub struct Channel{
    channel_id: String, // partition key
    updated_at: Timestamp, // cluster key
    subcribed_at: Timestamp,
    author_name: String,

}