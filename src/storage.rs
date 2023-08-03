multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct TokenTag<M: ManagedTypeApi> {
    pub display_name: ManagedBuffer<M>,
    pub nonce: u64,
    pub price: BigUint<M>,
    pub max_per_address: BigUint<M>,
}

#[multiversx_sc::module]
pub trait Storage {
    #[view(getCollectionTokenId)]
    #[storage_mapper("collectionTokenId")]
    fn collection_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getCollectionTokenName)]
    #[storage_mapper("collectionTokenName")]
    fn collection_token_name(&self) -> SingleValueMapper<ManagedBuffer>;

    #[view(isPaused)]
    #[storage_mapper("paused")]
    fn paused(&self, token_nonce: u64) -> SingleValueMapper<bool>;

    #[view(getAmountPerAddressTotal)]
    #[storage_mapper("amountPerAddressTotal")]
    fn amount_per_address_total(
        &self,
        token_nonce: u64,
        address: &ManagedAddress,
    ) -> SingleValueMapper<BigUint>;

    #[storage_mapper("tokenTag")]
    fn token_tag(&self, token_nonce: u64) -> SingleValueMapper<TokenTag<Self::Api>>;
}
