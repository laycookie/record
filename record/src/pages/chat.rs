use std::{collections::HashMap, error::Error};

use crate::AuthStore;

use super::{MyAppMessage, Page};
use adaptors::types::{MsgsStore, User};
use futures::{future::try_join_all, try_join};
use iced::{
    widget::{column, image, row, Button, Column, Text, TextInput},
    ContentFit, Length,
};
use smol::LocalExecutor;

#[derive(Debug, Clone)]
pub enum Message {
    OpenContacts,
    OpenConversation(MsgsStore),
}

// TODO: Automate
impl Into<MyAppMessage> for Message {
    fn into(self) -> MyAppMessage {
        MyAppMessage::Chat(self)
    }
}
//

pub struct MessangerWindow {
    auth_store: *mut AuthStore,
    main: Main,
    messangers_data: Vec<MsngrData>,
}

struct MsngrData {
    profile: User,
    contacts: Vec<User>,
    conversations: Vec<MsgsStore>,
    guilds: Vec<MsgsStore>,
    chat: HashMap<String, String>,
}

enum Main {
    Contacts,
    Chat(String),
}

impl MessangerWindow {
    pub fn new(auth_store: &mut AuthStore) -> Result<Self, Box<dyn Error>> {
        let ex = LocalExecutor::new();

        smol::block_on(ex.run(async {
            let msngrs = try_join_all(auth_store.get_messangers().iter().map(async move |m| {
                let q = m.auth.query().unwrap();
                try_join!(
                    q.get_profile(),
                    q.get_conversation(),
                    q.get_contacts(),
                    q.get_guilds(),
                )
            }))
            .await?
            .into_iter()
            .map(|(profile, conversations, contacts, guilds)| MsngrData {
                profile,
                contacts,
                conversations,
                guilds,
                chat: HashMap::new(),
            })
            .collect::<Vec<_>>();

            let window = MessangerWindow {
                auth_store,
                main: Main::Contacts,
                messangers_data: msngrs,
            };

            Ok(window)
        }))
    }
    fn get_auth_store(&self) -> &AuthStore {
        unsafe { &*self.auth_store }
    }
}

impl Page for MessangerWindow {
    fn update(&mut self, message: MyAppMessage) -> Option<Box<dyn Page>> {
        if let MyAppMessage::Chat(message) = message {
            match message {
                Message::OpenConversation(msgs_store) => {
                    let a = &self.get_auth_store().get_messangers()[0].auth;
                    let pq = a.param_query().unwrap();

                    smol::block_on(async {
                        let mess = pq.get_messanges(msgs_store, None).await;
                        println!("{:#?}", mess);
                    });

                    // self.main = Main::Chat(conversation.name);
                }
                Message::OpenContacts => self.main = Main::Contacts,
            }
        }

        None
    }

    fn view(&self) -> iced::Element<super::MyAppMessage> {
        let options = row![Text::new(&self.messangers_data[0].profile.username)];

        let navbar = self.messangers_data[0]
            .guilds
            .iter()
            .map(|i| {
                let image = match &i.icon {
                    Some(icon) => image(icon),
                    None => image("./public/imgs/placeholder.jpg"),
                };
                Button::new(
                    image
                        .height(Length::Fixed(48.0))
                        .width(Length::Fixed(48.0))
                        .content_fit(ContentFit::Cover),
                )
            })
            .fold(Column::new(), |column, widget| column.push(widget));

        let sidebar = column![
            Button::new("Contacts").on_press(MyAppMessage::Chat(Message::OpenContacts)),
            self.messangers_data[0]
                .conversations
                .iter()
                .map(|i| {
                    Button::new(i.name.as_str())
                        .on_press(Message::OpenConversation(i.to_owned()).into())
                })
                .fold(Column::new(), |column, widget| column.push(widget))
                .height(Length::Fill)
        ];

        let main = match &self.main {
            Main::Contacts => {
                let widget = Column::new();
                let widget = widget.push(TextInput::new("Search", ""));
                widget.push(
                    self.messangers_data[0]
                        .contacts
                        .iter()
                        .map(|i| Text::from(i.username.as_str()))
                        .fold(Column::new(), |column, widget| column.push(widget)),
                )
            }
            Main::Chat(id) => {
                let widget = Column::new();
                let chat = self.messangers_data[0].chat.get(id);
                let t = match chat {
                    Some(text) => text.as_str(),
                    None => "test",
                };
                let widget = widget.push(Text::from(t));
                widget
            }
        };

        column![options, row![navbar, sidebar, main]].into()
    }
}
