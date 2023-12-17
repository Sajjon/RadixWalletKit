use crate::v100::entity::persona::persona_data::entry::{BasePersonaDataEntry, Entry};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct PhoneNumber {
    number: String,
}

impl BasePersonaDataEntry for PhoneNumber {
    fn description(&self) -> String {
        self.number.clone()
    }
}
