use reqwest::{StatusCode, Url};

use super::get_url;

pub async fn request(client: &reqwest::Client) -> Result<bool, Box<dyn std::error::Error>> {
  let url: Url = get_url("/health");

  let response = client
    .get(url)
    .send()
    .await?;

  let status: StatusCode = response.status();

  Ok(status == StatusCode::OK)
}
