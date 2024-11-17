use std::{borrow::Borrow, cell::RefCell, fs::File, rc::Rc, str::FromStr};

use auth::{AuthStore, Platform};
use backend::Messenger;
#[cfg(all(not(debug_assertions), unix))]
use daemonize::Daemonize;
use slint::ComponentHandle;
use crate::ui::{chat_init, signin_init};

mod auth;
mod backend;
mod ui;
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
        fetch_data(&ui, &auth_store.clone());
    }

    // === Chat ===


    // === Form ===
    signin_init(&ui, &auth_store);

    ui.run().unwrap();
}

// TODO: Rename to explain that it is refreshes ui
fn fetch_data(ui: &MainWindow, auth_store: &Rc<RefCell<AuthStore>>) {
    let mut auth_store = auth_store.borrow_mut();

    let mut auths_to_remove = vec![];
    smol::block_on(async {
        for (i, auth) in auth_store.iter_mut().enumerate() {
            let messenger = auth.get_messanger();

            // Fetch data
            let profile = messenger.get_profile().await;
            let conversations = messenger.get_conversation().await;
            let contacts = messenger.get_contacts().await;

            // Check if any req failed
            let (Ok(profile), Ok(conv), Ok(contact)) = (profile, conversations, contacts) else {
                auths_to_remove.push(i);
                continue;
            };

            println!("{:#?}\n{:#?}\n{:#?}", profile, &conv, contact);
            // Update ui
            ui.set_page(Page::Main);
            chat_init(ui, conv);
        }
    });

    auths_to_remove.sort_by(|a, b| b.cmp(a));
    auths_to_remove.iter().for_each(|i| auth_store.remove(*i));
}
