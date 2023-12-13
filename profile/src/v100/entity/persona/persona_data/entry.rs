use super::entry_kinds::{
    associated_url::AssociatedURL, company_name::CompanyName, credit_card::CreditCard,
    date_of_birth::DateOfBirth, email_address::EmailAddress, name::Name, phone_number::PhoneNumber,
    postal_address::PostalAddress,
};
use serde::{Deserialize, Serialize};

pub trait BasePersonaDataEntry {
    fn embed(&self) -> Self;
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
            Self::Name(name) => Self::Name(name.embed()),
            Self::CompanyName(company) => Self::CompanyName(company.embed()),
            Self::CreditCard(credit_card) => Self::CreditCard(credit_card.embed()),
            Self::DateOfBirth(date_of_birth) => Self::DateOfBirth(date_of_birth.embed()),
            Self::EmailAddress(email) => Self::EmailAddress(email.embed()),
            Self::PhoneNumber(phone) => Self::PhoneNumber(phone.embed()),
            Self::PostalAddress(postal) => Self::PostalAddress(postal.embed()),
            Self::Url(url) => Self::Url(url.embed()),
        }
    }
    fn description(&self) -> String {
        match self {
            Self::Name(name) => name.description(),
            Self::CompanyName(company) => company.description(),
            Self::CreditCard(credit_card) => credit_card.description(),
            Self::DateOfBirth(date_of_birth) => date_of_birth.description(),
            Self::EmailAddress(email) => email.description(),
            Self::PhoneNumber(phone) => phone.description(),
            Self::PostalAddress(postal) => postal.description(),
            Self::Url(url) => url.description(),
        }
    }
}
