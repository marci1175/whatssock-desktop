use crate::HttpClient;
use anyhow::ensure;
use reqwest::Response;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

impl HttpClient {
    pub async fn fetch_login(
        &self,
        username: String,
        password: String,
    ) -> anyhow::Result<Response> {
        ensure!(!username.is_empty(), "Username must not be empty.");
        ensure!(!password.is_empty(), "Password must not be empty.");

        let response = self
            .client
            .get(format!("{}/api/user", self.base_url))
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&LoginRequest { username, password })?)
            .send()
            .await?;

        Ok(response)
    }
}
