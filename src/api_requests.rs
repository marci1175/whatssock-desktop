use crate::{HttpClient, LoginRequest, RegisterRequest};
use anyhow::ensure;
use reqwest::Response;

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
            .post(format!("{}/api/login", self.base_url))
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&LoginRequest { username, password })?)
            .send()
            .await?;

        let response_code = response.status().as_u16();

        ensure!(response_code == 200, "Response code: {response_code}");

        Ok(response)
    }

    pub async fn send_register_request(
        &self,
        username: String,
        password: String,
        email: String,
    ) -> anyhow::Result<Response> {
        ensure!(!username.is_empty(), "Username must not be empty.");
        ensure!(!password.is_empty(), "Password must not be empty.");
        ensure!(!email.is_empty(), "Email must not be empty.");

        let response = self
            .client
            .post(format!("{}/api/register", self.base_url))
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&RegisterRequest { username, password, email })?)
            .send()
            .await?;

        let response_code = response.status().as_u16();
        
        ensure!(response_code == 200, "Response code: {response_code}");

        Ok(response)
    }
}
