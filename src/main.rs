use std::collections::HashMap;
use gtk4::gio::Settings;
use gtk4::{gdk, gio, glib, Application, ApplicationWindow, Stack};
use gtk4::{prelude::*, CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::rc::Rc;
use std::sync::OnceLock;
use tokio::runtime::Runtime;
use tokio::sync::oneshot;
use ui::pages::{chat_page, login_page};
use crate::discord::rest_api::discord_endpoints::{ApiEndpoints, ApiResponse, AuthedUser};

pub mod discord;
pub mod ui;

pub struct LoginInfo {
    discord_token: Option<String>,
}

const APP_ID: &str = "laycookie.record";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(|app| {
        // Link css
        let css = CssProvider::new();

        let path = Path::new("src/ui/theme/main.css");
        css.load_from_path(path);

        gtk4::style_context_add_provider_for_display(
            &gdk::Display::default().unwrap(),
            &css,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        build_ui(app);
    });

    // Connect to "activate" signal of `app`
    // app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Record")
        .build();

    let stack = Stack::new();
    match get_tokens() {
        Some(login_info) => chat_page(stack.clone(), login_info, None),
        None => login_page(stack.clone()),
    };

    window.set_child(Some(&stack));
    // Present window
    window.present();
}

pub fn get_tokens() -> Option<LoginInfo> {
    let tokens_path = "public/loginInfo";

    match File::open(tokens_path) {
        Ok(mut login_info) => {
            let mut token_data = String::new();
            login_info.read_to_string(&mut token_data).unwrap();

            Some(LoginInfo {
                discord_token: Some(token_data),
            })
        }
        Err(_) => None,
    }
}

pub fn invalidete_token(tokens: &mut LoginInfo) {
    tokens.discord_token = None;
}

pub fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."))
}
