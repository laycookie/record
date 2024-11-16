use serde::Deserialize;

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

#[derive(Deserialize, Debug)]
pub struct Channel {
    pub flags: i32,
    pub icon: Option<String>,
    pub id: String,
    pub last_message_id: String,
    pub name: Option<String>,
    pub recipients: Vec<Recipient>,
}

#[derive(Deserialize, Debug)]
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
