use std::{collections::HashMap, error::Error, rc::Rc};

use super::{MyAppMessage, Page};
use adaptors::{
    types::{Conversation, User},
    Messanger as Auth,
};
use futures::try_join;
use iced::{
    widget::{column, row, Button, Column, Text, TextInput},
    Length,
};
use adaptors::types::Guild;

#[derive(Debug, Clone)]
pub(super) enum Message {
    OpenContacts,
    OpenConversation(String),
}

// TODO: Automate
impl Into<MyAppMessage> for Message {
    fn into(self) -> MyAppMessage {
        MyAppMessage::Chat(self)
    }
}
//

pub struct MessangerWindow {
    client_profile: User,
    main: Main,
    conversation_center: ConversationData,
}

enum Main {
    Contacts,
    Chat(String),
}


struct ConversationData {
    conversations: Vec<Conversation>,
    contacts: Vec<User>,
    // TODO: GUILDS GO HERE
    guilds: Vec<Guild>,
    chat: HashMap<String, String>,
}

impl MessangerWindow {
    pub fn new(auths: &Vec<Rc<dyn Auth>>) -> Result<Self, Box<dyn Error>> {
        let q = auths[0].query().unwrap();

        smol::block_on(async {
            let (profile, conversations, contacts, guilds) =
                try_join!(q.get_profile(), q.get_conversation(), q.get_contacts(), q.get_guilds())?;

            Ok(MessangerWindow {
                client_profile: profile,
                conversation_center: ConversationData {
                    guilds,
                    conversations,
                    contacts,
                    chat: HashMap::new(),
                },
                main: Main::Contacts,
            })
        })
    }
}

impl Page for MessangerWindow {
    fn update(&mut self, message: MyAppMessage) -> Option<Box<dyn Page>> {
        if let MyAppMessage::Chat(message) = message {
            match message {
                Message::OpenConversation(id) => self.main = Main::Chat(id),
                Message::OpenContacts => self.main = Main::Contacts,
            }
        }

        None
    }

    fn view(&self) -> iced::Element<super::MyAppMessage> {
        let options = row![Text::new(&self.client_profile.username)];

        let navbar = self.conversation_center
            .guilds
            .iter()
            .map(|i| Text::from(i.name.as_str()))
            .fold(Column::new(), |column, widget| column.push(widget));


        let sidebar = column![
            Button::new("Contacts").on_press(MyAppMessage::Chat(Message::OpenContacts)),
            self.conversation_center
                .conversations
                .iter()
                .map(|i| {
                    Button::new(i.name.as_str())
                        .on_press(Message::OpenConversation(i.id.clone()).into())
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
