multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::storage;
use crate::storage::TokenTag;

const ROYALTIES_MAX: u32 = 10_000;
const METADATA_KEY_NAME: &[u8] = "metadata:".as_bytes();
const ATTR_SEPARATOR: &[u8] = ";".as_bytes();
const URI_SLASH: &[u8] = "/".as_bytes();
const TAGS_KEY_NAME: &[u8] = "tags:".as_bytes();

#[derive(TypeAbi, TopEncode, TopDecode)]
pub enum SFTProperties {
    CanFreeze,
    CanWipe,
    CanPause,
    CanTransferCreateRole,
    CanChangeOwner,
    CanUpgrade,
    CanAddSpecialRoles,
}

#[derive(TypeAbi, TopEncode, TopDecode)]
pub enum SFTRoles {
    ESDTRoleNFTCreate,
    ESDTRoleNFTBurn,
    ESDTRoleNFTAddQuantity,
    ESDTTransferRole,
}

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
        token_properties: OptionalValue<MultiValueEncoded<SFTProperties>>,
    ) {
        let issue_cost = self.call_value().egld_value();
        require!(
            self.collection_token_id().is_empty(),
            "Token already issued!"
        );

        self.collection_token_name().set(&collection_token_name);

        let mut properties = SemiFungibleTokenProperties {
            can_freeze: false,
            can_wipe: false,
            can_pause: false,
            can_transfer_create_role: false,
            can_change_owner: false,
            can_upgrade: false,
            can_add_special_roles: true, // to proceed it is required anyway, so there is no sense to leave it false
        };

        let properties_option = token_properties.into_option();

        match properties_option {
            Some(value) => {
                for token_propery in value.into_iter() {
                    match token_propery {
                        SFTProperties::CanFreeze => properties.can_freeze = true,
                        SFTProperties::CanWipe => properties.can_wipe = true,
                        SFTProperties::CanPause => properties.can_pause = true,
                        SFTProperties::CanTransferCreateRole => {
                            properties.can_transfer_create_role = true
                        }
                        SFTProperties::CanChangeOwner => properties.can_change_owner = true,
                        SFTProperties::CanUpgrade => properties.can_upgrade = true,
                        SFTProperties::CanAddSpecialRoles => {
                            properties.can_add_special_roles = true
                        }
                    };
                }
            }
            None => {}
        }

        self.send()
            .esdt_system_sc_proxy()
            .issue_semi_fungible(
                issue_cost.clone_value(),
                &collection_token_name,
                &collection_token_ticker,
                properties,
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
                self.collection_token_id().set(&token_id.unwrap_esdt());
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
    fn set_local_roles(&self, token_roles: OptionalValue<MultiValueEncoded<SFTRoles>>) {
        require!(!self.collection_token_id().is_empty(), "Token not issued!");

        let mut roles: ManagedVec<EsdtLocalRole> = ManagedVec::new();
        let roles_option = token_roles.into_option();

        match roles_option {
            Some(value) => {
                for token_role in value.into_iter() {
                    match token_role {
                        SFTRoles::ESDTRoleNFTCreate => roles.push(EsdtLocalRole::NftCreate),
                        SFTRoles::ESDTRoleNFTAddQuantity => {
                            roles.push(EsdtLocalRole::NftAddQuantity)
                        }
                        SFTRoles::ESDTRoleNFTBurn => roles.push(EsdtLocalRole::NftBurn),
                        SFTRoles::ESDTTransferRole => roles.push(EsdtLocalRole::Transfer),
                    }
                }
            }
            // When not prvided these are required roles to proceed
            None => {
                roles.push(EsdtLocalRole::NftCreate);
                roles.push(EsdtLocalRole::NftAddQuantity);
            }
        }

        self.send()
            .esdt_system_sc_proxy()
            .set_special_roles(
                &self.blockchain().get_sc_address(),
                &self.collection_token_id().get(),
                roles.iter(),
            )
            .async_call()
            .call_and_exit();
    }

    // Create actual SFT with amount, assets etc.
    #[only_owner]
    #[endpoint(createToken)]
    fn create_token(
        &self,
        name: ManagedBuffer,
        selling_price: BigUint,
        metadata_ipfs_cid: ManagedBuffer,
        metadata_ipfs_file: ManagedBuffer,
        amount_of_tokens: BigUint,
        max_per_address: BigUint,
        royalties: BigUint,
        tags: ManagedBuffer,
        uris: MultiValueEncoded<ManagedBuffer>,
    ) {
        require!(royalties <= ROYALTIES_MAX, "Royalties cannot exceed 100%!");
        require!(
            amount_of_tokens >= 1,
            "Amount of tokens should be at least 1!"
        );
        require!(selling_price >= 0, "Selling price can not be less than 0!");

        require!(!self.collection_token_id().is_empty(), "Token not issued!");

        let metadata_key_name = ManagedBuffer::new_from_bytes(METADATA_KEY_NAME);
        let tags_key_name = ManagedBuffer::new_from_bytes(TAGS_KEY_NAME);
        let separator = ManagedBuffer::new_from_bytes(ATTR_SEPARATOR);
        let metadata_slash = ManagedBuffer::new_from_bytes(URI_SLASH);

        let mut attributes = ManagedBuffer::new();
        attributes.append(&tags_key_name);
        attributes.append(&tags);
        attributes.append(&separator);
        attributes.append(&metadata_key_name);
        attributes.append(&metadata_ipfs_cid);
        attributes.append(&metadata_slash);
        attributes.append(&metadata_ipfs_file);

        let hash_buffer = self.crypto().sha256(&attributes);
        let attributes_hash = hash_buffer.as_managed_buffer();

        let token_id = self.collection_token_id().get();

        let uris_vec = uris.into_vec_of_buffers();

        let token_nonce = self.send().esdt_nft_create(
            &token_id,
            &amount_of_tokens,
            &name,
            &royalties,
            &attributes_hash,
            &attributes,
            &uris_vec,
        );

        self.token_tag(token_nonce).set(TokenTag {
            display_name: name,
            nonce: token_nonce,
            price: selling_price,
            max_per_address,
        });

        self.paused(token_nonce).set(true);
    }

    // Increase the initial amount/supply for the token
    #[only_owner]
    #[endpoint(mint)]
    fn mint(&self, token_nonce: u64, amount: &BigUint) {
        let token_id = self.collection_token_id().get();
        self.send().esdt_local_mint(&token_id, token_nonce, amount);
    }

    // Decrease the initial amount/supply for the token
    #[only_owner]
    #[endpoint(burn)]
    fn burn(&self, token_nonce: u64, amount: &BigUint) {
        let token_id = self.collection_token_id().get();
        self.send().esdt_local_burn(&token_id, token_nonce, amount);
    }
}
