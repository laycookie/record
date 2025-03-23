use crate::{Messanger, MessangerQuery, ParameterizedMessangerQuery};

pub mod json_structs;
pub mod rest_api;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Discord {
    token: String, // TODO: Make it unsecure
}

impl Discord {
    pub fn new(token: &str) -> Discord {
        Discord {
            token: token.into(),
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
