use reqwest::{header::HeaderValue, StatusCode};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{copy, ErrorKind},
    path::Path,
    sync::Arc,
};
use tokio::sync::oneshot;

use crate::{discord::rest_api::discord_endpoints::Channel, runtime};

use super::discord_endpoints::{ApiEndpoints, ApiResponse};

pub async fn download_image(
    url: String,
    path: &Path,
    image_name: String,
) -> Result<(), Box<dyn std::error::Error>> {
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
    if !path.exists() {
        match fs::create_dir_all(path) {
            Ok(_) => println!("Directory created successfully."),
            Err(e) => println!("Failed to create directory: {}", e),
        }
    }
    let mut file = File::create(path.join(image_name)).unwrap();

    // Copy the content from the response to the file
    copy(&mut res.bytes().await?.as_ref(), &mut file)?;

    Ok(())
}

pub fn init_data(token: &String) -> Result<Vec<ApiResponse>, std::io::Error> {
    let token_arc = Arc::new(token.to_owned());

    let (tx, rx) = oneshot::channel();

    runtime().spawn(async move {
        let mut headers = HashMap::new();
        headers.insert("Authorization", token_arc.to_string());

        // let _open_channel = ApiEndpoints::GetChannels(Some("280418177702297605".into()))
        //     .get_req(headers.clone())
        //     .await;
        // ApiEndpoints::GetMessages("", Some("1269922738424905738".into()), 50)
        //     .call(headers.clone())
        //     .await;

        let channels = ApiEndpoints::GetChannels(None)
            .call(headers.clone())
            .await
            .unwrap();

        let friends = ApiEndpoints::FriendList
            .call(headers.clone())
            .await
            .unwrap();

        tx.send(vec![channels, friends]).unwrap();
    });

    Ok(rx.blocking_recv().unwrap())
}

struct BasicData {
    friends: Vec<serde_json::Value>,
    channels: Vec<Channel>,
}
