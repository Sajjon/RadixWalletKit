use crate::v100::address::IdentityAddress;
use crate::v100::entity::display_name::DisplayName;
use crate::v100::entity::entity_flags::EntityFlags;
use crate::v100::entity::persona::persona_data::persona_data::PersonaData;
use crate::v100::entity_security_state::EntitySecurityState;
use radix_engine_common::prelude::RefCell;
use serde::{Deserialize, Serialize};
use wallet_kit_common::{HasPlaceholder, NetworkID};

/// A network unique identity with a unique public address and a set of cryptographic
/// factors used to control it. The identity is either `virtual` or not. By "virtual"
/// we mean that the Radix Public Ledger does not yet know of the public address
/// of this identity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Persona {
    /// The ID of the network this persona exists on.
    #[serde(rename = "networkID")]
    pub network_id: NetworkID,

    /// The globally unique and identifiable Radix component address of this persona. Can be used as
    /// a stable ID. Cryptographically derived from a seeding public key which typically was created by
    /// the `DeviceFactorSource`
    pub address: IdentityAddress,

    /// Security of this persona
    security_state: EntitySecurityState,

    /// A required non empty display name, used by presentation layer and sent to Dapps when requested.
    display_name: RefCell<DisplayName>,

    /// An order set of `EntityFlag`s used to describe certain Off-ledger
    /// user state about Identities or Personas, such as if an entity is
    /// marked as hidden or not.
    pub flags: RefCell<EntityFlags>,

    persona_data: PersonaData,
}

#[cfg(any(test, feature = "placeholder"))]
impl Persona {
    pub fn new(
        address: IdentityAddress,
        display_name: DisplayName,
        persona_data: PersonaData,
    ) -> Self {
        Self {
            network_id: *address.network_id(),
            address,
            security_state: EntitySecurityState::placeholder(),
            display_name: RefCell::new(display_name),
            flags: RefCell::new(EntityFlags::default()),
            persona_data: persona_data,
        }
    }
}
