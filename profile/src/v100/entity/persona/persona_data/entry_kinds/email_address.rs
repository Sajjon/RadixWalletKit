use crate::v100::entity::persona::persona_data::entry::BasePersonaDataEntry;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct EmailAddress {}

impl BasePersonaDataEntry for EmailAddress {
    fn embed(&self) -> Self {
        self.embed()
    }

    fn description(&self) -> String {
        todo!()
    }
}
