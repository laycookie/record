use secure_string::SecureString;

use crate::backend::Messanger;

pub struct Discord {
    pub token: SecureString,
}

impl Messanger for Discord {
    fn get_contacts() {
        todo!()
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
