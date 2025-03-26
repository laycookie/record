use std::{collections::HashMap, error::Error};

use crate::AuthStore;

use super::{MyAppMessage, Page};
use adaptors::types::{MsgsStore, User};
use futures::try_join;
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
    client_profile: User,
    conversation_center: ConversationData,
}

enum Main {
    Contacts,
    Chat(String),
}

struct ConversationData {
    contacts: Vec<User>,
    conversations: Vec<MsgsStore>,
    guilds: Vec<MsgsStore>,
    chat: HashMap<String, String>,
}

impl MessangerWindow {
    pub fn new(auth_store: &mut AuthStore) -> Result<Self, Box<dyn Error>> {
        let ex = LocalExecutor::new();

        smol::block_on(ex.run(async {
            let messangers = auth_store.get_messangers();
            let q = messangers[0].auth.query().unwrap();

            let (profile, conversations, contacts, guilds) = try_join!(
                q.get_profile(),
                q.get_conversation(),
                q.get_contacts(),
                q.get_guilds()
            )?;

            let window = MessangerWindow {
                auth_store,
                client_profile: profile,
                conversation_center: ConversationData {
                    guilds,
                    conversations,
                    contacts,
                    chat: HashMap::new(),
                },
                main: Main::Contacts,
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
        let options = row![Text::new(&self.client_profile.username)];

        let navbar = self
            .conversation_center
            .guilds
            .iter()
            .map(|i| {
                let image = match &i.icon {
                    Some(icon) => {
                        println!("{:?}", icon);
                        image(icon)
                    }
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
            self.conversation_center
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
                    self.conversation_center
                        .contacts
                        .iter()
                        .map(|i| Text::from(i.username.as_str()))
                        .fold(Column::new(), |column, widget| column.push(widget)),
                )
            }
            Main::Chat(id) => {
                let widget = Column::new();
                let chat = self.conversation_center.chat.get(id);
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
