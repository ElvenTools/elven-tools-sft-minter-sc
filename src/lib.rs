#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod operations;
pub mod setup;
pub mod storage;

#[multiversx_sc::contract]
pub trait ElvenToolsSftMinter: storage::Storage + setup::Setup + operations::Operations {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
