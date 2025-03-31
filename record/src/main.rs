use auth::AuthStore;
use iced::{window, Element, Task};
use pages::{chat::MessangerWindow, Login, MyAppMessage, Page};

mod auth;
mod pages;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting");
    iced::daemon(App::title(), App::update, App::view)
        .run_with(|| {
            let app = App::default();
            let (_window_id, window_task) = window::open(window::Settings::default());

            (app, window_task.then(|_| Task::none()))
        })
        .inspect_err(|err| println!("{}", err))?;

    Ok(())
}

struct App {
    auth: Box<AuthStore>,
    memoryless_page: Box<dyn Page>,
}
impl Default for App {
    fn default() -> Self {
        let mut auth_store = Box::new(AuthStore::new("./LoginInfo".into()));

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
    fn title() -> &'static str {
        "record"
    }
    fn update(&mut self, message: MyAppMessage) -> impl Into<Task<MyAppMessage>> {
        let page = self.memoryless_page.update(message);
        if let Some(p) = page {
            self.memoryless_page = p;
        }
    }
    fn view(&self, _window: window::Id) -> Element<MyAppMessage> {
        self.memoryless_page.view()
    }
}
