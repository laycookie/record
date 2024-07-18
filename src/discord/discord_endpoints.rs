pub const FRIENDLIST_URL: &str = "https://discord.com/api/v9/users/@me/relationships";

pub enum ApiEndpoints {
    FriendList,
}

impl ApiEndpoints {
    pub fn get_url(&self) -> &str {
        match self {
            Self::FriendList => "https://discord.com/api/v9/users/@me/relationships",
        }
    }
}
