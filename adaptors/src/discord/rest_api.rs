use async_trait::async_trait;

use crate::{
    network::http_request,
    types::{Conversation, Guild, User},
    MessageLocation, MessangerQuery, ParameterizedMessangerQuery,
};

use super::{
    json_structs::{Channel, Friend, Profile},
    Discord,
};

impl Discord {
    fn get_auth_header(&self) -> Vec<(&str, String)> {
        vec![("Authorization", self.token.clone())]
    }
}

#[async_trait]
impl MessangerQuery for Discord {
    async fn get_profile(&self) -> Result<User, surf::Error> {
        let profile = http_request::<Profile>(
            surf::get("https://discord.com/api/v9/users/@me"),
            self.get_auth_header(),
        )
        .await?;
        Ok(profile.into())
    }
    async fn get_contacts(&self) -> Result<Vec<User>, surf::Error> {
        let friends = http_request::<Vec<Friend>>(
            surf::get("https://discord.com/api/v9/users/@me/relationships"),
            self.get_auth_header(),
        )
        .await?;
        Ok(friends.iter().map(|friend| friend.clone().into()).collect())
    }
    async fn get_conversation(&self) -> Result<Vec<Conversation>, surf::Error> {
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
    async fn get_guilds(&self) -> Result<Vec<Guild>, surf::Error> {
        todo!()
    }
}

#[async_trait]
impl ParameterizedMessangerQuery for Discord {
    async fn get_messanges(&self, before_message: &dyn MessageLocation) -> Result<(), surf::Error> {
        todo!()
    }
}
