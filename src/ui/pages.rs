use gtk4::{prelude::*, Button, Entry, Orientation, Stack, Image};
use std::cell::RefCell;
use std::rc::Rc;
use std::{fs::File, io::Write};
use gtk4::glib::clone;
use super::components::components::{Channels, Chat, Component, FriendList, Guilds};
use gtk4::glib;
use crate::discord::rest_api::discord_endpoints::ApiResponse;
use crate::discord::rest_api::utils::init_data;
use crate::LoginInfo;

pub fn login_page(parent_stack: Stack) {
    let login = gtk4::Box::new(Orientation::Vertical, 5);
    let token_entry = Entry::new();
    token_entry.set_placeholder_text(Some("Place token here."));
    login.append(&token_entry);

    let submit = Button::new();
    submit.set_label("Submit");
    login.append(&submit);
    parent_stack.add_child(&login);

    let submit_token = {
        let token_entry = token_entry.clone();
        move || {
            let entered_token = String::from(token_entry.text());
            if entered_token.is_empty() {
                return;
            }
            let mut data_file = File::create("./public/loginInfo").expect("creation failed");
            data_file
                .write_all(entered_token.as_bytes())
                .expect("Write Failed");
            let user = LoginInfo {
                discord_token: Some(entered_token.clone()),
            };
            let data = match init_data(&entered_token) {
                Ok(json) => {
                    json
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };


            chat_page(parent_stack.clone(), user, Some(data));
            parent_stack.set_visible_child_name("chats");
            parent_stack.remove(&login);
        }
    };

    submit.connect_clicked({
        let submit_token = submit_token.clone();
        move |_| submit_token()
    });
    token_entry.connect_activate({
        let submit_token = submit_token.clone();
        move |_| submit_token()
    });
}

pub fn chat_page(parent_stack: Stack, token_data: LoginInfo, info: Option<Vec<ApiResponse>>) {
    //TODO:Connect Websocket
    let info = info.unwrap_or_else(|| init_data(&token_data.discord_token.unwrap()).unwrap());
    let sections = gtk4::Box::new(Orientation::Horizontal, 0);

    // === Main Panel ===
    let chat_area = Stack::new();
    chat_area.set_hexpand(true);

    let chat = Rc::new(RefCell::new(Chat::new()));
    let mut friend_list = FriendList::new(chat.clone(), chat_area.clone());

    chat_area.add_named(&friend_list.friend_list_element, Some("friend_element"));
    chat_area.add_named(&chat.borrow().chat_element, Some("chat_element"));

    // === Sidebar ===
    let sidebar = gtk4::Box::new(Orientation::Vertical, 20);

    //Guild Panel
    let mut guild_bar = Guilds::new(chat_area.clone());

    //==="Friend" Button===
    let friend_box = gtk4::Box::new(Orientation::Vertical, 5);
    let friends = Button::new();
    friends.set_label("Friends");
    friends.connect_clicked(clone!(
        @weak chat_area =>
        move |_| {
            chat_area.set_visible_child_name("friend_element");
        }));

    sidebar.append(&friends);

    // DM list
    let mut channel_list = Channels::new(chat, chat_area.clone());
    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_child(Some(&channel_list.channels_element));

    sidebar.append(&scroll);
    // ===
    sections.append(&guild_bar.guilds_element);
    sections.append(&sidebar);
    sections.append(&chat_area);

    // === INITING DATA FROM SERVER


    for i in info {
        match i {
            ApiResponse::Friends(friends) => {
                friend_list.load_new_data(friends);
            }
            ApiResponse::Channels(channels) => {
                channel_list.load_new_data(channels);
            }
            ApiResponse::Guilds(guilds) => {
                guild_bar.load_new_data(guilds);
            }
            _ => println!("nothing")
        }
    }

    parent_stack.add_named(&sections, Some("chats"));
}

