use std::fmt::Debug;

use discord::rest_api::Discord;

use crate::auth::{Auth, Platform};
use crate::Conversation;

pub mod utils;
pub mod discord {
    pub mod json_structs;
    pub mod rest_api;
}

pub trait Messenger {
    // Get data about the messanger TODO: Figure out if we even need it here
    fn new(auth: Auth) -> impl Messenger {
        match auth.platform {
            Platform::Discord => Discord { token: auth.token },
            Platform::Unkown => todo!("Handle unkown messanger"),
        }
    }

    // Fetch contacts
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
