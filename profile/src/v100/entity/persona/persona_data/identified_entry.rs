use derive_more::Display;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Name;

type PersonaDataEntryID = Uuid;

// TODO: Needs to be made generic when adding more entry_kinds. Right now uniffi::Record complains if the function is generic.
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

impl Default for IdentifiedEntry {
    fn default() -> Self {
        Self {
            id: Default::default(),
            value: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::v100::entity::persona::Variant;

    use super::*;

    #[test]
    fn new() {
        let identified_entry = IdentifiedEntry::new(
            Uuid::nil(),
            Name::new(Variant::Western, "Wayne", "Bruce", "Batman"),
        );
        assert_eq!(
            identified_entry.id,
            Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap()
        );
        assert_eq!(
            identified_entry.value,
            Name::new(Variant::Western, "Wayne", "Bruce", "Batman")
        )
    }

    #[test]
    fn display() {
        let identified_entry = IdentifiedEntry::new(
            Uuid::nil(),
            Name::new(Variant::Western, "Wayne", "Bruce", "Batman"),
        );
        assert_eq!(
            format!("{identified_entry}"),
            "Bruce Batman Wayne \n id: 00000000-0000-0000-0000-000000000000"
        );
    }
}
