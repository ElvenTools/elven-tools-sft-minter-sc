multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait Storage {
  #[view(getTokenSellingPrice)]
    #[storage_mapper("tokenSellingPrice")]
    fn token_selling_price(&self) -> SingleValueMapper<BigUint>;

    #[view(getTokenDisplayName)]
    #[storage_mapper("tokenDisplayName")]
    fn token_display_name(&self) -> SingleValueMapper<ManagedBuffer>;

    #[view(getCollectionTokenId)]
    #[storage_mapper("collectionTokenId")]
    fn collection_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getCollectionTokenName)]
    #[storage_mapper("collectionTokenName")]
    fn collection_token_name(&self) -> SingleValueMapper<ManagedBuffer>;
}
