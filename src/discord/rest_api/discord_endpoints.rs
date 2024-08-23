use std::{collections::HashMap, error::Error, io::ErrorKind};
use reqwest::{header::HeaderValue, StatusCode};
use serde::Deserialize;


pub const DISCORD_GATEWAY: &str = "wss://gateway.discord.gg/?v=10&encoding=json";

pub enum ApiEndpoints {
    FriendList,
    GetChannels(Option<String>),              // user ID
    GetMessages(String, Option<String>, u32), // Channel ID, Load before message, Message Limit
    GetUser,
}

impl ApiEndpoints {
    pub fn get_url(&self) -> String {
        match self {
            Self::FriendList => "https://discord.com/api/v9/users/@me/relationships".into(),
            Self::GetChannels(_) => "https://discord.com/api/v10/users/@me/channels".into(),
            Self::GetMessages(channel_id, before_message, message_limit) => {
                let before = match before_message {
                    Some(before_message) => format!("before={}&", before_message),
                    None => "".into(),
                };

                format!(
                    "https://discord.com/api/v9/channels/{}/messages?{}limit={}",
                    channel_id, before, message_limit
                )
            }
            Self::GetUser => "https://discord.com/api/v9/users/@me".into(),
        }
    }

    pub async fn call(
        &self,
        headers: HashMap<&str, String>,
    ) -> Result<ApiResponse, Box<dyn Error>> {
        let client = reqwest::Client::new();


        let mut request = match self {
            ApiEndpoints::FriendList => client.get(self.get_url()),
            ApiEndpoints::GetChannels(user_id) => {
                if let Some(user_id) = user_id {
                    let request = client.post(self.get_url());
                    let mut json = HashMap::new();
                    json.insert("recipients", [user_id.clone()]);
                    request.json(&json)
                } else {
                    client.get(self.get_url())
                }
            }
            ApiEndpoints::GetMessages(_, _, _) => client.get(self.get_url()),
            ApiEndpoints::GetUser => client.get(self.get_url()),
        };

        for (key, value) in headers {
            request = request.header(key, HeaderValue::from_str(value.as_str()).unwrap());
        }


        let response = request.send().await?;
        let response_status = response.status();

        if response_status != StatusCode::OK {
            return Err(Box::new(std::io::Error::new(ErrorKind::Other, "stuff")));
        }

        Ok(match self {
            Self::FriendList => ApiResponse::Friends(response.json::<Vec<Friend>>().await.unwrap()),

            Self::GetChannels(recipient) => match recipient {
                None => ApiResponse::Channels(response.json::<Vec<Channel>>().await.unwrap()),
                Some(_) => ApiResponse::Channels(vec![response.json::<Channel>().await.unwrap()]),
            }

            Self::GetMessages(_, _, _) => {

                ApiResponse::Messeges(response.json::<Vec<Message>>().await.unwrap())
            }

            Self::GetUser => ApiResponse::User(response.json::<AuthedUser>().await.unwrap()),
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
    Messeges(Vec<Message>),
    User(AuthedUser),
}
#[derive(Deserialize, Debug)]
pub struct AuthedUser {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct Message {
    //pub attachments: Vec<String>,
    pub author: User,
    pub channel_id: String,
    //pub components: Vec<String>,
    pub content: String,
    pub edited_timestamp: Option<String>,
    // pub embeds: Vec<u32>,
    pub flags: u32,
    pub id: String,
    pub mention_everyone: bool,
    pub pinned: bool,
    pub reactions: Option<Vec<Reaction>>,
    pub timestamp: String,
    pub tts: bool,
}
// pub mention_roles: Vec<String>,
// pub mentions: Vec<String>,
// type: u32,
#[derive(Deserialize, Debug)]
pub struct Reaction {
    pub burst_colors: Vec<String>,
    pub burst_count: u32,
    pub burst_me: bool,
    pub count: u32,
    pub count_details: CountDetails,
    pub emoji: Emoji,
    pub me: bool,
    pub me_burst: bool,
}

#[derive(Deserialize, Debug)]
pub struct Emoji {
    pub id: Option<String>,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct CountDetails {
    pub burst: u32,
    pub normal: u32,
}

#[derive(Deserialize, Debug)]
pub struct Friend {
    pub id: String,
    pub nickname: Option<String>,
    pub since: Option<String>,
    #[serde(rename = "type")]
    pub type_of: i32,
    pub user: User,
    pub is_spam_request: bool,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub avatar: Option<String>,
    pub avatar_decoration_data: Option<AvatarDecoration>,
    pub clan: Option<Clan>,
    pub discriminator: String,
    pub id: String,
    pub public_flags: i32,
    pub global_name: Option<String>,
    pub username: String,
}
#[derive(Deserialize, Debug)]
pub struct AvatarDecoration {
    pub asset: String,
    pub expires_at: Option<i32>,
    pub sku_id: String,
}
#[derive(Deserialize, Debug)]
pub struct Channel {
    pub id: String,
    #[serde(rename = "type")]
    pub type_of: i32,
    pub last_message_id: Option<String>,
    pub flags: i32,
    pub recipients: Vec<Recipient>,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub owner_id: Option<String>,
    pub last_pin_timestamp: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Recipient {
    pub id: String,
    pub username: String,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
    pub avatar_decoration_data: Option<AvatarDecoration>,
    pub discriminator: String,
    pub public_flags: i32,
    pub clan: Option<Clan>,
    pub system: Option<bool>,
    pub bot: Option<bool>,
    pub accent_color: Option<u32>,
    pub banner: Option<String>,
    pub banner_color: Option<String>,
    pub flags: Option<i32>,

}
#[derive(Deserialize, Debug)]
pub struct Clan {
    pub id: Option<String>,
    pub identity_enabled: bool,
    pub identity_guild_id: String,
    pub tag: String,
}