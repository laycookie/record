use reqwest::{header::HeaderValue, StatusCode};
use std::{
    collections::HashMap,
    fs::File,
    io::{copy, ErrorKind},
    path::Path,
    sync::Arc,
};
use tokio::sync::oneshot;

use crate::runtime;

use super::discord_endpoints::ApiEndpoints;

pub async fn discord_api_call(
    endpoint: ApiEndpoints,
    headers: HashMap<&str, String>,
) -> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let mut request = client.get(endpoint.get_url());
    for (key, value) in headers {
        request = request.header(key, HeaderValue::from_str(value.as_str()).unwrap());
    }

    let response = request.send().await?;
    let response_status = response.status();
    let response_text: String = response.text().await?;
    let response_text: serde_json::Value = serde_json::from_str(response_text.as_str())?;

    Ok(Response {
        header: response_status,
        body: response_text,
    })
}

pub struct Response {
    header: StatusCode,
    body: serde_json::Value,
}

impl Response {
    pub fn is_sucessful(&self) -> bool {
        self.header == StatusCode::OK
    }
}

pub async fn download_image(url: String, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    // Send HTTP GET request to the specified URL
    let req = client.get(url);
    let res = req.send().await?;

    // Check if the request was successful
    if !res.status().is_success() {
        return Err(format!("Failed to download image: HTTP {}", res.status()).into());
    }

    // Create a file at the specified path
    println!("{:#?}", path);
    let mut file = File::create(path).unwrap();

    // Copy the content from the response to the file
    copy(&mut res.bytes().await?.as_ref(), &mut file)?;

    Ok(())
}

pub fn init_data(token: &String) -> Result<serde_json::Value, std::io::Error> {
    let token_arc = Arc::new(token.to_owned());

    let (tx, rx) = oneshot::channel();

    runtime().spawn(async move {
        let mut headers = HashMap::new();
        headers.insert("Authorization", token_arc.to_string());
        let res = discord_api_call(ApiEndpoints::FriendList, headers)
            .await
            .expect("Discord API failed to process request to validate the token");

        if res.is_sucessful() {
            tx.send(Ok(res.body)).unwrap();
        } else {
            println!("{:#?}", res.body);
            tx.send(Err(std::io::Error::new(ErrorKind::Other, "stuff")))
                .unwrap();
        }
    });

    rx.blocking_recv().unwrap()
}

struct BasicData {
    friends: Vec<serde_json::Value>,
}
