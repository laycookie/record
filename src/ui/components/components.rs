use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};

use gtk4::{prelude::*, Align, Button, Entry, Image, Label, Orientation, Stack, Widget};

struct Message {
    sender_other_then_client: Option<String>,
    message_text: Label,
    message_element: gtk4::Box,
}
struct Channel {
    channel_id: String,
    channel_element: gtk4::Button,
}
// ===
//
pub struct Channels {
    channels: Vec<Channel>,
    chat: Rc<RefCell<Chat>>,
    chat_stack: Stack,
    pub channels_element: gtk4::Box,
}

trait ChatSelecter {
    fn get_chat_stack(&self) -> Stack;
    fn get_chat(&self) -> impl IsA<Widget>;
}

impl Channels {
    pub(crate) fn new(chat: Rc<RefCell<Chat>>, chat_stack: Stack) -> Self {
        let channels_element = gtk4::Box::new(Orientation::Vertical, 5);
        let channels = vec![];

        Self {
            channels,
            channels_element,
            chat,
            chat_stack,
        }
    }
    pub(crate) fn add_channel(&mut self, channel_id: String, username: String, icon_path: PathBuf) {
        let button_contents = gtk4::Box::new(Orientation::Horizontal, 5);
        button_contents.set_width_request(120);

        let username_label = Label::new(Some(&username));
        let avatar = Image::from_file(icon_path.clone());

        button_contents.append(&avatar);
        button_contents.append(&username_label);

        let contact = Button::new();

        contact.connect_clicked({
            let chat_stack = self.chat_stack.clone();
            let chat = self.chat.clone();
            let channel_id = channel_id.clone();
            move |_| {
                let mut chat = (*chat).borrow_mut();
                chat_stack.set_visible_child(&chat.chat_element);
                // I have no clue why I got to copy all of this but Im too tiered of fighting with
                // the compiler at this point.
                chat.switch_chat(username.clone(), icon_path.clone(), channel_id.clone());
            }
        });
        contact.set_child(Some(&button_contents));

        self.channels_element.append(&contact);
        self.channels.push(Channel {
            channel_id,
            channel_element: contact,
        });
    }
    pub fn remove_channel(&mut self, channel_id: String) {
        self.channels.retain(|channel| {
            if channel.channel_id == channel_id {
                self.channels_element.remove(&channel.channel_element);
                return true;
            }
            false
        });
    }
}

pub struct FriendList {
    friends: HashMap<String, Button>,
    pub friend_list_element: gtk4::Box,
    chat_stack: Stack,
    chat: Rc<RefCell<Chat>>,
}

impl FriendList {
    pub(crate) fn new(chat: Rc<RefCell<Chat>>, chat_stack: Stack) -> Self {
        let friend_list_element = gtk4::Box::new(Orientation::Vertical, 4);
        let friends = HashMap::new();

        Self {
            friends,
            friend_list_element,
            chat_stack,
            chat,
        }
    }
    pub fn add_friend(&mut self, user_id: String, username: String, icon_path: PathBuf) {
        let user_box = gtk4::Box::new(Orientation::Horizontal, 5);

        let username_label = Label::new(Some(&username));
        let pfp = Image::from_file(icon_path.clone());

        user_box.append(&pfp);
        user_box.append(&username_label);

        let button = Button::new();
        button.connect_clicked({
            let chat_stack = self.chat_stack.clone();
            let chat = self.chat.clone();
            let user_id = user_id.clone();
            move |_| {
                let mut chat = (*chat).borrow_mut();
                chat_stack.set_visible_child(&chat.chat_element);
                // I have no clue why I got to copy all of this but Im too tiered of fighting with
                // the compiler at this point.
                chat.open_chat(username.clone(), icon_path.clone(), user_id.clone());
            }
        });
        button.set_child(Some(&user_box));

        self.friend_list_element.append(&button);
        self.friends.insert(user_id, button);
    }
    fn remove_friend(&mut self, user_id: String) {
        let friend_element = self.friends.get(&user_id).unwrap();
        self.friend_list_element.remove(friend_element);
        self.friends.remove(&user_id);
    }
}

pub struct Chat {
    chat_label: Label,
    chat_icon: Image,
    messages_element: gtk4::Box,
    messages: Vec<Message>,
    pub chat_element: gtk4::Box,
}

impl Chat {
    pub(crate) fn new() -> Self {
        let chat_element = gtk4::Box::new(Orientation::Vertical, 0);
        // Chat infographic
        let chat_label = Label::new(None);
        let chat_icon = Image::new();
        let chat_info = gtk4::Box::new(Orientation::Horizontal, 0);

        chat_info.append(&chat_icon);
        chat_info.append(&chat_label);

        chat_element.append(&chat_info);
        // Messages
        let message_container = gtk4::ScrolledWindow::new();
        let messeges = gtk4::Box::new(Orientation::Vertical, 0);
        messeges.set_vexpand(true);

        message_container.set_child(Some(&messeges));
        chat_element.append(&message_container);
        // Text field
        let messege_field = Entry::new();

        chat_element.append(&messege_field);

        Self {
            chat_label,
            chat_icon,
            messages: vec![],
            messages_element: messeges,
            chat_element,
        }
    }

    fn append_message(&mut self, text: String, sender: Option<String>) {
        let message_box = gtk4::Box::new(Orientation::Horizontal, 0);
        let message = gtk4::Label::new(Some(&text));

        message.add_css_class("message");
        if sender.is_none() {
            message_box.set_halign(Align::End);
            message.add_css_class("user-message");
        }

        message_box.append(&message);
        self.messages_element.append(&message_box);

        self.messages.push(Message {
            sender_other_then_client: sender,
            message_text: message,
            message_element: message_box,
        });
    }

    fn clear_messages(&mut self) {
        for message in &self.messages {
            self.messages_element.remove(&message.message_element);
        }
        self.messages.clear();
    }

    fn switch_chat(&mut self, name: String, icon_path: PathBuf, channel_id: String) {
        // Switch chat Info
        self.chat_label.set_text(&name);
        self.chat_icon.set_from_file(Some(icon_path));
        // Remove old messages
        self.clear_messages();
        // TODO: Add new messeges
    }

    fn open_chat(&mut self, name: String, icon_path: PathBuf, channel_id: String) {
        // Switch chat Info
        self.chat_label.set_text(&name);
        self.chat_icon.set_from_file(Some(icon_path));
        // Remove old messages
        self.clear_messages();
        // TODO: Add new messeges
    }
}
