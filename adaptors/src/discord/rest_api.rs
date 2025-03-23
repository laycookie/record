use std::error::Error;

use async_trait::async_trait;

use crate::{
    discord::json_structs::Message,
    network::http_request,
    types::{Conversation, Guild as GlobalGuild, Message as GlobalMessage, User},
    MessangerQuery, MsgLocation, ParameterizedMessangerQuery,
};

use super::{
    json_structs::{Channel, Friend, Guild, Profile},
    Discord,
};

impl Discord {
    fn get_auth_header(&self) -> Vec<(&str, String)> {
        vec![("Authorization", self.token.clone())]
    }
}

#[async_trait]
impl MessangerQuery for Discord {
    async fn get_profile(&self) -> Result<User, Box<dyn Error>> {
        let profile = http_request::<Profile>(
            surf::get("https://discord.com/api/v9/users/@me"),
            self.get_auth_header(),
        )
        .await?;
        Ok(profile.into())
    }
    async fn get_contacts(&self) -> Result<Vec<User>, Box<dyn Error>> {
        let friends = http_request::<Vec<Friend>>(
            surf::get("https://discord.com/api/v9/users/@me/relationships"),
            self.get_auth_header(),
        )
        .await?;
        Ok(friends.iter().map(|friend| friend.clone().into()).collect())
    }
    async fn get_conversation(&self) -> Result<Vec<Conversation>, Box<dyn Error>> {
        let channels = http_request::<Vec<Channel>>(
            surf::get("https://discord.com/api/v10/users/@me/channels"),
            self.get_auth_header(),
        )
        .await?;
        Ok(channels
            .iter()
            .map(|channel| channel.clone().into())
            .collect())
    }
    async fn get_guilds(&self) -> Result<Vec<GlobalGuild>, Box<dyn Error>> {
        let guilds = http_request::<Vec<Guild>>(
            surf::get("https://discord.com/api/v10/users/@me/guilds"),
            self.get_auth_header(),
        )
        .await?;
        Ok(guilds.iter().map(|guild| guild.clone().into()).collect())
    }
}

#[async_trait]
impl ParameterizedMessangerQuery for Discord {
    async fn get_messanges(
        &self,
        location: MsgLocation,
    ) -> Result<Vec<GlobalMessage>, Box<dyn Error>> {
        let MsgLocation::Discord { channed_id, before } = location else {
            return Err(panic!("temp"));
        };

        let messages = http_request::<Vec<Message>>(
            surf::get(format!(
                "https://discord.com/api/v10/channels/{}/messages?{}",
                channed_id, before,
            )),
            self.get_auth_header(),
        )
        .await?;

        Ok(messages.iter().map(|message| message.into()).collect())
    }
}
