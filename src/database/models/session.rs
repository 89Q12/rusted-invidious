use scylla::macros::{IntoUserType,FromUserType,FromRow};
use scylla::cql_to_rust::FromCqlVal;
use scylla::frame::value::Timestamp;

/// Represents a user session stored in the database
#[derive(Debug,IntoUserType, FromUserType,FromRow)]
pub struct UserSession{
    uuid: String, // partition key
    session_id: String, // clustering key
    issued: Timestamp,
}