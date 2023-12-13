use identified_vec::{identified_vec::IdentifiedVec, identified_vec_of::IdentifiedVecOf};
use identified_vec::{Error, Identifiable};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::Add;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct IdentifiedEntry<T> {
    id: Uuid,
    pub value: T,
}

impl<T> Identifiable for IdentifiedEntry<T> {
    type ID = Uuid;

    fn id(&self) -> Self::ID {
        self.id
    }
}

impl<T: Display> IdentifiedEntry<T> {
    pub fn description(&self) -> String {
        format!("value: {} id: {} ", self.value, self.id)
    }
}

impl<T> IdentifiedEntry<T> {
    pub fn new(id: Uuid, value: T) -> Self {
        Self { id, value }
    }
}

// impl<T: Identifiable> IdentifiedEntry<T> {
//     pub fn placeholder_with_values(value: T) -> Self {
//         Self {
//             id: Uuid::from(T::id(&T)),
//             value,
//         }
//     }
// }

#[cfg(test)]
struct User {
    id: Uuid,
    name: String,
}
#[cfg(test)]
impl Identifiable for User {
    type ID = Uuid;
    fn id(&self) -> Self::ID {
        self.id
    }
}
#[cfg(test)]
impl User {
    fn new(id: Uuid, name: &str) -> Self {
        if name.is_empty() {
            panic!("name cannot be empty")
        }
        Self {
            id,
            name: name.to_string(),
        }
    }

    pub fn blob() -> Self {
        User::new(
            Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
            "Blob",
        )
    }
    pub fn blob_jr() -> Self {
        User::new(
            Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
            "Blob, Jr.",
        )
    }
    pub fn blob_sr() -> Self {
        User::new(
            Uuid::from_str("CCCCCCCC-3333-4444-5555-CCCCCCCCCCCC").unwrap(),
            "Blob, Sr.",
        )
    }
}

// #[cfg(test)]
// impl IdentifiedEntry<String> {
//     pub fn placeholder_entry_1() -> Self {
//         IdentifiedEntry::new(Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(), "Entry 1".to_string())
//     }

//     pub fn placeholder_entry_2() -> Self {
//         IdentifiedEntry::new(Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(), "Entry 2".to_string())
//     }

//     pub fn placeholder_entry_3() -> Self {
//         IdentifiedEntry::new(Uuid::from_str("CCCCCCCC-3333-4444-5555-CCCCCCCCCCCC").unwrap(), "Entry 3".to_string())
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    type ID = Uuid;

    // #[test]
    // fn new_identified_entry() {
    //     let id: ID = Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap();
    //     let value = "Entry 1";
    //     let identified_entry = IdentifiedEntry::new(id, value.to_string());
    //     assert_eq!(identified_entry, IdentifiedEntry::placeholder_entry_1())
    // }

    // #[test]
    // fn display_description() {
    //     let description = IdentifiedEntry::description(&IdentifiedEntry::placeholder_entry_1());
    //     assert_eq!(
    //         description,
    //         format!(
    //             "value: {} id: {} ",
    //             "Entry 1",
    //             "AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA".to_lowercase()
    //         )
    //     )
    // }

    // #[test]
    // fn new_collection_duplicate_values() {
    //     let collection = IdentifiedVecOf::from_iter([
    //         IdentifiedEntry::new,
    //         IdentifiedEntry::new(
    //             Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
    //             "Entry 1".to_string(),
    //         ),
    //     ]);
    //     assert_eq!(
    //         collection,
    //         IdentifiedVecOf::from_iter([
    //             IdentifiedEntry::new(
    //                 Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
    //                 "Entry 1".to_string()
    //             ),
    //             IdentifiedEntry::new(
    //                 Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
    //                 "Entry 1".to_string()
    //             )
    //         ])
    //     );
    //     let collection = CollectionOfIdentifiedEntries::from_iter([
    //         IdentifiedEntry::placeholder_entry_1(),
    //         IdentifiedEntry::new(
    //             Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
    //             "Entry 1".to_string()
    //         )
    //     ])
    //     // let coll = IdentifiedVecOf::<IdentifiedEntry<User>>::from_iter([
    //     //     IdentifiedEntry::placeholder_with_values(id, value),
    //     //     IdentifiedEntry::new(
    //     //         Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
    //     //         "Entry 1"
    //     //     )
    //     // ]);
    //     let collection_of_identified_entries =
    //         CollectionOfIdentifiedEntries::from(coll);
    //     assert_eq!(
    //         collection_of_identified_entries,
    //         CollectionOfIdentifiedEntries::new(IdentifiedVecOf::from_iter([
    //             IdentifiedEntry::placeholder_entry_1(),
    //             IdentifiedEntry::new(
    //                 Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
    //                 "Entry 1"
    //             )
    //         ]))
    //     );
    // }

    // #[test]
    // fn new_collection() {
    //     let collection_1: IdentifiedVec<Uuid, IdentifiedEntry<&str>> =
    //         IdentifiedVecOf::from_iter([
    //             IdentifiedEntry::placeholder_entry_1(),
    //             IdentifiedEntry::placeholder_entry_2(),
    //         ]);

    //     let collection = CollectionOfIdentifiedEntries::from(collection_1);
    //     assert_eq!(
    //         collection,
    //         vec![
    //             IdentifiedEntry::new(
    //                 Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
    //                 "Test"
    //             ),
    //             IdentifiedEntry::new(
    //                 Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
    //                 "Second Test"
    //             )
    //         ]
    //     );

    //     let collection_of_identified_entries =
    //         CollectionOfIdentifiedEntries::new(collection).unwrap();
    //     assert_eq!(
    //         collection_of_identified_entries,
    //         CollectionOfIdentifiedEntries {
    //             collection: vec![first_entry, second_entry]
    //         }
    //     );
    // }

    // #[test]
    // fn add_entry_collection() {
    //     let first_entry = IdentifiedEntry::new(
    //         Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
    //         "Test",
    //     );
    //     let second_entry = IdentifiedEntry::new(
    //         Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
    //         "Second Test",
    //     );

    //     let collection = vec![first_entry.clone(), second_entry.clone()];
    //     assert_eq!(
    //         collection,
    //         vec![
    //             IdentifiedEntry::new(
    //                 Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
    //                 "Test"
    //             ),
    //             IdentifiedEntry::new(
    //                 Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
    //                 "Second Test"
    //             )
    //         ]
    //     );

    //     let mut collection_of_identified_entries =
    //         CollectionOfIdentifiedEntries::new(collection).unwrap();
    //     assert_eq!(
    //         collection_of_identified_entries,
    //         CollectionOfIdentifiedEntries {
    //             collection: vec![first_entry.clone(), second_entry.clone()]
    //         }
    //     );

    //     let new_entry = IdentifiedEntry::new(
    //         Uuid::from_str("CCCCCCCC-3333-4444-5555-CCCCCCCCCCCC").unwrap(),
    //         "New Entry",
    //     );

    //     collection_of_identified_entries.add(new_entry.clone()).unwrap();

    //     assert_eq!(
    //         collection_of_identified_entries,
    //         CollectionOfIdentifiedEntries {
    //             collection: vec![first_entry, second_entry, new_entry]
    //         }
    //     )
    // }

    // #[test]
    // fn add_entry_collection_duplicate_ids() {
    //     let first_entry = IdentifiedEntry::new(
    //         Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
    //         "Test",
    //     );
    //     let second_entry = IdentifiedEntry::new(
    //         Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
    //         "Second Test",
    //     );

    //     let collection = vec![first_entry.clone(), second_entry.clone()];
    //     assert_eq!(
    //         collection,
    //         vec![
    //             IdentifiedEntry::new(
    //                 Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
    //                 "Test"
    //             ),
    //             IdentifiedEntry::new(
    //                 Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
    //                 "Second Test"
    //             )
    //         ]
    //     );

    //     let mut collection_of_identified_entries =
    //         CollectionOfIdentifiedEntries::new(collection).unwrap();
    //     assert_eq!(
    //         collection_of_identified_entries,
    //         CollectionOfIdentifiedEntries {
    //             collection: vec![first_entry.clone(), second_entry.clone()]
    //         }
    //     );

    //     let new_entry = IdentifiedEntry::new(
    //         Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
    //         "New Entry",
    //     );

    //     let result_after_add = collection_of_identified_entries.add(new_entry.clone());

    //     assert_eq!(
    //         result_after_add,
    //         Err(CollectionError::DuplicateIDOfValuesFound)
    //     );
    // }

    // #[test]
    // fn add_entry_collection_duplicate_values() {
    //     let first_entry = IdentifiedEntry::new(
    //         Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
    //         "Test",
    //     );
    //     let second_entry = IdentifiedEntry::new(
    //         Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
    //         "Second Test",
    //     );

    //     let collection = vec![first_entry.clone(), second_entry.clone()];
    //     assert_eq!(
    //         collection,
    //         vec![
    //             IdentifiedEntry::new(
    //                 Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
    //                 "Test"
    //             ),
    //             IdentifiedEntry::new(
    //                 Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
    //                 "Second Test"
    //             )
    //         ]
    //     );

    //     let mut collection_of_identified_entries =
    //         CollectionOfIdentifiedEntries::new(collection).unwrap();
    //     assert_eq!(
    //         collection_of_identified_entries,
    //         CollectionOfIdentifiedEntries {
    //             collection: vec![first_entry.clone(), second_entry.clone()]
    //         }
    //     );

    //     let new_entry = IdentifiedEntry::new(
    //         Uuid::from_str("CCCCCCCC-3333-4444-5555-CCCCCCCCCCCC").unwrap(),
    //         "Second Test",
    //     );

    //     let result_after_add = collection_of_identified_entries.add(new_entry.clone());

    //     assert_eq!(result_after_add, Err(CollectionError::DuplicateValuesFound));
    // }

    // #[test]
    // fn update_entry_collection() {
    //     let first_entry = IdentifiedEntry::new(
    //         Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
    //         "Test",
    //     );
    //     let second_entry = IdentifiedEntry::new(
    //         Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
    //         "Second Test",
    //     );

    //     let collection = vec![first_entry.clone(), second_entry.clone()];
    //     assert_eq!(
    //         collection,
    //         vec![
    //             IdentifiedEntry::new(
    //                 Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
    //                 "Test"
    //             ),
    //             IdentifiedEntry::new(
    //                 Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
    //                 "Second Test"
    //             )
    //         ]
    //     );

    //     let mut collection_of_identified_entries =
    //         CollectionOfIdentifiedEntries::new(collection).unwrap();
    //     assert_eq!(
    //         collection_of_identified_entries,
    //         CollectionOfIdentifiedEntries {
    //             collection: vec![first_entry.clone(), second_entry.clone()]
    //         }
    //     );

    //     let updated_entry = IdentifiedEntry::new(
    //         Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
    //         "Updated Value",
    //     );

    //     collection_of_identified_entries.update(updated_entry.clone()).unwrap();

    //     assert_eq!(
    //         collection_of_identified_entries,
    //         CollectionOfIdentifiedEntries {
    //             collection: vec![first_entry, updated_entry]
    //         }
    //     );
    // }

    // #[test]
    // fn update_entry_collection_id_not_found() {
    //     let first_entry = IdentifiedEntry::new(
    //         Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
    //         "Test",
    //     );
    //     let second_entry = IdentifiedEntry::new(
    //         Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
    //         "Second Test",
    //     );

    //     let collection = vec![first_entry.clone(), second_entry.clone()];
    //     assert_eq!(
    //         collection,
    //         vec![
    //             IdentifiedEntry::new(
    //                 Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
    //                 "Test"
    //             ),
    //             IdentifiedEntry::new(
    //                 Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
    //                 "Second Test"
    //             )
    //         ]
    //     );

    //     let mut collection_of_identified_entries =
    //         CollectionOfIdentifiedEntries::new(collection).unwrap();
    //     assert_eq!(
    //         collection_of_identified_entries,
    //         CollectionOfIdentifiedEntries {
    //             collection: vec![first_entry.clone(), second_entry.clone()]
    //         }
    //     );

    //     let updated_entry = IdentifiedEntry::new(
    //         Uuid::from_str("CCCCCCCC-3333-4444-5555-CCCCCCCCCCCC").unwrap(),
    //         "Updated Value",
    //     );

    //     let result = collection_of_identified_entries.update(updated_entry.clone());

    //     assert_eq!(
    //         result,
    //         Err(CollectionError::PersonaFieldCollectionValueWithIDNotFound)
    //     );
    // }

    // #[test]
    // fn description() {
    //     let first_entry = IdentifiedEntry::new(
    //         Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
    //         "Test",
    //     );
    //     let second_entry = IdentifiedEntry::new(
    //         Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
    //         "Second Test",
    //     );

    //     let collection = vec![first_entry.clone(), second_entry.clone()];
    //     assert_eq!(
    //         collection,
    //         vec![
    //             IdentifiedEntry::new(
    //                 Uuid::from_str("AAAAAAAA-9999-8888-7777-AAAAAAAAAAAA").unwrap(),
    //                 "Test"
    //             ),
    //             IdentifiedEntry::new(
    //                 Uuid::from_str("BBBBBBBB-0000-1111-2222-BBBBBBBBBBBB").unwrap(),
    //                 "Second Test"
    //             )
    //         ]
    //     );

    //     let collection_of_identified_entries =
    //         CollectionOfIdentifiedEntries::new(collection).unwrap();
    //     assert_eq!(
    //         collection_of_identified_entries,
    //         CollectionOfIdentifiedEntries {
    //             collection: vec![first_entry.clone(), second_entry.clone()]
    //         }
    //     );

    //     let description = collection_of_identified_entries.description();
    //     assert_eq!(
    //         description,
    //         format!(
    //             "value: {} id: {} value: {} id: {} ",
    //             first_entry.value, first_entry.id, second_entry.value, second_entry.id
    //         )
    //     )
    // }
}
