use reqwest::{Client, Error, Response};
use std::collections::HashMap;
use std::env;

fn build_url_with_endpoint(endpoint: &str) -> String {
    let base_url = match env::var("SPOOLMAN_URL") {
        Ok(url) => format!("{}/api/v1", url),
        Err(_) => {
            eprintln!("Environment variable SPOOLMAN_URL not set.\nUsing default SPOOLMAN_URL=http://localhost:8000");
            "http://localhost:8000".to_string()
        }
    };

    format!(
        "{base_url}/{endpoint}",
        base_url = base_url,
        endpoint = endpoint
    )
}

fn build_client() -> Result<Client, Error> {
    let http_proxy = match env::var("http_proxy") {
        Ok(proxy) => Some(reqwest::Proxy::http(&proxy).unwrap()),
        Err(_) => None,
    };

    let https_proxy = match env::var("https_proxy") {
        Ok(proxy) => Some(reqwest::Proxy::https(&proxy).unwrap()),
        Err(_) => None,
    };

    let mut builder = reqwest::Client::builder();
    if let Some(proxy) = http_proxy {
        builder = builder.proxy(proxy);
    }
    if let Some(proxy) = https_proxy {
        builder = builder.proxy(proxy);
    }
    let client = builder.build()?;

    Ok(client)
}

pub async fn get(endpoint: &str) -> Result<Response, Error> {
    let url = build_url_with_endpoint(endpoint);
    #[cfg(debug_assertions)]
    println!("GET  {}", url);

    let client = build_client()?;
    let response = client.get(url).send().await?;

    return Ok(response);
}

pub async fn put<K, V>(endpoint: &str, params: &[(K, V)]) -> Result<Response, Error>
where
    K: std::cmp::Eq + std::hash::Hash + serde::Serialize,
    V: serde::Serialize,
{
    let url = build_url_with_endpoint(endpoint);
    #[cfg(debug_assertions)]
    println!("PUT  {}", url);

    let mut map = HashMap::new();
    for (key, value) in params {
        map.insert(key, value);
    }

    let client = build_client()?;
    let response = client.put(url).json(&map).send().await?;

    return Ok(response);
}
