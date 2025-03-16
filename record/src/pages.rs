pub mod chat;
pub mod login;

use chat::Message as MessangerMessage;
pub use login::Login;
use login::Message as LoginMessage;

#[derive(Debug, Clone)]
pub enum MyAppMessage {
    Login(LoginMessage),
    Chat(MessangerMessage),
}

pub trait Page {
    fn update(&mut self, message: MyAppMessage) -> Option<Box<dyn Page>>;
    fn view(&self) -> iced::Element<MyAppMessage>;
}
