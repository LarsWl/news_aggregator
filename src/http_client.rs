use reqwest;

pub struct HttpClient();

impl HttpClient {
  pub async fn fetch_get_html(uri: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(uri).await?;

    Ok(resp.text().await?)
  }
}