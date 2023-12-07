use crate::v100::entity::persona::persona_data::collection_error::CollectionError;
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
        if self
            .collection
            .iter()
            .map(|item| item.value)
            .collect::<Vec<Value>>()
            .contains(&field.value)
        {
            return Err(CollectionError::DuplicateValuesFound);
        }

        if self
            .collection
            .iter()
            .map(|item| item.id)
            .collect::<Vec<Uuid>>()
            .contains(&field.id)
        {
            return Err(CollectionError::DuplicateIDOfValuesFound);
        }

        self.collection.push(field);

        Ok(())
    }

    pub fn update(&mut self, updated: IdentifiedEntry<Value>) -> Result<(), CollectionError> {
        if !self
            .collection
            .iter()
            .map(|item| item.id)
            .collect::<Vec<Uuid>>()
            .contains(&updated.id)
        {
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
