use reqwest::Client;

pub mod api_requests;
pub mod ui;

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