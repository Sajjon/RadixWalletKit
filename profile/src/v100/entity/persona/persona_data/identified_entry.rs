use derive_more::Display;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Name;

type PersonaDataEntryID = Uuid;

// TODO: Needs to be made generic. Right now uniffi::Record complains if the function is generic.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq, Display, uniffi::Record)]
#[display("{} \n id: {}", value, id)]
pub struct IdentifiedEntry {
    id: PersonaDataEntryID,
    value: Name,
}


impl IdentifiedEntry {
    pub fn new(id: PersonaDataEntryID, value: Name) -> Self {
        Self { id, value }
    }
}