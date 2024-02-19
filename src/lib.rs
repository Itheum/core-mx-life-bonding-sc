#![no_std]

use config::State;

use crate::{
    errors::{
        ERR_CONTRACT_INACTIVE, ERR_ENDPOINT_CALLABLE_ONLY_BY_SC, ERR_INVALID_AMOUNT_SENT,
        ERR_INVALID_LOCK_PERIOD, ERR_INVALID_TOKEN_IDENTIFIER,
    },
    storage::Bond,
};

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
    fn init(&self) {
        self.contract_state().set(State::Inactive);
        self.minimum_penalty().set(500);
        self.maximum_penalty().set(10_000);
        self.withdraw_penalty().set(8_000);
    }

    #[upgrade]
    fn upgrade(&self) {
        self.contract_state().set(State::Inactive);
    }

    #[payable("*")]
    #[endpoint(bond)]
    fn bond(
        &self,
        original_caller: ManagedAddress,
        token_identifier: TokenIdentifier,
        nonce: u64,
        lock_period: u16, //days
    ) {
        only_active!(self, ERR_CONTRACT_INACTIVE);
        require!(
            self.blockchain()
                .is_smart_contract(&self.blockchain().get_caller()),
            ERR_ENDPOINT_CALLABLE_ONLY_BY_SC
        );
        let payment = self.call_value().single_esdt();

        require!(
            payment.token_identifier == self.bond_token().get(),
            ERR_INVALID_TOKEN_IDENTIFIER
        );

        // [TO DO] check token_identifier is accepted (not really needed as this endpoint will be called by the minting contract)

        require!(
            self.lock_periods().contains(&lock_period),
            ERR_INVALID_LOCK_PERIOD
        );
        require!(
            self.lock_period_bond_amount(lock_period).is_empty(),
            ERR_INVALID_LOCK_PERIOD
        ); // check not really needed

        let bond = self.lock_period_bond_amount(lock_period).get();

        require!(payment.amount == bond, ERR_INVALID_AMOUNT_SENT);

        let current_timestamp = self.blockchain().get_block_timestamp();
        let unbound_timestamp = current_timestamp + self.trasform_days_in_seconds(lock_period);

        let bond = Bond {
            address: original_caller.clone(),
            token_identifier,
            nonce,
            lock_period,
            bond_timestamp: current_timestamp,
            unbound_timestamp,
            bond_amount: payment.amount,
        };

        self.address_bonds(&original_caller).insert(bond.clone());
        self.bonds().insert(bond);
    }
}
