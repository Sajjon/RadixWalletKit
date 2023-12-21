use super::entry_kinds::{EmailAddress, Name, PhoneNumber};
use serde::{Deserialize, Serialize};

pub trait BasePersonaDataEntry {
    fn description(&self) -> String;
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum Entry {
    Name(Name),

    EmailAddress(EmailAddress),
    PhoneNumber(PhoneNumber),
}

impl BasePersonaDataEntry for Entry {
    fn description(&self) -> String {
        match self {
            Self::Name(name) => name.description(),
            Self::EmailAddress(email_address) => email_address.description(),
            Self::PhoneNumber(phone_number) => phone_number.description(),
        }
    }
}
