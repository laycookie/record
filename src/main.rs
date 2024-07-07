use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow, Stack};
use ui::pages::{chat_page, login_page};

pub mod ui;

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
    // ===
    login_page(stack_rc.clone());

    chat_page(stack_rc.clone());
    // ===

    window.set_child(Some(&stack));

    // Present window
    window.present();
}
