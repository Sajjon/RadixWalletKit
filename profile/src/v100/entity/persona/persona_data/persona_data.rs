use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq, uniffi::Record,
)]
pub struct PersonaData {}

impl Default for PersonaData {
    fn default() -> Self {
        Self {}
    }
}
