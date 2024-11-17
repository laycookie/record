use secure_string::SecureString;

use super::{
    super::utils::{http_request, Request_Type},
    json_structs::{Channel, Friend, Profile},
};
use crate::backend::Messenger;

pub struct Discord {
    pub token: SecureString,
}

impl Messenger for Discord {
    type Contacts = Vec<Friend>;
    async fn get_contacts(&self) -> Result<Self::Contacts, surf::Error> {
        let header = vec![("Authorization", self.token.clone().into_unsecure())];
        Ok(http_request::<Self::Contacts>(
            "https://discord.com/api/v9/users/@me/relationships",
            header,
            Request_Type::GET,
        )
        .await?)
    }

    type Conversation = Channel;
    // type Conversations = serde_json::Value;
    async fn get_conversation(&self) -> Result<Vec<Self::Conversation>, surf::Error> {
        // List of DMs
        let header = vec![("Authorization", self.token.clone().into_unsecure())];
        Ok(http_request::<Vec<Self::Conversation>>(
            "https://discord.com/api/v10/users/@me/channels",
            header,
            Request_Type::GET,
        )
        .await?)
    }

    type Guilds = Vec<serde_json::Value>;
    async fn get_guilds(&self) -> Result<Self::Guilds, surf::Error> {
        todo!();
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

    type Profile = Profile;
    async fn get_profile(&self) -> Result<Self::Profile, surf::Error> {
        let header = vec![("Authorization", self.token.clone().into_unsecure())];
        Ok(http_request::<Self::Profile>(
            "https://discord.com/api/v9/users/@me",
            header,
            Request_Type::GET,
        )
        .await?)
    }
}
