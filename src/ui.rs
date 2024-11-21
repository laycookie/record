use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;
use secure_string::SecureString;
use slint::ComponentHandle;
use crate::auth::{AuthStore, Platform};
use crate::{fetch_data, ChatGlobal, Conversation, MainWindow, SignInGlobal};
use crate::backend::Messenger;

pub fn signin_init(ui: &MainWindow, auth_store: &Rc<RefCell<AuthStore>>) {
    let form = ui.global::<SignInGlobal>();
    form.on_tokenSubmit(
        {
            let auth_store = auth_store.clone();
            let ui = ui.clone_strong();
            move |string_auth| {
                // Add auth to store
                let platform = Platform::from_str(&string_auth.platform.to_string()).unwrap();

                let token = string_auth.token.to_string();
                // open & refresh ui
                if let Err(error) = fetch_data(platform.clone(), SecureString::from_str(&token.clone()).unwrap(), &ui) {
                    eprintln!("Error Status: {}", error.status());
                    return;
                }
                //Store token if successful fetch
                (*auth_store).borrow_mut().add(platform, token);
            }
        });
}
pub fn chat_init(ui: &MainWindow, conv: Vec<impl Into<Conversation>>) {
    let chat = ui.global::<ChatGlobal>();
    let conversation = Rc::new(slint::VecModel::<Conversation>::from(vec![]));
    chat.set_conversations(conversation.clone().into());
    for convo in conv {
        conversation.push(convo.into());
    }
}