// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

pub struct LifeBondingContractProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for LifeBondingContractProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = LifeBondingContractProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        LifeBondingContractProxyMethods { wrapped_tx: tx }
    }
}

pub struct LifeBondingContractProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> LifeBondingContractProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init(
        self,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> LifeBondingContractProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn upgrade(
        self,
    ) -> TxTypedUpgrade<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_upgrade()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> LifeBondingContractProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn bond<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<u64>,
        Arg3: ProxyArg<u64>,
    >(
        self,
        original_caller: Arg0,
        token_identifier: Arg1,
        nonce: Arg2,
        lock_period_seconds: Arg3,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("bond")
            .argument(&original_caller)
            .argument(&token_identifier)
            .argument(&nonce)
            .argument(&lock_period_seconds)
            .original_result()
    }

    pub fn withdraw<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<u64>,
    >(
        self,
        token_identifier: Arg0,
        nonce: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("withdraw")
            .argument(&token_identifier)
            .argument(&nonce)
            .original_result()
    }

    pub fn renew<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<u64>,
    >(
        self,
        token_identifier: Arg0,
        nonce: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("renew")
            .argument(&token_identifier)
            .argument(&nonce)
            .original_result()
    }

    pub fn add_proof(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("proof")
            .original_result()
    }

    pub fn claim_refund<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<u64>,
    >(
        self,
        token_identifier: Arg0,
        nonce: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("claimRefund")
            .argument(&token_identifier)
            .argument(&nonce)
            .original_result()
    }

    pub fn set_vault_nonce<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<u64>,
    >(
        self,
        token_identifier: Arg0,
        nonce: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setVaultNonce")
            .argument(&token_identifier)
            .argument(&nonce)
            .original_result()
    }

    pub fn top_up_vault<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<u64>,
    >(
        self,
        token_identifier: Arg0,
        nonce: Arg1,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("topUpVault")
            .argument(&token_identifier)
            .argument(&nonce)
            .original_result()
    }

    pub fn stake_rewards<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        original_caller: Arg0,
        token_identifier: Arg1,
        amount: Arg2,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("stakeRewards")
            .argument(&original_caller)
            .argument(&token_identifier)
            .argument(&amount)
            .original_result()
    }

    pub fn compensation_blacklist<
        Arg0: ProxyArg<u64>,
    >(
        self,
        compensation_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getCompensationBlacklist")
            .argument(&compensation_id)
            .original_result()
    }

    pub fn total_bond_amount(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getTotalBondAmount")
            .original_result()
    }

    pub fn address_vault_nonce<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
    >(
        self,
        address: Arg0,
        token_identifier: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAddressVaultNone")
            .argument(&address)
            .argument(&token_identifier)
            .original_result()
    }

    pub fn liveliness_stake_address(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getLivelinessStakeAddress")
            .original_result()
    }

    pub fn get_bond<
        Arg0: ProxyArg<u64>,
    >(
        self,
        bond_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, Bond<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getBond")
            .argument(&bond_id)
            .original_result()
    }

    pub fn get_compensation<
        Arg0: ProxyArg<u64>,
    >(
        self,
        compensation_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, Compensation<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getCompensation")
            .argument(&compensation_id)
            .original_result()
    }

    pub fn get_compensations<
        Arg0: ProxyArg<MultiValueEncoded<Env::Api, MultiValue2<TokenIdentifier<Env::Api>, u64>>>,
    >(
        self,
        input: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedVec<Env::Api, Compensation<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getCompensations")
            .argument(&input)
            .original_result()
    }

    pub fn get_paged_compensations<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<u64>,
    >(
        self,
        start_index: Arg0,
        end_index: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedVec<Env::Api, Compensation<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getPagedCompensations")
            .argument(&start_index)
            .argument(&end_index)
            .original_result()
    }

    pub fn get_address_refund_for_compensation<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<u64>,
    >(
        self,
        address: Arg0,
        token_identifier: Arg1,
        nonce: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, Option<Refund<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAddressRefundForCompensation")
            .argument(&address)
            .argument(&token_identifier)
            .argument(&nonce)
            .original_result()
    }

    pub fn get_address_refund_for_compensations<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<MultiValueEncoded<Env::Api, u64>>,
    >(
        self,
        address: Arg0,
        compensation_ids: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedVec<Env::Api, Refund<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAddressRefundForCompensations")
            .argument(&address)
            .argument(&compensation_ids)
            .original_result()
    }

    pub fn get_bonds_by_token_identifier_nonce<
        Arg0: ProxyArg<MultiValueEncoded<Env::Api, MultiValue2<TokenIdentifier<Env::Api>, u64>>>,
    >(
        self,
        input: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedVec<Env::Api, Bond<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getBondsByTokenIdentifierNonce")
            .argument(&input)
            .original_result()
    }

    pub fn get_bonds<
        Arg0: ProxyArg<MultiValueEncoded<Env::Api, u64>>,
    >(
        self,
        bond_ids: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedVec<Env::Api, Bond<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getBonds")
            .argument(&bond_ids)
            .original_result()
    }

    pub fn get_address_bonds<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedVec<Env::Api, Bond<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAddressBonds")
            .argument(&address)
            .original_result()
    }

    pub fn get_address_bonds_avg_score<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAddressBondsAvgScore")
            .argument(&address)
            .original_result()
    }

    pub fn get_address_bonds_total_value<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAddressBondsTotalValue")
            .argument(&address)
            .original_result()
    }

    pub fn get_all_bonds(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedVec<Env::Api, Bond<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAllBonds")
            .original_result()
    }

    pub fn get_paged_bonds<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<u64>,
    >(
        self,
        start_index: Arg0,
        end_index: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedVec<Env::Api, Bond<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getPagedBonds")
            .argument(&start_index)
            .argument(&end_index)
            .original_result()
    }

    pub fn get_bonds_len(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, usize> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getBondsLen")
            .original_result()
    }

    pub fn get_compensations_len(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, usize> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getCompensationsLen")
            .original_result()
    }

    pub fn get_lock_periods_bonds(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, (ManagedVec<Env::Api, u64>, ManagedVec<Env::Api, BigUint<Env::Api>>)> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getLockPeriodsBonds")
            .original_result()
    }

    pub fn get_contract_configuration(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ContractConfiguration<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getContractConfiguration")
            .original_result()
    }

    pub fn initiate_bond_for_address<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<u64>,
    >(
        self,
        address: Arg0,
        token_identifier: Arg1,
        nonce: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("initiateBond")
            .argument(&address)
            .argument(&token_identifier)
            .argument(&nonce)
            .original_result()
    }

    pub fn add_to_black_list<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>>,
    >(
        self,
        compensation_id: Arg0,
        addresses: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setBlacklist")
            .argument(&compensation_id)
            .argument(&addresses)
            .original_result()
    }

    pub fn remove_from_black_list<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>>,
    >(
        self,
        compensation_id: Arg0,
        addresses: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("removeBlacklist")
            .argument(&compensation_id)
            .argument(&addresses)
            .original_result()
    }

    pub fn initiate_refund<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<u64>,
        Arg2: ProxyArg<u64>,
    >(
        self,
        token_identifier: Arg0,
        nonce: Arg1,
        timestamp: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("initiateRefund")
            .argument(&token_identifier)
            .argument(&nonce)
            .argument(&timestamp)
            .original_result()
    }

    pub fn sanction<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<u64>,
        Arg2: ProxyArg<PenaltyType>,
        Arg3: ProxyArg<OptionalValue<u64>>,
    >(
        self,
        token_identifier: Arg0,
        nonce: Arg1,
        penalty: Arg2,
        custom_penalty: Arg3,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("sanction")
            .argument(&token_identifier)
            .argument(&nonce)
            .argument(&penalty)
            .argument(&custom_penalty)
            .original_result()
    }

    pub fn modify_bond<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<u64>,
    >(
        self,
        token_identifier: Arg0,
        nonce: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("modifyBond")
            .argument(&token_identifier)
            .argument(&nonce)
            .original_result()
    }

    pub fn set_contract_state_active(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setContractStateActive")
            .original_result()
    }

    pub fn set_contract_state_inactive(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setContractStateInactive")
            .original_result()
    }

    pub fn set_accepted_callers<
        Arg0: ProxyArg<MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>>,
    >(
        self,
        callers: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setAcceptedCallers")
            .argument(&callers)
            .original_result()
    }

    pub fn remove_accepted_callers<
        Arg0: ProxyArg<MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>>,
    >(
        self,
        callers: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("removeAcceptedCallers")
            .argument(&callers)
            .original_result()
    }

    pub fn set_bond_token<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
    >(
        self,
        token_identifier: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setBondToken")
            .argument(&token_identifier)
            .original_result()
    }

    pub fn add_lock_periods_with_bonds<
        Arg0: ProxyArg<MultiValueEncoded<Env::Api, MultiValue2<u64, BigUint<Env::Api>>>>,
    >(
        self,
        args: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("addPeriodsBonds")
            .argument(&args)
            .original_result()
    }

    pub fn remove_lock_periods_with_bonds<
        Arg0: ProxyArg<MultiValueEncoded<Env::Api, u64>>,
    >(
        self,
        lock_periods: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("removePeriodsBonds")
            .argument(&lock_periods)
            .original_result()
    }

    pub fn set_minimum_penalty<
        Arg0: ProxyArg<u64>,
    >(
        self,
        penalty: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setMinimumPenalty")
            .argument(&penalty)
            .original_result()
    }

    pub fn set_maximum_penalty<
        Arg0: ProxyArg<u64>,
    >(
        self,
        penalty: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setMaximumPenalty")
            .argument(&penalty)
            .original_result()
    }

    pub fn set_withdraw_penalty<
        Arg0: ProxyArg<u64>,
    >(
        self,
        penalty: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setWithdrawPenalty")
            .argument(&penalty)
            .original_result()
    }

    pub fn set_liveliness_stake_address<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setLivelinessStakeAddress")
            .argument(&address)
            .original_result()
    }

    pub fn set_administrator<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        administrator: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setAdministrator")
            .argument(&administrator)
            .original_result()
    }

    pub fn contract_state(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, State> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getContractState")
            .original_result()
    }

    pub fn administrator(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAdministrator")
            .original_result()
    }

    pub fn accepted_callers(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAcceptedCallers")
            .original_result()
    }

    pub fn bond_payment_token(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getBondPaymentToken")
            .original_result()
    }

    pub fn lock_periods(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, u64>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getLockPeriods")
            .original_result()
    }

    pub fn lock_period_bond_amount<
        Arg0: ProxyArg<u64>,
    >(
        self,
        lock_period: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getLockPeriodBondAmount")
            .argument(&lock_period)
            .original_result()
    }

    pub fn minimum_penalty(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getMinimumPenalty")
            .original_result()
    }

    pub fn maximum_penalty(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getMaximumPenalty")
            .original_result()
    }

    pub fn withdraw_penalty(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getWithdrawPenalty")
            .original_result()
    }
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedDecode, NestedEncode, ManagedVecItem)]
pub struct Bond<Api>
where
    Api: ManagedTypeApi,
{
    pub bond_id: u64,
    pub address: ManagedAddress<Api>,
    pub token_identifier: TokenIdentifier<Api>,
    pub nonce: u64,
    pub lock_period: u64,
    pub bond_timestamp: u64,
    pub unbond_timestamp: u64,
    pub bond_amount: BigUint<Api>,
    pub remaining_amount: BigUint<Api>,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedDecode, NestedEncode, ManagedVecItem)]
pub struct Compensation<Api>
where
    Api: ManagedTypeApi,
{
    pub compensation_id: u64,
    pub token_identifier: TokenIdentifier<Api>,
    pub nonce: u64,
    pub accumulated_amount: BigUint<Api>,
    pub proof_amount: BigUint<Api>,
    pub end_date: u64,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedDecode, NestedEncode, ManagedVecItem)]
pub struct Refund<Api>
where
    Api: ManagedTypeApi,
{
    pub address: ManagedAddress<Api>,
    pub proof_of_refund: EsdtTokenPayment<Api>,
    pub compensation_id: u64,
}

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub struct ContractConfiguration<Api>
where
    Api: ManagedTypeApi,
{
    pub contract_state: State,
    pub bond_payment_token_identifier: TokenIdentifier<Api>,
    pub lock_periods: ManagedVec<Api, u64>,
    pub bond_amounts: ManagedVec<Api, BigUint<Api>>,
    pub minimum_penalty: u64,
    pub maximum_penalty: u64,
    pub withdraw_penalty: u64,
    pub accepted_callers: ManagedVec<Api, ManagedAddress<Api>>,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedDecode, NestedEncode, ManagedVecItem)]
pub enum State {
    Inactive,
    Active,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedDecode, NestedEncode, ManagedVecItem)]
pub enum PenaltyType {
    Minimum,
    Custom,
    Maximum,
}
