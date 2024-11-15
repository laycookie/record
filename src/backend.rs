use discord::rest_api::Discord;

use crate::auth::{Auth, Platform};

pub mod utils;
pub mod discord {
    pub mod rest_api;
}

pub trait Messanger {
    // Get data about the messanger TODO: Figure out if we even need it here
    fn new(auth: Auth) -> impl Messanger {
        match auth.platform {
            Platform::Discord => Discord { token: auth.token },
            Platform::Unkown => todo!("Handle unkown messanger"),
        }
    }

    // Fetch contacts
    async fn get_contacts(&self) -> Result<(), surf::Error>; // Users from friendlists e.t.c.
    fn get_conversations(); // Also known as DMs
    fn get_guilds(); // Large groups that can have over a 100 people in them.

    // Fetch data based on contacts
    fn get_messanges();
    fn get_profile();
}
