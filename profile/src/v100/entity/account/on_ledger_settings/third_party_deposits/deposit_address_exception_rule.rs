use std::fmt::Display;

use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use strum::FromRepr;

/// The exception kind for deposit address
#[derive(
    Serialize,
    Deserialize,
    FromRepr,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Sequence,
)]
#[serde(rename_all = "camelCase")]
pub enum DepositAddressExceptionRule {
    /// A resource can always be deposited in to the account by third-parties
    Allow,
    /// A resource can never be deposited in to the account by third-parties
    Deny,
}

impl DepositAddressExceptionRule {
    pub fn discriminant(&self) -> String {
        format!("{}", self)
    }
}

impl Display for DepositAddressExceptionRule {
    fn fmt(
        &self,
        f: &mut radix_engine_common::prelude::fmt::Formatter<'_>,
    ) -> radix_engine_common::prelude::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use wallet_kit_common::{assert_json_roundtrip, assert_json_value_eq_after_roundtrip};

    use super::DepositAddressExceptionRule;

    #[test]
    fn json_roundtrip_accept_all() {
        assert_json_value_eq_after_roundtrip(&DepositAddressExceptionRule::Deny, json!("deny"));
        assert_json_roundtrip(&DepositAddressExceptionRule::Deny);
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", DepositAddressExceptionRule::Deny), "Deny");

        // `discriminant` uses Display
        assert_eq!(DepositAddressExceptionRule::Allow.discriminant(), "Allow");
    }
}
