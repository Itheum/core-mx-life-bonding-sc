multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone, PartialEq, Eq, Debug, Copy,
)]
pub enum PenaltyType {
    Minimum,
    Custom,
    Maximum,
}

#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone, PartialEq, Eq, Debug,
)]
pub struct Bond<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub token_identifier: TokenIdentifier<M>,
    pub nonce: u64,
    pub lock_period: u16,
    pub bond_timestamp: u64,
    pub unbound_timestamp: u64,
    pub bond_amount: BigUint<M>,
}

#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone, PartialEq, Eq, Debug,
)]
pub struct Compensation<M: ManagedTypeApi> {
    pub token_identifier: TokenIdentifier<M>,
    pub nonce: u64,
    pub total_compenstation_amount: BigUint<M>,
}

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getAcceptedToken)]
    #[storage_mapper("accepted_token")]
    fn accepted_token(&self) -> SingleValueMapper<TokenIdentifier>; // accepted SFT // [TO DO] read token identifier from minting contract

    #[view(getAcceptedCallers)]
    #[storage_mapper("accepted_callers")]
    fn accepted_callers(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getBondToken)]
    #[storage_mapper("bond_token")]
    fn bond_token(&self) -> SingleValueMapper<TokenIdentifier>; // bonding token

    #[view(getLockPeriods)]
    #[storage_mapper("lock_periods")]
    fn lock_periods(&self) -> SetMapper<u16>; // list of lock periods in days // max_value = 65535 ~ 179 years

    #[view(getLockPeriodBondAmount)]
    #[storage_mapper("lock_period_bond_amount")]
    fn lock_period_bond_amount(&self, lock_period: u16) -> SingleValueMapper<BigUint>; // bonds based on lock_period if 0 then period not accepted

    #[view(getMinimumPenalty)]
    #[storage_mapper("minimum_penalty")]
    fn minimum_penalty(&self) -> SingleValueMapper<u64>; // percentage

    #[view(getMaximumPenalty)]
    #[storage_mapper("maximum_penalty")]
    fn maximum_penalty(&self) -> SingleValueMapper<u64>; // percentage 100% = 10_000

    #[view(getWithdrawPenalty)]
    #[storage_mapper("withdraw_penalty")]
    fn withdraw_penalty(&self) -> SingleValueMapper<u64>; // percentage

    #[view(getCompensations)]
    #[storage_mapper("compensations")]
    fn compensations(
        &self,
        token_identifier: &TokenIdentifier,
        nonce: u64,
    ) -> SingleValueMapper<Compensation<Self::Api>>;

    #[storage_mapper("bond_address")]
    fn bond_address(&self, bond_id: u64) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("bond_token_identifier")]
    fn bond_token_identifier(&self, bond_id: u64) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("bond_nonce")]
    fn bond_nonce(&self, bond_id: u64) -> SingleValueMapper<u64>;

    #[storage_mapper("bond_lock_period")]
    fn bond_lock_period(&self, bond_id: u64) -> SingleValueMapper<u16>;

    #[storage_mapper("bond_bond_timestamp")]
    fn bond_timestamp(&self, bond_id: u64) -> SingleValueMapper<u64>;

    #[storage_mapper("bond_unbound_timestamp")]
    fn unbound_timestamp(&self, bond_id: u64) -> SingleValueMapper<u64>;

    #[storage_mapper("bond_bond_amount")]
    fn bond_amount(&self, bond_id: u64) -> SingleValueMapper<BigUint>;

    #[view(getAddressBonds)]
    #[storage_mapper("address_bonds")]
    fn address_bonds(&self, address: &ManagedAddress) -> UnorderedSetMapper<u64>;

    #[view(getBonds)]
    #[storage_mapper("bonds")]
    fn bonds(&self) -> UnorderedSetMapper<u64>;

    fn next_bond_id(&self) -> u64 {
        let next_bond_id = self.last_bond_id().get() + 1;
        self.last_bond_id().set(next_bond_id);
        next_bond_id
    }

    #[view(getLastBondId)]
    #[storage_mapper("lastBondId")]
    fn last_bond_id(&self) -> SingleValueMapper<u64>;
}
