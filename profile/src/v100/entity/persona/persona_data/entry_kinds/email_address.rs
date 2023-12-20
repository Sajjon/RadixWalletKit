use crate::v100::entity::persona::persona_data::entry::{BasePersonaDataEntry, Entry};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct EmailAddress {
    email: String,
}

impl BasePersonaDataEntry for EmailAddress {
    fn description(&self) -> String {
        self.email.clone()
    }
}

impl EmailAddress {
    pub fn new() -> Self {
        Self {
            email: String::new(),
        }
    }
}

impl Default for EmailAddress {
    fn default() -> Self {
        Self {
            email: "".to_string(),
        }
    }
}
