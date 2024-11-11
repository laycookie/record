use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Read},
    path::PathBuf,
    str::FromStr,
};

use strum::EnumString;

use discord::rest_api::Discord;

pub mod discord {
    pub mod rest_api;
}

#[derive(EnumString)]
pub enum Platform {
    Discord,
    Unkown,
}

struct Auth {
    platform: Platform,
    token: String,
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

        let auths = buf_reader
            .lines()
            .map(|auth_line| {
                let auth_line = auth_line.unwrap(); // For now we don't handle this
                let (platform, token) = auth_line.split_once(":").unwrap();
                Auth {
                    platform: Platform::from_str(platform).unwrap(),
                    token: token.into(),
                }
            })
            .collect::<Vec<_>>();

        AuthStore {
            file: auth_file,
            auths,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.auths.is_empty()
    }
}

pub trait Messanger {
    // Get data about the messanger
    fn new(platform: Platform) -> impl Messanger {
        match platform {
            Platform::Discord => Discord {
                token: "test".into(),
            },
            Platform::Unkown => todo!("temp"),
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
