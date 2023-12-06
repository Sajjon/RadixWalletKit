use serde::{Deserialize, Serialize};

//Need to implement https://github.com/radixdlt/babylon-wallet-ios/blob/main/RadixWallet/Profile/Entity/PersonaData/PersonaData.swift
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct PersonaData {
    name: String,

    #[serde(rename = "dateOfBirth")]
    date_of_birth: String,

    #[serde(rename = "companyName")]
    company_name: String,

    #[serde(rename = "emailAddresses")]
    email_addresses: String,

    #[serde(rename = "phoneNumbers")]
    phone_numbers: String,

    urls: String,

    #[serde(rename = "postalAddresses")]
    postal_addresses: String,

    #[serde(rename = "creditCards")]
    credit_cards: String,
}
