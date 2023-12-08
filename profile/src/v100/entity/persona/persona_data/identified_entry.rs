use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use std::cmp::Ordering;
use std::fmt::Display;
use uuid::Uuid;
use wallet_kit_common::error::collection_error::CollectionError;

#[derive(Serialize, Deserialize, Clone, Debug, Default, Hash, Ord, Eq, PartialOrd, PartialEq)]
pub struct IdentifiedEntry<Value: Eq + Ord> {
    id: ID,
    value: Value,
}

type ID = Uuid;
impl<Value: Display + std::cmp::Ord> IdentifiedEntry<Value> {
    pub fn new(id: ID, value: Value) -> Self {
        Self { id, value }
    }

    pub fn description(&self) -> String {
        format!("value: {} id: {} ", self.value, self.id)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default, Hash, Ord, PartialOrd)]
pub struct CollectionOfIdentifiedEntries<Value: Ord> {
    collection: Vec<IdentifiedEntry<Value>>,
}

impl<Value: std::cmp::Eq + std::cmp::Ord + Copy + std::fmt::Display>
    CollectionOfIdentifiedEntries<Value>
{
    pub fn default() -> Self {
        Self { collection: vec![] }
    }

    pub fn new(collection: Vec<IdentifiedEntry<Value>>) -> Result<Self, CollectionError> {
        let mut set: Vec<_> = collection.iter().map(|entry| &entry.value).collect();
        set.sort();
        set.dedup();

        if set.len() != collection.len() {
            return Err(CollectionError::DuplicateValuesFound);
        } else {
            Ok(CollectionOfIdentifiedEntries {
                collection: collection,
            })
        }
    }

    pub fn add(&mut self, field: IdentifiedEntry<Value>) -> Result<(), CollectionError> {
        if self.collection.iter().any(|i| i.value == field.value) {
            return Err(CollectionError::DuplicateValuesFound);
        }

        if self.collection.iter().any(|i| i.id == field.id) {
            return Err(CollectionError::DuplicateIDOfValuesFound);
        }

        self.collection.push(field);

        Ok(())
    }

    pub fn update(&mut self, updated: IdentifiedEntry<Value>) -> Result<(), CollectionError> {
        if !self.collection.iter().any(|i| i.id == updated.id) {
            return Err(CollectionError::PersonaFieldCollectionValueWithIDNotFound);
        }
        let index = self
            .collection
            .iter()
            .position(|value| value.id.to_owned() == updated.id)
            .expect("Should find index since collection contains the ID");

        self.collection[index] = updated;

        Ok(())
    }

    pub fn description(self) -> String {
        self.collection
            .iter()
            .map(|entry| IdentifiedEntry::description(entry))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn new_identified_entry() {
        let id: ID = Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap();
        let value = "Test";
        let identified_entry = IdentifiedEntry::new(id, value);
        assert_eq!((identified_entry.id, identified_entry.value), (id, value))
    }

    #[test]
    fn display_description() {
        let id: ID = Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap();
        let value = "Test";
        let identified_entry = IdentifiedEntry::new(id, value);
        assert_eq!((identified_entry.id, identified_entry.value), (id, value));

        let description = IdentifiedEntry::description(&identified_entry);
        assert_eq!(description, format!("value: {} id: {} ", value, id))
    }

    #[test]
    fn default_empty_collection() {
        let default: CollectionOfIdentifiedEntries<_> =
            CollectionOfIdentifiedEntries::<u8>::default();
        assert_eq!(default.collection, vec![])
    }

    #[test]
    fn new_collection_duplicate_values() {
        let first_entry = IdentifiedEntry::new(
            Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
            "Test",
        );
        let second_entry = IdentifiedEntry::new(
            Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
            "Test",
        );

        let collection = vec![first_entry, second_entry];
        assert_eq!(
            collection,
            vec![
                IdentifiedEntry::new(
                    Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
                    "Test"
                ),
                IdentifiedEntry::new(
                    Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
                    "Test"
                )
            ]
        );

        let collection_of_identified_entries = CollectionOfIdentifiedEntries::new(collection);
        assert_eq!(
            collection_of_identified_entries,
            Err(CollectionError::DuplicateValuesFound)
        );
    }

    #[test]
    fn new_collection() {
        let first_entry = IdentifiedEntry::new(
            Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
            "Test",
        );
        let second_entry = IdentifiedEntry::new(
            Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
            "Second Test",
        );

        let collection = vec![first_entry.clone(), second_entry.clone()];
        assert_eq!(
            collection,
            vec![
                IdentifiedEntry::new(
                    Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
                    "Test"
                ),
                IdentifiedEntry::new(
                    Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
                    "Second Test"
                )
            ]
        );

        let collection_of_identified_entries =
            CollectionOfIdentifiedEntries::new(collection).unwrap();
        assert_eq!(
            collection_of_identified_entries,
            CollectionOfIdentifiedEntries {
                collection: vec![first_entry, second_entry]
            }
        );
    }

    #[test]
    fn add_entry_collection() {
        let first_entry = IdentifiedEntry::new(
            Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
            "Test",
        );
        let second_entry = IdentifiedEntry::new(
            Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
            "Second Test",
        );

        let collection = vec![first_entry.clone(), second_entry.clone()];
        assert_eq!(
            collection,
            vec![
                IdentifiedEntry::new(
                    Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
                    "Test"
                ),
                IdentifiedEntry::new(
                    Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
                    "Second Test"
                )
            ]
        );

        let mut collection_of_identified_entries =
            CollectionOfIdentifiedEntries::new(collection).unwrap();
        assert_eq!(
            collection_of_identified_entries,
            CollectionOfIdentifiedEntries {
                collection: vec![first_entry.clone(), second_entry.clone()]
            }
        );

        let new_entry = IdentifiedEntry::new(
            Uuid::from_str("CCCCCCCC-3333-4444-5555-CCCCCCCCCCCC").unwrap(),
            "New Entry",
        );

        collection_of_identified_entries.add(new_entry.clone());

        assert_eq!(
            collection_of_identified_entries,
            CollectionOfIdentifiedEntries {
                collection: vec![first_entry, second_entry, new_entry]
            }
        )
    }

    #[test]
    fn add_entry_collection_duplicate_ids() {
        let first_entry = IdentifiedEntry::new(
            Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
            "Test",
        );
        let second_entry = IdentifiedEntry::new(
            Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
            "Second Test",
        );

        let collection = vec![first_entry.clone(), second_entry.clone()];
        assert_eq!(
            collection,
            vec![
                IdentifiedEntry::new(
                    Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
                    "Test"
                ),
                IdentifiedEntry::new(
                    Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
                    "Second Test"
                )
            ]
        );

        let mut collection_of_identified_entries =
            CollectionOfIdentifiedEntries::new(collection).unwrap();
        assert_eq!(
            collection_of_identified_entries,
            CollectionOfIdentifiedEntries {
                collection: vec![first_entry.clone(), second_entry.clone()]
            }
        );

        let new_entry = IdentifiedEntry::new(
            Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
            "New Entry",
        );

        let result_after_add = collection_of_identified_entries.add(new_entry.clone());

        assert_eq!(
            result_after_add,
            Err(CollectionError::DuplicateIDOfValuesFound)
        );
    }

    #[test]
    fn add_entry_collection_duplicate_values() {
        let first_entry = IdentifiedEntry::new(
            Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
            "Test",
        );
        let second_entry = IdentifiedEntry::new(
            Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
            "Second Test",
        );

        let collection = vec![first_entry.clone(), second_entry.clone()];
        assert_eq!(
            collection,
            vec![
                IdentifiedEntry::new(
                    Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
                    "Test"
                ),
                IdentifiedEntry::new(
                    Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
                    "Second Test"
                )
            ]
        );

        let mut collection_of_identified_entries =
            CollectionOfIdentifiedEntries::new(collection).unwrap();
        assert_eq!(
            collection_of_identified_entries,
            CollectionOfIdentifiedEntries {
                collection: vec![first_entry.clone(), second_entry.clone()]
            }
        );

        let new_entry = IdentifiedEntry::new(
            Uuid::from_str("CCCCCCCC-3333-4444-5555-CCCCCCCCCCCC").unwrap(),
            "Second Test",
        );

        let result_after_add = collection_of_identified_entries.add(new_entry.clone());

        assert_eq!(result_after_add, Err(CollectionError::DuplicateValuesFound));
    }

    #[test]
    fn update_entry_collection() {
        let first_entry = IdentifiedEntry::new(
            Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
            "Test",
        );
        let second_entry = IdentifiedEntry::new(
            Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
            "Second Test",
        );

        let collection = vec![first_entry.clone(), second_entry.clone()];
        assert_eq!(
            collection,
            vec![
                IdentifiedEntry::new(
                    Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
                    "Test"
                ),
                IdentifiedEntry::new(
                    Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
                    "Second Test"
                )
            ]
        );

        let mut collection_of_identified_entries =
            CollectionOfIdentifiedEntries::new(collection).unwrap();
        assert_eq!(
            collection_of_identified_entries,
            CollectionOfIdentifiedEntries {
                collection: vec![first_entry.clone(), second_entry.clone()]
            }
        );

        let updated_entry = IdentifiedEntry::new(
            Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
            "Updated Value",
        );

        collection_of_identified_entries.update(updated_entry.clone());

        assert_eq!(
            collection_of_identified_entries,
            CollectionOfIdentifiedEntries {
                collection: vec![first_entry, updated_entry]
            }
        );
    }

    #[test]
    fn update_entry_collection_id_not_found() {
        let first_entry = IdentifiedEntry::new(
            Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
            "Test",
        );
        let second_entry = IdentifiedEntry::new(
            Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
            "Second Test",
        );

        let collection = vec![first_entry.clone(), second_entry.clone()];
        assert_eq!(
            collection,
            vec![
                IdentifiedEntry::new(
                    Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
                    "Test"
                ),
                IdentifiedEntry::new(
                    Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
                    "Second Test"
                )
            ]
        );

        let mut collection_of_identified_entries =
            CollectionOfIdentifiedEntries::new(collection).unwrap();
        assert_eq!(
            collection_of_identified_entries,
            CollectionOfIdentifiedEntries {
                collection: vec![first_entry.clone(), second_entry.clone()]
            }
        );

        let updated_entry = IdentifiedEntry::new(
            Uuid::from_str("CCCCCCCC-3333-4444-5555-CCCCCCCCCCCC").unwrap(),
            "Updated Value",
        );

        let result = collection_of_identified_entries.update(updated_entry.clone());

        assert_eq!(
            result,
            Err(CollectionError::PersonaFieldCollectionValueWithIDNotFound)
        );
    }

    #[test]
    fn description() {
        let first_entry = IdentifiedEntry::new(
            Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
            "Test",
        );
        let second_entry = IdentifiedEntry::new(
            Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
            "Second Test",
        );

        let collection = vec![first_entry.clone(), second_entry.clone()];
        assert_eq!(
            collection,
            vec![
                IdentifiedEntry::new(
                    Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
                    "Test"
                ),
                IdentifiedEntry::new(
                    Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
                    "Second Test"
                )
            ]
        );

        let mut collection_of_identified_entries =
            CollectionOfIdentifiedEntries::new(collection).unwrap();
        assert_eq!(
            collection_of_identified_entries,
            CollectionOfIdentifiedEntries {
                collection: vec![first_entry.clone(), second_entry.clone()]
            }
        );

        let description = collection_of_identified_entries.description();
        assert_eq!(
            description,
            format!(
                "value: {} id: {} value: {} id: {} ",
                first_entry.value, first_entry.id, second_entry.value, second_entry.id
            )
        )
    }
}
