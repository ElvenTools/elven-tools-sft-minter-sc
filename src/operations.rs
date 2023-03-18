multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::storage;

#[multiversx_sc::module]
pub trait Operations: storage::Storage {
  #[endpoint(buy)]
  fn buy(&self) {
    // TODO
  }
}
