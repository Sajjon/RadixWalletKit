use crate::v100::entity::persona::persona_data::entry::{BasePersonaDataEntry, Entry};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PhoneNumber {
    number: String,
}

impl BasePersonaDataEntry for PhoneNumber {
    fn description(&self) -> String {
        self.number.clone()
    }
}

impl PhoneNumber {
    pub fn new() -> Self {
        Self {
            number: String::new(),
        }
    }
}

impl Default for PhoneNumber {
    fn default() -> Self {
        Self {
            number: "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::v100::entity::persona::persona_data::entry_kinds::phone_number;
    use std::str::FromStr;

    use super::*;

    #[test]
    fn new() {
        let phone_number = PhoneNumber {
            number: String::new(),
        };
        assert_eq!(PhoneNumber::new(), phone_number)
    }

    #[test]
    fn description() {
        let description = format!("0731999999");
        let phone_number = PhoneNumber {
            number: "0731999999".to_string(),
        };
        assert_eq!(phone_number.description(), description)
    }

    #[test]
    fn default() {
        assert_eq!(
            PhoneNumber::default(),
            PhoneNumber {
                number: "".to_string()
            }
        )
    }
}
