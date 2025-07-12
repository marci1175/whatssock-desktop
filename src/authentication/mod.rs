use serde::{Deserialize, Serialize};

pub mod auth;

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct UserSession {
    user_id: i32,
    session_token: [u8; 32],
}
