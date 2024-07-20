

pub enum ApiEndpoints {
    FriendList,
    DiscordGateway
}

impl ApiEndpoints {
    pub fn get_url(&self) -> &str {
        match self {
            Self::FriendList => "https://discord.com/api/v9/users/@me/relationships",
            Self::DiscordGateway => "wss://gateway.discord.gg/?v=10&encoding=json"
        }
    }
}
