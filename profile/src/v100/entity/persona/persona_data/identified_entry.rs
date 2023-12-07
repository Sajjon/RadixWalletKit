use serde::{Deserialize, Serialize};
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
        format!(
            "
        {}
        id: {}
        ",
            self.id, self.value
        )
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default, Hash, Ord, PartialOrd)]
pub struct CollectionOfIdentifiedEntries<Value: Ord> {
    collection: Vec<IdentifiedEntry<Value>>,
}

impl<Value: Default + std::cmp::Eq + std::cmp::Ord + Copy> CollectionOfIdentifiedEntries<Value> {
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
            Ok(CollectionOfIdentifiedEntries::default())
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
            .position(|value| value.to_owned() == updated)
            .expect("Should find index since collection contains the ID");

        self.collection.insert(index, updated);

        Ok(())
    }

    pub fn description(&self) -> String {
        self.collection
            .iter()
            .map(|_| Self::description(self))
            .collect()
    }
}
