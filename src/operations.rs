multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::storage;

#[multiversx_sc::module]
pub trait Operations: storage::Storage {
  #[only_user_account]
  #[payable("EGLD")]
  #[endpoint(buy)]
  fn buy(&self, amount_of_tokens: u32, nonce: u64) {
    require!(!self.collection_token_id().is_empty(), "Collection token not issued!");
    require!(!self.token_price_tag(nonce).is_empty(), "SFT token with such nonce doesn't exist");
    require!(
      amount_of_tokens > 0,
      "The number of tokens provided can't be less than 1!"
    );
    require!(
      self.token_price_tag(nonce).get().max_per_address >= amount_of_tokens,
      "The number of tokens has to be less than or equal the maximum per address"
    );

    let payment_amount = self.call_value().egld_value();
    let single_payment_amount = &payment_amount / amount_of_tokens;
    let price_tag = self.token_price_tag(nonce).get().price;

    require!(
        single_payment_amount == price_tag,
        "Invalid amount as payment. Check payment per one token and amount of tokens you want to buy."
    );

    let collection_token = self.collection_token_id().get();
    let caller = self.blockchain().get_caller();

    self.send()
      .direct_esdt(&caller, &collection_token, nonce, &BigUint::from(amount_of_tokens));

    let payment_nonce: u64 = 0;
    let payment_token = &EgldOrEsdtTokenIdentifier::egld();

    let owner = self.blockchain().get_owner_address();
    self.send()
      .direct(&owner, &payment_token, payment_nonce, &payment_amount);
  }
}
