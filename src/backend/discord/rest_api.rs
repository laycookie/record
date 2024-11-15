use std::collections::HashMap;

use secure_string::SecureString;

use super::super::utils::http_get;
use crate::backend::Messanger;

pub struct Discord {
    pub token: SecureString,
}

impl Messanger for Discord {
    async fn get_contacts(&self) -> Result<(), surf::Error> {
        let header = vec![("Authorization", self.token.clone().into_unsecure())];
        let json = http_get::<serde_json::Value>(
            "https://discord.com/api/v9/users/@me/relationships",
            header,
        )
        .await?;

        println!("{:#?}", json);

        Ok(())
    }

    fn get_conversations() {
        todo!()
    }

    fn get_guilds() {
        todo!()
    }

    fn get_messanges() {
        todo!()
    }

    fn get_profile() {
        todo!()
    }
}
