use std::sync::Arc;

use auth::AuthStore;
use iced::{window, Element, Task};
use pages::{chat::MessangerWindow, Login, MyAppMessage, Page};
use smol::lock::RwLock;

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
    memoryless_page: Box<dyn Page>,
}
impl Default for App {
    fn default() -> Self {
        let auth_store = AuthStore::new("./LoginInfo".into());
        let is_store_empty = auth_store.is_empty();
        let auth_store = Arc::new(RwLock::new(auth_store));

        let memoryless_page: Box<dyn Page>;
        if is_store_empty {
            memoryless_page = Box::new(Login::new(auth_store.clone()));
        } else {
            let m =
                smol::block_on(async { MessangerWindow::new(auth_store.clone()).await.unwrap() });
            memoryless_page = Box::new(m);
        }

        Self { memoryless_page }
    }
}
impl App {
    fn title() -> &'static str {
        "record"
    }
    fn update(&mut self, message: MyAppMessage) -> impl Into<Task<MyAppMessage>> {
        match self.memoryless_page.update(message) {
            pages::UpdateResult::Page(page) => {
                self.memoryless_page = page;
                Task::none()
            }
            pages::UpdateResult::Task(task) => task,
            pages::UpdateResult::None => Task::none(),
        }
        // let result = self.memoryless_page.update(message);
        // if let Some(p) = page {
        //     self.memoryless_page = p;
        // }

        // task
    }
    fn view(&self, _window: window::Id) -> Element<MyAppMessage> {
        self.memoryless_page.view()
    }
}
