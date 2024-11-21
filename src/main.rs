use std::{cell::RefCell, rc::Rc};
use secure_string::SecureString;
use auth::{AuthStore};
use backend::Messenger;
#[cfg(all(not(debug_assertions), unix))]
use daemonize::Daemonize;
use slint::ComponentHandle;
use surf::StatusCode;
use crate::auth::Platform;
use crate::ui::{chat_init, signin_init};

mod auth;
mod backend;
mod ui;

slint::include_modules!();

fn main() {
    // Token Store
    let mut auth_store = AuthStore::new("public/LoginInfo".into());

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
    if !auth_store.is_empty() {
        auth_store.retain_and_rewrite(|auth| {
            match fetch_data(auth.platform.clone(), auth.token.clone(), &ui) {
                Ok(..) => true,
                Err(error) if error.status() == StatusCode::Unauthorized => {
                    eprintln!("Token expired");
                    false
                }
                _ => {
                    eprintln!("There has been an issue with internet connection");
                    true
                }
            }
        });
    }

    signin_init(&ui, &Rc::new(RefCell::new(auth_store)));
    ui.run().unwrap();
}

// TODO: Rename to explain that it is refreshes ui
fn fetch_data(platform: Platform, token: SecureString, ui: &MainWindow) ->
Result<(), surf::Error> {
    smol::block_on(async {
        let messenger = platform.get_messanger(token);
        let profile = messenger.get_profile().await?;
        let conversations = messenger.get_conversation().await?;
        let contacts = messenger.get_contacts().await?;
        ui.set_page(Page::Main);
        chat_init(&ui, conversations);
        Ok(())
    })
}