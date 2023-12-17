use crate::v100::entity::persona::persona_data::entry::{BasePersonaDataEntry, Entry};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    variant: Variant,
    family_name: String,
    given_names: String,
    nickname: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Variant {
    Eastern,
    Western,
}

impl Name {
    pub fn description(&self) -> String {
        let components = match self.variant {
            Variant::Eastern => vec![
                self.family_name.clone(),
                self.nickname.clone(),
                self.given_names.clone(),
            ],
            Variant::Western => vec![
                self.given_names.clone(),
                self.nickname.clone(),
                self.family_name.clone(),
            ],
        };
        components.join(" ")
    }

    pub fn new() -> Self {
        Self {
            variant: Variant::Western,
            family_name: String::new(),
            given_names: String::new(),
            nickname: String::new(),
        }
    }
}
impl BasePersonaDataEntry for Name {
    fn description(&self) -> String {
        self.description()
    }
}
