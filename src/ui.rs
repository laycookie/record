use crate::auth::{Auth, AuthStore, Platform};
use crate::{fetch_data, ChatGlobal, Conversation, MainWindow, SignInGlobal};
use secure_string::SecureString;
use slint::{ComponentHandle as _, VecModel};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
use std::str::FromStr;

pub fn signin_init(ui: &MainWindow, auth_store: &Rc<RefCell<AuthStore>>) {
    let form = ui.global::<SignInGlobal>();

    form.on_tokenSubmit({
        let auth_store = auth_store.clone();
        let ui = ui.clone_strong();
        move |string_auth| {
            // Add auth to store
            let platform = Platform::from_str(&string_auth.platform.to_string()).unwrap();
            let token = string_auth.token.to_string();

            // open & refresh ui
            if let Err(error) = fetch_data(
                Auth::new(platform.clone(), SecureString::from_str(&token).unwrap()),
                &ui,
            ) {
                eprintln!("Error Status: {}", error.status());
                return;
            }
            //Store token if successful fetch
            (*auth_store).borrow_mut().add(platform, token);
        }
    });
}

pub struct GlobalConversationData {
    // Id in this case is defined as a Vec<u8> were the first byte is Platform field as u8, followed by
    // chennel_id that was converted to Vec<u8>
    id_to_auths: Rc<RefCell<HashMap<Vec<u8>, Auth>>>,
    conversations: Rc<VecModel<Conversation>>,
}
impl GlobalConversationData {
    pub fn new(ui: &MainWindow) -> Self {
        let chat = ui.global::<ChatGlobal>();

        // Link slint data with rust
        let conversations = Rc::new(slint::VecModel::<Conversation>::from(vec![]));
        chat.set_conversations(conversations.clone().into());

        // Set callbacks
        let id_to_auths = Rc::new(RefCell::new(HashMap::new()));
        chat.on_set_selected_conversation({
            let id_to_auths = id_to_auths.clone();
            move |conversation| {
                // User is unable to influice the platform field, so if this unwrap is causing an
                // error this is caused by the bug in the code.
                let platform = Platform::from_str(&conversation.platform.to_string()).unwrap();
                let chennel_id = conversation.id.to_string().as_bytes().to_owned();
                let id = [vec![platform as u8], chennel_id].concat();
                println!("{:?}", id_to_auths.borrow().get(&id));
            }
        });

        Self {
            id_to_auths,
            conversations,
        }
    }

    pub fn add_conversation(
        &self,
        conversation: impl Into<Conversation> + Clone + Debug,
        associated_auth: Auth,
    ) {
        let ui_conversation = Into::<Conversation>::into(conversation.clone());

        let platform = Platform::from_str(&associated_auth.platform.to_string()).unwrap();
        let chennel_id = ui_conversation.id.to_string().as_bytes().to_owned();

        let id = [vec![platform as u8], chennel_id].concat();

        (*self.id_to_auths)
            .borrow_mut()
            .insert(id, associated_auth.clone());
        self.conversations.push(ui_conversation);
    }
}
