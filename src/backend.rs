use std::{
    cell::RefCell,
    fmt::Display,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    str::FromStr,
};

use secure_string::SecureString;
use strum::EnumString;

use discord::rest_api::Discord;

pub mod discord {
    pub mod rest_api;
}

#[derive(Debug, Clone, EnumString)]
pub enum Platform {
    Discord,
    Unkown,
}

#[derive(Clone)]
pub struct Auth {
    platform: Platform,
    token: SecureString,
}

impl Display for Auth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token = self.token.clone().into_unsecure(); // TODO: Zeroize
        let r = write!(f, "{:?}:{}", self.platform, token);
        r
    }
}

pub struct AuthStore {
    file: File,
    auths: Vec<Auth>,
}

impl AuthStore {
    pub fn new(path: PathBuf) -> AuthStore {
        let auth_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)
            .unwrap();

        let buf_reader = BufReader::new(&auth_file);

        let mut auths = vec![];
        for auth_line in buf_reader.lines() {
            let auth_line = auth_line.unwrap(); // For now we don't handle this

            let (platform, token) = match auth_line.split_once(":") {
                Some(auth_data) => auth_data,
                None => continue,
            };

            auths.push(Auth {
                platform: Platform::from_str(platform).unwrap(),
                token: token.into(),
            });
        }

        AuthStore {
            file: auth_file,
            auths,
        }
    }

    pub fn add(&mut self, platform: Platform, token: String) {
        let auth = Auth {
            platform,
            token: token.into(),
        };
        // Add to vec
        self.auths.push(auth.clone());
        // Add to File
        write!(self.file, "{}\n", auth).unwrap();
    }

    pub fn is_empty(&self) -> bool {
        self.auths.is_empty()
    }
}

pub trait Messanger {
    // Get data about the messanger
    fn new(auth: Auth) -> impl Messanger {
        match auth.platform {
            Platform::Discord => Discord { token: auth.token },
            Platform::Unkown => todo!("Handle unkown messanger"),
        }
    }

    // Fetch contacts
    fn get_contacts(); // Users from friendlists e.t.c.
    fn get_conversations(); // Also known as DMs
    fn get_guilds(); // Large groups that can have over a 100 people in them.

    // Fetch data based on contacts
    fn get_messanges();
    fn get_profile();
}
