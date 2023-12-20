// use super::{
//     entry::{BasePersonaDataEntry, Entry},
//     identified_entry::IdentifiedEntry,
// };
// use identified_vec::{newtype_identified_vec, IsIdentifiedVecOf};
// use identified_vec::{Error, Identifiable, IdentifiedVec, IdentifiedVecOf, IsIdentifiedVec};
// use serde::{Deserialize, Serialize};
// use std::{fmt::Debug, ops::Add};
// use uuid::Uuid;

// newtype_identified_vec!(of: IdentifiedEntry<Entry>, named: CollectionOfIdentifiedEntries);

// impl CollectionOfIdentifiedEntries {
//     fn description(&self) -> String {
//         self.0
//             .clone()
//             .into_iter()
//             .map(|entry| entry.value.description().add(", "))
//             .collect()
//     }
// }

// impl CollectionOfIdentifiedEntries {
//     pub fn new() -> Self {
//         Self {
//             0: IdentifiedVecOf::<IdentifiedEntry<Entry>>::new(),
//         }
//     }

//     pub fn from_iter<I>(unique_elements: I) -> Self
//     where
//         I: IntoIterator<Item = IdentifiedEntry<Entry>>,
//     {
//         let mut _self: CollectionOfIdentifiedEntries = Self::new();
//         unique_elements
//             .into_iter()
//             .for_each(|e| _ = _self.append(e));
//         return _self;
//     }

//     pub fn update(&mut self, updated_element: IdentifiedEntry<Entry>) -> Result<(), Error> {
//         let result = self.0.try_update(updated_element);

//         match result {
//             Ok(_) => Ok(()),
//             Err(err) => Err(err),
//         }
//     }

//     pub fn append(&mut self, updated_element: IdentifiedEntry<Entry>) -> Result<(), Error> {
//         let result = self.0.try_append_new(updated_element);
//         match result {
//             Ok(_) => Ok(()),
//             Err(err) => Err(err),
//         }
//     }
// }

// impl Default for CollectionOfIdentifiedEntries {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::str::FromStr;
//     type ID = Uuid;

//     // #[test]
//     // fn default_empty_collection() {
//     //     let default: CollectionOfIdentifiedEntries<T> = CollectionOfIdentifiedEntries::default();
//     //     assert_eq!(default.collection, IdentifiedVec::new())
//     // }
// }
