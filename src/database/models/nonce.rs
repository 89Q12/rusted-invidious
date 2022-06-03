use scylla::macros::{IntoUserType,FromUserType};
use scylla::cql_to_rust::FromCqlVal;
use scylla::frame::value::Timestamp;

/// This represents a token to verify mitigate replyattacks
/// it associates a session to a token that can be verified that the request is legit
#[derive(Debug,IntoUserType, FromUserType)]
pub struct NonceToken{
    nonce: String,
    expires: Timestamp,
}