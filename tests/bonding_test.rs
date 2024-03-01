use core_mx_life_bonding_sc::{
    admin::ProxyTrait as _,
    config::{ProxyTrait as _, State},
    storage::PenaltyType,
    views::ProxyTrait as _,
};

use core_mx_life_bonding_sc::ProxyTrait as _;
use multiversx_sc::{
    codec::multi_types::{MultiValue2, OptionalValue},
    storage::mappers::SingleValue,
    types::{Address, MultiValueEncoded},
};
use multiversx_sc_scenario::{
    api::StaticApi,
    managed_address, managed_biguint, managed_token_id,
    num_bigint::BigUint,
    scenario_model::{
        Account, AddressValue, ScCallStep, ScDeployStep, ScQueryStep, SetStateStep, TxExpect,
    },
    ContractInfo, ScenarioWorld,
};

const BONDING_CONTRACT_PATH: &str = "mxsc:output/core-mx-life-bonding-sc.msxc.json";
const BONDING_CONTRACT_ADDRESS_EXPR: &str = "sc:core-mx-life-bonding-sc";

const OWNER_BONDING_CONTRACT_ADDRESS_EXPR: &str = "address:owner";

const ITHEUM_TOKEN_IDENTIFIER_EXPR: &str = "str:ITHEUM-12345";
const ITHEUM_TOKEN_IDENTIFIER: &[u8] = b"ITHEUM-12345";

const DATA_NFT_IDENTIFIER_EXPR: &str = "str:DATANFT-12345";
const DATA_NFT_IDENTIFIER: &[u8] = b"DATANFT-12345";

const ADMIN_BONDING_CONTRACT_ADDRESS_EXPR: &str = "address:admin";

const FIRST_USER_ADDRESS_EXPR: &str = "address:first_user";
const SECOND_USER_ADDRESS_EXPR: &str = "address:second_user";
const THIRD_USER_ADDRESS_EXPR: &str = "address:third_user";

type Contract = ContractInfo<core_mx_life_bonding_sc::Proxy<StaticApi>>;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("");

    blockchain.register_contract(
        BONDING_CONTRACT_PATH,
        core_mx_life_bonding_sc::ContractBuilder,
    );
    blockchain
}

struct ContractState {
    world: ScenarioWorld,
    contract: Contract,
    contract_owner: Address,
    admin: Address,
    first_user_address: Address,
    second_user_address: Address,
    third_user_address: Address,
}

impl ContractState {
    fn new() -> Self {
        let mut world = world();

        world.set_state_step(
            SetStateStep::new()
                .put_account(
                    OWNER_BONDING_CONTRACT_ADDRESS_EXPR,
                    Account::new()
                        .nonce(1)
                        .balance("1_000")
                        .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "10_000"),
                )
                .new_address(
                    OWNER_BONDING_CONTRACT_ADDRESS_EXPR,
                    1,
                    BONDING_CONTRACT_ADDRESS_EXPR,
                )
                .put_account(
                    ADMIN_BONDING_CONTRACT_ADDRESS_EXPR,
                    Account::new()
                        .nonce(1)
                        .balance("1_000")
                        .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "10_000"),
                )
                .put_account(
                    FIRST_USER_ADDRESS_EXPR,
                    Account::new()
                        .nonce(1)
                        .balance("1_000")
                        .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "1_000"),
                )
                .put_account(
                    SECOND_USER_ADDRESS_EXPR,
                    Account::new()
                        .nonce(1)
                        .balance("1_000")
                        .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "1_000"),
                )
                .put_account(
                    THIRD_USER_ADDRESS_EXPR,
                    Account::new()
                        .nonce(1)
                        .balance("1_000")
                        .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "1_000"),
                ),
        );

        let contract = Contract::new(BONDING_CONTRACT_ADDRESS_EXPR);

        let contract_owner = AddressValue::from(OWNER_BONDING_CONTRACT_ADDRESS_EXPR).to_address();
        let admin = AddressValue::from(ADMIN_BONDING_CONTRACT_ADDRESS_EXPR).to_address();
        let first_user_address = AddressValue::from(FIRST_USER_ADDRESS_EXPR).to_address();
        let second_user_address = AddressValue::from(SECOND_USER_ADDRESS_EXPR).to_address();
        let third_user_address = AddressValue::from(THIRD_USER_ADDRESS_EXPR).to_address();

        Self {
            world,
            contract,
            contract_owner,
            admin,
            first_user_address,
            second_user_address,
            third_user_address,
        }
    }

    fn deploy(&mut self) -> &mut Self {
        let bonding_contract_code = self.world.code_expression(BONDING_CONTRACT_PATH);

        self.world.sc_deploy(
            ScDeployStep::new()
                .from(OWNER_BONDING_CONTRACT_ADDRESS_EXPR)
                .code(bonding_contract_code)
                .call(self.contract.init()),
        );
        self
    }

    fn set_administrator(
        &mut self,
        caller: &str,
        administrator: Address,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(
                    self.contract
                        .set_administrator(managed_address!(&administrator)),
                )
                .expect(tx_expect),
        );
        self
    }

    fn pause_contract(&mut self, caller: &str, expect: Option<TxExpect>) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.set_contract_state_inactive())
                .expect(tx_expect),
        );
        self
    }

    fn unpause_contract(&mut self, caller: &str, expect: Option<TxExpect>) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.set_contract_state_active())
                .expect(tx_expect),
        );
        self
    }

    fn check_contract_state(&mut self, contract_state: State) -> &mut Self {
        self.world.sc_query(
            ScQueryStep::new()
                .call(self.contract.contract_state())
                .expect_value(SingleValue::from(contract_state)),
        );
        self
    }

    fn set_accepted_caller(
        &mut self,
        caller: &str,
        address: Address,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        let mut arg = MultiValueEncoded::new();

        arg.push(managed_address!(&address));

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.set_accepted_callers(arg))
                .expect(tx_expect),
        );
        self
    }

    fn set_blacklist(
        &mut self,
        caller: &str,
        compensation_id: u64,
        address: Address,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        let mut arg = MultiValueEncoded::new();
        arg.push(managed_address!(&address));

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.add_to_black_list(compensation_id, arg))
                .expect(tx_expect),
        );
        self
    }

    fn remove_blacklist(
        &mut self,
        caller: &str,
        compensation_id: u64,
        address: Address,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        let mut arg = MultiValueEncoded::new();
        arg.push(managed_address!(&address));

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.remove_from_black_list(compensation_id, arg))
                .expect(tx_expect),
        );
        self
    }

    fn remove_accepted_caller(
        &mut self,
        caller: &str,
        address: Address,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        let mut arg = MultiValueEncoded::new();

        arg.push(managed_address!(&address));

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.remove_accepted_callers(arg))
                .expect(tx_expect),
        );
        self
    }

    fn set_bond_token(
        &mut self,
        caller: &str,
        token_identifier: &[u8],
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(
                    self.contract
                        .set_bond_token(managed_token_id!(token_identifier)),
                )
                .expect(tx_expect),
        );
        self
    }

    fn set_lock_period_and_bond(
        &mut self,
        caller: &str,
        lock_period: u64,
        bond: u64,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        let mut arg = MultiValueEncoded::new();
        arg.push(MultiValue2((lock_period, managed_biguint!(bond))));

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.set_lock_periods_with_bonds(arg))
                .expect(tx_expect),
        );
        self
    }

    fn remove_lock_period_and_bond(
        &mut self,
        caller: &str,
        lock_period: u64,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        let mut arg = MultiValueEncoded::new();
        arg.push(lock_period);
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.remove_lock_periods_with_bonds(arg))
                .expect(tx_expect),
        );
        self
    }

    fn set_minimum_penalty(
        &mut self,
        caller: &str,
        minimum_penalty: u64,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.set_minimum_penalty(minimum_penalty))
                .expect(tx_expect),
        );
        self
    }

    fn set_maximum_penalty(
        &mut self,
        caller: &str,
        maximum_penalty: u64,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.set_maximum_penalty(maximum_penalty))
                .expect(tx_expect),
        );
        self
    }

    fn set_withdraw_penalty(
        &mut self,
        caller: &str,
        withdraw_penalty: u64,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.set_withdraw_penalty(withdraw_penalty))
                .expect(tx_expect),
        );
        self
    }

    fn initiate_refund(
        &mut self,
        caller: &str,
        token_identifier: &[u8],
        nonce: u64,
        end_timestamp: u64,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.initiate_refund(
                    managed_token_id!(token_identifier),
                    nonce,
                    end_timestamp,
                ))
                .expect(tx_expect),
        );
        self
    }

    fn sanction(
        &mut self,
        caller: &str,
        token_identifier: &[u8],
        nonce: u64,
        penalty: PenaltyType,
        custom_penalty: OptionalValue<u64>,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.sanction(
                    managed_token_id!(token_identifier),
                    nonce,
                    penalty,
                    custom_penalty,
                ))
                .expect(tx_expect),
        );
        self
    }

    fn modify_bond(
        &mut self,
        caller: &str,
        token_identifier: &[u8],
        nonce: u64,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(
                    self.contract
                        .modify_bond(managed_token_id!(token_identifier), nonce),
                )
                .expect(expect.unwrap_or(TxExpect::ok())),
        );
        self
    }

    fn mock_bond_storage(&mut self) -> &mut Self {
        // trigger error not implemented
        panic!("Error: Not implemented");
    }

    fn withdraw(
        &mut self,
        caller: &str,
        token_identifier: &[u8],
        nonce: u64,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(
                    self.contract
                        .withdraw(managed_token_id!(token_identifier), nonce),
                )
                .expect(expect.unwrap_or(TxExpect::ok())),
        );
        self
    }

    fn renew(
        &mut self,
        caller: &str,
        token_identifier: &[u8],
        nonce: u64,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(
                    self.contract
                        .renew(managed_token_id!(token_identifier), nonce),
                )
                .expect(expect.unwrap_or(TxExpect::ok())),
        );
        self
    }

    fn proof(
        &mut self,
        caller: &str,
        payment_token_identifier: &[u8],
        payment_token_nonce: u64,
        payment_amount: BigUint,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .esdt_transfer(
                    payment_token_identifier,
                    payment_token_nonce,
                    payment_amount,
                )
                .expect(expect.unwrap_or(TxExpect::ok())),
        );
        self
    }

    fn claim_refund(
        &mut self,
        token_identifier: &[u8],
        nonce: u64,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(OWNER_BONDING_CONTRACT_ADDRESS_EXPR)
                .call(
                    self.contract
                        .claim_refund(managed_token_id!(token_identifier), nonce),
                )
                .expect(expect.unwrap_or(TxExpect::ok())),
        );
        self
    }
}

#[test]
fn deploy_and_pause() {
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
