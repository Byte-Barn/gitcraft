use std::time::Duration;

use anyhow::anyhow;
use reqwest::blocking::Client;

pub struct Fetcher {
    client: Client,
}

impl Fetcher {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .user_agent("gitcraft-fetcher")
                .build()
                .unwrap(),
        }
    }

    /// Fetch raw content from a URL with retry logic
    pub fn fetch_content(&self, url: &str) -> anyhow::Result<String> {
        self.retry(|| self.client.get(url).send())
            .and_then(|response| {
                if !response.status().is_success() {
                    return Err(anyhow!(
                        "Failed to fetch from {}: HTTP {} ({})",
                        url,
                        response.status().as_u16(),
                        response.status().canonical_reason().unwrap_or("Unknown")
                    ));
                }
                Ok(response)
            })
            .and_then(|response| {
                response
                    .text()
                    .map_err(|e| anyhow!("Failed to read response: {}", e))
            })
    }

    /// Fetch and parse JSON from a URL with retry logic
    pub fn fetch_json(&self, url: &str) -> anyhow::Result<serde_json::Value> {
        self.retry(|| self.client.get(url).send())
            .and_then(|response| {
                if !response.status().is_success() {
                    return Err(anyhow!(
                        "JSON request failed with status {}: {}",
                        response.status(),
                        url
                    ));
                }
                Ok(response)
            })
            .and_then(|response| {
                response
                    .json()
                    .map_err(|e| anyhow!("Failed to parse JSON: {}", e))
            })
    }

    /// Retry logic with exponential backoff (max 3 attempts)
    fn retry<F>(&self, mut f: F) -> anyhow::Result<reqwest::blocking::Response>
    where
        F: FnMut() -> Result<reqwest::blocking::Response, reqwest::Error>,
    {
        let mut attempts = 0;
        loop {
            match f() {
                Ok(response) => return Ok(response),
                Err(e) => {
                    attempts += 1;
                    if attempts >= 3 {
                        return Err(anyhow!("Request failed after 3 attempts: {}", e));
                    }
                    std::thread::sleep(Duration::from_millis(100 * (2_u64.pow(attempts - 1))));
                }
            }
        }
    }
}
