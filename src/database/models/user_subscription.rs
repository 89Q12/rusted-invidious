use scylla::macros::{IntoUserType,FromUserType,FromRow};
use scylla::cql_to_rust::FromCqlVal;
/// Represents a channel that a user has subcribed to 
#[derive(Debug,IntoUserType, FromUserType,FromRow)]
pub struct UserSubscribed {
    uuid: String, // partition key
    channel_id: String,
}