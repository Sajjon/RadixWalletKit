use crate::v100::entity::persona::persona_data::entry::BasePersonaDataEntry;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn new() {
        let email = EmailAddress {
            email: String::new(),
        };
        assert_eq!(EmailAddress::new(), email)
    }

    #[test]
    fn description() {
        let description = String::from_str("apa@apa.se").unwrap();
        let email_address = EmailAddress {
            email: "apa@apa.se".to_string(),
        };
        assert_eq!(email_address.description(), description)
    }

    #[test]
    fn default() {
        let default = "".to_string();
        assert_eq!(
            EmailAddress::default(),
            EmailAddress {
                email: "".to_string()
            }
        )
    }
}
