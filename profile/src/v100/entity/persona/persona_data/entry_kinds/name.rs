use crate::v100::entity::persona::persona_data::entry::{BasePersonaDataEntry, Entry};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Getters)]
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

impl Variant {
    fn new() -> Self {
        Variant::Western
    }
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

impl Default for Name {
    fn default() -> Self {
        Self {
            variant: Variant::Western,
            family_name: "Blob".to_string(),
            given_names: "Blob Jr.".to_string(),
            nickname: "Blobbie".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn new() {
        let name = Name {
            variant: Variant::new(),
            family_name: String::new(),
            given_names: String::new(),
            nickname: String::new(),
        };
        assert_eq!(Name::new(), name)
    }

    #[test]
    fn description() {
        let description = format!("Blob Jr. Blobbie Blob");
        let name = Name {
            variant: Variant::Western,
            family_name: "Blob".to_string(),
            given_names: "Blob Jr.".to_string(),
            nickname: "Blobbie".to_string(),
        };
        assert_eq!(name.description(), description)
    }

    #[test]
    fn default() {
        assert_eq!(
            Name::default(),
            Name {
                variant: Variant::Western,
                family_name: "Blob".to_string(),
                given_names: "Blob Jr.".to_string(),
                nickname: "Blobbie".to_string()
            }
        )
    }
}
