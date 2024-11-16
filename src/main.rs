use std::{borrow::Borrow, cell::RefCell, fs::File, rc::Rc, str::FromStr};

use auth::{AuthStore, Platform};
use backend::Messenger;
#[cfg(all(not(debug_assertions), unix))]
use daemonize::Daemonize;
use slint::ComponentHandle;
use surf::StatusCode;
use crate::backend::discord::rest_api;

mod auth;
mod backend;

slint::include_modules!();

fn main() {
    // Token Store
    let auth_store = Rc::new(RefCell::new(AuthStore::new("public/LoginInfo".into())));

    #[cfg(not(debug_assertions))]
    {
        #[cfg(unix)]
        {
            let stdout = File::create("/tmp/record.out").unwrap();
            let stderr = File::create("/tmp/record.err").unwrap();

            let daemonize = Daemonize::new()
                .pid_file("/tmp/record.pid")
                .stdout(stdout)
                .stderr(stderr);

            match daemonize.start() {
                Ok(_) => println!("Daemon started"),
                Err(e) => eprintln!("Error, {}", e),
            }
        }
    }

    let ui = MainWindow::new().unwrap();

    // === Sign in, if user has a token ===
    if !(*auth_store).borrow().is_empty() {
        let mut auth_store = (*auth_store).borrow_mut();

        let mut auths_to_remove = vec![];
        smol::block_on(async {
            for (i, auth) in auth_store.iter_mut().enumerate() {
                let messenger = auth.get_messanger();

                let convo = messenger.get_conversation().await;

                println!("{:#?}", convo);
                if let Err(_) = convo {
                    auths_to_remove.push(i);
                } else {
                    ui.set_page(Page::Main)
                };
            }
        });

        auths_to_remove.sort_by(|a, b| b.cmp(a));
        auths_to_remove.iter().for_each(|i| auth_store.remove(*i));
    }

    // === Chat ===
    let chat = ui.global::<ChatGlobal>();
    let conversations = Rc::new(slint::VecModel::<Conversation>::from(vec![]));
    chat.set_conversations(conversations.clone().into());
    conversations.push(Conversation {
        id: "test".into(),
        image: "".into(),
        name: "abc".into(),
        platform: "Discord".into(),
    });

    // === Form ===
    let form = ui.global::<SignInGlobal>();
    form.on_tokenSubmit({
        let ui = ui.clone_strong();
        let auth_store = auth_store.clone();
        move |string_auth| {
            let platform = Platform::from_str(&string_auth.platform.to_string()).unwrap();
            let token = string_auth.token.to_string();
            let token_check: Result<serde_json::Value, surf::Error> = smol::block_on({
                let token = token.clone();
                async {
                    match platform {
                        Platform::Discord => {
                            let discord = rest_api::Discord { token: token.clone().into() };
                            if let Err(_) = discord.get_profile().await {
                                return Err(surf::Error::from_str(
                                    StatusCode::Unauthorized,
                                    "TODO: prob. an outdated token",
                                ));
                            }
                            //Check if the token for this platform exists, delete and add the new one
                            (*auth_store)
                                .borrow_mut()
                                .add(Platform::from(platform), token);
                        }
                        _ => return Err(surf::Error::from_str(
                            StatusCode::Unauthorized,
                            "TODO: prob. an outdated token",
                        )),
                    }
                    Ok(serde_json::Value::Null)
                }
            });
            if token_check.is_err() { return; }
            ui.set_page(Page::Main);
        }
    });

    ui.run().unwrap();
}
