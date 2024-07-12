use gtk4::{prelude::*, Button, Stack, Entry, Orientation};
use std::rc::Rc;
use std::{fs::File, io::Write, sync::Arc};
use reqwest::header::HeaderValue;
use crate::runtime;
use crate::discord::{get_data::get_login_by_token, discord_endpoints::AUTH_URL};
use crate::LoginInfo;


pub fn login_page(parent_stack: Rc<Stack>) {

    let login = gtk4::Box::new(Orientation::Vertical, 5);
    let token_entry = Entry::new();
    token_entry.set_placeholder_text(Some("Place token here."));
    login.append(&token_entry);

    let submit_token = Button::new();
    submit_token.set_label("Submit");
    login.append(&submit_token);

    parent_stack.add_child(&login);

    submit_token.connect_clicked( move |_| {
        let entered_text = String::from(token_entry.text());

        if entered_text.is_empty() {
            return;
        }

        let user = LoginInfo {
            discord_token: Some(entered_text),
        };

        if let Some(token) = &user.discord_token {

            let mut data_file = File::create("./public/loginInfo").expect("creation failed");
            data_file.write_all(token.as_bytes()).expect("Write Failed");
            let token = Arc::new(token.clone());

            runtime().spawn( async move {
                let header_value = HeaderValue::from_str(&token.clone());
                let response = get_login_by_token(AUTH_URL, header_value.unwrap()).await;
            });

            chat_page(parent_stack.clone(), user);
            parent_stack.set_visible_child_name("chats");
            parent_stack.remove(&login);
        } else {
            println!("No token entered.");
        }

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

