use std::{error::Error, rc::Rc};

use super::{MyAppMessage, Page};
use adaptors::{
    types::{Conversation, User},
    Messanger as Auth,
};
use futures::join;
use iced::{
    widget::{column, row, Button, Column, Text},
    Length,
};

#[derive(Debug, Clone)]
pub(super) enum Message {}

pub struct MessangerWindow {
    client_profile: User,
    navbar: Vec<NavElement>,
    conversation_center: ConversationCenter,
}

struct NavElement {
    label: String,
}
struct ConversationCenter {
    conversations: Vec<Conversation>,
    contacts: Vec<User>,
    selected_chat: Option<u32>,
    chat: Vec<i32>,
}

impl MessangerWindow {
    pub fn new(auths: &Vec<Rc<dyn Auth>>) -> Result<Self, Box<dyn std::error::Error>> {
        let q = auths[0].query().unwrap();

        smol::block_on(async {
            let (profile, conversations, contacts) =
                join!(q.get_profile(), q.get_conversation(), q.get_contacts());

            Ok(MessangerWindow {
                client_profile: profile?,
                navbar: vec![NavElement {
                    label: String::from("Guilds"),
                }],
                conversation_center: ConversationCenter {
                    conversations: conversations?,
                    contacts: contacts?,
                    selected_chat: None,
                    chat: vec![],
                },
            })
        })
    }
}

impl Page for MessangerWindow {
    fn update(&mut self, message: MyAppMessage) -> Option<Box<dyn Page>> {
        if let MyAppMessage::Chat(message) = message {
            match message {}
        }

        None
    }

    fn view(&self) -> iced::Element<super::MyAppMessage> {
        let navbar = self
            .navbar
            .iter()
            .map(|i| Text::from(i.label.as_str()))
            .fold(Column::new(), |column, widget| column.push(widget));

        let sidebar = {
            let cont = self
                .conversation_center
                .contacts
                .iter()
                .map(|i| Button::new(i.username.as_str()))
                .fold(Column::new(), |column, widget| column.push(widget))
                .height(Length::Fill);

            let profile = row![Text::from(self.client_profile.username.as_str())];

            column![cont, profile]
        };

        let chat = row![navbar, sidebar, "Chat"];
        chat.into()
    }
}
