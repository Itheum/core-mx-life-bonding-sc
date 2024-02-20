use crate::{
    config::State,
    errors::{
        ERR_BOND_NOT_FOUND, ERR_ENDPOINT_CALLABLE_ONLY_BY_SC, ERR_INVALID_PENALTY_VALUE,
        ERR_INVALID_TOKEN_IDENTIFIER, ERR_NOT_PRIVILEGED,
    },
    only_privileged,
    storage::{self, PenaltyType},
};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait AdminModule: crate::config::ConfigModule + storage::StorageModule {
    #[endpoint(sanction)]
    fn sanction(
        &self,
        address: ManagedAddress,
        token_identifier: TokenIdentifier,
        nonce: u64,
        penalty: PenaltyType,
        custom_penalty: OptionalValue<u64>,
    ) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);

        require!(
            !self
                .address_bonds(&address, &token_identifier, nonce)
                .is_empty(),
            ERR_BOND_NOT_FOUND
        );

        let mut bond = self.address_bonds(&address, &token_identifier, nonce).get();

        let penalty = match penalty {
            PenaltyType::Minimum => self.minimum_penalty().get(),
            PenaltyType::Custom => {
                if let Some(custom_value) = custom_penalty.into_option() {
                    require!(
                        custom_value <= 10_000 && custom_value > 0,
                        ERR_INVALID_PENALTY_VALUE
                    );
                    custom_value
                } else {
                    sc_panic!(ERR_INVALID_PENALTY_VALUE);
                }
            }
            PenaltyType::Maximum => self.maximum_penalty().get(),
        };

        let penalty_amount =
            &bond.bond_amount * &BigUint::from(penalty) / &BigUint::from(10_000u64);

        bond.bond_amount = &bond.bond_amount - &penalty_amount;

        self.address_bonds(&address, &token_identifier, nonce)
            .set(bond);

        let mut compensation = self.compensations(&token_identifier, nonce).get();
        compensation.total_compenstation_amount += &penalty_amount; // Update total compensation amount

        self.compensations(&token_identifier, nonce)
            .set(compensation);
    }

    #[endpoint(modifyBond)]
    fn modify_bond(&self, address: ManagedAddress, token_identifier: TokenIdentifier, nonce: u64) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        require!(
            !self
                .address_bonds(&address, &token_identifier, nonce)
                .is_empty(),
            ERR_BOND_NOT_FOUND
        );
        let mut bond = self.address_bonds(&address, &token_identifier, nonce).get();
        let current_timestamp = self.blockchain().get_block_timestamp();
        bond.unbound_timestamp = current_timestamp;
        self.address_bonds(&address, &token_identifier, nonce)
            .set(bond);
    }

    #[endpoint(setContractStateActive)]
    fn set_contract_state_active(&self) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        self.contract_state().set(State::Active);
    }

    #[endpoint(setContractStateInactive)]
    fn set_contract_state_inactive(&self) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        self.contract_state().set(State::Inactive);
    }

    #[endpoint(setAcceptedCallers)]
    fn set_accepted_callers(&self, callers: MultiValueEncoded<ManagedAddress>) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        for caller in callers.into_iter() {
            require!(
                self.blockchain().is_smart_contract(&caller),
                ERR_ENDPOINT_CALLABLE_ONLY_BY_SC
            );
            self.accepted_callers().insert(caller);
        }
    }

    #[endpoint(setBondToken)]
    fn set_bond_token(&self, token_identifier: TokenIdentifier) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        require!(
            token_identifier.is_valid_esdt_identifier(),
            ERR_INVALID_TOKEN_IDENTIFIER
        );
        self.bond_token().set(token_identifier);
    }

    #[endpoint(setAcceptedToken)]
    fn set_accepted_token(&self, token_identifier: TokenIdentifier) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        self.accepted_token().set(token_identifier);
    }

    // lock period (in days) and bond amount
    #[endpoint(setLockPeriodsAndBonds)]
    fn set_lock_periods_and_bonds(&self, args: MultiValueEncoded<MultiValue2<u16, BigUint>>) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        for input in args.into_iter() {
            let (lock_period, bond) = input.into_tuple();
            self.lock_periods().insert(lock_period.clone());
            self.lock_period_bond_amount(lock_period).set(bond);
        }
    }

    #[endpoint(setMinimumPenalty)]
    fn set_minimum_penalty(&self, penalty: u64) {
        require!(penalty <= 5_000 && penalty > 0, ERR_INVALID_PENALTY_VALUE);
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        self.minimum_penalty().set(penalty);
    }

    #[endpoint(setMaximumPenalty)]
    fn set_maximum_penalty(&self, penalty: u64) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        require!(penalty <= 10_000 && penalty > 0, ERR_INVALID_PENALTY_VALUE);
        self.maximum_penalty().set(penalty);
    }

    #[endpoint(setWithdrawPenalty)]
    fn set_withdraw_penalty(&self, penalty: u64) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        require!(penalty <= 10_000 && penalty > 0, ERR_INVALID_PENALTY_VALUE);
        self.withdraw_penalty().set(penalty);
    }
}
