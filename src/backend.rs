use discord::rest_api::Discord;

use crate::auth::{Auth, Platform};

pub mod utils;
pub mod discord {
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
    async fn get_contacts(&self) -> Result<(), surf::Error>; // Users from friendlists e.t.c.
    async fn get_channels(&self, user_id : Option<String>) -> Result<(), surf::Error>; // Also known as DMs
    async fn get_guilds(&self) -> Result<(), surf::Error>; // Large groups that can have over a 100 people in them.

    // Fetch data based on contacts
    async fn get_messanges(&self, channel_id: String, before_message: Option<String>, msg_limit: u32) -> Result<(), surf::Error>;
    async fn get_profile(&self) -> Result<(), surf::Error>;
}
