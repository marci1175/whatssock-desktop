use reqwest::Response;

use crate::HttpClient;

impl HttpClient {
    pub async fn fetch_login(
        &self,
        username: String,
        password: String,
    ) -> anyhow::Result<Response> {
        let response = self
            .client
            .post(format!("{}/api/user", self.base_url))
            .body(format!("{username};{password}"))
            .send()
            .await?;

        Ok(response)
    }
}
