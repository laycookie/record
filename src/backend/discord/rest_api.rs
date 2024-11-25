use super::{
    super::utils::{http_request, Request_Type},
    json_structs::{Channel, Friend, Profile},
};
use crate::{
    auth::Auth,
    backend::{Messanger, MessengerHTTP},
};

pub struct Discord<'a> {
    pub auth: &'a Auth,
}

impl Messanger for Discord<'_> {
    fn recover_auth(&self) -> &Auth {
        self.auth
    }
}

impl MessengerHTTP for Discord<'_> {
    type Contacts = Vec<Friend>;
    async fn get_contacts(&self) -> Result<Self::Contacts, surf::Error> {
        let header = vec![("Authorization", self.auth.token.clone().into_unsecure())];
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
        let header = vec![("Authorization", self.auth.token.clone().into_unsecure())];
        http_request::<Vec<Self::Conversation>>(
            "https://discord.com/api/v10/users/@me/channels",
            header,
            Request_Type::GET,
        )
        .await
    }

    type Guilds = Vec<serde_json::Value>;
    async fn get_guilds(&self) -> Result<Self::Guilds, surf::Error> {
        todo!();
    }

    type Profile = Profile;
    async fn get_profile(&self) -> Result<Self::Profile, surf::Error> {
        let header = vec![("Authorization", self.auth.token.clone().into_unsecure())];
        Ok(http_request::<Self::Profile>(
            "https://discord.com/api/v9/users/@me",
            header,
            Request_Type::GET,
        )
        .await?)
    }

    type MessageOrigin = MessageOrigin;
    async fn get_messanges(
        &self,
        message_origin: MessageOrigin,
    ) -> Result<serde_json::Value, surf::Error> {
        let before = match message_origin.load_from_message {
            Some(before_message) => format!("before={}&", before_message),
            None => "".into(),
        };
        let link = format!(
            "https://discord.com/api/v9/channels/{}/messages?{}limit={}",
            message_origin.channel_id, before, message_origin.message_limit
        );

        let header = vec![("Authorization", self.auth.token.clone().into_unsecure())];
        Ok(http_request::<serde_json::Value>(&link, header, Request_Type::GET).await?)
    }
}

#[derive(Debug)]
pub struct MessageOrigin {
    channel_id: String,
    message_limit: String,
    load_from_message: Option<String>,
}
