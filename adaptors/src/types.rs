#[derive(Debug)]
pub struct User {
    pub id: String,
    pub username: String,
}
#[derive(Debug)]
pub struct Conversation {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
}

pub struct Guild {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
}
