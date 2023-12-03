use serde::{Deserialize, Serialize};

use super::ledger_hardware_wallet_model::LedgerHardwareWalletModel;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct LedgerHardwareWalletHint {
    /// "Orange, scratched"
    pub name: String,

    /// E.g. `nanoS+`
    pub model: LedgerHardwareWalletModel,
}

impl LedgerHardwareWalletHint {
    pub fn new(name: &str, model: LedgerHardwareWalletModel) -> Self {
        Self {
            name: name.to_string(),
            model,
        }
    }
}

impl LedgerHardwareWalletHint {
    pub fn placeholder() -> Self {
        Self::new("Orange, scratched", LedgerHardwareWalletModel::NanoSPlus)
    }
}

#[cfg(test)]
mod tests {
    use wallet_kit_common::json::assert_eq_after_json_roundtrip;

    use super::LedgerHardwareWalletHint;
    #[test]
    fn json_roundtrip() {
        let model = LedgerHardwareWalletHint::placeholder();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "name": "Orange, scratched",
                "model": "nanoS+"
            }
            "#,
        );
    }
}