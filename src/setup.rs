multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::storage;

const TEST_ENTRY: &[u8] = "test".as_bytes();

#[multiversx_sc::module]
pub trait Setup: storage::Storage {
    // Issue main collection token/handler
    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueToken)]
    fn issue_token(
        &self,
        collection_token_name: ManagedBuffer,
        collection_token_ticker: ManagedBuffer,
    ) {
        let issue_cost = self.call_value().egld_value();
        require!(self.sft_token_id().is_empty(), "Token already issued!");

        self.collection_token_name().set(&collection_token_name);

        self.send()
            .esdt_system_sc_proxy()
            .issue_semi_fungible(
                issue_cost,
                &collection_token_name,
                &collection_token_ticker,
                SemiFungibleTokenProperties {
                    can_freeze: false,
                    can_wipe: false,
                    can_pause: false,
                    can_transfer_create_role: false,
                    can_change_owner: false,
                    can_upgrade: false,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(self.callbacks().issue_callback())
            .call_and_exit();
    }

    // Issue callback: Set the token id in storage or return founds when error
    #[callback]
    fn issue_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<EgldOrEsdtTokenIdentifier>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                self.sft_token_id().set(&token_id.unwrap_esdt());
            }
            ManagedAsyncCallResult::Err(_) => {
                let caller = self.blockchain().get_owner_address();
                let returned = self.call_value().egld_or_single_esdt();
                if returned.token_identifier.is_egld() && returned.amount > 0 {
                    self.send()
                        .direct(&caller, &returned.token_identifier, 0, &returned.amount);
                }
            }
        }
    }

    // Set roles for the SFT token
    #[only_owner]
    #[endpoint(setLocalRoles)]
    fn set_local_roles(&self) {
        require!(!self.sft_token_id().is_empty(), "Token not issued!");

        self.send()
            .esdt_system_sc_proxy()
            .set_special_roles(
                &self.blockchain().get_sc_address(),
                &self.sft_token_id().get(),
                (&[
                    EsdtLocalRole::NftCreate,
                    EsdtLocalRole::NftAddQuantity,
                    EsdtLocalRole::NftBurn
                ][..])
                    .into_iter()
                    .cloned(),
            )
            .async_call()
            .call_and_exit();
    }

    // Create actual SFT with amount, assets etc. 
    #[only_owner]
    #[endpoint(createToken)]
    fn create_token(&self) {
        // TODO: just for testing
        self.create_token_testing()
            .set(ManagedBuffer::new_from_bytes(TEST_ENTRY));
    }
}
