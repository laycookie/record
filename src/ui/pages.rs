use gtk4::{prelude::*, Button, Entry, Label, Orientation, Stack};

use super::components::components::user_button;
use crate::discord::get_data::init_data;
use crate::{LoginInfo, runtime};
use std::{fs::File, io::Write, rc::Rc};

use crate::discord::websocket;
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
    runtime().spawn( async move
    {
        websocket::websocket_init(&token_data.discord_token.unwrap()).await;
    });
    let sections = gtk4::Box::new(Orientation::Horizontal, 0);
    let selected_chat = gtk4::Stack::new();
    let selected_chat_rc = Rc::new(selected_chat.clone());
    {
        // === Friend List ===
        let friend_list = gtk4::Box::new(Orientation::Vertical, 4);

        let test = Label::new(Some("friend list"));
        friend_list.append(&test);

        selected_chat.add_named(&friend_list, Some("friend list"));
        // === Chat ===
        let chat = gtk4::Box::new(Orientation::Vertical, 4);

        let test = Label::new(Some("chat"));
        chat.append(&test);

        selected_chat.add_named(&chat, Some("chat"));
    }
    // === Sidebar ===
    let sidebar = gtk4::Box::new(Orientation::Vertical, 20);

    {
        let menue = gtk4::Box::new(Orientation::Vertical, 5);
        let friends = Button::new();
        friends.set_label("Friends");
        friends.connect_clicked(move |_| {
            selected_chat_rc.set_visible_child_name("friend list");
        });
        menue.append(&friends);
        sidebar.append(&menue);
    }

    {
        let scroll = gtk4::ScrolledWindow::new();
        scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);

        let contact_list = gtk4::Box::new(Orientation::Vertical, 5);
        scroll.set_child(Some(&contact_list));
        for _ in 0..20 {
            user_button(&contact_list, Rc::new(selected_chat.clone()));
        }
        sidebar.append(&scroll);
    }

    // ===
    sections.append(&sidebar);
    sections.append(&selected_chat);

    parent_stack.add_named(&sections, Some("chats"));
}
