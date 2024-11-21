use std::fmt::Debug;
use crate::Conversation;

pub mod utils;
pub mod discord {
    pub mod json_structs;
    pub mod rest_api;
}

pub trait Messenger {
    type Contacts: Debug;
    async fn get_contacts(&self) -> Result<Self::Contacts, surf::Error>; // Users from friendlists e.t.c.
    type Conversation: Debug + Into<Conversation>;
    async fn get_conversation(&self) -> Result<Vec<Self::Conversation>, surf::Error>; // List of DMs
    type Guilds: Debug;
    async fn get_guilds(&self) -> Result<Self::Guilds, surf::Error>; // Large groups that can have over a 100 people in them.

    // Fetch data based on contacts
    async fn get_messanges(
        &self,
        channel_id: String,
        before_message: Option<String>,
        msg_limit: u32,
    ) -> Result<serde_json::Value, surf::Error>;

    type Profile: Debug;
    async fn get_profile(&self) -> Result<Self::Profile, surf::Error>;
}
