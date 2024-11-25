use std::path::Path;

use crate::{auth::Platform, network_req::cache_download, Conversation};
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use slint::{format, SharedString};

#[derive(Deserialize, Debug)]
pub struct Message {
    pub attachments: Vec<String>,
    pub author: User,
    pub channel_id: String,
    pub components: Vec<String>,
    pub content: String,
    pub edited_timestamp: Option<String>,
    pub embeds: Vec<u32>,
    pub flags: u32,
    pub id: String,
    pub mention_everyone: bool,
    // pub mention_roles: Vec<String>,
    // pub mentions: Vec<String>,
    pub pinned: bool,
    pub reactions: Option<Vec<Reaction>>,
    pub timestamp: String,
    pub tts: bool,
    // type: u32,
}

#[derive(Deserialize, Debug)]
pub struct Reaction {
    pub burst_colors: Vec<String>,
    pub burst_count: u32,
    pub burst_me: bool,
    pub count: u32,
    pub count_details: CountDetails,
    pub emoji: Emoji,
    pub me: bool,
    pub me_burst: bool,
}

#[derive(Deserialize, Debug)]
pub struct Emoji {
    pub id: Option<String>,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct CountDetails {
    pub burst: u32,
    pub normal: u32,
}

#[derive(Deserialize, Debug)]
pub struct Friend {
    pub id: String,
    pub is_spam_request: bool,
    pub nickname: Option<String>,
    pub since: String,
    // pub type: i32,
    pub user: User,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub avatar: Option<String>,
    pub avatar_decoration_data: Option<String>,
    pub clan: Option<String>,
    pub discriminator: String,
    pub id: String,
    pub username: String,
}

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
    pub id: String,
    #[serde(rename = "type")]
    pub channel_type: ChannelTypes,
    pub flags: i32,
    pub icon: Option<String>,
    pub last_message_id: String,
    pub name: Option<String>,
    pub recipients: Vec<Recipient>,
}

impl Into<Conversation> for Channel {
    fn into(self) -> Conversation {
        let id: SharedString = self.id.clone().into();
        let name;
        let image;

        match self.channel_type {
            ChannelTypes::DM => {
                let recipient = &self.recipients[0];
                name = recipient
                    .global_name
                    .clone()
                    .unwrap_or(recipient.username.clone())
                    .into();

                let avatar_id = recipient.avatar.clone();

                if let Some(avatar_id) = avatar_id {
                    let url = format!(
                        "https://cdn.discordapp.com/avatars/{}/{}.png?size=80",
                        recipient.id, avatar_id
                    );
                    let path = format!("public/Discord/Users/{}", id).to_string();
                    let file_name = format!("{}.png", avatar_id);

                    // TODO: Make this proper async
                    smol::block_on(async {
                        cache_download(&url, path.clone().into(), &file_name).await;
                    });

                    let p = format!("{}/{}", path, file_name);
                    image = slint::Image::load_from_path(Path::new(&p.to_string())).unwrap();
                } else {
                    image = slint::Image::load_from_path(Path::new("public/Assets/avatar.png"))
                        .unwrap();
                }
            }
            _ => todo!(),
        };

        Conversation {
            id,
            image,
            name,
            platform: Platform::Discord.to_string().into(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Recipient {
    pub avatar: Option<String>,
    pub avatar_decoration_data: Option<String>,
    pub clan: Option<String>,
    pub discriminator: String,
    pub global_name: Option<String>,
    pub id: String,
    pub public_flags: i32,
    pub username: String,
}

#[derive(Deserialize, Debug)]
pub struct Profile {
    pub accent_color: Option<String>,
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
