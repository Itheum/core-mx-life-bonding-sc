use crate::contexts::mappers::object_to_id_mapper::ObjectToIdMapper;

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
    TopEncode,
    TopDecode,
    NestedEncode,
    NestedDecode,
    TypeAbi,
    Clone,
    ManagedVecItem,
    PartialEq,
    Eq,
    Debug,
)]
pub struct Bond<M: ManagedTypeApi> {
    pub bond_id: u64,
    pub address: ManagedAddress<M>,
    pub token_identifier: TokenIdentifier<M>,
    pub nonce: u64,
    pub lock_period: u64, //seconds
    pub bond_timestamp: u64,
    pub unbound_timestamp: u64,
    pub bond_amount: BigUint<M>,
}

#[derive(
    TopEncode,
    TopDecode,
    NestedEncode,
    NestedDecode,
    TypeAbi,
    Clone,
    PartialEq,
    Eq,
    Debug,
    ManagedVecItem,
)]
pub struct Compensation<M: ManagedTypeApi> {
    pub compensation_id: u64,
    pub token_identifier: TokenIdentifier<M>,
    pub nonce: u64,
    pub accumulated_amount: BigUint<M>,
    pub proof_amount: BigUint<M>,
    pub end_date: u64,
}

#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone, PartialEq, Eq, Debug,
)]
pub struct Refund<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub proof_of_refund: EsdtTokenPayment<M>,
    pub compensation_id: u64,
}

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getAcceptedCallers)]
    #[storage_mapper("accepted_callers")]
    fn accepted_callers(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getBondPaymentToken)]
    #[storage_mapper("bond_payment_token")]
    fn bond_payment_token(&self) -> SingleValueMapper<TokenIdentifier>; // bonding token

    #[view(getLockPeriods)]
    #[storage_mapper("lock_periods")]
    fn lock_periods(&self) -> SetMapper<u64>; // list of lock periods in days // max_value = 65535 ~ 179 years

    #[view(getLockPeriodBondAmount)]
    #[storage_mapper("lock_period_bond_amount")]
    fn lock_period_bond_amount(&self, lock_period: u64) -> SingleValueMapper<BigUint>; // bonds based on lock_period if 0 then period not accepted

    #[view(getMinimumPenalty)]
    #[storage_mapper("minimum_penalty")]
    fn minimum_penalty(&self) -> SingleValueMapper<u64>; // percentage

    #[view(getMaximumPenalty)]
    #[storage_mapper("maximum_penalty")]
    fn maximum_penalty(&self) -> SingleValueMapper<u64>; // percentage 100% = 10_000

    #[view(getWithdrawPenalty)]
    #[storage_mapper("withdraw_penalty")]
    fn withdraw_penalty(&self) -> SingleValueMapper<u64>; // percentage

    #[storage_mapper("compensation_token_identifer")]
    fn compensation_token_identifer(
        &self,
        compensation_id: u64,
    ) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("compensation_nonce")]
    fn compensation_nonce(&self, compensation_id: u64) -> SingleValueMapper<u64>;

    #[storage_mapper("compensation_accumulated_amount")]
    fn compensation_accumulated_amount(&self, compensation_id: u64) -> SingleValueMapper<BigUint>;

    #[storage_mapper("compensation_proof_amount")]
    fn compensation_proof_amount(&self, compensation_id: u64) -> SingleValueMapper<BigUint>;

    #[storage_mapper("compensation_end_date")]
    fn compensation_end_date(&self, compensation_id: u64) -> SingleValueMapper<u64>;

    #[storage_mapper("compensations_ids")]
    fn compensations_ids(&self) -> ObjectToIdMapper<Self::Api, (TokenIdentifier, u64)>;

    #[storage_mapper("compensations_ids")]
    fn compensations(&self) -> UnorderedSetMapper<u64>;

    #[storage_mapper("bond_address")]
    fn bond_address(&self, bond_id: u64) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("bond_token_identifier")]
    fn bond_token_identifier(&self, bond_id: u64) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("bond_nonce")]
    fn bond_nonce(&self, bond_id: u64) -> SingleValueMapper<u64>;

    #[storage_mapper("bond_lock_period")]
    fn bond_lock_period(&self, bond_id: u64) -> SingleValueMapper<u64>;

    #[storage_mapper("bond_timestamp")]
    fn bond_timestamp(&self, bond_id: u64) -> SingleValueMapper<u64>;

    #[storage_mapper("unbound_timestamp")]
    fn unbound_timestamp(&self, bond_id: u64) -> SingleValueMapper<u64>;

    #[storage_mapper("bond_amount")]
    fn bond_amount(&self, bond_id: u64) -> SingleValueMapper<BigUint>;

    #[storage_mapper("token_identifier_nonce_to_id")]
    fn bonds_ids(&self) -> ObjectToIdMapper<Self::Api, (TokenIdentifier, u64)>;

    #[storage_mapper("address_bonds")]
    fn address_bonds(&self, address: &ManagedAddress) -> UnorderedSetMapper<u64>;

    #[storage_mapper("bonds")]
    fn bonds(&self) -> UnorderedSetMapper<u64>;

    #[storage_mapper("refund_whitelist")]
    fn refund_whitelist(&self, compensation_id: u64) -> WhitelistMapper<ManagedAddress>;

    #[storage_mapper("address_refund")]
    fn address_refund(
        &self,
        address: &ManagedAddress,
        compensation_id: u64,
    ) -> SingleValueMapper<Refund<Self::Api>>;
}
