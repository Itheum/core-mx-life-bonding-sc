#![no_std]

use config::State;

use crate::{
    config::SECONDS_IN_DAY,
    contexts::base::BondCache,
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
pub mod contexts;
pub mod errors;
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

        // [TO DO] check if bond was already created for this token identifier and nonce

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

        let bond_id = self.next_bond_id();

        let mut bond_cache = BondCache::new(self, bond_id);

        bond_cache.address = original_caller;
        bond_cache.token_identifier = token_identifier;
        bond_cache.nonce = nonce;
        bond_cache.lock_period = lock_period;
        bond_cache.bond_timestamp = current_timestamp;
        bond_cache.unbound_timestamp = unbound_timestamp;
        bond_cache.bond_amount = payment.amount;

        self.address_bonds(&original_caller).insert(bond_id);
        self.bonds().insert(bond_id);

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

        // [TO DO] check if bond exists based on token_identifier and nonce

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

    #[endpoint(renew)]
    fn renew(
        &self,
        token_identifier: TokenIdentifier,
        nonce: u64,
        new_lock_period: OptionalValue<u16>,
    ) {
        only_active!(self, ERR_CONTRACT_INACTIVE);
        let caller = self.blockchain().get_caller();
        require!(
            !self
                .address_bonds(&caller, &token_identifier, nonce)
                .is_empty(),
            ERR_BOND_NOT_FOUND
        );
        let mut bond = self.address_bonds(&caller, &token_identifier, nonce).get();
        let current_timestamp = self.blockchain().get_block_timestamp();

        let new_lock_period = match new_lock_period.into_option() {
            Some(value) => value,     // new value
            None => bond.lock_period, // old value
        };

        if bond.unbound_timestamp > current_timestamp {
            let remaining_time = bond.unbound_timestamp - current_timestamp;
            let remaining_lock_period = remaining_time / SECONDS_IN_DAY;
            bond.unbound_timestamp =
                current_timestamp + self.trasform_days_in_seconds(new_lock_period);
            bond.lock_period = new_lock_period + remaining_lock_period as u16;
        } else {
            bond.unbound_timestamp =
                current_timestamp + self.trasform_days_in_seconds(new_lock_period);
            bond.lock_period = new_lock_period;
            bond.bond_timestamp = current_timestamp;
        }

        self.address_bonds(&caller, &token_identifier, nonce)
            .set(bond);
    }
}
