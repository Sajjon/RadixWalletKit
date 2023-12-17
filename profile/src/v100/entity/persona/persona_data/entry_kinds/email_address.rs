use crate::v100::entity::persona::persona_data::entry::{BasePersonaDataEntry, Entry};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct EmailAddress {
    email: String,
}

impl BasePersonaDataEntry for EmailAddress {
    fn description(&self) -> String {
        self.email.clone()
    }
}
