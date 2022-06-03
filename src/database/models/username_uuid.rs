use scylla::macros::{IntoUserType,FromUserType};
use scylla::cql_to_rust::FromCqlVal;
use scylla::frame::value::Timestamp;

/// Used to query the uuid of a user by name.
#[derive(Debug,IntoUserType, FromUserType)]
pub struct UsernameUuid{
    name: String, // Primary key
    uuid: String,
}