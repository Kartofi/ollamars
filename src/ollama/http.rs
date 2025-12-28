use reqwest::{Client, StatusCode};

/// Checks if url loads and returns code 200
pub async fn check_url(url: &str) -> bool {
    let client = Client::new();

let req = client.get(url).send().await;
    match req {
        Ok(res) => res.status() == StatusCode::OK,
        Err(_) => false,
    } }
