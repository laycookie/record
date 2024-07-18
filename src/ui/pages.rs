use gtk4::{prelude::*, Button, Entry, Orientation, Stack};

use crate::discord::get_data::init_data;
use crate::LoginInfo;
use std::{fs::File, io::Write, rc::Rc};

pub fn login_page(parent_stack: Rc<Stack>) {
    let login = gtk4::Box::new(Orientation::Vertical, 5);
    let token_entry = Entry::new();
    token_entry.set_placeholder_text(Some("Place token here."));
    login.append(&token_entry);

    let submit_token = Button::new();
    submit_token.set_label("Submit");
    login.append(&submit_token);
    parent_stack.add_child(&login);

    submit_token.connect_clicked(move |_| {
        let entered_token = String::from(token_entry.text());
        if entered_token.is_empty() {
            return;
        }

        let _data = match init_data(&entered_token) {
            Ok(json) => json,
            Err(_) => return,
        };
        if init_data(&entered_token).is_err() {
            eprintln!("Token invalide");
            return;
        }

        let mut data_file = File::create("./public/loginInfo").expect("creation failed");
        data_file
            .write_all(entered_token.as_bytes())
            .expect("Write Failed");

        let user = LoginInfo {
            discord_token: Some(entered_token),
        };

        chat_page(parent_stack.clone(), user);
        parent_stack.set_visible_child_name("chats");
        parent_stack.remove(&login);
    });
}

pub fn chat_page(parent_stack: Rc<Stack>, token_data: LoginInfo) {
    let sections = gtk4::Box::new(Orientation::Horizontal, 0);
    let chats = gtk4::Box::new(Orientation::Vertical, 5);
    sections.append(&chats);

    for _ in 0..5 {
        let chat = Button::new();
        chat.set_label("test");
        chats.append(&chat);
    }

    parent_stack.add_named(&sections, Some("chats"));
}
