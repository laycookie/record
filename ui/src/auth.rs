use adaptors::{discord::Discord, Messanger};
use std::{
    fs::{File, OpenOptions},
    future::Future,
    io::{BufRead, BufReader, Seek, SeekFrom, Write},
    path::PathBuf,
    pin::Pin,
    rc::Rc,
    str::FromStr,
};
use strum::{Display, EnumString};

#[derive(Debug, Clone, EnumString, Display)]
#[repr(u8)]
pub(crate) enum Platform {
    Discord,
}
impl Platform {
    pub fn get_messanger(&self, auth: String) -> Rc<dyn Messanger> {
        match self {
            Platform::Discord => Rc::new(Discord::new(&auth)),
        }
    }
}

type AuthChangeCallback = dyn Fn(Vec<Rc<dyn Messanger>>) -> Pin<Box<dyn Future<Output = ()>>>;
pub(super) struct AuthStore {
    messangers: Vec<Rc<dyn Messanger>>,
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
            .unwrap();

        let buf_reader = BufReader::new(&auth_file);

        let mut messangers = Vec::new();
        for auth_line in buf_reader.lines() {
            let auth_line = auth_line.unwrap(); // For now we don't handle this

            let (platform, token) = match auth_line.split_once(":") {
                Some(auth_data) => auth_data,
                None => continue,
            };

            // In theory should never return false
            let mes: Rc<dyn Messanger> = match Platform::from_str(platform) {
                Ok(Platform::Discord) => Rc::new(Discord::new(token)),
                Err(_) => todo!(),
            };
            messangers.push(mes);
        }
        AuthStore {
            file: auth_file,
            messangers,
            auth_change_listeners: Vec::new(),
        }
    }

    pub fn get_messangers(&self) -> &Vec<Rc<dyn Messanger>> {
        &self.messangers
    }

    pub fn add_listner(&mut self, callback: Box<AuthChangeCallback>) {
        self.auth_change_listeners.push(callback);
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Rc<dyn Messanger>) -> bool,
    {
        self.messangers.retain(f);
        self.file_sync();
        self.dispatch_callbacks();
    }

    // TODO: If something happens to the PC during a write to a file, the app
    // has no way to recover, so we should prob. impliment some messures
    // to prevent this in the future.
    /// Does not trigger callbacks
    pub fn add_auth_silently(&mut self, messangers: Rc<dyn Messanger>) -> bool {
        if !self.messangers.contains(&messangers) {
            self.messangers.push(messangers);
            self.file_sync();
            return true;
        }
        false
    }
    pub fn add_auth(&mut self, messangers: Rc<dyn Messanger>) -> bool {
        if self.add_auth_silently(messangers) {
            self.dispatch_callbacks();
            return true;
        }
        false
    }

    pub fn dispatch_callbacks(&self) {
        smol::block_on(async {
            for c in self.auth_change_listeners.iter() {
                let messangers = self.get_messangers().to_owned();
                c(messangers).await;
            }
        });
    }

    fn file_sync(&mut self) {
        // Prefferably I should just be writing to a new file, and then
        // just swap the files when I'm finished writing, but realisticly
        // there is no point in this type of redundncy at this point in the
        // project.
        self.file.seek(SeekFrom::Start(0)).unwrap();
        self.file.set_len(0).unwrap();
        self.messangers.iter().for_each(|messangers| {
            writeln!(
                self.file,
                "{}:{}",
                messangers.as_ref().name(),
                messangers.as_ref().auth()
            )
            .unwrap();
        });
    }

    pub fn is_empty(&self) -> bool {
        self.messangers.is_empty()
    }
}
