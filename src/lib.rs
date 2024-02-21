#![no_std]

use config::State;

use crate::{
    config::SECONDS_IN_DAY,
    contexts::base::BondCache,
    errors::{
        ERR_BOND_ALREADY_CREATED, ERR_BOND_NOT_FOUND, ERR_CONTRACT_INACTIVE,
        ERR_ENDPOINT_CALLABLE_ONLY_BY_ACCEPTED_CALLERS, ERR_ENDPOINT_CALLABLE_ONLY_BY_SC,
        ERR_INVALID_AMOUNT_SENT, ERR_INVALID_LOCK_PERIOD, ERR_INVALID_TOKEN_IDENTIFIER,
    },
    storage::Compensation,
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
        require_contract_active!(self, ERR_CONTRACT_INACTIVE);
        require!(
            self.blockchain()
                .is_smart_contract(&self.blockchain().get_caller()),
            ERR_ENDPOINT_CALLABLE_ONLY_BY_SC
        );

        require!(
            self.accepted_callers()
                .contains(&self.blockchain().get_caller()),
            ERR_ENDPOINT_CALLABLE_ONLY_BY_ACCEPTED_CALLERS
        );

        let payment = self.call_value().single_esdt();

        require!(
            payment.token_identifier == self.bond_payment_token().get(),
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

        let check_bond_id = self
            .object_to_id()
            .get_id((token_identifier.clone(), nonce));

        require!(
            !self.object_to_id().contains_id(check_bond_id),
            ERR_BOND_ALREADY_CREATED
        );

        let bond_id = self
            .object_to_id()
            .insert_new((token_identifier.clone(), nonce));

        self.bond_address(bond_id).set(original_caller.clone());
        self.bond_token_identifier(bond_id)
            .set(token_identifier.clone());
        self.bond_nonce(bond_id).set(nonce);
        self.bond_lock_period(bond_id).set(lock_period);
        self.bond_timestamp(bond_id).set(current_timestamp);
        self.unbound_timestamp(bond_id).set(unbound_timestamp);
        self.bond_amount(bond_id).set(payment.amount);

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
        require_contract_active!(self, ERR_CONTRACT_INACTIVE);
        let caller = self.blockchain().get_caller();

        let bond_id = self.object_to_id().get_id((token_identifier, nonce));

        require!(self.object_to_id().contains_id(bond_id), ERR_BOND_NOT_FOUND);

        let mut bond_cache = BondCache::new(self, bond_id);

        let current_timestamp = self.blockchain().get_block_timestamp();

        if bond_cache.unbound_timestamp >= current_timestamp {
            let penalty_amount = &bond_cache.bond_amount
                * &BigUint::from(self.withdraw_penalty().get())
                / &BigUint::from(10_000u64);

            self.send().direct_esdt(
                &caller,
                &self.bond_payment_token().get(),
                0u64,
                &(&bond_cache.bond_amount - &penalty_amount),
            );

            let mut compensation = self
                .compensations(&bond_cache.token_identifier, nonce)
                .get();
            compensation.total_compenstation_amount += &penalty_amount;

            self.compensations(&bond_cache.token_identifier, nonce)
                .set(compensation);
        } else {
            self.send().direct_esdt(
                &caller,
                &self.bond_payment_token().get(),
                0u64,
                &bond_cache.bond_amount,
            );
        }

        bond_cache.clear();
        self.object_to_id().remove_by_id(bond_id);
        self.address_bonds(&caller).swap_remove(&bond_id);
        self.bonds().swap_remove(&bond_id);
    }

    #[endpoint(renew)]
    fn renew(
        &self,
        token_identifier: TokenIdentifier,
        nonce: u64,
        new_lock_period: OptionalValue<u16>,
    ) {
        require_contract_active!(self, ERR_CONTRACT_INACTIVE);
        let caller = self.blockchain().get_caller();

        let bond_id = self
            .object_to_id()
            .get_id_or_insert((token_identifier, nonce));

        require!(self.object_to_id().contains_id(bond_id), ERR_BOND_NOT_FOUND);

        let mut bond_cache = BondCache::new(self, bond_id);

        require!(bond_cache.address == caller, ERR_BOND_NOT_FOUND);

        let current_timestamp = self.blockchain().get_block_timestamp();

        let new_lock_period = match new_lock_period.into_option() {
            Some(value) => value,           // new value
            None => bond_cache.lock_period, // old value
        };

        if bond_cache.unbound_timestamp > current_timestamp {
            let remaining_time = bond_cache.unbound_timestamp - current_timestamp;
            let remaining_lock_period = remaining_time / SECONDS_IN_DAY;
            bond_cache.unbound_timestamp =
                current_timestamp + self.trasform_days_in_seconds(new_lock_period);
            bond_cache.lock_period = new_lock_period + remaining_lock_period as u16;
        } else {
            bond_cache.unbound_timestamp =
                current_timestamp + self.trasform_days_in_seconds(new_lock_period);
            bond_cache.lock_period = new_lock_period;
            bond_cache.bond_timestamp = current_timestamp;
        }
    }
}
