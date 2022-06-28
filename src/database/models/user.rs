use scylla::macros::{IntoUserType,FromUserType, FromRow};
use scylla::cql_to_rust::FromCqlVal;

/// Represents a user queried from the database
#[derive(Debug,IntoUserType, FromUserType,FromRow)]
pub struct DBUser {
    uuid: String, // partition key
    name: String, // clustering key
    password: String,
    token: String,
    feed_needs_update: bool,
}

impl DBUser {
    pub fn is_authenticated(&self) -> bool {
        self.name.is_empty()
    }
}