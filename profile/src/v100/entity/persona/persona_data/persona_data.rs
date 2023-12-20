use super::{
    entry::BasePersonaDataEntry,
    entry_kinds::{email_address::EmailAddress, name::Name, phone_number::PhoneNumber},
    identified_entry::IdentifiedEntry,
};
use crate::identified_vec_via::IdentifiedVecVia;
use identified_vec::{Identifiable, IdentifiedVec, IsIdentifiedVec};
use serde::{Deserialize, Serialize};

type IdentifiedName = IdentifiedEntry<Name>;
type IdentifiedEmailAddresses = IdentifiedVecVia<IdentifiedEntry<EmailAddress>>;
type IdentifiedPhoneNumbers = IdentifiedVecVia<IdentifiedEntry<PhoneNumber>>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PersonaData {
    name: Option<IdentifiedName>,
    email_addresses: IdentifiedEmailAddresses,
    phone_numbers: IdentifiedPhoneNumbers,
}

impl PersonaData {
    pub fn description(&self) -> String {
        let mut entries: String = String::new();
        let name = self
            .name
            .clone()
            .expect("No name found")
            .value
            .description();
        let email_addresses: Vec<String> = self
            .email_addresses
            .clone()
            .into_iter()
            .map(|element| element.value.description())
            .collect();

        let phone_numbers: Vec<String> = self
            .phone_numbers
            .clone()
            .into_iter()
            .map(|element| element.value.description())
            .collect();
        entries.insert_str(0, &name);
        entries.extend(vec![phone_numbers.as_slice(), email_addresses.as_slice()].concat());

        entries
    }
}
