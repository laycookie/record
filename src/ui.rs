use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;
use slint::ComponentHandle;
use crate::auth::{AuthStore, Platform};
use crate::{fetch_data, ChatGlobal, Conversation, MainWindow, SignInGlobal};
use crate::backend::Messenger;

pub fn signin_init(ui: &MainWindow, auth_store: &Rc<RefCell<AuthStore>>) {
    let form = ui.global::<SignInGlobal>();
    form.on_tokenSubmit({
        let ui = ui.clone_strong();
        let auth_store = auth_store.clone();
        move |string_auth| {
            // Add auth to store
            let platform = Platform::from_str(&string_auth.platform.to_string()).unwrap();
            let token = string_auth.token.to_string();
            (*auth_store)
                .borrow_mut()
                .add(Platform::from(platform), token);

            // open & refresh ui
            fetch_data(&ui, &auth_store);
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