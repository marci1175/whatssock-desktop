use dioxus::prelude::Routable;
use reqwest::Client;
use dioxus::prelude::*;
pub mod api_requests;
pub mod ui;
pub mod authentication;
use crate::{authentication::UserSession, ui::{login::Login, main_page::MainPage, not_found::NotFound, register::Register}};

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
    MainPage { },
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}