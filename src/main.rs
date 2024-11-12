use std::{borrow::BorrowMut, cell::RefCell, rc::Rc, str::FromStr, sync::Mutex};

use backend::{AuthStore, Platform};
#[cfg(all(not(debug_assertions), unix))]
use daemonize::Daemonize;

mod backend;

slint::include_modules!();
fn main() {
    // Token Store
    let auth_store = Rc::new(Mutex::new(AuthStore::new("public/LoginInfo".into())));

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

    let form = ui.global::<SignInGlobal>();
    form.on_tokenSubmit({
        let auth_store = auth_store.clone();
        move |string_auth| {
            let platform = Platform::from_str(&string_auth.platform.to_string()).unwrap();
            let token = string_auth.token.to_string();
            auth_store
                .lock()
                .unwrap()
                .add(Platform::from(platform), token);
        }
    });

    if !auth_store.lock().unwrap().is_empty() {
        ui.set_page(Page::Main);
    }

    ui.run().unwrap();
}
