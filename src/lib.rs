#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod setup;
pub mod storage;
pub mod operations;

#[multiversx_sc::contract]
pub trait ElvenToolsSftMinter: storage::Storage + setup::Setup + operations::Operations {
    #[init]
    fn init(&self) {}
}
