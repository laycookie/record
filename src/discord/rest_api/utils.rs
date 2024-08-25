use reqwest::{header::HeaderValue, StatusCode};
use std::{collections::HashMap, fs::{self, File}, io::{copy, ErrorKind}, io, path::Path, sync::Arc};
use std::error::Error;

use tokio::sync::{oneshot};

use crate::{discord::rest_api::discord_endpoints::Channel, get_tokens, runtime};

use super::discord_endpoints::{ApiEndpoints, ApiResponse, AuthedUser};

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

pub fn init_data(token: &String) -> Result<Vec<ApiResponse>, io::Error> {
    let token_arc = Arc::new(token.to_owned());
    let (tx, rx) = oneshot::channel();

    runtime().spawn(async move {
        let mut headers = HashMap::new();
        headers.insert("Authorization", token_arc.to_string());
        let channels = match ApiEndpoints::GetChannels(None).call(headers.clone()).await {
            Ok(channel) => channel,
            Err(_) => {
                tx.send(Err(io::Error::new(ErrorKind::NotFound, "Error in finding channels of a user"))).unwrap();
                return;
            }
        };
        let friends = ApiEndpoints::FriendList
            .call(headers.clone())
            .await
            .unwrap();
        let guilds = ApiEndpoints::GetGuilds
            .call(headers)
            .await
            .unwrap();

        tx.send(Ok(vec![channels, friends, guilds])).unwrap();
    });

    rx.blocking_recv().unwrap()
}
//Temporary function, we will remove it once we have a proper way to make non-blocking API requests.
pub(crate) fn get_discord_user_info() -> AuthedUser {
    let (tx, rx) = oneshot::channel();
    runtime().spawn(async move {
        let mut headers = HashMap::new();
        headers.insert("Authorization", get_tokens().unwrap().discord_token.unwrap());

        let messages = ApiEndpoints::GetUser.call(headers).await.unwrap();
        tx.send(messages).unwrap();
    });
    if let ApiResponse::User(user) = rx.blocking_recv().unwrap() {
        user
    } else {
        panic!("User data not found.")
    }
}
struct BasicData {
    friends: Vec<serde_json::Value>,
    channels: Vec<Channel>,
}


