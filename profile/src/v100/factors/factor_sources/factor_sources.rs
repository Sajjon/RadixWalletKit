use crate::prelude::*;

impl Identifiable for FactorSource {
    type ID = FactorSourceID;

    fn id(&self) -> Self::ID {
        self.factor_source_id()
    }
}

/// A collection of FactorSources generated by a wallet or manually added by user.
/// MUST never be empty.
pub type FactorSources = IdentifiedVecVia<FactorSource>;

#[uniffi::export]
pub fn new_factor_sources_placeholder() -> FactorSources {
    FactorSources::placeholder()
}
#[uniffi::export]
pub fn new_factor_sources_placeholder_other() -> FactorSources {
    FactorSources::placeholder_other()
}

impl FactorSources {
    /// Panics if `device_factor_source` is not using Babylon crypto parameters
    /// AND marked "main".
    pub fn with_bdfs(device_factor_source: DeviceFactorSource) -> Self {
        assert!(device_factor_source.is_main_bdfs());
        Self::from_iter([device_factor_source.into()])
    }

    /// Panics if this `FactorSources` is empty.
    pub fn assert_not_empty(&self) {
        assert_ne!(
            self.len(),
            0,
            "FactorSources empty, which must never happen."
        )
    }
}

impl HasPlaceholder for FactorSources {
    fn placeholder() -> Self {
        Self::from_iter([
            FactorSource::placeholder_device(),
            FactorSource::placeholder_ledger(),
        ])
    }

    fn placeholder_other() -> Self {
        Self::from_iter([
            FactorSource::placeholder_device_olympia(),
            FactorSource::placeholder_device_babylon(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn identifiable_id_uses_factor_source_id() {
        assert_eq!(
            FactorSource::placeholder_device().id(),
            FactorSource::placeholder_device().factor_source_id()
        )
    }

    #[test]
    fn inequality() {
        assert_ne!(
            FactorSources::placeholder(),
            FactorSources::placeholder_other()
        );
    }

    #[test]
    fn duplicates_are_prevented() {
        assert_eq!(
            FactorSources::from_iter(
                [FactorSource::placeholder(), FactorSource::placeholder()]
                    .into_iter()
            )
            .len(),
            1
        )
    }

    #[test]
    fn json_roundtrip_placeholder() {
        let sut = FactorSources::placeholder();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            [
                {
                    "discriminator": "device",
                    "device": {
                        "id": {
                            "kind": "device",
                            "body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
                        },
                        "common": {
                            "flags": ["main"],
                            "addedOn": "2023-09-11T16:05:56.000Z",
                            "cryptoParameters": {
                                "supportedCurves": ["curve25519"],
                                "supportedDerivationPathSchemes": ["cap26"]
                            },
                            "lastUsedOn": "2023-09-11T16:05:56.000Z"
                        },
                        "hint": {
                            "name": "Unknown Name",
                            "model": "iPhone",
                            "mnemonicWordCount": 24
                        }
                    }
                },
                {
                    "discriminator": "ledgerHQHardwareWallet",
                    "ledgerHQHardwareWallet": {
                        "id": {
                            "kind": "ledgerHQHardwareWallet",
                            "body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
                        },
                        "common": {
                            "addedOn": "2023-09-11T16:05:56.000Z",
                            "cryptoParameters": {
                                "supportedCurves": ["curve25519"],
                                "supportedDerivationPathSchemes": ["cap26"]
                            },
                            "flags": ["main"],
                            "lastUsedOn": "2023-09-11T16:05:56.000Z"
                        },
                        "hint": {
                            "name": "Orange, scratched",
                            "model": "nanoS+"
                        }
                    }
                }
            ]
            "#,
        )
    }
}

#[cfg(test)]
mod uniffi_tests {
    use crate::{
        new_factor_sources_placeholder, new_factor_sources_placeholder_other,
        HasPlaceholder,
    };

    use super::FactorSources;

    #[test]
    fn equality_placeholders() {
        assert_eq!(
            FactorSources::placeholder(),
            new_factor_sources_placeholder()
        );
        assert_eq!(
            FactorSources::placeholder_other(),
            new_factor_sources_placeholder_other()
        );
    }
}
