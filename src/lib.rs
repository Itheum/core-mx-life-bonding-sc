#![no_std]

multiversx_sc::imports!();

pub mod errors;
pub mod storage;
pub mod views;

#[multiversx_sc::contract]
pub trait LifeBondingContract: storage::StorageModule + views::ViewsModule {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
