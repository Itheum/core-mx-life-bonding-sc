mod bonding_state;
mod endpoints;

use core_mx_life_bonding_sc::{
    config::{ConfigModule, State},
    storage::StorageModule,
    LifeBondingContract,
};
use multiversx_sc_scenario::{
    api::SingleTxApi, managed_address, managed_token_id, scenario_model::AddressValue,
};

#[test]
fn bonding_ready_test() {
    let bond_contract = core_mx_life_bonding_sc::contract_obj::<SingleTxApi>();

    bond_contract.init();

    assert_eq!(false, bond_contract.contract_is_ready());

    bond_contract.administrator().set(managed_address!(
        &AddressValue::from("address:admin").to_address()
    ));

    assert_eq!(false, bond_contract.contract_is_ready());

    bond_contract.contract_state().set(State::Active);

    assert_eq!(false, bond_contract.contract_is_ready());

    bond_contract
        .accepted_callers()
        .insert(managed_address!(
            &AddressValue::from("address:caller").to_address()
        ));

    assert_eq!(false, bond_contract.contract_is_ready());

    bond_contract
        .bond_payment_token()
        .set(managed_token_id!(b"TEST-1234"));

    assert_eq!(false, bond_contract.contract_is_ready());

    bond_contract.lock_periods().insert(1);

    assert_eq!(true, bond_contract.contract_is_ready());

    bond_contract.lock_periods().remove(&1);

    assert_eq!(false, bond_contract.contract_is_ready());

    bond_contract.lock_periods().insert(1);

    assert_eq!(true, bond_contract.contract_is_ready());

    bond_contract.contract_state().clear();

    assert_eq!(false, bond_contract.contract_is_ready());
}
