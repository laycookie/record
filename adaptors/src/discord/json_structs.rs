use crate::types::{Conversation, User as GlobalUser};
use serde::Deserialize;
use serde_repr::Deserialize_repr;

// === Users ===

#[derive(Deserialize)]
pub struct Profile {
    accent_color: Option<String>,
    authenticator_types: Vec<String>,
    avatar: Option<String>,
    avatar_decoration_data: Option<String>,
    banner: Option<String>,
    banner_color: Option<String>,
    bio: String,
    clan: Option<String>,
    discriminator: String,
    email: String,
    flags: i32,
    global_name: String,
    id: String,
    linked_users: Vec<String>,
    locale: String,
    mfa_enabled: bool,
    nsfw_allowed: bool,
    phone: Option<String>,
    premium_type: i32,
    public_flags: i32,
    username: String,
    verified: bool,
}
impl Into<GlobalUser> for Profile {
    fn into(self) -> GlobalUser {
        GlobalUser {
            id: self.id,
            username: self.username,
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct User {
    avatar: Option<String>,
    avatar_decoration_data: Option<String>,
    clan: Option<String>,
    discriminator: String,
    id: String,
    username: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Friend {
    id: String,
    is_spam_request: bool,
    nickname: Option<String>,
    since: String,
    // type: i32,
    user: User,
}
impl Into<GlobalUser> for Friend {
    fn into(self) -> GlobalUser {
        GlobalUser {
            id: self.id,
            username: self.user.username,
        }
    }
}
#[derive(Deserialize, Debug, Clone)]
pub struct Recipient {
    avatar: Option<String>,
    avatar_decoration_data: Option<String>,
    clan: Option<String>,
    discriminator: String,
    global_name: Option<String>,
    id: String,
    public_flags: i32,
    username: String,
}

// === Chennels ===

#[derive(Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum ChannelTypes {
    GuildText,
    DM,
    GuildVoice,
    GroupDM,
    GuildCategory,
    GuildAnnouncement,
    AnnouncementThread,
    PublicThread,
    PrivateThread,
    GuildStageVoice,
    GuildDirectory,
    GuildForum,
    GuildMedia,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Channel {
    id: String,
    #[serde(rename = "type")]
    channel_type: ChannelTypes,
    flags: i32,
    icon: Option<String>,
    last_message_id: String,
    name: Option<String>,
    recipients: Vec<Recipient>,
}
impl Into<Conversation> for Channel {
    fn into(self) -> Conversation {
        Conversation {
            id: self.id,
            name: self.name.unwrap_or(match self.recipients.get(0) {
                Some(test) => test.username.clone(),
                None => "Fix later".to_string(),
            }),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct CountDetails {
    burst: u32,
    normal: u32,
}

#[derive(Deserialize, Debug)]
pub struct Emoji {
    id: Option<String>,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct Reaction {
    burst_colors: Vec<String>,
    burst_count: u32,
    burst_me: bool,
    count: u32,
    count_details: CountDetails,
    emoji: Emoji,
    me: bool,
    me_burst: bool,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    attachments: Vec<String>,
    author: User,
    channel_id: String,
    components: Vec<String>,
    content: String,
    edited_timestamp: Option<String>,
    embeds: Vec<u32>,
    flags: u32,
    id: String,
    mention_everyone: bool,
    // mention_roles: Vec<String>,
    // mentions: Vec<String>,
    pinned: bool,
    reactions: Option<Vec<Reaction>>,
    timestamp: String,
    tts: bool,
    // type: u32,
}
