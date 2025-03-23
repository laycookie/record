use adaptors::{discord::Discord, Messanger as Auth};
use std::{
    fs::{File, OpenOptions},
    future::Future,
    io::{BufRead, BufReader, Seek, SeekFrom, Write},
    path::PathBuf,
    pin::Pin,
    str::FromStr,
};

use crate::pages::login::Platform;

// TODO: Check why this req. pub
pub(crate) struct Messanger {
    pub(crate) auth: Box<dyn Auth>,
    on_disk: bool,
}

type AuthChangeCallback = dyn Fn(&[Messanger]) -> Pin<Box<dyn Future<Output = ()>>>;
pub(super) struct AuthStore {
    messangers: Vec<Messanger>,
    file: File,
    auth_change_listeners: Vec<Box<AuthChangeCallback>>,
}

impl<'a> AuthStore {
    pub fn new(path: PathBuf) -> Self {
        let auth_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)
            .expect(format!("{:#?}", path).as_str());

        let buf_reader = BufReader::new(&auth_file);

        let mut messangers = Vec::new();
        for auth_line in buf_reader.lines() {
            let auth_line = auth_line.unwrap(); // For now we don't handle this

            let (platform, token) = match auth_line.split_once(":") {
                Some(auth_data) => auth_data,
                None => continue,
            };

            // In theory should never return false
            let auth: Box<dyn Auth> = match Platform::from_str(platform).unwrap() {
                Platform::Discord => Box::new(Discord::new(token)),
                Platform::Test => todo!(),
            };

            messangers.push(Messanger {
                auth,
                on_disk: false,
            });
        }
        AuthStore {
            file: auth_file,
            messangers,
            auth_change_listeners: Vec::new(),
        }
    }

    // TODO: This should return a slice
    pub fn get_messangers(&self) -> &[Messanger] {
        &self.messangers[..]
    }

    pub fn add_listner(&mut self, callback: Box<AuthChangeCallback>) {
        self.auth_change_listeners.push(callback);
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Messanger) -> bool,
    {
        self.messangers.retain(f);
        self.save_on_disk();
        self.dispatch_callbacks();
    }

    fn contains_auth(&self, auth: &Box<dyn Auth>) -> bool {
        for i in self.get_messangers() {
            if &i.auth == auth {
                return true;
            }
        }
        false
    }

    /// Does not trigger callbacks
    pub fn add_auth(&mut self, auth: Box<dyn Auth>) -> bool {
        if !self.contains_auth(&auth) {
            self.messangers.push(Messanger {
                auth,
                on_disk: true,
            });
            self.save_on_disk();
            // self.dispatch_callbacks();
            return true;
        }
        false
    }

    pub fn dispatch_callbacks(&self) {
        smol::block_on(async {
            for c in self.auth_change_listeners.iter() {
                let messangers = self.get_messangers();
                c(messangers).await;
            }
        });
    }

    fn save_on_disk(&mut self) {
        // Preferably I should just be writing to a new file, and then
        // just swap the files when I'm finished writing, but realistically
        // there is no point in this type of redundancy at this point in the
        // project.
        self.file.seek(SeekFrom::Start(0)).unwrap();
        self.file.set_len(0).unwrap();
        self.messangers.iter_mut().for_each(|messangers| {
            if messangers.on_disk == false {
                return;
            }

            let auth = messangers.auth.as_ref();
            writeln!(self.file, "{}:{}", auth.name(), auth.auth()).unwrap();
        });
    }

    pub fn is_empty(&self) -> bool {
        self.messangers.is_empty()
    }
}
