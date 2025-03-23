use auth::AuthStore;
use iced::{window, Element, Font, Task};
use pages::{chat::MessangerWindow, Login, MyAppMessage, Page};

mod auth;
mod pages;

const ICON_FONT: Font = Font::with_name("icons");

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting");
    iced::daemon(App::title(), App::update, App::view)
        .run_with(|| App::new())
        .inspect_err(|err| println!("{}", err))?;

    // iced::application(App::title(), App::update, App::view).run();
    // .font(include_bytes!("../fonts/icons.ttf").as_slice())
    Ok(())
}

struct App {
    // TODO: This is a hack but as of now I don't now of a better way
    auth: Box<AuthStore>,
    memoryless_page: Box<dyn Page>,
}
impl Default for App {
    fn default() -> Self {
        let mut auth_store = Box::new(AuthStore::new("./public/LoginInfo".into()));

        let memoryless_page: Box<dyn Page>;
        if auth_store.is_empty() {
            memoryless_page = Box::new(Login::new(&mut auth_store));
        } else {
            let m = MessangerWindow::new(&mut auth_store).unwrap();
            memoryless_page = Box::new(m);
        }

        Self {
            memoryless_page,
            auth: auth_store,
        }
    }
}
impl App {
    fn new() -> (Self, Task<MyAppMessage>) {
        // Init app
        let app = Self::default();

        // Open a window
        let (_window_id, window_task) = window::open(window::Settings {
            ..Default::default()
        });

        let task = window_task.then(|_| Task::none());

        (app, task)
    }
    fn title() -> &'static str {
        "record"
    }
    fn update(&mut self, message: MyAppMessage) -> impl Into<Task<MyAppMessage>> {
        let page = self.memoryless_page.update(message);
        if let Some(p) = page {
            self.memoryless_page = p;
        }
    }
    fn view(&self, window: window::Id) -> Element<MyAppMessage> {
        self.memoryless_page.view()
    }
    // fn view(&self) -> Element<MyAppMessage> {
    //     // self.memoryless_page.view()
    //     Column::new().into()
    // }
}
