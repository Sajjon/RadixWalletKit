use serde::{Deserialize, Serialize};

use super::IdentifiedEntry;

#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq, uniffi::Record,
)]
pub struct PersonaData {
    name: IdentifiedEntry,
}

impl Default for PersonaData {
    fn default() -> Self {
        Self {
            name: IdentifiedEntry::default(),
        }
    }
}
