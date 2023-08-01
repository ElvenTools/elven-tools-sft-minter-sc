multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::storage;
use crate::storage::TokenTag;

#[multiversx_sc::module]
pub trait Operations: storage::Storage {
    #[only_user_account]
    #[payable("EGLD")]
    #[endpoint(buy)]
    fn buy(&self, amount_of_tokens: u32, nonce: u64) {
        let token_tag = self.token_tag(nonce).get();
        require!(
            !self.collection_token_id().is_empty(),
            "Collection token not issued!"
        );
        require!(
            self.paused().is_empty(),
            "The minting is paused or haven't started yet!"
        );
        require!(
            !self.token_tag(nonce).is_empty(),
            "SFT token with such nonce doesn't exist"
        );
        require!(
            amount_of_tokens > 0,
            "The number of tokens provided can't be less than 1!"
        );
        require!(
          token_tag.max_per_address >= amount_of_tokens,
            "The number of tokens has to be less than or equal the maximum per address"
        );

        let caller = self.blockchain().get_caller();

        let tokens_per_address = self.tokens_per_address_total(nonce, &caller).get();
        let tokens_limit_per_address = token_tag.max_per_address;

        let tokens_left_to_mint: BigUint;

        if tokens_limit_per_address < tokens_per_address {
            tokens_left_to_mint = BigUint::zero();
        } else {
            tokens_left_to_mint = tokens_limit_per_address - tokens_per_address;
        }

        require!(
            tokens_left_to_mint > 0 && tokens_left_to_mint >= amount_of_tokens,
            "You can't buy such an amount of tokens. Check the limits per one address!"
        );

        let payment_amount = self.call_value().egld_value();
        let single_payment_amount = payment_amount.clone_value() / amount_of_tokens;
        let token_tag = self.token_tag(nonce).get().price;

        require!(
            single_payment_amount == token_tag,
            "Invalid amount as payment. Check payment per one token and amount of tokens you want to buy."
        );

        let collection_token = self.collection_token_id().get();
        let caller = self.blockchain().get_caller();

        self.send().direct_esdt(
            &caller,
            &collection_token,
            nonce,
            &BigUint::from(amount_of_tokens),
        );

        let payment_nonce: u64 = 0;
        let payment_token = &EgldOrEsdtTokenIdentifier::egld();

        let owner = self.blockchain().get_owner_address();
        self.send()
            .direct(&owner, &payment_token, payment_nonce, &payment_amount);

        let tokens_per_address_total = self.tokens_per_address_total(nonce, &caller).get();

        self.tokens_per_address_total(nonce, &caller)
            .set(tokens_per_address_total + amount_of_tokens);
    }

    // As an owner, claim Smart Contract balance - temporary solution for royalities, the SC has to be payable to be able to get royalties
    #[only_owner]
    #[endpoint(claimScFunds)]
    fn claim_sc_funds(&self) {
        self.send().direct_egld(
            &self.blockchain().get_caller(),
            &self
                .blockchain()
                .get_sc_balance(&EgldOrEsdtTokenIdentifier::egld(), 0),
        );
    }

    #[only_owner]
    #[endpoint(setNewPrice)]
    fn set_new_price(&self, nonce: u64, new_price: BigUint) {
        require!(
            !self.token_tag(nonce).is_empty(),
            "SFT token with such nonce doesn't exist"
        );
        require!(new_price >= 0, "Selling price can not be less than 0!");

        let token_tag = self.token_tag(nonce).get();

        let new_token_tag = TokenTag {
            price: new_price,
            ..token_tag
        };

        self.token_tag(nonce).set(new_token_tag);
    }

    #[only_owner]
    #[endpoint(setNewTokensLimitPerAddress)]
    fn set_new_tokens_limit_per_address(&self, nonce: u64, limit: BigUint) {
        let token_tag = self.token_tag(nonce).get();

        let new_token_tag = TokenTag {
            max_per_address: limit,
            ..token_tag
        };

        self.token_tag(nonce).set(new_token_tag);
    }

    #[only_owner]
    #[endpoint(pauseSelling)]
    fn pause_selling(&self) {
        let paused = true;
        self.paused().set(&paused);
    }

    #[only_owner]
    #[endpoint(startSelling)]
    fn start_selling(&self) {
        require!(!self.collection_token_id().is_empty(), "Token not issued!");
        self.paused().clear();
    }

    #[view(getPrice)]
    fn get_price(&self, token_nonce: u64) -> BigUint {
        self.token_tag(token_nonce).get().price
    }

    #[view(getTokenDisplayName)]
    fn get_token_display_name(&self, token_nonce: u64) -> ManagedBuffer {
        self.token_tag(token_nonce).get().display_name
    }

    #[view(getMaxAmountPerAddress)]
    fn get_max_amount_per_address(&self, token_nonce: u64) -> BigUint {
        self.token_tag(token_nonce).get().max_per_address
    }
}
