use crate::storage;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone, PartialEq, Eq, Debug, Copy,
)]
pub enum State {
    Inactive,
    Active,
}

pub const COMPENSATION_SAFE_PERIOD: u64 = 86_400;

#[multiversx_sc::module]
pub trait ConfigModule: storage::StorageModule {
    #[only_owner]
    #[endpoint(setAdministrator)]
    fn set_administrator(&self, administrator: ManagedAddress) {
        self.administrator().set(administrator);
    }

    #[view(getContractState)]
    #[storage_mapper("contract_state")]
    fn contract_state(&self) -> SingleValueMapper<State>;

    #[view(getAdministrator)]
    #[storage_mapper("administrator")]
    fn administrator(&self) -> SingleValueMapper<ManagedAddress>;

    #[inline]
    fn is_contract_owner(&self, address: &ManagedAddress) -> bool {
        &(self.blockchain().get_owner_address()) == address
    }

    #[inline]
    fn is_admin(&self, address: &ManagedAddress) -> bool {
        &(self.administrator().get()) == address
    }

    #[inline]
    fn is_privileged(&self, address: &ManagedAddress) -> bool {
        self.is_contract_owner(address) || self.is_admin(address)
    }

    fn contract_is_ready(&self) -> bool {
        let mut is_ready = true;

        if !self.is_state_active(self.contract_state().get()) {
            is_ready = false;
        }

        if self.administrator().is_empty() {
            is_ready = false;
        }
        if self.accepted_callers().len() == 0 {
            is_ready = false;
        }
        if self.bond_payment_token().is_empty() {
            is_ready = false;
        }
        if self.lock_periods().len() == 0 {
            is_ready = false;
        }
        is_ready
    }

    #[inline]
    fn is_state_active(&self, state: State) -> bool {
        state == State::Active
    }
}
