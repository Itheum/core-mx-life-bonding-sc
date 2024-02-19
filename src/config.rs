multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone, PartialEq, Eq, Debug, Copy,
)]
pub enum State {
    Inactive,
    Active,
}

#[multiversx_sc::module]
pub trait ConfigModule {
    #[inline]
    fn is_contract_owner(&self, address: &ManagedAddress) -> bool {
        &(self.blockchain().get_owner_address()) == address
    }

    #[inline]
    fn is_admin(&self, address: &ManagedAddress) -> bool {
        &(self.administrator().get()) == address
    }

    fn is_privileged(&self, address: &ManagedAddress) -> bool {
        self.is_contract_owner(address) || self.is_admin(address)
    }

    #[inline]
    fn is_state_active(&self, state: State) -> bool {
        state == State::Active
    }

    #[view(getContractState)]
    #[storage_mapper("contract_state")]
    fn contract_state(&self) -> SingleValueMapper<State>;

    #[view(getAdministrator)]
    #[storage_mapper("administrator")]
    fn administrator(&self) -> SingleValueMapper<ManagedAddress>;
}
