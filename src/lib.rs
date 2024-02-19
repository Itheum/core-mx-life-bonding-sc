#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod admin;
pub mod config;
pub mod errors;
pub mod macros;
pub mod storage;
pub mod views;

#[multiversx_sc::contract]
pub trait LifeBondingContract:
    storage::StorageModule + views::ViewsModule + admin::AdminModule + config::ConfigModule
{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
