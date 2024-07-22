use std::{
    path::{Path, PathBuf},
    rc::Rc,
};

use gtk4::{prelude::*, Button, Image, Label, Orientation, Stack};

pub fn user_button(
    parent: &gtk4::Box,
    selected_chat: &Stack,
    chat_data: &ChatData,
    user_id: String,
) {
    let user = DiscordUser {
        username: "test".to_string(),
        pfp: Path::new("").to_owned(),
        id: user_id,
    };

    let button_contents = gtk4::Box::new(Orientation::Horizontal, 5);
    button_contents.set_vexpand(true);

    let username = Label::new(Some(&user.username));
    let avatar = Image::from_file(&user.pfp);

    button_contents.append(&avatar);
    button_contents.append(&username);

    let open_chat = Button::new();
    open_chat.connect_clicked({
        let selected_chat = selected_chat.clone();
        let chat_data = chat_data.clone();
        move |_| {
            chat_data.set_current_chat(&user);
            selected_chat.set_visible_child_name("chat");
        }
    });
    open_chat.set_child(Some(&button_contents));
    parent.append(&open_chat);
}

pub fn chat(parent: Rc<Stack>) {
    let chat_window = gtk4::Box::new(Orientation::Vertical, 0);

    let chat = gtk4::Box::new(Orientation::Vertical, 4);
    let messege_entery = gtk4::Entry::new();

    let username = Label::new(None);
    chat.append(&username);

    chat_window.append(&chat);
    chat_window.append(&messege_entery);

    parent.add_named(&chat_window, Some("chat"));

    // Rc::new(ChatData { user: DiscordUser {} })
}

#[derive(Clone)]
pub struct ChatData {
    pub username: Label,
    pub pfp: Image,
}

impl ChatData {
    pub fn set_current_chat(&self, user: &DiscordUser) {
        self.username.set_label(&user.username);
        self.pfp.set_from_file(Some(&user.pfp));
    }
}

pub fn friend_list(parent: Rc<Stack>, friends: &Vec<DiscordUser>) {
    let friend_list = gtk4::Box::new(Orientation::Vertical, 4);

    for user in friends {
        let test = Button::new();
        test.set_label(&user.username);
        friend_list.append(&test);
    }

    parent.add_named(&friend_list, Some("friend list"));
}

pub struct DiscordUser {
    pub username: String,
    pub pfp: PathBuf,
    pub id: String,
}

impl DiscordUser {
    pub fn new(username: String) -> Self {
        Self {
            username,
            pfp: Path::new("public/assets/PlaceHolderPfp.jpg").to_owned(),
            id: "temp".to_string(),
        }
    }
}
