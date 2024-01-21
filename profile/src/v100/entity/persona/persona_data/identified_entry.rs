use std::str::FromStr;

use derive_more::Display;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::HasPlaceholder;

use super::{Name, Variant};

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

impl HasPlaceholder for IdentifiedEntry {
    fn placeholder() -> Self {
        IdentifiedEntry::new(
            Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap(),
            Name::placeholder(),
        )
    }

    fn placeholder_other() -> Self {
        IdentifiedEntry::new(
            Uuid::from_str("00000000-0000-0000-0000-000000000002").unwrap(),
            Name::placeholder_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use serde_json::json;

    use crate::{
        assert_eq_after_json_roundtrip, v100::entity::persona::Variant,
    };

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

    #[test]
    fn placeholder() {
        let placeholder = IdentifiedEntry::placeholder();
        assert_eq!(
            placeholder.id.to_string(),
            "00000000-0000-0000-0000-000000000001"
        );
        assert_eq!(placeholder.value.to_string(), "Bruce Batman Wayne");
    }

    #[test]
    fn placeholder_other() {
        let placeholder = IdentifiedEntry::placeholder_other();
        assert_eq!(
            placeholder.id.to_string(),
            "00000000-0000-0000-0000-000000000002"
        );
        assert_eq!(placeholder.value.to_string(), "Jun-fan Bruce Lee");
    }

    #[test]
    fn default() {
        let default = IdentifiedEntry::default();
        assert_eq!(
            default.id.to_string(),
            "00000000-0000-0000-0000-000000000000"
        );
        assert_eq!(default.value.to_string(), "  ")
    }

    #[test]
    fn json_roundtrip_batman() {
        let model = IdentifiedEntry::placeholder();
        assert_eq_after_json_roundtrip(
            &model,
            r#"{
            "id": "00000000-0000-0000-0000-000000000001",
            "value": {
              "variant": "Western",
              "family_name": "Wayne",
              "given_name": "Bruce",
              "nickname": "Batman"
            }
          }
        "#,
        )
    }

    #[test]
    fn json_roundtrip_bruce_lee() {
        let model = IdentifiedEntry::placeholder_other();
        assert_eq_after_json_roundtrip(
            &model,
            r#"{
            "id": "00000000-0000-0000-0000-000000000002",
            "value": {
              "variant": "Eastern",
              "family_name": "Jun-fan",
              "given_name": "Lee",
              "nickname": "Bruce"
            }
          }
        "#,
        )
    }
}
