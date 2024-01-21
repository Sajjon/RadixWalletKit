use super::IdentifiedEntry;
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    Hash,
    Eq,
    Display,
    uniffi::Record,
)]
#[display("{}", name)]
pub struct PersonaData {
    name: IdentifiedEntry,
}

impl PersonaData {
    pub fn new(name: IdentifiedEntry) -> Self {
        Self { name }
    }
}

impl Default for PersonaData {
    fn default() -> Self {
        Self {
            name: IdentifiedEntry::default(),
        }
    }
}
