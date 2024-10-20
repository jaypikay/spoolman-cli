use crate::config::{read_config, Config};
use reqwest::{Client, Error, Response};
use std::collections::HashMap;
use std::env;

pub struct SpoolmanAPI {
    base_url: String,
}

impl SpoolmanAPI {
    pub fn new() -> SpoolmanAPI {
        let config: Config = read_config().unwrap();
        SpoolmanAPI {
            base_url: config.spoolman.url,
        }
    }

    fn build_url_with_endpoint(&self, endpoint: &str) -> String {
        format!(
            "{base_url}/api/v1/{endpoint}",
            base_url = self.base_url,
            endpoint = endpoint
        )
    }

    fn build_client(&self) -> Result<Client, Error> {
        let http_proxy = match env::var("http_proxy") {
            Ok(proxy) => Some(reqwest::Proxy::http(proxy).unwrap()),
            Err(_) => None,
        };

        let https_proxy = match env::var("https_proxy") {
            Ok(proxy) => Some(reqwest::Proxy::https(proxy).unwrap()),
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

    pub async fn get(&self, endpoint: &str) -> Result<Response, Error> {
        let url = self.build_url_with_endpoint(endpoint);
        #[cfg(debug_assertions)]
        println!("GET  {}", url);

        let client = self.build_client()?;
        let response = client.get(url).send().await?;

        Ok(response)
    }

    pub async fn put<K, V>(&self, endpoint: &str, params: &[(K, V)]) -> Result<Response, Error>
    where
        K: std::cmp::Eq + std::hash::Hash + serde::Serialize,
        V: serde::Serialize,
    {
        let url = self.build_url_with_endpoint(endpoint);
        #[cfg(debug_assertions)]
        println!("PUT  {}", url);

        let mut map = HashMap::new();
        for (key, value) in params {
            map.insert(key, value);
        }

        let client = self.build_client()?;
        let response = client.put(url).json(&map).send().await?;

        Ok(response)
    }
}
