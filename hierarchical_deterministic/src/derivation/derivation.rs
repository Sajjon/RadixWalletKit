use crate::bip32::{hd_path::HDPath, hd_path_component::HDPathComponent};

use super::derivation_path_scheme::DerivationPathScheme;

pub trait Derivation: Sized {
    fn hd_path(&self) -> &HDPath;

    fn to_string(&self) -> String {
        self.hd_path().to_string()
    }

    fn scheme(&self) -> DerivationPathScheme;

    fn last_component(&self) -> &HDPathComponent {
        self.hd_path().components().last().unwrap()
    }
}