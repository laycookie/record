use std::collections::HashMap;
use std::sync::Arc;
use reqwest::header::{HeaderMap, HeaderValue};

pub async fn discord_api_call(url: &str, headers: HashMap<&str, Arc<String>>) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut request = client.get(url);
    for (key, value) in headers
    {
        request = request.header(key, HeaderValue::from_str(value.as_str()).unwrap());
    }
    let response = request.send().await?;
    println!("Status: {}", response.status());
    let response_text: String = response.text().await?;
    let response_text: serde_json::Value = serde_json::from_str(response_text.as_str())?;
    Ok(response_text)
}