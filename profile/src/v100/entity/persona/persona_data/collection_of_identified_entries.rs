use identified_vec::{Error, Identifiable, IdentifiedVec, IdentifiedVecOf};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display},
    ops::Add,
};

use super::identified_entry::IdentifiedEntry;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CollectionOfIdentifiedEntries<T>
where
    T: Identifiable + Debug + Clone,
{
    collection: IdentifiedVec<<IdentifiedEntry<T> as Identifiable>::ID, IdentifiedEntry<T>>,
}

impl<T> CollectionOfIdentifiedEntries<T>
where
    T: Identifiable + Default + Debug + Clone,
{
    pub fn default() -> Self {
        Self {
            collection: IdentifiedVecOf::<IdentifiedEntry<T>>::new(),
        }
    }
}

impl<T> CollectionOfIdentifiedEntries<IdentifiedEntry<T>>
where
    T: Debug + Clone + Identifiable + Display,
{
    pub fn description(self) -> String {
        self.collection
            .iter()
            .map(|entry| entry.value.description().add(", "))
            .collect()
    }
}
impl<T> CollectionOfIdentifiedEntries<T>
where
    T: Identifiable + Clone + Debug,
{
    pub fn new_collection_identified_entry() -> Self {
        Self {
            collection: IdentifiedVec::new(),
        }
    }
}

impl<T> CollectionOfIdentifiedEntries<T>
where
    T: Identifiable + Clone + Debug,
{
    pub fn from_iter<I>(unique_elements: I) -> Self
    where
        I: IntoIterator<Item = IdentifiedEntry<T>>,
    {
        let mut _self: CollectionOfIdentifiedEntries<T> = Self::new_collection_identified_entry();
        unique_elements
            .into_iter()
            .for_each(|e| _ = _self.append(e));
        return _self;
    }
}

impl<T> CollectionOfIdentifiedEntries<T>
where
    T: Identifiable + Clone + Debug,
{
    pub fn from(collection: IdentifiedVecOf<IdentifiedEntry<T>>) -> Self {
        Self { collection }
    }

    pub fn new() -> Self {
        Self {
            collection: IdentifiedVecOf::<IdentifiedEntry<T>>::new(),
        }
    }

    pub fn update(&mut self, updated_element: IdentifiedEntry<T>) -> Result<(), Error> {
        let result = self.collection.try_update(updated_element);

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub fn append(&mut self, updated_element: IdentifiedEntry<T>) -> Result<(), Error> {
        let result = self.collection.try_append(updated_element);
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}

#[test]
fn default_empty_collection() {
    let default: CollectionOfIdentifiedEntries<IdentifiedEntry<u8>> =
        CollectionOfIdentifiedEntries::<IdentifiedEntry<u8>>::default();
    assert_eq!(default.collection, IdentifiedVec::new())
}
