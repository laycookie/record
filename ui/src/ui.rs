use adaptors::{types::Conversation as Conv, Messanger};
use slint::{ComponentHandle, VecModel};

use crate::{auth::Platform, ChatGlobal, Conversation, MainWindow, Page, SignInGlobal};
use std::{
    cell::{Cell, RefCell},
    error::Error,
    rc::Rc,
    str::FromStr,
};

use crate::auth::AuthStore;

pub struct AuthManagmentForm<'a> {
    ui: &'a MainWindow,
    auth_store: Rc<RefCell<AuthStore>>,
}
impl<'a> AuthManagmentForm<'a> {
    pub fn new(ui: &'a MainWindow, auth_store: Rc<RefCell<AuthStore>>) -> Self {
        let form = ui.global::<SignInGlobal>();

        form.on_tokenSubmit({
            let auth_store = auth_store.clone();
            let ui = ui.clone_strong();
            move |string_auth| {
                // Add auth to store
                let platform = Platform::from_str(&string_auth.platform.to_string()).unwrap();
                // TODO: Generalize later
                let auth = string_auth.token.to_string();
                let messanger = platform.get_messanger(auth);
                auth_store.borrow_mut().add_auth(messanger);
                ui.set_page(Page::Main);
            }
        });

        Self { ui, auth_store }
    }
}

// === Chat ===

struct Chat {
    // messages: Vec<_>,
}

impl Chat {
    fn new() -> Self {
        Self {}
    }
}

pub struct ConversationCenter {
    conversation: ConversationData,
    contacts: (),
    chat: Chat,
}
impl ConversationCenter {
    pub fn new(ui: &MainWindow, auth: &mut AuthStore) -> Self {
        Self {
            conversation: ConversationData::new(ui, auth),
            contacts: (),
            chat: Chat::new(),
        }
    }
}

// ===
impl From<&Conv> for Conversation {
    fn from(value: &Conv) -> Self {
        Conversation {
            id: value.id.clone().into(),
            name: value.name.clone().into(),
        }
    }
}

struct ConversationData {
    conversations: Rc<Cell<Vec<(Rc<dyn Messanger>, Vec<Conv>)>>>,
    ui_conversations: Rc<VecModel<Conversation>>,
}
impl ConversationData {
    fn new(ui: &MainWindow, auths: &mut AuthStore) -> Self {
        // UI
        let chat = ui.global::<ChatGlobal>();

        let conversations = Rc::new(Cell::new(Vec::new()));
        let ui_conversations = Rc::new(slint::VecModel::<Conversation>::from(vec![]));
        chat.set_conversations(ui_conversations.clone().into());

        let conversation_data = Self {
            conversations: conversations.clone(),
            ui_conversations: ui_conversations.clone(),
        };
        // Callbacks
        auths.add_listner(Box::new(move |messangers| {
            let conv = conversations.clone();
            let ui = ui_conversations.clone();
            Box::pin(async move {
                let mut new = Vec::new();
                for messanger in messangers.iter() {
                    let q = messanger.query().unwrap();
                    let c = q.get_conversation().await.unwrap();

                    new.push((messanger.clone(), c));
                    println!("Update ConversationData");
                }

                new.iter()
                    .for_each(|(_, c)| c.iter().for_each(|conv| ui.push(Conversation::from(conv))));
                conv.set(new);
            })
        }));

        conversation_data
    }
}
