use std::pin::Pin;

use auth::AuthStore;
use iced::{window, Element, Font, Task};
use pages::{Login, MyAppMessage, Page};

mod auth;
mod pages;

const ICON_FONT: Font = Font::with_name("icons");

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    iced::daemon(App::title(), App::update, App::view)
        .run_with(|| App::new())
        .inspect_err(|err| println!("{}", err))?;
    // .font(include_bytes!("../fonts/icons.ttf").as_slice())
    Ok(())
}

struct App {
    // TODO: This is a hack but as of now I don't now of a better way
    auth: Pin<Box<AuthStore>>,
    memoryless_page: Box<dyn Page>,
}
impl App {
    fn new() -> (Self, Task<MyAppMessage>) {
        // Init app
        let mut auth = Box::pin(AuthStore::new("./public/LoginInfo".into()));
        let app = Self {
            memoryless_page: Box::new(Login::new(&mut auth)),
            auth,
        };

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
}
