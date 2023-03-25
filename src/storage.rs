multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct TokenPriceTag<M: ManagedTypeApi> {
    pub display_name: ManagedBuffer<M>,
    pub nonce: u64,
    pub price: BigUint<M>,
    pub max_per_address: BigUint<M>,
}

#[multiversx_sc::module]
pub trait Storage {
    #[view(getTokenPriceTag)]
    #[storage_mapper("tokenPriceTag")]
    fn token_price_tag(&self, token_nonce: u64) -> SingleValueMapper<TokenPriceTag<Self::Api>>;

    #[view(getCollectionTokenId)]
    #[storage_mapper("collectionTokenId")]
    fn collection_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getCollectionTokenName)]
    #[storage_mapper("collectionTokenName")]
    fn collection_token_name(&self) -> SingleValueMapper<ManagedBuffer>;
}
