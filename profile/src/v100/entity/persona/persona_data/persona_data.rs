use serde::{Deserialize, Serialize};

use super::{
    entry_kinds::{
        associated_url::AssociatedURL, company_name::CompanyName, credit_card::CreditCard,
        date_of_birth::DateOfBirth, email_address::EmailAddress, name::Name,
        phone_number::PhoneNumber, postal_address::PostalAddress,
    },
    identified_entry::IdentifiedEntry,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PersonaData {
    name: IdentifiedEntry<Name>,

    date_of_birth: IdentifiedEntry<DateOfBirth>,

    company_name: IdentifiedEntry<CompanyName>,

    email_addresses: IdentifiedEntry<EmailAddress>,

    phone_numbers: IdentifiedEntry<PhoneNumber>,

    urls: IdentifiedEntry<AssociatedURL>,

    postal_addresses: IdentifiedEntry<PostalAddress>,

    credit_cards: IdentifiedEntry<CreditCard>,
}
