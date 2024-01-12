use crate::prelude::*;

impl Wallet {
    pub fn load_private_device_factor_source(
        &self,
        device_factor_source: DeviceFactorSource,
    ) -> Result<PrivateHierarchicalDeterministicFactorSource> {
        self.wallet_client_storage
            .load_mnemonic_with_passphrase(&device_factor_source.id)
            .map(|mwp| PrivateHierarchicalDeterministicFactorSource::new(mwp, device_factor_source))
    }
    pub fn load_private_device_factor_source_by_id(
        &self,
        id: &FactorSourceIDFromHash,
    ) -> Result<PrivateHierarchicalDeterministicFactorSource> {
        let device_factor_source = self.profile().device_factor_source_by_id(id)?;
        self.load_private_device_factor_source(device_factor_source)
    }
}

//========
// SET - Account
//========
#[uniffi::export]
impl Wallet {
    /// Creates a new non securified account **WITHOUT** saving it, using the `main` "Babylon" `DeviceFactorSource` and the "next" index for this FactorSource
    /// as derivation path.
    pub fn create_new_account(&self, network_id: NetworkID, name: DisplayName) -> Result<Account> {
        let profile = &self.profile();
        let bdfs = profile.bdfs();
        let index = profile.next_derivation_index_for_entity(EntityKind::Accounts, network_id);
        let number_of_accounts_on_network = profile
            .networks
            .get(&network_id)
            .map(|n| n.accounts.len())
            .unwrap_or(0);

        let appearance_id =
            AppearanceID::from_number_of_accounts_on_network(number_of_accounts_on_network);

        let factor_instance = self
            .load_private_device_factor_source(bdfs)
            .map(|p| p.derive_account_creation_factor_instance(network_id, index))?;

        let account = Account::new(factor_instance, name, appearance_id);

        Ok(account)
    }

    /// Returns `Ok(())` if the `account` was new and successfully saved. If saving failed or if the account was already present in Profile, an
    /// error is thrown (strict).
    pub fn save_new_account(&self, account: Account) -> Result<()> {
        // TODO: clean this up, BAD code. messy, mostly because of (my) bad IdentifiedVec API.
        let network_id = account.network_id.clone();
        let err_exists = CommonError::AccountAlreadyPresent(account.id().clone());
        self.try_write(|mut p| {
            let networks = &mut p.networks;
            if networks.contains_id(&network_id) {
                networks
                    .try_update_with(&network_id, |network| {
                        if network.accounts.append(account.clone()).0 {
                            Ok(network.clone())
                        } else {
                            return Err(err_exists.clone());
                        }
                    })
                    .and_then(|r| if r { Ok(()) } else { Err(err_exists.clone()) })
            } else {
                let network = Network::new(network_id, Accounts::from_iter([account.to_owned()]));
                networks.append(network);
                Ok(())
            }
        })
    }

    /// Create a new Account and saves it into the active Profile.
    pub fn create_and_save_new_account(
        &self,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<Account> {
        let account = self.create_new_account(network_id, name)?;
        self.save_new_account(account.clone())?;
        Ok(account)
    }

    /// Updates the display name of account with the provided address, throws an error if the account is unknown to the wallet.
    pub fn change_name_of_account(
        &self,
        address: AccountAddress,
        to: DisplayName,
    ) -> Result<Account> {
        self.write(|mut p| p.update_account(&address, |a| a.display_name = to.to_owned()))
            .ok_or_else(|| CommonError::UnknownAccount)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn change_display_name_of_accounts() {
        let profile = Profile::placeholder();
        let wallet = Wallet::ephemeral(profile.clone());
        let account = wallet.read(|p| p.networks[0].accounts[0].clone());
        assert_eq!(account.display_name.value, "Alice");
        assert!(wallet
            .change_name_of_account(account.address, DisplayName::new("Stella").unwrap())
            .is_ok());
        wallet.read(|p| assert_eq!(p.networks[0].accounts[0].display_name.value, "Stella"));

        assert_eq!(
            wallet.change_name_of_account(
                AccountAddress::placeholder_other(),
                DisplayName::new("not used").unwrap()
            ),
            Err(CommonError::UnknownAccount)
        );
    }
}