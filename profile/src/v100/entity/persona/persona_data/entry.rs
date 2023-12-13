use super::entry_kinds::{
    associated_url::AssociatedURL, company_name::CompanyName, credit_card::CreditCard,
    date_of_birth::DateOfBirth, email_address::EmailAddress, name::Name, phone_number::PhoneNumber,
    postal_address::PostalAddress,
};
use serde::{Deserialize, Serialize};

pub trait BasePersonaDataEntry {
    fn embed(&self) -> Entry;
    fn description(&self) -> String;
}

#[derive(Serialize, Deserialize)]
pub enum Entry {
    Name(Name),
    DateOfBirth(DateOfBirth),
    CompanyName(CompanyName),

    EmailAddress(EmailAddress),
    PhoneNumber(PhoneNumber),
    Url(AssociatedURL),
    PostalAddress(PostalAddress),
    CreditCard(CreditCard),
}

impl BasePersonaDataEntry for Entry {
    fn embed(&self) -> Self {
        match self {
            Self::Name(name) => name.embed(),
            Self::CompanyName(company) => company.embed(),
            Self::CreditCard(credit_card) => credit_card.embed(),
            Self::DateOfBirth(date_of_birth) => date_of_birth.embed(),
            Self::EmailAddress(email_address) => email_address.embed(),
            Self::PhoneNumber(phone_number) => phone_number.embed(),
            Self::PostalAddress(postal_address) => postal_address.embed(),
            Self::Url(url) => url.embed(),
        }
    }

    fn description(&self) -> String {
        match self {
            Self::Name(name) => name.description(),
            Self::CompanyName(company) => company.description(),
            Self::CreditCard(credit_card) => credit_card.description(),
            Self::DateOfBirth(date_of_birth) => date_of_birth.description(),
            Self::EmailAddress(email_address) => email_address.description(),
            Self::PhoneNumber(phone_number) => phone_number.description(),
            Self::PostalAddress(postal_address) => postal_address.description(),
            Self::Url(url) => url.description(),
        }
    }
}
