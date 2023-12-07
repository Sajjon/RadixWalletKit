use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, Default, Hash)]
pub struct IdentifiedEntry<Value> {
    id: ID,
    value: Value,
}
impl<Value: Eq> Eq for IdentifiedEntry<Value> {}
impl<Value: Ord + Iterator> Ord for IdentifiedEntry<Value>
where
    <Value as Iterator>::Item: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        (&self.value, &self.id).cmp(&(&other.value, &other.id))
    }
}

impl<Value: PartialOrd + Iterator + Ord> PartialOrd for IdentifiedEntry<Value>
where
    <Value as Iterator>::Item: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl<Value> PartialEq for IdentifiedEntry<Value> {
    fn eq(&self, other: &Self) -> bool {
        (&self.id) == (&other.id)
    }
}

type ID = Uuid;
impl<Value: Display> IdentifiedEntry<Value> {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default, Hash)]
pub struct CollectionOfIdentifiedEntries<Value> {
    collection: Vec<IdentifiedEntry<Value>>,
}

impl<Value: Default + std::cmp::Eq + std::cmp::Ord> CollectionOfIdentifiedEntries<Value> {
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
        // Need to fix double reference of field.value
        if self
            .collection
            .iter()
            .map(|item| &item.value)
            .collect::<Vec<_>>()
            .contains(&&field.value)
        {
            return Err(CollectionError::DuplicateValuesFound);
        }

        if self
            .collection
            .iter()
            .map(|item| item.id)
            .collect::<Vec<_>>()
            .contains(&field.id)
        {
            return Err(CollectionError::DuplicateIDOfValuesFound);
        }

        self.collection.push(field);

        Ok(())
    }
}

use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum CollectionError {
    #[error("Duplicate values found when construction collection")]
    DuplicateValuesFound,

    #[error("Duplicate id of values found when construction collection")]
    DuplicateIDOfValuesFound,
}
