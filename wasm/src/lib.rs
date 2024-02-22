// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           32
// Async Callback (empty):               1
// Total number of exported functions:  34

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    core_mx_life_bonding_sc
    (
        init => init
        upgrade => upgrade
        bond => bond
        withdraw => withdraw
        renew => renew
        getAcceptedCallers => accepted_callers
        getBondPaymentToken => bond_payment_token
        getLockPeriods => lock_periods
        getLockPeriodBondAmount => lock_period_bond_amount
        getMinimumPenalty => minimum_penalty
        getMaximumPenalty => maximum_penalty
        getWithdrawPenalty => withdraw_penalty
        getCompensations => compensations
        getBond => get_bond
        getCompensation => get_compensation
        getBondsByTokenIdentifierNonce => get_bonds_by_token_identifier_nonce
        getBonds => get_bonds
        getAddressBonds => get_address_bonds
        getAllBonds => get_all_bonds
        getLockPeriodsBonds => get_lock_periods_bonds
        sanction => sanction
        modifyBond => modify_bond
        setContractStateActive => set_contract_state_active
        setContractStateInactive => set_contract_state_inactive
        setAcceptedCallers => set_accepted_callers
        setBondToken => set_bond_token
        setPeriodsBonds => set_lock_periods_and_bonds
        setMinimumPenalty => set_minimum_penalty
        setMaximumPenalty => set_maximum_penalty
        setWithdrawPenalty => set_withdraw_penalty
        setAdministrator => set_administrator
        getContractState => contract_state
        getAdministrator => administrator
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
