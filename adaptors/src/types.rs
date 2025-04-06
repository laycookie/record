use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct MsgsStore {
    // pub(crate) hash: Option<String>, // Used in cases where ID can change
    pub(crate) id: String, // ID of a location
    pub name: String,
    pub icon: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
}

#[derive(Debug)]
pub struct Message {
    pub(crate) id: String,
    pub sender: User,
    pub text: String,
}
