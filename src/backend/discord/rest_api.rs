use std::collections::HashMap;

use secure_string::SecureString;

use super::super::utils::{Request_Type, http_request};
use crate::backend::Messenger;

pub struct Discord {
    pub token: SecureString,
}

impl Messenger for Discord {
    async fn get_contacts(&self) -> Result<(), surf::Error> {
        let header = vec![("Authorization", self.token.clone().into_unsecure())];
        let json = http_request::<serde_json::Value>(
            "https://discord.com/api/v9/users/@me/relationships",
            header,
            Request_Type::GET,
        )
            .await?;

        println!("{:#?}", json);

        Ok(())
    }

    async fn get_channels(&self, user_id: Option<String>) -> Result<(), surf::Error> {
        let mut header = vec![("Authorization", self.token.clone().into_unsecure())];
        match user_id {
            Some(user_id) => {
                header.push(("recipients", user_id));
                let json = http_request::<serde_json::Value>(
                    "https://discord.com/api/v10/users/@me/channels",
                    header,
                    Request_Type::POST,
                ).await?;
            }
            None => {
                let json = http_request::<serde_json::Value>(
                    "https://discord.com/api/v10/users/@me/channels",
                    header,
                    Request_Type::GET,
                ).await?;
            }
        }


        Ok(())
    }

    async fn get_guilds(&self) -> Result<(), surf::Error> {
        let header = vec![("Authorization", self.token.clone().into_unsecure())];
        let json = http_request::<serde_json::Value>(
            "https://discord.com/api/v9/users/@me/relationships",
            header,
            Request_Type::GET,
        )
            .await?;

        println!("{:#?}", json);

        Ok(())
    }

    async fn get_messanges(&self, channel_id: String, before_message: Option<String>, msg_limit: u32) -> Result<(), surf::Error> {
        let before = match before_message {
            Some(before_message) => format!("before={}&", before_message),
            None => "".into(),
        };
        let link = format!(
            "https://discord.com/api/v9/channels/{}/messages?{}limit={}",
            channel_id, before, msg_limit
        );
        let header = vec![("Authorization", self.token.clone().into_unsecure())];
        let json = http_request::<serde_json::Value>(
            "https://discord.com/api/v9/users/@me/relationships",
            header,
            Request_Type::GET,
        ).await?;
        Ok(())
    }

    async fn get_profile(&self) -> Result<(), surf::Error> {
        let header = vec![("Authorization", self.token.clone().into_unsecure())];
        let json = http_request::<serde_json::Value>(
            "https://discord.com/api/v9/users/@me",
            header,
            Request_Type::GET,
        ).await?;

        println!("{:#?}", json);

        Ok(())
    }
}
