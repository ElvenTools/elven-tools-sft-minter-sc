multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait Storage {
    // Storage move to separate module
    #[view(getSftTokenId)]
    #[storage_mapper("sftTokenId")]
    fn sft_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getCollectionTokenName)]
    #[storage_mapper("collectionTokenName")]
    fn collection_token_name(&self) -> SingleValueMapper<ManagedBuffer>;

    #[storage_mapper("createTokenTesting")]
    fn create_token_testing(&self) -> SingleValueMapper<ManagedBuffer>;
}
