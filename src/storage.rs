multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getAcceptedToken)]
    #[storage_mapper("accepted_token")]
    fn accepted_token(&self) -> SingleValueMapper<TokenIdentifier>; // bonding token

    #[view(getBondToken)]
    #[storage_mapper("bond_token")]
    fn bond_token(&self) -> SingleValueMapper<TokenIdentifier>; // bonding token

    #[view(getLockPeriods)]
    #[storage_mapper("lock_periods")]
    fn lock_periods(&self) -> SetMapper<u16>; // list of lock periods in days // max_value = 65535 ~ 179 years

    #[view(getBonds)]
    #[storage_mapper("minimum_bond")]
    fn bonds(&self) -> SetMapper<BigUint>; // list of bonds amount based on lock period

    #[view(getMinimumPenalty)]
    #[storage_mapper("minimum_penalty")]
    fn minimum_penalty(&self) -> SingleValueMapper<u64>; // percentage

    #[view(getMaximumPenalty)]
    #[storage_mapper("maximum_penalty")]
    fn maximum_penalty(&self) -> SingleValueMapper<u64>; // percentage 100% = 10_000

    #[view(getWithdrawPenalty)]
    #[storage_mapper("withdraw_penalty")]
    fn withdraw_penalty(&self) -> SingleValueMapper<u64>; // percentage
}
