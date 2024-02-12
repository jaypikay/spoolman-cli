const BASE_URL: &str = "https://spools.lan.dingsi.net/api/v1";

use reqwest::{Error, Response};

pub async fn get(endpoint: &str) -> Result<Response, Error> {
    let url = format!(
        "{base_url}/{endpoint}",
        base_url = BASE_URL,
        endpoint = endpoint
    );
    println!("{}", url);

    let response = reqwest::get(url).await?;

    return Ok(response);
}
