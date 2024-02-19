multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getAcceptedToken)]
    #[storage_mapper("accepted_token")]
    fn accepted_token(&self) -> SingleValueMapper<TokenIdentifier>; // bonding token

    #[view(getMinimumLockPeriod)]
    #[storage_mapper("minimum_lock_period")]
    fn minimum_lock_period(&self) -> SingleValueMapper<u64>;

    #[view(getMinimumBond)]
    #[storage_mapper("minimum_bond")]
    fn minimum_bond(&self) -> SingleValueMapper<BigUint>;

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
