use serde_repr::{Deserialize_repr, Serialize_repr};

/// The
#[derive(
    Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
#[repr(u16)] // most likely will will not do more than 65536 iterations.
pub enum ProfileSnapshotVersion {
    /// The version we went live with on Babylon mainnet 2023-09-28,
    /// shipped with iOS 1.0.0 (7) and Android v 1.0.0.
    V100 = 100,
}

impl Default for ProfileSnapshotVersion {
    fn default() -> Self {
        Self::V100
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use wallet_kit_test_utils::json::{assert_eq_after_json_roundtrip, assert_json_value_fails};

    use super::ProfileSnapshotVersion;

    #[test]
    fn json() {
        assert_eq_after_json_roundtrip(&ProfileSnapshotVersion::V100, "100");
        assert_json_value_fails::<ProfileSnapshotVersion>(json!(99));
        assert_json_value_fails::<ProfileSnapshotVersion>(json!("99"));
        assert_json_value_fails::<ProfileSnapshotVersion>(json!("100"));
        assert_json_value_fails::<ProfileSnapshotVersion>(json!("v100"));
        assert_json_value_fails::<ProfileSnapshotVersion>(json!("V100"));
    }
}