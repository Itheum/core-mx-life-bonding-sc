use crate::{
    config::State,
    errors::{ERR_INVALID_PENALTY_VALUE, ERR_NOT_PRIVILEGED},
    only_privileged, storage,
};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait AdminModule: crate::config::ConfigModule + storage::StorageModule {
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

    #[endpoint(setAcceptedToken)]
    fn set_accepted_token(&self, token_identifier: TokenIdentifier) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        self.accepted_token().set(token_identifier);
    }

    // days and amount needed to be bounded
    #[endpoint(setLockPeriodsAndBonds)]
    fn set_lock_periods_and_bonds(&self, args: MultiValueEncoded<MultiValue2<u16, BigUint>>) {
        only_privileged!(self, ERR_NOT_PRIVILEGED);
        for input in args.into_iter() {
            let (lock_period, bond) = input.into_tuple();
            self.lock_periods().insert(lock_period); // same index
            self.bonds().insert(bond); // same index
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
