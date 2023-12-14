use super::{entry::BasePersonaDataEntry, identified_entry::IdentifiedEntry};
use identified_vec::{Error, Identifiable, IdentifiedVec, IdentifiedVecOf};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, ops::Add};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CollectionOfIdentifiedEntries {
    collection: IdentifiedVecOf<IdentifiedEntry>,
}

impl Default for CollectionOfIdentifiedEntries {
    fn default() -> Self {
        Self {
            collection: IdentifiedVecOf::<IdentifiedEntry>::new(),
        }
    }
}

impl BasePersonaDataEntry for CollectionOfIdentifiedEntries {
    // Not sure how to handle this embed. TODO!
    fn embed(&self) -> super::entry::Entry {
        todo!()
        // self.collection.iter().map(|x| x.embed()).last().expect("TODO")
    }

    fn description(&self) -> String {
        self.collection
            .iter()
            .map(|entry| entry.value.description().add(", "))
            .collect()
    }
}

impl CollectionOfIdentifiedEntries {
    pub fn from(collection: IdentifiedVecOf<IdentifiedEntry>) -> Self {
        Self { collection }
    }

    pub fn new() -> Self {
        Self {
            collection: IdentifiedVecOf::<IdentifiedEntry>::new(),
        }
    }

    pub fn from_iter<I>(unique_elements: I) -> Self
    where
        I: IntoIterator<Item = IdentifiedEntry>,
    {
        let mut _self: CollectionOfIdentifiedEntries = Self::new();
        unique_elements
            .into_iter()
            .for_each(|e| _ = _self.append(e));
        return _self;
    }

    pub fn update(&mut self, updated_element: IdentifiedEntry) -> Result<(), Error> {
        let result = self.collection.try_update(updated_element);

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub fn append(&mut self, updated_element: IdentifiedEntry) -> Result<(), Error> {
        let result = self.collection.try_append_new(updated_element);
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    type ID = Uuid;

    #[test]
    fn default_empty_collection() {
        let default: CollectionOfIdentifiedEntries = CollectionOfIdentifiedEntries::default();
        assert_eq!(default.collection, IdentifiedVec::new())
    }
}
