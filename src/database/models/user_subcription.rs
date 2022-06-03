use scylla::macros::{IntoUserType,FromUserType};
use scylla::cql_to_rust::FromCqlVal;
use scylla::frame::value::Timestamp;

/// Represents a channel that a user has subcribed to 
#[derive(Debug,IntoUserType, FromUserType)]
pub struct UserSubscribed {
    uuid: String, // partition key
    channel_id: String, // clustering key
}