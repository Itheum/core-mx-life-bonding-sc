#![no_std]

use config::State;

use crate::{
    errors::{
        ERR_BOND_NOT_FOUND, ERR_CONTRACT_INACTIVE, ERR_ENDPOINT_CALLABLE_ONLY_BY_SC,
        ERR_INVALID_AMOUNT_SENT, ERR_INVALID_LOCK_PERIOD, ERR_INVALID_TOKEN_IDENTIFIER,
    },
    storage::{Bond, Compensation},
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
            !self.lock_period_bond_amount(lock_period).is_empty(),
            ERR_INVALID_LOCK_PERIOD
        ); // check not really needed

        let bond_amount = self.lock_period_bond_amount(lock_period).get();

        require!(payment.amount == bond_amount, ERR_INVALID_AMOUNT_SENT);

        let current_timestamp = self.blockchain().get_block_timestamp();
        let unbound_timestamp = current_timestamp + self.trasform_days_in_seconds(lock_period);

        let bond = Bond {
            address: original_caller.clone(),
            token_identifier: token_identifier.clone(),
            nonce: nonce.clone(),
            lock_period,
            bond_timestamp: current_timestamp,
            unbound_timestamp,
            bond_amount: payment.amount,
        };

        self.address_bonds(&original_caller, &token_identifier, nonce)
            .set(bond.clone());
        self.bonds().insert(bond);

        // create compensation storage on bond if not exists
        if self.compensations(&token_identifier, nonce).is_empty() {
            let compensation = Compensation {
                token_identifier: token_identifier.clone(),
                nonce,
                total_compenstation_amount: BigUint::from(0u64),
            };

            self.compensations(&token_identifier, nonce)
                .set(compensation);
        }
    }

    #[endpoint(withdraw)]
    fn withdraw(&self, token_identifier: TokenIdentifier, nonce: u64) {
        only_active!(self, ERR_CONTRACT_INACTIVE);
        let caller = self.blockchain().get_caller();

        require!(
            !self
                .address_bonds(&caller, &token_identifier, nonce)
                .is_empty(),
            ERR_BOND_NOT_FOUND
        );

        let bond = self.address_bonds(&caller, &token_identifier, nonce).get();

        let current_timestamp = self.blockchain().get_block_timestamp();

        if bond.unbound_timestamp >= current_timestamp {
            let penalty_amount = &bond.bond_amount * &BigUint::from(self.withdraw_penalty().get())
                / &BigUint::from(10_000u64);

            self.send().direct_esdt(
                &caller,
                &self.bond_token().get(),
                0u64,
                &(&bond.bond_amount - &penalty_amount),
            );

            let mut compensation = self.compensations(&token_identifier, nonce).get();
            compensation.total_compenstation_amount += &penalty_amount; // Update total compensation amount

            self.compensations(&token_identifier, nonce)
                .set(compensation);
        } else {
            self.send()
                .direct_esdt(&caller, &self.bond_token().get(), 0u64, &bond.bond_amount);
        }
    }
}
