use secure_string::SecureString;

use super::{
    super::utils::{http_request, Request_Type},
    json_structs::Channel,
};
use crate::backend::Messenger;

pub struct Discord {
    pub token: SecureString,
}

impl Messenger for Discord {
    async fn get_contacts(&self) -> Result<serde_json::Value, surf::Error> {
        let header = vec![("Authorization", self.token.clone().into_unsecure())];
        Ok(http_request::<serde_json::Value>(
            "https://discord.com/api/v9/users/@me/relationships",
            header,
            Request_Type::GET,
        )
        .await?)
    }

    type Conversations = Vec<Channel>;
    // type Conversations = serde_json::Value;
    async fn get_conversation(&self) -> Result<Self::Conversations, surf::Error> {
        // List of DMs
        let header = vec![("Authorization", self.token.clone().into_unsecure())];
        Ok(http_request::<Self::Conversations>(
            "https://discord.com/api/v10/users/@me/channels",
            header,
            Request_Type::GET,
        )
        .await?)
    }

    async fn get_guilds(&self) -> Result<serde_json::Value, surf::Error> {
        let header = vec![("Authorization", self.token.clone().into_unsecure())];
        Ok(http_request::<serde_json::Value>(
            "https://discord.com/api/v9/users/@me/relationships",
            header,
            Request_Type::GET,
        )
        .await?)
    }

    async fn get_messanges(
        &self,
        channel_id: String,
        before_message: Option<String>,
        msg_limit: u32,
    ) -> Result<serde_json::Value, surf::Error> {
        let before = match before_message {
            Some(before_message) => format!("before={}&", before_message),
            None => "".into(),
        };
        let link = format!(
            "https://discord.com/api/v9/channels/{}/messages?{}limit={}",
            channel_id, before, msg_limit
        );

        let header = vec![("Authorization", self.token.clone().into_unsecure())];
        Ok(http_request::<serde_json::Value>(&link, header, Request_Type::GET).await?)
    }

    async fn get_profile(&self) -> Result<serde_json::Value, surf::Error> {
        let header = vec![("Authorization", self.token.clone().into_unsecure())];
        Ok(http_request::<serde_json::Value>(
            "https://discord.com/api/v9/users/@me",
            header,
            Request_Type::GET,
        )
        .await?)
    }
}
