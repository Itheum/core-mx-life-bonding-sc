use crate::{config::State, errors::ERR_NOT_PRIVILEGED, only_privileged};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait AdminModule: crate::config::ConfigModule {
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
}
