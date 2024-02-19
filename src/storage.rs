multiversx_sc::imports!();
multiversx_sc::derive_imports!();

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

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getAcceptedToken)]
    #[storage_mapper("accepted_token")]
    fn accepted_token(&self) -> SingleValueMapper<TokenIdentifier>; // accepted SFT // [TO DO] read token identifier from minting contract

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

    #[view(getAddressBonds)]
    #[storage_mapper("address_bonds")]
    fn address_bonds(&self, address: &ManagedAddress) -> UnorderedSetMapper<Bond<Self::Api>>;

    #[view(getBonds)]
    #[storage_mapper("bonds")]
    fn bonds(&self) -> UnorderedSetMapper<Bond<Self::Api>>;
}
