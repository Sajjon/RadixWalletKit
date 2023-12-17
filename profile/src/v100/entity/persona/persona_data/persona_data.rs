use serde::{Deserialize, Serialize};

use super::{
    collection_of_identified_entries::CollectionOfIdentifiedEntries,
    // collection_of_identified_entries::CollectionOfIdentifiedEntries,
    entry_kinds::{email_address::EmailAddress, name::Name},
    identified_entry::IdentifiedEntry,
};

type IdentifiedName = IdentifiedEntry<Name>;
// type IdentifiedEmailAddresses = CollectionOfIdentifiedEntries ;
// type IdentifiedPhoneNumbers = CollectionOfIdentifiedEntries;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PersonaData {
    name: Option<IdentifiedName>,
    //     email_addresses: IdentifiedEmailAddresses,

    //     phone_numbers: Option<IdentifiedPhoneNumbers>,
}
