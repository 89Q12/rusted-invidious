use scylla::macros::{IntoUserType,FromUserType};
use scylla::cql_to_rust::FromCqlVal;

/// Represents a user queried from the database
#[derive(Debug,IntoUserType, FromUserType)]
pub struct User {
    uuid: String, // Primary key
    name: String,
    password: String,
    token: String,
    feed_needs_update: bool,
}