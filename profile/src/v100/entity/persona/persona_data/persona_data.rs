use super::{
    entry_kinds::{email_address::EmailAddress, name::Name, phone_number::PhoneNumber},
    identified_entry::IdentifiedEntry,
};
use identified_vec::{newtype_identified_vec, IdentifiedVec, IdentifiedVecOf, IsIdentifiedVecOf};
use serde;
use serde::{Deserialize, Serialize};

type IdentifiedName = IdentifiedEntry<Name>;

// newtype_identified_vec!(of: u8, named: IdentifiedEmailAddresses);
// newtype_identified_vec!(of: IdentifiedEntry<PhoneNumber>, named: IdentifiedPhoneNumbers);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
// #[serde(rename_all = "camelCase")]
pub struct PersonaData {
    name: Option<IdentifiedName>,
    // email_addresses: IdentifiedEmailAddresses,
    // phone_numbers: IdentifiedPhoneNumbers,
    entries: Vec<String>,
}

impl PersonaData {
    pub fn new(&self) -> Self {
        Self {
            name: None,
            // email_addresses: IdentifiedEmailAddresses::new(),
            // phone_numbers: IdentifiedPhoneNumbers::new(),
            entries: todo!(),
        }
    }

    // pub fn description(&self) -> String {
    //     self.entries.iter().map(|entry| entry)
    // }
}
