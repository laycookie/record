use std::rc::Rc;
use std::{io::Write, fs::File};
use gtk4::{prelude::*, Button, Stack};
use gtk4::{Entry, Orientation};
struct LoginInfo {
    discord_token: Option<String>
}
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

        let entered_text = token_entry.text().trim().to_string();

        let user = LoginInfo {
            // Filter will convert "Some" to "None" if the entry is empty
            discord_token: Some(entered_text).filter(|s| !s.is_empty()),
        };

        if let Some(token) = &user.discord_token {
            //TODO: add a token validator. If the token is true, save it into file.
            let mut data_file = File::create("loginInfo").expect("creation failed");
            data_file.write(token.as_bytes()).expect("Write Failed");

            parent_stack.set_visible_child_name("chats");
            parent_stack.remove(&login);

        } else {
            println!("No token entered.");
        }

    });
}

pub fn chat_page(parent_stack: Rc<Stack>) {
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
