use serde::{Deserialize, Serialize};

use super::{
    entry_kinds::{
        associated_url::AssociatedURL, company_name::CompanyName, credit_card::CreditCard,
        date_of_birth::DateOfBirth, email_address::EmailAddress, name::Name,
        phone_number::PhoneNumber, postal_address::PostalAddress,
    },
    identified_entry::IdentifiedEntry,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PersonaData {
    name: IdentifiedEntry,

    date_of_birth: IdentifiedEntry,

    company_name: IdentifiedEntry,

    email_addresses: IdentifiedEntry,

    phone_numbers: IdentifiedEntry,

    urls: IdentifiedEntry,

    postal_addresses: IdentifiedEntry,

    credit_cards: IdentifiedEntry,
}
