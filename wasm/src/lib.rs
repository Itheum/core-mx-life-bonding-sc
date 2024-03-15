// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           47
// Async Callback (empty):               1
// Total number of exported functions:  49

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
        proof => add_proof
        claimRefund => claim_refund
        getCompensationBlacklist => compensation_blacklist
        getBond => get_bond
        getCompensation => get_compensation
        getCompensations => get_compensations
        getPagedCompensations => get_paged_compensations
        getAddressRefundForCompensation => get_address_refund_for_compensation
        getBondsByTokenIdentifierNonce => get_bonds_by_token_identifier_nonce
        getBonds => get_bonds
        getAddressBonds => get_address_bonds
        getAllBonds => get_all_bonds
        getPagedBonds => get_paged_bonds
        getBondsLen => get_bonds_len
        getCompensationsLen => get_compensations_len
        getLockPeriodsBonds => get_lock_periods_bonds
        getContractConfiguration => get_contract_configuration
        initiateBond => initiate_bond_for_address
        setBlacklist => add_to_black_list
        removeBlacklist => remove_from_black_list
        initiateRefund => initiate_refund
        sanction => sanction
        modifyBond => modify_bond
        setContractStateActive => set_contract_state_active
        setContractStateInactive => set_contract_state_inactive
        setAcceptedCallers => set_accepted_callers
        removeAcceptedCallers => remove_accepted_callers
        setBondToken => set_bond_token
        addPeriodsBonds => add_lock_periods_with_bonds
        removePeriodsBonds => remove_lock_periods_with_bonds
        setMinimumPenalty => set_minimum_penalty
        setMaximumPenalty => set_maximum_penalty
        setWithdrawPenalty => set_withdraw_penalty
        setAdministrator => set_administrator
        getContractState => contract_state
        getAdministrator => administrator
        getAcceptedCallers => accepted_callers
        getBondPaymentToken => bond_payment_token
        getLockPeriods => lock_periods
        getLockPeriodBondAmount => lock_period_bond_amount
        getMinimumPenalty => minimum_penalty
        getMaximumPenalty => maximum_penalty
        getWithdrawPenalty => withdraw_penalty
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
