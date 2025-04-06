pub mod chat;
pub mod login;

use chat::Message as MessangerMessage;
use iced::Task;
pub use login::Login;
use login::Message as LoginMessage;

#[derive(Debug, Clone)]
pub enum MyAppMessage {
    Login(LoginMessage),
    Chat(MessangerMessage),
}

pub enum UpdateResult<M> {
    Page(Box<dyn Page>),
    Task(Task<M>),
    None,
}

pub trait Page {
    fn update(&mut self, message: MyAppMessage) -> UpdateResult<MyAppMessage>;
    fn view(&self) -> iced::Element<MyAppMessage>;
}
