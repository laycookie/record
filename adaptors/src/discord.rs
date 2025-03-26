use std::cell::Cell;

use crate::{Messanger, MessangerQuery, ParameterizedMessangerQuery};

pub mod json_structs;
pub mod rest_api;

pub struct Discord {
    token: String, // TODO: Make it secure
    // Data
    dms: Cell<Vec<json_structs::Channel>>,
}

impl Discord {
    pub fn new(token: &str) -> Discord {
        Discord {
            token: token.into(),
            dms: Cell::new(Vec::new()),
        }
    }
}

impl Messanger for Discord {
    fn name(&self) -> String {
        "Discord".into()
    }
    fn auth(&self) -> String {
        self.token.clone()
    }
    fn query(&self) -> Option<&dyn MessangerQuery> {
        Some(self)
    }
    fn param_query(&self) -> Option<&dyn ParameterizedMessangerQuery> {
        Some(self)
    }
}
