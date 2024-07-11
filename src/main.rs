use std::fs::File;
use std::io::Read;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow, Stack};
use ui::pages::{chat_page, login_page};

pub mod ui;

pub struct LoginInfo {
    discord_token: Option<String>,
}

const APP_ID: &str = "org.gtk_rs.record";

fn main() -> glib::ExitCode {
    //TEST
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

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
    let stack_rc = Rc::new(stack.clone());
    match get_tokens() {
        Some(login_info) => chat_page(stack_rc.clone(), login_info),
        None => login_page(stack_rc.clone()),
    };

    window.set_child(Some(&stack));

    // Present window
    window.present();
}

fn get_tokens() -> Option<LoginInfo> {
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
