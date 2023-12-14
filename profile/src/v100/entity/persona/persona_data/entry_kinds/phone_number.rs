use crate::v100::entity::persona::persona_data::entry::{BasePersonaDataEntry, Entry};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct PhoneNumber {}

impl BasePersonaDataEntry for PhoneNumber {
    fn embed(&self) -> Entry {
        todo!()
    }

    fn description(&self) -> String {
        todo!()
    }
}
