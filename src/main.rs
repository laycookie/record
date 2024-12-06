use crate::auth::AuthStore;
use crate::ui::signin_init;

use auth::Auth;
use backend::MessengerHTTP;
#[cfg(all(not(debug_assertions), unix))]
use daemonize::Daemonize;
use slint::ComponentHandle;
use std::{cell::RefCell, rc::Rc};
use surf::StatusCode;
use ui::GlobalConversationData;

mod auth;
mod backend;
mod network_req;
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
        auth_store.retain_and_rewrite(|auth| match fetch_data(auth.clone(), &ui) {
            Ok(..) => true,
            Err(error) if error.status() == StatusCode::Unauthorized => {
                eprintln!("Token expired");
                false
            }
            Err(error) => {
                eprintln!("Error Status: {}", error.status());
                true
            }
        });
    } else {
        signin_init(&ui, &Rc::new(RefCell::new(auth_store)));
    }
    ui.run().unwrap();
}

// TODO: Rename to explain that it is refreshes ui
fn fetch_data(auth: Auth, ui: &MainWindow) -> Result<(), surf::Error> {
    smol::block_on(async {
        let messenger = auth.get_messanger();

        let profile = messenger.get_profile().await?;
        let conversations = messenger.get_conversation().await?;
        let contacts = messenger.get_contacts().await?;

        let ui_conversations = GlobalConversationData::new(&ui);
        for con in conversations {
            ui_conversations.add_conversation(con, auth.clone());
        }
        // Update ui
        ui.set_page(Page::Main);
        Ok(())
    })
}
