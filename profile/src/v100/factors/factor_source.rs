use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

use super::factor_sources::device_factor_source::device_factor_source::DeviceFactorSource;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(remote = "Self")]
pub enum FactorSource {
    #[serde(rename = "device")]
    Device(DeviceFactorSource),
}

impl<'de> Deserialize<'de> for FactorSource {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // https://github.com/serde-rs/serde/issues/1343#issuecomment-409698470
        #[derive(Deserialize, Serialize)]
        struct Wrapper {
            #[serde(rename = "discriminator")]
            _ignore: String,
            #[serde(flatten, with = "FactorSource")]
            inner: FactorSource,
        }
        Wrapper::deserialize(deserializer).map(|w| w.inner)
    }
}

impl Serialize for FactorSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("FactorSource", 2)?;
        match self {
            FactorSource::Device(device) => {
                let discriminant = "device";
                state.serialize_field("discriminator", discriminant)?;
                state.serialize_field(discriminant, device)?;
            }
        }
        state.end()
    }
}
#[cfg(test)]
mod tests {
    use wallet_kit_common::json::assert_eq_after_json_roundtrip;

    use crate::v100::factors::factor_sources::device_factor_source::device_factor_source::DeviceFactorSource;

    use super::FactorSource;

    #[test]
    fn json_roundtrip_from_device() {
        let model = FactorSource::Device(DeviceFactorSource::placeholder());
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "discriminator": "device",
                "device": {
                    "id": {
                        "kind": "device",
                        "body": "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"
                    },
                    "common": {
                        "flags": ["main"],
                        "addedOn": "2023-09-11T16:05:56",
                        "cryptoParameters": {
                            "supportedCurves": ["curve25519"],
                            "supportedDerivationPathSchemes": ["cap26"]
                        },
                        "lastUsedOn": "2023-09-11T16:05:56"
                    },
                    "hint": {
                        "name": "Unknown Name",
                        "model": "iPhone",
                        "mnemonicWordCount": 24
                    }
                }
            }
            "#,
        )
    }
}
