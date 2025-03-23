use crate::{discord::json_structs::Channel, MsgLocation};

//TODO: Automate
#[derive(Debug, Clone)]
pub enum PlatformData {
    Discord(Channel),
}
impl PlatformData {
    pub fn get_location(&self) -> MsgLocation {
        match self {
            PlatformData::Discord(channel) => MsgLocation::Discord {
                channed_id: channel.id.clone(),
                before: channel.last_message_id.clone(),
            },
        }
    }
}

#[derive(Debug)]
pub struct User {
    // pub id: String,
    pub username: String,
}

#[derive(Debug, Clone)]
pub struct Conversation {
    pub id: String,
    pub name: String,
    pub platform_data: PlatformData,
}

pub struct Guild {
    // pub id: String,
    pub name: String,
}

#[derive(Debug)]
pub struct Message {
    pub sender: User,
    pub text: String,
}
