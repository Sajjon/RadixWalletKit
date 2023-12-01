use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

use super::badge_virtual_source::FactorInstanceBadgeVirtualSource;
use enum_as_inner::EnumAsInner;
/// Either a "physical" badge (NFT) or some source for recreation of a producer
/// of a virtual badge (signature), e.g. a HD derivation path, from which a private key
/// is derived which produces virtual badges (signatures).
#[derive(Serialize, Deserialize, EnumAsInner, Clone, Debug, PartialEq, Eq)]
#[serde(remote = "Self")]
pub enum FactorInstanceBadge {
    #[serde(rename = "virtualSource")]
    Virtual(FactorInstanceBadgeVirtualSource),
}

impl FactorInstanceBadge {
    pub fn placeholder() -> Self {
        FactorInstanceBadge::Virtual(FactorInstanceBadgeVirtualSource::placeholder())
    }
}

impl<'de> Deserialize<'de> for FactorInstanceBadge {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // https://github.com/serde-rs/serde/issues/1343#issuecomment-409698470
        #[derive(Deserialize, Serialize)]
        struct Wrapper {
            #[serde(rename = "discriminator")]
            _ignore: String,
            #[serde(flatten, with = "FactorInstanceBadge")]
            inner: FactorInstanceBadge,
        }
        Wrapper::deserialize(deserializer).map(|w| w.inner)
    }
}

impl Serialize for FactorInstanceBadge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("FactorInstanceBadge", 2)?;
        match self {
            FactorInstanceBadge::Virtual(virtual_source) => {
                let discriminant = "virtualSource";
                state.serialize_field("discriminator", discriminant)?;
                state.serialize_field(discriminant, virtual_source)?;
            }
        }
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use wallet_kit_common::json::assert_eq_after_json_roundtrip;

    use super::FactorInstanceBadge;
    #[test]
    fn json_roundtrip() {
        let model = FactorInstanceBadge::placeholder();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
			{
				"virtualSource": {
					"hierarchicalDeterministicPublicKey": {
						"publicKey": {
							"curve": "curve25519",
							"compressedData": "d24cc6af91c3f103d7f46e5691ce2af9fea7d90cfb89a89d5bba4b513b34be3b"
						},
						"derivationPath": {
							"scheme": "cap26",
							"path": "m/44H/1022H/1H/525H/1460H/0H"
						}
					},
					"discriminator": "hierarchicalDeterministicPublicKey"
				},
				"discriminator": "virtualSource"
			}
            "#,
        );
    }
}