use scylla::macros::{IntoUserType,FromUserType,FromRow};
use scylla::cql_to_rust::FromCqlVal;

/// Used to query the uuid of a user by name.
#[derive(Debug,IntoUserType, FromUserType,FromRow)]
pub struct UsernameUuid{
    name: String, // Primary key
    uuid: String,
}