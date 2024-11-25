use crate::auth::Auth;
use crate::Conversation;
use std::fmt::Debug;

pub mod utils;
pub mod discord {
    pub mod json_structs;
    pub mod rest_api;
}

pub trait Messanger: MessengerHTTP {
    fn recover_auth(&self) -> &Auth;
}

pub trait MessengerHTTP {
    type Profile: Debug;
    async fn get_profile(&self) -> Result<Self::Profile, surf::Error>; // Fetch clint profile
    type Contacts: Debug;
    async fn get_contacts(&self) -> Result<Self::Contacts, surf::Error>; // Users from friendlists e.t.c.
    type Conversation: Debug + Clone + Into<Conversation>;
    async fn get_conversation(&self) -> Result<Vec<Self::Conversation>, surf::Error>; // List of DMs
    type Guilds: Debug;
    async fn get_guilds(&self) -> Result<Self::Guilds, surf::Error>; // Large groups that can have over a 100 people in them.

    // Fetch data based on contacts
    type MessageOrigin: Debug;
    async fn get_messanges(
        &self,
        message_origin: Self::MessageOrigin,
    ) -> Result<serde_json::Value, surf::Error>;
}
