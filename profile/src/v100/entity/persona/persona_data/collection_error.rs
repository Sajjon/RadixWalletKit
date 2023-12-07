use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum CollectionError {
    #[error("Duplicate values found when construction collection")]
    DuplicateValuesFound,

    #[error("Duplicate id of values found when construction collection")]
    DuplicateIDOfValuesFound,

    #[error("No Persona field value with that ID found")]
    PersonaFieldCollectionValueWithIDNotFound,
}
