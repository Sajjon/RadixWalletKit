use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[cfg(any(test, feature = "placeholder"))]
use wallet_kit_common::HasPlaceholder;

/// Settings related to displaying of information to the user inside the app.
///
/// **N.B. neither of these settings are in fact not yet used by clients.**
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Getters)]
#[serde(rename_all = "camelCase")]
pub struct AppDisplay {
    /// If we should show the aggregate value of users portfolio in fiat currency
    /// of hide it.
    is_currency_amount_visible: bool,

    /// Which fiat currency the prices are measured in.
    fiat_currency_price_target: FiatCurrency,
}

impl AppDisplay {
    pub fn new(is_currency_amount_visible: bool, fiat_currency: FiatCurrency) -> Self {
        Self {
            is_currency_amount_visible,
            fiat_currency_price_target: fiat_currency,
        }
    }
}

impl Default for AppDisplay {
    fn default() -> Self {
        Self::new(true, FiatCurrency::default())
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl HasPlaceholder for AppDisplay {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::new(true, FiatCurrency::default())
    }

    /// A placeholder used to facilitate unit tests.
    fn placeholder_other() -> Self {
        Self::new(false, FiatCurrency::default())
    }
}

/// Fiat currency to measure and display the value of some XRD or other Radix assets value/worth in.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FiatCurrency {
    /// American dollars.
    #[serde(rename = "usd")]
    USD,
}

impl Default for FiatCurrency {
    /// American dollars.
    fn default() -> Self {
        Self::USD
    }
}

#[cfg(test)]
mod tests {
    use wallet_kit_common::assert_eq_after_json_roundtrip;
    use wallet_kit_common::HasPlaceholder;

    use super::AppDisplay;
    use super::FiatCurrency;

    #[test]
    fn equality() {
        assert_eq!(AppDisplay::placeholder(), AppDisplay::placeholder());
        assert_eq!(
            AppDisplay::placeholder_other(),
            AppDisplay::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(AppDisplay::placeholder(), AppDisplay::placeholder_other());
    }

    #[test]
    fn usd_is_default() {
        assert_eq!(
            AppDisplay::default().fiat_currency_price_target,
            FiatCurrency::USD
        );
    }

    #[test]
    fn fiat_worth_is_visible_by_default() {
        assert_eq!(AppDisplay::default().is_currency_amount_visible, true);
    }

    #[test]
    fn json_roundtrip() {
        let sut = AppDisplay::placeholder();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "fiatCurrencyPriceTarget": "usd",
                "isCurrencyAmountVisible": true
            }
            "#,
        )
    }
}
