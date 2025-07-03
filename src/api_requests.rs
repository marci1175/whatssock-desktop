use reqwest::Response;

pub async fn fetch_login(client: reqwest::Client, username: String, password: String) -> anyhow::Result<Response> {
    let response = client.post("[::1]/api/login").body(format!("{username};{password}")).send().await?;

    Ok(response)
}