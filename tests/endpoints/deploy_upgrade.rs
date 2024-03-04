use core_mx_life_bonding_sc::{admin, config::State};
use multiversx_sc_scenario::scenario_model::{ScCallStep, SetStateStep, TxExpect};

use crate::bonding_state::bonding_state::{
    ContractState, ADMIN_BONDING_CONTRACT_ADDRESS_EXPR, BONDING_CONTRACT_ADDRESS_EXPR,
    BONDING_CONTRACT_PATH, FIRST_USER_ADDRESS_EXPR, OWNER_BONDING_CONTRACT_ADDRESS_EXPR,
};

#[test]
pub fn deploy_and_pause() {
    let mut state = ContractState::new();
    let admin = state.admin.clone();
    state
        .deploy()
        .set_administrator(
            OWNER_BONDING_CONTRACT_ADDRESS_EXPR,
            admin,
            Some(TxExpect::ok()),
        )
        .pause_contract(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, Some(TxExpect::ok()));
    state.check_contract_state(State::Inactive);

    state.unpause_contract(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, Some(TxExpect::ok()));
    state.check_contract_state(State::Active);

    state.pause_contract(
        FIRST_USER_ADDRESS_EXPR,
        Some(TxExpect::user_error("str:Not privileged")),
    );

    state.pause_contract(ADMIN_BONDING_CONTRACT_ADDRESS_EXPR, Some(TxExpect::ok()));

    state.check_contract_state(State::Inactive);
}

#[test]
pub fn deploy_and_upgrade() {
    let mut state = ContractState::new();
    let bonding_contract_code = state.world.code_expression(BONDING_CONTRACT_PATH);

    state.deploy();
    state.unpause_contract(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, Some(TxExpect::ok()));

    state.check_contract_state(State::Active);

    state.world.sc_call(
        ScCallStep::new()
            .from(OWNER_BONDING_CONTRACT_ADDRESS_EXPR)
            .to(BONDING_CONTRACT_ADDRESS_EXPR)
            .function("upgradeContract")
            .argument(&bonding_contract_code)
            .argument("0x0502") // codeMetadata
            .expect(TxExpect::ok()),
    );

    state.check_contract_state(State::Inactive);
}
