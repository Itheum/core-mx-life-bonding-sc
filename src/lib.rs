#![no_std]

use config::State;

use crate::{
    config::COMPENSATION_SAFE_PERIOD,
    contexts::{bond_cache::BondCache, compensation_cache::CompensationCache},
    errors::{
        ERR_BOND_NOT_FOUND, ERR_CONTRACT_NOT_READY, ERR_ENDPOINT_CALLABLE_ONLY_BY_ACCEPTED_CALLERS,
        ERR_INVALID_AMOUNT, ERR_INVALID_LOCK_PERIOD, ERR_INVALID_TIMELINE_TO_PROOF,
        ERR_INVALID_TIMELINE_TO_REFUND, ERR_INVALID_TOKEN_IDENTIFIER,
        ERR_PENALTIES_EXCEED_WITHDRAWAL_AMOUNT, ERR_REFUND_NOT_FOUND,
    },
    storage::Refund,
};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod admin;
pub mod config;
pub mod contexts;
pub mod errors;
pub mod events;
pub mod storage;
pub mod views;
#[multiversx_sc::contract]
pub trait LifeBondingContract:
    storage::StorageModule
    + views::ViewsModule
    + admin::AdminModule
    + config::ConfigModule
    + events::EventsModule
{
    #[init]
    fn init(&self) {
        self.contract_state().set(State::Inactive);
        self.contract_state_event(State::Inactive);

        self.minimum_penalty().set(500);
        self.maximum_penalty().set(10_000);
        self.withdraw_penalty().set(8_000);

        self.minimum_penalty_event(500);
        self.maximum_penalty_event(10_000);
        self.withdraw_penalty_event(8_000);
    }

    #[upgrade]
    fn upgrade(&self) {
        self.contract_state().set(State::Inactive);
        self.contract_state_event(State::Inactive);
    }

    #[payable("*")]
    #[endpoint(bond)]
    fn bond(
        &self,
        original_caller: ManagedAddress,
        token_identifier: TokenIdentifier,
        nonce: u64,
        lock_period_seconds: u64,
    ) {
        require_contract_ready!(self, ERR_CONTRACT_NOT_READY);
        let caller = self.blockchain().get_caller();

        let bond_id = self
            .bonds_ids()
            .get_id_or_insert((token_identifier.clone(), nonce));
        require!(
            self.blockchain()
                .is_smart_contract(&self.blockchain().get_caller())
                && self
                    .accepted_callers()
                    .contains(&self.blockchain().get_caller())
                || self.is_privileged(&caller)
                || self.address_bonds(&caller).contains(&bond_id),
            ERR_ENDPOINT_CALLABLE_ONLY_BY_ACCEPTED_CALLERS
        );

        let payment = self.call_value().single_esdt();

        require!(
            payment.token_identifier == self.bond_payment_token().get(),
            ERR_INVALID_TOKEN_IDENTIFIER
        );

        require!(
            self.lock_periods().contains(&lock_period_seconds),
            ERR_INVALID_LOCK_PERIOD
        );
        require!(
            !self.lock_period_bond_amount(lock_period_seconds).is_empty(),
            ERR_INVALID_LOCK_PERIOD
        );

        let bond_amount = self.lock_period_bond_amount(lock_period_seconds).get();

        require!(payment.amount == bond_amount, ERR_INVALID_AMOUNT);

        let current_timestamp = self.blockchain().get_block_timestamp();
        let unbond_timestamp = current_timestamp + lock_period_seconds;

        self.bond_address(bond_id).set(original_caller.clone());
        self.bond_token_identifier(bond_id)
            .set(token_identifier.clone());
        self.bond_nonce(bond_id).set(nonce);
        self.bond_lock_period(bond_id).set(lock_period_seconds);
        self.bond_timestamp(bond_id).set(current_timestamp);
        self.unbond_timestamp(bond_id).set(unbond_timestamp);
        self.bond_amount(bond_id).set(payment.amount.clone());
        self.remaining_amount(bond_id).set(payment.amount);

        self.address_bonds(&original_caller).insert(bond_id);
        self.bonds().insert(bond_id);

        let compensation_id = self
            .compensations_ids()
            .insert_new((token_identifier.clone(), nonce));

        self.compensations().insert(compensation_id);

        self.compensation_token_identifer(compensation_id)
            .set(token_identifier);
        self.compensation_nonce(compensation_id).set(nonce);
        self.compensation_accumulated_amount(compensation_id)
            .set(BigUint::zero());
        self.compensation_proof_amount(compensation_id)
            .set(BigUint::zero());
        self.compensation_end_date(compensation_id).set(0u64);

        self.bond_event(&self.get_bond(bond_id));
        self.compensation_event(&self.get_compensation(compensation_id));
    }

    #[endpoint(withdraw)]
    fn withdraw(&self, token_identifier: TokenIdentifier, nonce: u64) {
        require_contract_ready!(self, ERR_CONTRACT_NOT_READY);
        let caller = self.blockchain().get_caller();

        let bond_id = self
            .bonds_ids()
            .get_id_non_zero((token_identifier.clone(), nonce));
        let compensation_id = self
            .compensations_ids()
            .get_id_non_zero((token_identifier, nonce));

        let bond_cache = BondCache::new(self, bond_id);

        require!(bond_cache.address == caller, ERR_BOND_NOT_FOUND);

        let current_timestamp = self.blockchain().get_block_timestamp();

        let mut compensation_cache = CompensationCache::new(self, compensation_id);

        let mut penalty_amount = BigUint::zero();
        if bond_cache.unbond_timestamp >= current_timestamp {
            penalty_amount = &bond_cache.bond_amount
                * &BigUint::from(self.withdraw_penalty().get())
                / &BigUint::from(10_000u64);

            require!(
                &bond_cache.remaining_amount > &penalty_amount,
                ERR_PENALTIES_EXCEED_WITHDRAWAL_AMOUNT
            );
            require!(
                &bond_cache.remaining_amount - &penalty_amount
                    >= compensation_cache.accumulated_amount,
                ERR_PENALTIES_EXCEED_WITHDRAWAL_AMOUNT
            );

            self.send().direct_esdt(
                &caller,
                &self.bond_payment_token().get(),
                0u64,
                &(&bond_cache.remaining_amount - &penalty_amount),
            );

            compensation_cache.accumulated_amount += &penalty_amount;
        } else {
            self.send().direct_esdt(
                &caller,
                &self.bond_payment_token().get(),
                0u64,
                &bond_cache.remaining_amount,
            );

            self.compensations().swap_remove(&compensation_id);
        }

        self.withdraw_event(
            &bond_id,
            &caller,
            &(&bond_cache.remaining_amount - &penalty_amount),
            &penalty_amount,
        );

        self.bonds().swap_remove(&bond_id);
    }

    #[endpoint(renew)]
    fn renew(&self, token_identifier: TokenIdentifier, nonce: u64) {
        require_contract_ready!(self, ERR_CONTRACT_NOT_READY);
        let caller = self.blockchain().get_caller();

        let bond_id = self.bonds_ids().get_id_non_zero((token_identifier, nonce));

        let mut bond_cache = BondCache::new(self, bond_id);

        require!(bond_cache.address == caller, ERR_BOND_NOT_FOUND);

        require!(
            self.lock_periods().contains(&bond_cache.lock_period),
            ERR_INVALID_LOCK_PERIOD
        );

        let current_timestamp = self.blockchain().get_block_timestamp();

        bond_cache.unbond_timestamp = current_timestamp + bond_cache.lock_period;
        bond_cache.bond_timestamp = current_timestamp;

        self.renew_event(&bond_id, &caller, &bond_cache.unbond_timestamp);
    }

    #[payable("*")]
    #[endpoint(proof)]
    fn add_proof(&self) {
        require_contract_ready!(self, ERR_CONTRACT_NOT_READY);
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().single_esdt();

        let compensation_id = self
            .compensations_ids()
            .get_id_non_zero((payment.token_identifier.clone(), payment.token_nonce));

        let mut compensation_cache = CompensationCache::new(self, compensation_id);

        let current_timestamp = self.blockchain().get_block_timestamp();

        require!(
            current_timestamp <= compensation_cache.end_date && compensation_cache.end_date != 0u64,
            ERR_INVALID_TIMELINE_TO_PROOF
        );

        compensation_cache.proof_amount += &payment.amount;

        self.proof_event(
            &compensation_id,
            &payment.token_identifier,
            &payment.token_nonce,
            &payment.amount,
        );

        let refund = Refund {
            compensation_id,
            address: caller.clone(),
            proof_of_refund: payment,
        };

        self.address_refund(&caller, compensation_id).set(refund);
    }

    #[endpoint(claimRefund)]
    fn claim_refund(&self, token_identifier: TokenIdentifier, nonce: u64) {
        require_contract_ready!(self, ERR_CONTRACT_NOT_READY);
        let caller = self.blockchain().get_caller();

        let compensation_id = self
            .compensations_ids()
            .get_id_non_zero((token_identifier, nonce));

        let mut compensation_cache = CompensationCache::new(self, compensation_id);

        let current_timestamp = self.blockchain().get_block_timestamp();

        require!(
            current_timestamp > compensation_cache.end_date + COMPENSATION_SAFE_PERIOD,
            ERR_INVALID_TIMELINE_TO_REFUND
        );

        require!(
            !self.address_refund(&caller, compensation_id).is_empty(),
            ERR_REFUND_NOT_FOUND
        );

        if self
            .compensation_blacklist(compensation_id)
            .contains(&caller)
        {
            let address_refund = self.address_refund(&caller, compensation_id).get();

            self.send()
                .direct_non_zero_esdt_payment(&caller, &address_refund.proof_of_refund);

            compensation_cache.proof_amount -= &address_refund.proof_of_refund.amount;
            self.compensation_blacklist(compensation_id)
                .swap_remove(&caller);
            self.address_refund(&caller, compensation_id).clear();
        } else {
            let mut sum_of_blacklist_refunds = BigUint::zero();

            for address in self.compensation_blacklist(compensation_id).into_iter() {
                if !self.address_refund(&address, compensation_id).is_empty() {
                    sum_of_blacklist_refunds += self
                        .address_refund(&address, compensation_id)
                        .get()
                        .proof_of_refund
                        .amount;
                }
            }

            let refund = self.address_refund(&caller, compensation_id).get();

            let compensation_per_token = &compensation_cache.accumulated_amount
                / &(&compensation_cache.proof_amount - &sum_of_blacklist_refunds);

            let refund_amount = &refund.proof_of_refund.amount * &compensation_per_token;

            compensation_cache.accumulated_amount -= &refund_amount;
            compensation_cache.proof_amount -= &refund.proof_of_refund.amount;

            let mut payments = ManagedVec::new();

            self.claim_refund_event(
                &compensation_id,
                &caller,
                &refund.proof_of_refund.token_identifier,
                &refund.proof_of_refund.token_nonce,
                &refund.proof_of_refund.amount,
                &self.bond_payment_token().get(),
                &0u64,
                &refund_amount,
            );

            payments.push(refund.proof_of_refund);
            payments.push(EsdtTokenPayment::new(
                self.bond_payment_token().get(),
                0u64,
                refund_amount,
            ));

            self.send().direct_multi(&caller, &payments);

            self.address_refund(&caller, compensation_id).clear();
        }

        if compensation_cache.accumulated_amount == BigUint::zero()
            && compensation_cache.proof_amount == BigUint::zero()
        {
            self.compensations().swap_remove(&compensation_id);
        }
    }
}
