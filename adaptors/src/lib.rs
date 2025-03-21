use std::error::Error;

use async_trait::async_trait;
use types::{Conversation, Guild, User};

pub mod discord;
mod network;
pub mod types;

pub trait Messanger {
    // ID & Auth
    fn name(&self) -> String;
    fn auth(&self) -> String;
    // Features - TODO: Remove when up-casting will become stable https://github.com/rust-lang/rust/issues/65991
    fn query(&self) -> Option<&dyn MessangerQuery> {
        None
    }
}
impl PartialEq for dyn Messanger {
    fn eq(&self, other: &Self) -> bool {
        format!("{}{}", self.name(), self.auth()) == format!("{}{}", other.name(), other.auth())
    }
}

#[async_trait]
pub trait MessangerQuery {
    async fn get_profile(&self) -> Result<User, Box<dyn Error>>; // Fetch client profile
    async fn get_contacts(&self) -> Result<Vec<User>, Box<dyn Error>>; // Users from friend list etc
    async fn get_conversation(&self) -> Result<Vec<Conversation>, Box<dyn Error>>; // List of DMs
    async fn get_guilds(&self) -> Result<Vec<Guild>, Box<dyn Error>>; // Large groups that can have over a 100 people in them.
}

// ===

pub(crate) trait MessageLocation {
    fn get_link(&self) -> String;
}

#[async_trait]
trait ParameterizedMessangerQuery: Messanger {
    async fn get_messanges(&self, location: &dyn MessageLocation) -> Result<(), surf::Error>;
}
