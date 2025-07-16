use serde::{Deserialize, Serialize};

pub mod auth;

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct UserSession {
    user_id: i32,
    session_token: [u8; 32],
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct FetchChatroomRequest {
    pub user_session: UserSession,
    pub chatroom_id: String,
    pub password: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct FetchChatroomResponse {
    pub chatroom_id: String,
    pub chatroom_name: String,
    /// The reason it is an option is because this is what diesel returns
    pub participants: Vec<Option<i32>>,
    pub is_direct_message: bool,
    pub last_message_id: Option<i32>,
}