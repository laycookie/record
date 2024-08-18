use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};
use std::path::Path;
use gtk4::{prelude::*, Align, Button, Entry, Image, Label, Orientation, Stack, Widget};
use tokio::sync::oneshot;
use crate::discord::rest_api::discord_endpoints;
use crate::discord::rest_api::discord_endpoints::{ApiEndpoints, ApiResponse, Friend};
use crate::discord::rest_api::utils::download_image;
use crate::{get_tokens, runtime};


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
trait ChatSelecter {
    fn get_chat_stack(&self) -> Stack;
    fn get_chat(&self) -> impl IsA<Widget>;
}
pub struct Channels {
    channels: Vec<Channel>,
    chat: Rc<RefCell<Chat>>,
    chat_stack: Stack,
    pub channels_element: gtk4::Box,
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
                chat.open_chat(username.clone(), icon_path.clone(), channel_id.clone());
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
                let user_id = user_id.clone();
                let mut chat = (*chat).borrow_mut();
                chat_stack.set_visible_child(&chat.chat_element);

                let (tx, rx) = oneshot::channel();
                runtime().spawn(async move {
                    let mut headers = HashMap::new();
                    headers.insert("Authorization", get_tokens().unwrap().discord_token.unwrap());

                    let messages = ApiEndpoints::GetChannels(Some(user_id)).call(headers).await.unwrap();
                    tx.send(messages).unwrap();
                });
                if let ApiResponse::Channels(channels) = rx.blocking_recv().unwrap() {
                    chat.open_chat(username.clone(), icon_path.clone(), channels[0].id.clone());
                }
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
    pub selected_channel_id: Option<String>,
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
            selected_channel_id: None,
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

    fn open_chat(&mut self, name: String, icon_path: PathBuf, channel_id: String) {

        if self.selected_channel_id == Some(channel_id.clone()) {
            return;
        }
        self.selected_channel_id.replace(channel_id.clone());

        // Switch chat Info
        self.chat_label.set_text(&name);
        self.chat_icon.set_from_file(Some(icon_path));
        // Remove old messages
        self.clear_messages();
        let (tx, rx) = oneshot::channel();
        runtime().spawn(async move {
            let mut headers = HashMap::new();
            headers.insert("Authorization", get_tokens().unwrap().discord_token.unwrap());

            let messages = ApiEndpoints::GetMessages(channel_id, None, 50).call(headers).await.unwrap();
            tx.send(messages).unwrap();
        });

        if let ApiResponse::Messeges(messages) = rx.blocking_recv().unwrap() {
            self.load_new_data(messages);
        }
    }
}


pub trait Component<T> {
    fn load_new_data(&mut self, data: Vec<T>);
}
impl Component<discord_endpoints::Channel> for Channels {
    fn load_new_data(&mut self, data: Vec<discord_endpoints::Channel>) {
        for c in data {
            let recipient = c.recipients.last().unwrap();
            let channel_id = c.id.clone();

            let username = match c.name {
                Some(name) => name,
                None => recipient.username.clone(),
            };

            let (url, data_path, pfp_id) = match c.icon {
                Some(pfp) => (
                    format!(
                        "https://cdn.discordapp.com/channel-icons/{}/{}.png?size=80",
                        c.id, pfp
                    ),
                    Path::new(&format!("public/Discord/Channels/{}", channel_id))
                        .to_owned(),
                    pfp,
                ),
                None => (
                    format!(
                        "https://cdn.discordapp.com/avatars/{}/{}.png?size=80",
                        recipient.id, recipient.avatar.clone()
                    ),
                    Path::new(&format!("public/Discord/Users/{}", recipient.id))
                        .to_owned(),
                    recipient.avatar.clone(),
                ),
            };

            let pfp = data_path.join(&pfp_id);

            if !pfp.exists() {
                runtime().spawn({
                    async move {
                        download_image(url, &data_path, pfp_id).await.unwrap();
                    }
                });
            }

            self.add_channel(channel_id, username, pfp);
        }
    }
}

impl Component<Friend> for FriendList {
    fn load_new_data(&mut self, data: Vec<Friend>) {
        for f in data {
            let user_id = f.user.id;
            let username = f.user.username;
            let pfp_id = f.user.avatar.unwrap();

            let url = format!(
                "https://cdn.discordapp.com/avatars/{}/{}.png?size=80",
                user_id, pfp_id
            );
            let user_path =
                Path::new(&format!("public/Discord/Users/{}", user_id)).to_owned();
            let pfp = user_path.join(&pfp_id);

            if !pfp.exists() {
                runtime().spawn({
                    async move {
                        download_image(url, &user_path, pfp_id).await.unwrap();
                    }
                });
            }
            self.add_friend(user_id, username, pfp);
        }
    }
}

impl Component<discord_endpoints::Message> for Chat {
    fn load_new_data(&mut self, data: Vec<discord_endpoints::Message>) {
        for message in data.into_iter().rev() {
            self.append_message(message.content, Some(message.author.username));
        }
    }
}