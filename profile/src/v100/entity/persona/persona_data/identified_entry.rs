use derive_getters::Getters;
use identified_vec::Identifiable;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Getters)]
pub struct IdentifiedEntry<T> {
    id: Uuid,
    value: T,
}

impl<T> Identifiable for IdentifiedEntry<T> {
    type ID = Uuid;
    fn id(&self) -> Self::ID {
        self.id
    }
}

impl<T: Default> IdentifiedEntry<T> {
    pub fn new() -> Self {
        Self {
            id: Uuid::nil(),
            value: T::default(),
        }
    }
}

impl<T> IdentifiedEntry<T> {
    pub fn from(id: Uuid, value: T) -> Self {
        Self { id, value }
    }
}

impl<T: Display> Display for IdentifiedEntry<T> {
    fn fmt(
        &self,
        f: &mut radix_engine_common::prelude::fmt::Formatter<'_>,
    ) -> radix_engine_common::prelude::fmt::Result {
        write!(f, "{}, {}", self.value, self.id)
    }
}

impl<T: Debug + Display> IdentifiedEntry<T> {
    pub fn description(&self) -> String {
        format!("value: {} id: {:#?} ", self.value, self.id)
    }
}

#[cfg(test)]
impl IdentifiedEntry<String> {
    pub fn placeholder_entry_1() -> Self {
        Self {
            id: Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
            value: "Entry 1".to_string(),
        }
    }

    pub fn placeholder_entry_2() -> Self {
        Self {
            id: Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
            value: "Entry 2".to_string(),
        }
    }

    pub fn placeholder_entry_3() -> Self {
        Self {
            id: Uuid::from_str("CCCCCCCC-3333-4444-5555-CCCCCCCCCCCC").unwrap(),
            value: "Entry 3".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::v100::entity::persona::persona_data::entry_kinds::EmailAddress;

    use super::*;
    use std::str::FromStr;
    type ID = Uuid;

    #[test]
    fn from_identified_entry() {
        let id: ID = Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap();
        let value = "Entry 1";
        let identified_entry = IdentifiedEntry::from(id, value.to_string());
        assert_eq!(identified_entry, IdentifiedEntry::placeholder_entry_1())
    }

    #[test]
    fn display_description() {
        let description = IdentifiedEntry::description(&IdentifiedEntry::placeholder_entry_1());
        assert_eq!(
            description,
            format!(
                "value: {} id: {} ",
                "Entry 1",
                "AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA".to_lowercase()
            )
        )
    }

    #[test]
    fn new_identified_entry() {
        let identified_email: IdentifiedEntry<EmailAddress> = IdentifiedEntry::from(
            Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap(),
            EmailAddress::default(),
        );
        assert_eq!(IdentifiedEntry::<EmailAddress>::new(), identified_email)
    }
}
