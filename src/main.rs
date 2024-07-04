use gtk4::{glib, Application, ApplicationWindow, Button, Entry};
use gtk4::{prelude::*, Orientation};

const APP_ID: &str = "org.gtk_rs.record";

fn main() -> glib::ExitCode {
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

    login(&window);

    // Present window
    window.present();
}

fn login(window: &ApplicationWindow) {
    // Create a vertical box layout
    let vbox = gtk4::Box::new(Orientation::Vertical, 5);

    // Token Enetery field
    let entry = Entry::new();
    entry.set_placeholder_text(Some("Place token here."));
    vbox.append(&entry);

    // Submit Token
    let submit = Button::new();
    submit.set_label("Submit");
    submit.connect_clicked(move |_| {
        let token = entry.text();
        println!("{}", token);
    });
    vbox.append(&submit);

    window.set_child(Some(&vbox));
}
