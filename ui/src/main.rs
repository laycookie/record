use adaptors::MessangerQuery;
use auth::AuthStore;
use daemonize::Daemonize;
use std::{cell::RefCell, error::Error, fs::File, rc::Rc};
use tray_icon::TrayIconBuilder;
use ui::{AuthManagmentForm, ConversationCenter};

mod auth;
mod ui;

slint::include_modules!();

fn main() {
    let mut auth_store = AuthStore::new("./ui/public/LoginInfo".into());
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

    let tray_icon = TrayIconBuilder::new()
        .with_tooltip("system-tray - tray icon")
        .build()
        .unwrap();

    let ui = MainWindow::new().unwrap();
    ConversationCenter::new(&ui, &mut auth_store);

    let auth_store = Rc::new(RefCell::new(auth_store));
    AuthManagmentForm::new(&ui, auth_store.clone());

    if {
        let borrowed_auth = auth_store.borrow();
        !borrowed_auth.is_empty()
    } {
        auth_store.borrow_mut().retain(|message| true);
        ui.set_page(Page::Main);
    };

    ui.run().unwrap();
}

// === Networ ===
