use std::rc::Rc;

use gtk4::{prelude::*, Button, Stack};
use gtk4::{Entry, Orientation};

pub fn login_page(parent_stack: Rc<Stack>) {
    let login = gtk4::Box::new(Orientation::Vertical, 5);

    let token_entry = Entry::new();
    token_entry.set_placeholder_text(Some("Place token here."));
    login.append(&token_entry);

    let submit_token = Button::new();
    submit_token.set_label("Sumbit");
    login.append(&submit_token);

    parent_stack.add_child(&login);

    submit_token.connect_clicked(move |_| {
        parent_stack.set_visible_child_name("chats");
        parent_stack.remove(&login);
    });
}

pub fn chat_page(parrent_stack: Rc<Stack>) {
    let sections = gtk4::Box::new(Orientation::Horizontal, 0);
    let chats = gtk4::Box::new(Orientation::Vertical, 5);
    sections.append(&chats);

    for _ in 0..5 {
        let chat = Button::new();
        chat.set_label("test");
        chats.append(&chat);
    }

    parrent_stack.add_named(&sections, Some("chats"));
}
