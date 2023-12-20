use super::{
    entry::BasePersonaDataEntry,
    entry_kinds::{email_address::EmailAddress, name::Name, phone_number::PhoneNumber},
    identified_entry::IdentifiedEntry,
};
use crate::identified_vec_via::IdentifiedVecVia;
use identified_vec::{IsIdentifiableVecOfVia, IsIdentifiedVecOf};
use serde::{Deserialize, Serialize};
use url::form_urlencoded::Target;

type IdentifiedName = IdentifiedEntry<Name>;
type IdentifiedEmailAddresses = IdentifiedVecVia<IdentifiedEntry<EmailAddress>>;
type IdentifiedPhoneNumbers = IdentifiedVecVia<IdentifiedEntry<PhoneNumber>>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PersonaData {
    name: Option<IdentifiedName>,
    email_addresses: IdentifiedEmailAddresses,
    phone_numbers: IdentifiedPhoneNumbers,
    entries: Vec<String>,
}

impl PersonaData {
    pub fn new() -> Self {
        Self {
            name: None,
            email_addresses: IdentifiedEmailAddresses::new(),
            phone_numbers: IdentifiedPhoneNumbers::new(),
            entries: vec![],
        }
    }

    pub fn fill_entries(&self) -> Vec<String> {
        let mut entries = vec![];

        entries.append({
            self.email_addresses
                .clone()
                .into_iter()
                .map(|entry| entry.value.description())
                .collect::<Vec<String>>()
                .as_mut()
        });
        entries.append({
            self.phone_numbers
                .clone()
                .into_iter()
                .map(|entry| entry.value.description())
                .collect::<Vec<String>>()
                .as_mut()
        });
        entries.push(
            self.name
                .clone()
                .expect("No name found")
                .value
                .description(),
        );
        entries
    }

    pub fn description(&self) -> String {
        self.entries.join("\n")
    }
}
