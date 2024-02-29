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
    scenario_model::{Account, AddressValue, ScCallStep, ScDeployStep, ScQueryStep, SetStateStep},
    testing_framework::ScCallMandos,
    ContractInfo, ScenarioWorld,
};

const BONDING_CONTRACT_PATH: &str = "mxsc:output/core-mx-life-bonding-sc.mxsc.json";
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
    blockchain.set_current_dir_from_workspace("..");

    blockchain.register_contract(
        BONDING_CONTRACT_ADDRESS_EXPR,
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
                .from(BONDING_CONTRACT_ADDRESS_EXPR)
                .code(bonding_contract_code)
                .call(self.contract.init()),
        );
        self
    }

    fn pause_contract(&mut self, address: &str) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(address)
                .call(self.contract.set_contract_state_inactive()),
        );
        self
    }

    fn unpause_contract(&mut self, address: &str) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(address)
                .call(self.contract.set_contract_state_active()),
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

    fn set_accepted_caller(&mut self, caller: &str, address: Address) -> &mut Self {
        let mut arg = MultiValueEncoded::new();

        arg.push(managed_address!(&address));

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.set_accepted_callers(arg)),
        );
        self
    }

    fn set_blacklist(&mut self, caller: &str, compensation_id: u64, address: Address) -> &mut Self {
        let mut arg = MultiValueEncoded::new();
        arg.push(managed_address!(&address));

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.add_to_black_list(compensation_id, arg)),
        );
        self
    }

    fn remove_blacklist(
        &mut self,
        caller: &str,
        compensation_id: u64,
        address: Address,
    ) -> &mut Self {
        let mut arg = MultiValueEncoded::new();
        arg.push(managed_address!(&address));

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.remove_from_black_list(compensation_id, arg)),
        );
        self
    }

    fn remove_accepted_caller(&mut self, caller: &str, address: Address) -> &mut Self {
        let mut arg = MultiValueEncoded::new();

        arg.push(managed_address!(&address));

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.remove_accepted_callers(arg)),
        );
        self
    }

    fn set_bond_token(&mut self, caller: &str, token_identifier: &[u8]) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new().from(caller).call(
                self.contract
                    .set_bond_token(managed_token_id!(token_identifier)),
            ),
        );
        self
    }

    fn set_lock_period_and_bond(&mut self, caller: &str, lock_period: u64, bond: u64) -> &mut Self {
        let mut arg = MultiValueEncoded::new();
        arg.push(MultiValue2((lock_period, managed_biguint!(bond))));

        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.set_lock_periods_with_bonds(arg)),
        );
        self
    }

    fn remove_lock_period_and_bond(&mut self, caller: &str, lock_period: u64) -> &mut Self {
        let mut arg = MultiValueEncoded::new();
        arg.push(lock_period);
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.remove_lock_periods_with_bonds(arg)),
        );
        self
    }

    fn set_minimum_penalty(&mut self, caller: &str, minimum_penalty: u64) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.set_minimum_penalty(minimum_penalty)),
        );
        self
    }

    fn set_maximum_penalty(&mut self, caller: &str, maximum_penalty: u64) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.set_maximum_penalty(maximum_penalty)),
        );
        self
    }

    fn set_withdraw_penalty(&mut self, caller: &str, withdraw_penalty: u64) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.set_withdraw_penalty(withdraw_penalty)),
        );
        self
    }

    fn initiate_refund(
        &mut self,
        caller: &str,
        token_identifier: &[u8],
        nonce: u64,
        end_timestamp: u64,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.initiate_refund(
                    managed_token_id!(token_identifier),
                    nonce,
                    end_timestamp,
                )),
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
    ) -> &mut Self {
        self.world
            .sc_call(ScCallStep::new().from(caller).call(self.contract.sanction(
                managed_token_id!(token_identifier),
                nonce,
                penalty,
                custom_penalty,
            )));
        self
    }

    fn modify_bond(&mut self, caller: &str, token_identifier: &[u8], nonce: u64) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new().from(caller).call(
                self.contract
                    .modify_bond(managed_token_id!(token_identifier), nonce),
            ),
        );
        self
    }
}
