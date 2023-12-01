use serde::{de, Deserializer, Serialize, Serializer};

use crate::{
    bip32::{hd_path::HDPath, hd_path_component::HDPathValue},
    cap26::cap26_path::cap26_path::CAP26Path,
    derivation::{derivation::Derivation, derivation_path_scheme::DerivationPathScheme},
    hdpath_error::HDPathError,
};

/// Use it with `GetIDPath::default()` to create the path `m/44'/1022'/365'`
/// which is used by all hierarchal deterministic factor sources to derive
/// the special root key which we hash to form the `FactorSourceIDFromHash`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GetIDPath(HDPath);

impl Derivation for GetIDPath {
    fn hd_path(&self) -> &HDPath {
        &self.0
    }
    fn scheme(&self) -> DerivationPathScheme {
        DerivationPathScheme::Cap26
    }
}

impl Default for GetIDPath {
    fn default() -> Self {
        Self(HDPath::from_str("m/44H/1022H/365H").expect("Valid path"))
    }
}

impl GetIDPath {
    pub fn embed(&self) -> CAP26Path {
        CAP26Path::GetID(self.clone())
    }

    pub const LAST_COMPONENT_VALUE: HDPathValue = 365;

    pub fn from_str(s: &str) -> Result<Self, HDPathError> {
        use HDPathError::*;
        let (path, components) = HDPath::try_parse_base(s, HDPathError::InvalidDepthOfCAP26Path)?;
        if path.depth() != 3 {
            return Err(InvalidDepthOfCAP26Path);
        }
        let value = HDPath::parse_try_map(&components, 2, Box::new(|v| Ok(v)))?;
        if value != Self::LAST_COMPONENT_VALUE {
            return Err(InvalidGetIDPath(value));
        }
        let hd_path = HDPath::from_components(components);
        assert!(Self(hd_path) == Self::default());
        return Ok(Self::default());
    }
}

impl Serialize for GetIDPath {
    /// Serializes this `AccountAddress` into its bech32 address string as JSON.
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for GetIDPath {
    /// Tries to deserializes a JSON string as a bech32 address into an `AccountAddress`.
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<GetIDPath, D::Error> {
        let s = String::deserialize(d)?;
        GetIDPath::from_str(&s).map_err(de::Error::custom)
    }
}

impl TryInto<GetIDPath> for &str {
    type Error = HDPathError;

    fn try_into(self) -> Result<GetIDPath, Self::Error> {
        GetIDPath::from_str(self)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use wallet_kit_common::json::assert_json_value_eq_after_roundtrip;

    use crate::{derivation::derivation::Derivation, hdpath_error::HDPathError};

    use super::GetIDPath;

    #[test]
    fn to_string() {
        assert_eq!(GetIDPath::default().to_string(), "m/44H/1022H/365H");
    }

    #[test]
    fn from_str() {
        assert_eq!(
            GetIDPath::default(),
            GetIDPath::from_str("m/44H/1022H/365H").unwrap()
        );
    }
    #[test]
    fn invalid_value() {
        assert_eq!(
            GetIDPath::from_str("m/44H/1022H/1337H"),
            Err(HDPathError::InvalidGetIDPath(1337))
        );
    }
    #[test]
    fn invalid_depth() {
        assert_eq!(
            GetIDPath::from_str("m/44H/1022H"),
            Err(HDPathError::InvalidDepthOfCAP26Path)
        );
    }

    #[test]
    fn json_roundtrip() {
        let str = "m/44H/1022H/365H";
        let parsed: GetIDPath = str.try_into().unwrap();
        assert_json_value_eq_after_roundtrip(&parsed, json!(str));
    }
}