use std::{collections::HashMap, error::Error, io::ErrorKind};

use reqwest::{header::HeaderValue, StatusCode};
use serde::Deserialize;
use serde_json::Value;

pub enum ApiEndpoints {
    FriendList,
    DiscordGateway,
    GetChannels(Option<String>), // user ID
    GetMessages(String, String), // Channel ID, Message Limit
}

impl ApiEndpoints {
    pub fn get_url(&self) -> String {
        match self {
            Self::FriendList => "https://discord.com/api/v9/users/@me/relationships".into(),
            Self::GetChannels(_) => "https://discord.com/api/v10/users/@me/channels".into(),
            Self::GetMessages(channel_id, message_limit) => format!(
                "https://discord.com/api/v9/channels/{}/messages?limit={}",
                channel_id, message_limit
            ),
        }
    }

    pub async fn get_req(&self, headers: HashMap<&str, String>) {
        let client = reqwest::Client::new();
        let mut request = client.get(self.get_url());

        for (key, value) in headers {
            request = request.header(key, HeaderValue::from_str(value.as_str()).unwrap());
        }

        // match self {
        //     ApiEndpoints::GetChannels(user_id) => {
        //         if let Some(user_id) = user_id {
        //             println!("{{\"recipients\":[\"{}\"]}}", user_id.clone());
        //             request = request.body(format!("{{\"recipients\":[\"{}\"]}}", user_id.clone()));
        //             println!("test");
        //         }
        //     }
        //     ApiEndpoints::FriendList => {}
        //     _ => {}
        // }

        let response = request.send().await.unwrap();
        let response_text: String = response.text().await.unwrap();

        println!("{:#?}", response_text);
    }

    pub async fn call(
        &self,
        headers: HashMap<&str, String>,
    ) -> Result<ApiResponse, Box<dyn Error>> {
        let client = reqwest::Client::new();

        let mut request = client.get(self.get_url());
        for (key, value) in headers {
            request = request.header(key, HeaderValue::from_str(value.as_str()).unwrap());
        }

        let response = request.send().await?;
        let response_status = response.status();

        if response_status != StatusCode::OK {
            return Err(Box::new(std::io::Error::new(ErrorKind::Other, "stuff")));
        }

        let response_text: String = response.text().await?;

        Ok(match self {
            Self::FriendList => {
                let a = serde_json::from_str::<Value>(response_text.as_str()).unwrap();
                let a = a.as_array().unwrap();
                let a: Vec<Friend> = a
                    .iter()
                    .map(|e| serde_json::from_value(e.clone()).unwrap())
                    .collect();

                ApiResponse::Friends(a)
            }
            Self::DiscordGateway => todo!(),
            Self::GetChannels(_) => {
                let a = serde_json::from_str::<Value>(response_text.as_str()).unwrap();
                let a = a.as_array().unwrap();
                let a: Vec<Channel> = a
                    .iter()
                    .map(|e| serde_json::from_value(e.clone()).unwrap())
                    .collect();
                ApiResponse::Channels(a)
            }
            Self::GetMessages(_, _) => todo!(),
        })
    }
}

trait ApiRes: Sized + for<'a> Deserialize<'a> {
    fn get_url() -> String;

    fn get_request_builder() -> reqwest::RequestBuilder {
        let client = reqwest::Client::new();
        client.get(Self::get_url())
    }

    async fn gen_new(&self, headers: HashMap<&str, String>) -> Result<Self, Box<dyn Error>> {
        let mut request = Self::get_request_builder();

        for (key, value) in headers {
            request = request.header(key, HeaderValue::from_str(value.as_str()).unwrap());
        }

        let response = request.send().await?;
        let response_status = response.status();

        if response_status != StatusCode::OK {
            return Err(Box::new(std::io::Error::new(ErrorKind::Other, "stuff")));
        }

        let response_text: String = response.text().await?;

        let a = serde_json::from_str::<Self>(response_text.as_str()).unwrap();
        Ok(a)
    }
}

#[derive(Debug)]
pub enum ApiResponse {
    Friends(Vec<Friend>),
    Channels(Vec<Channel>),
}

#[derive(Deserialize, Debug)]
pub struct Friend {
    pub id: String,
    pub nickname: Option<String>,
    pub since: String,
    // pub type: i32,
    pub user: User,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub avatar: String,
    pub avatar_decoration_data: Option<String>,
    pub clan: Option<String>,
    pub discriminator: String,
    pub id: String,
    pub username: String,
}

#[derive(Deserialize, Debug)]
pub struct Channel {
    pub flags: i32,
    pub icon: Option<String>,
    pub id: String,
    pub last_message_id: String,
    pub name: Option<String>,
    pub recipients: Vec<Recipient>,
}

#[derive(Deserialize, Debug)]
pub struct Recipient {
    pub avatar: String,
    pub avatar_decoration_data: Option<String>,
    pub clan: Option<String>,
    pub discriminator: String,
    pub global_name: Option<String>,
    pub id: String,
    pub public_flags: i32,
    pub username: String,
}
