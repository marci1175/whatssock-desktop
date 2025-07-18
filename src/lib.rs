use std::{fs, path::PathBuf, sync::LazyLock};

use dioxus::prelude::Routable;
use dioxus::prelude::*;
use dirs::{data_dir, data_local_dir};
use reqwest::Client;
pub mod api_requests;
pub mod authentication;
pub mod ui;
use crate::{
    authentication::UserSession,
    ui::{login::Login, main_page::MainPage, not_found::NotFound, register::Register},
};

#[derive(Debug, Clone)]
pub struct HttpClient {
    pub client: Client,
    pub base_url: String,
}

impl HttpClient {
    pub fn new(client: Client, base_url: String) -> Self {
        Self { client, base_url }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct RegisterRequest {
    username: String,
    password: String,
    email: String,
}

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[route("/")]
    Login {},
    #[route("/register")]
    Register {},
    #[route("/chats")]
    MainPage {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

pub static COOKIE_SAVE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut cookie_save_path = data_local_dir().unwrap();

    cookie_save_path.push("/.whatssock");

    if !fs::exists(&cookie_save_path).unwrap_or_default() {
        fs::create_dir_all(&cookie_save_path).unwrap_or_default();
    }

    cookie_save_path.push("user_session");

    cookie_save_path
});

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct UserInformation {
    pub username: String,
    pub chatrooms_joined: Vec<Option<i32>>,
}
