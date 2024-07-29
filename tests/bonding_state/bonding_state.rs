use core_mx_life_bonding_sc::{
    admin::ProxyTrait as _,
    config::{ProxyTrait as _, State},
    storage::PenaltyType,
};

use core_mx_life_bonding_sc::ProxyTrait as _;
use multiversx_sc::{
    codec::multi_types::{MultiValue2, OptionalValue},
    storage::mappers::SingleValue,
    types::{Address, MultiValueEncoded},
};
use multiversx_sc_scenario::{
    api::StaticApi,
    imports::MxscPath,
    managed_address, managed_biguint, managed_token_id,
    num_bigint::BigUint,
    scenario_model::{
        Account, AddressValue, BytesValue, ScCallStep, ScDeployStep, ScQueryStep, SetStateStep,
        TxExpect,
    },
    ContractInfo, ScenarioWorld,
};

pub const BONDING_CONTRACT_PATH: &str = "mxsc:output/core-mx-life-bonding-sc.msxc.json";
pub const BONDING_CONTRACT_ADDRESS_EXPR: &str = "sc:core-mx-life-bonding-sc";

pub const LIVELINESS_STAKE_OWNER_ADDRESS_EXPR: &str = "address:core-mx-life-bonding-sc";

pub const LIVELINESS_STAKE_PATH: &str =
    "mxsc:test_external_contracts/core-mx-life-bonding-sc.mxsc.json";

pub const LIVELINESS_STAKE_CONTRACT_ADDRESS_EXPR: &str = "sc:core-mx-liveliness-stake";

pub const MINTER_CONTRACT_ADDRESS_EXPR: &str = "sc:minter";

pub const OWNER_BONDING_CONTRACT_ADDRESS_EXPR: &str = "address:owner";

pub const ITHEUM_TOKEN_IDENTIFIER_EXPR: &str = "str:ITHEUM-fce905";
pub const ITHEUM_TOKEN_IDENTIFIER: &[u8] = b"ITHEUM-fce905";

pub const ANOTHER_TOKEN_IDENTIFIER_EXPR: &str = "str:ANOTHER-fce905";

pub const DATA_NFT_IDENTIFIER_EXPR: &str = "str:DATANFT-12345";
pub const DATA_NFT_IDENTIFIER: &[u8] = b"DATANFT-12345";

pub const ADMIN_BONDING_CONTRACT_ADDRESS_EXPR: &str = "address:admin";

pub const FIRST_USER_ADDRESS_EXPR: &str = "address:first_user";
pub const SECOND_USER_ADDRESS_EXPR: &str = "address:second_user";
pub const THIRD_USER_ADDRESS_EXPR: &str = "address:third_user";

type Contract = ContractInfo<core_mx_life_bonding_sc::Proxy<StaticApi>>;

pub fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("");

    blockchain.register_contract(
        BONDING_CONTRACT_PATH,
        core_mx_life_bonding_sc::ContractBuilder,
    );

    blockchain.register_contract(
        LIVELINESS_STAKE_PATH,
        core_mx_liveliness_stake::ContractBuilder,
    );

    blockchain
}

pub struct ContractState {
    pub world: ScenarioWorld,
    pub contract: Contract,
    pub contract_owner: Address,
    pub admin: Address,
    pub first_user_address: Address,
    pub second_user_address: Address,
    pub third_user_address: Address,
}

impl ContractState {
    pub fn new() -> Self {
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
                    LIVELINESS_STAKE_OWNER_ADDRESS_EXPR,
                    Account::new().nonce(1).balance("1_000"),
                )
                .new_address(
                    LIVELINESS_STAKE_OWNER_ADDRESS_EXPR,
                    1,
                    LIVELINESS_STAKE_CONTRACT_ADDRESS_EXPR,
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
                        .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "100")
                        .esdt_nft_balance(DATA_NFT_IDENTIFIER_EXPR, 1u64, 2u64, None::<BytesValue>),
                )
                .put_account(
                    SECOND_USER_ADDRESS_EXPR,
                    Account::new()
                        .nonce(1)
                        .balance("1_000")
                        .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "100")
                        .esdt_nft_balance(DATA_NFT_IDENTIFIER_EXPR, 1u64, 2u64, None::<BytesValue>),
                )
                .put_account(
                    THIRD_USER_ADDRESS_EXPR,
                    Account::new()
                        .nonce(1)
                        .balance("1_000")
                        .esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "100")
                        .esdt_nft_balance(DATA_NFT_IDENTIFIER_EXPR, 1u64, 2u64, None::<BytesValue>),
                )
                .put_account(
                    MINTER_CONTRACT_ADDRESS_EXPR,
                    Account::new()
                        .nonce(1)
                        .owner(OWNER_BONDING_CONTRACT_ADDRESS_EXPR)
                        .code(world.code_expression(BONDING_CONTRACT_PATH))
                        .balance("1_000")
                        .esdt_balance(ANOTHER_TOKEN_IDENTIFIER_EXPR, "100"),
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

    pub fn default_deploy_and_set(&mut self, lock_period: u64, bond_amount: u64) -> &mut Self {
        let admin = self.admin.clone();
        self.deploy()
            .deploy_liveliness_stake()
            .set_administrator(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, admin.clone(), None)
            .set_accepted_caller(OWNER_BONDING_CONTRACT_ADDRESS_EXPR, admin.clone(), None)
            .set_bond_token(
                OWNER_BONDING_CONTRACT_ADDRESS_EXPR,
                ITHEUM_TOKEN_IDENTIFIER,
                None,
            )
            .set_lock_period_and_bond(
                OWNER_BONDING_CONTRACT_ADDRESS_EXPR,
                lock_period,
                bond_amount,
                None,
            )
            .set_liveliness_stake_contract(
                OWNER_BONDING_CONTRACT_ADDRESS_EXPR,
                AddressValue::from(LIVELINESS_STAKE_CONTRACT_ADDRESS_EXPR).to_address(),
                None,
            );

        self
    }

    pub fn deploy_liveliness_stake(&mut self) -> &mut Self {
        let liveliness_stake_contract_code = self.world.code_expression(LIVELINESS_STAKE_PATH);

        self.world.sc_deploy(
            ScDeployStep::new()
                .from(LIVELINESS_STAKE_OWNER_ADDRESS_EXPR)
                .code(liveliness_stake_contract_code)
                .call(self.contract.init()),
        );
        self
    }

    pub fn deploy(&mut self) -> &mut Self {
        let bonding_contract_code = self.world.code_expression(BONDING_CONTRACT_PATH);

        self.world.sc_deploy(
            ScDeployStep::new()
                .from(OWNER_BONDING_CONTRACT_ADDRESS_EXPR)
                .code(bonding_contract_code)
                .call(self.contract.init()),
        );
        self
    }

    pub fn set_administrator(
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

    pub fn set_liveliness_stake_contract(
        &mut self,
        caller: &str,
        liveliness_stake: Address,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(
                    self.contract
                        .set_liveliness_stake_address(managed_address!(&liveliness_stake)),
                )
                .expect(tx_expect),
        );
        self
    }

    pub fn initiate_bond_for_address(
        &mut self,
        caller: &str,
        address: Address,
        token_identifier: &[u8],
        nonce: u64,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.initiate_bond_for_address(
                    managed_address!(&address),
                    managed_token_id!(token_identifier),
                    nonce,
                ))
                .expect(tx_expect),
        );
        self
    }

    pub fn pause_contract(&mut self, caller: &str, expect: Option<TxExpect>) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.set_contract_state_inactive())
                .expect(tx_expect),
        );
        self
    }

    pub fn unpause_contract(&mut self, caller: &str, expect: Option<TxExpect>) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .call(self.contract.set_contract_state_active())
                .expect(tx_expect),
        );
        self
    }

    pub fn check_contract_state(&mut self, contract_state: State) -> &mut Self {
        self.world.sc_query(
            ScQueryStep::new()
                .call(self.contract.contract_state())
                .expect_value(SingleValue::from(contract_state)),
        );
        self
    }

    pub fn set_accepted_caller(
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

    pub fn set_blacklist(
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

    pub fn remove_blacklist(
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

    pub fn remove_accepted_caller(
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

    pub fn set_bond_token(
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

    pub fn bond(
        &mut self,
        caller: &str,
        original_owner: Address,
        token_identifier: &[u8],
        nonce: u64,
        lock_period: u64,
        payment: (&str, u64, u64),
        expect: Option<TxExpect>,
    ) -> &mut Self {
        let tx_expect = expect.unwrap_or(TxExpect::ok());
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .esdt_transfer(payment.0, payment.1, BigUint::from(payment.2))
                .call(self.contract.bond(
                    managed_address!(&original_owner),
                    managed_token_id!(token_identifier),
                    nonce,
                    lock_period,
                ))
                .expect(tx_expect),
        );
        self
    }

    pub fn set_lock_period_and_bond(
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
                .call(self.contract.add_lock_periods_with_bonds(arg))
                .expect(tx_expect),
        );
        self
    }

    pub fn remove_lock_period_and_bond(
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

    pub fn set_minimum_penalty(
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

    pub fn set_maximum_penalty(
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

    pub fn set_withdraw_penalty(
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

    pub fn initiate_refund(
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

    pub fn sanction(
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

    pub fn modify_bond(
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

    pub fn withdraw(
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

    pub fn renew(
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

    pub fn proof(
        &mut self,
        caller: &str,
        payment_token_identifier: &[u8],
        payment_token_nonce: u64,
        payment_amount: u64,
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
                .call(self.contract.add_proof())
                .expect(expect.unwrap_or(TxExpect::ok())),
        );
        self
    }

    pub fn claim_refund(
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
                        .claim_refund(managed_token_id!(token_identifier), nonce),
                )
                .expect(expect.unwrap_or(TxExpect::ok())),
        );
        self
    }

    pub fn set_vault_nonce(
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
                        .set_vault_nonce(managed_token_id!(token_identifier), nonce),
                )
                .expect(expect.unwrap_or(TxExpect::ok())),
        );
        self
    }

    pub fn top_up_vault(
        &mut self,
        caller: &str,
        payment: (&str, u64, u64),
        token_identifier: &[u8],
        nonce: u64,
        expect: Option<TxExpect>,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(caller)
                .esdt_transfer(payment.0, payment.1, payment.2)
                .call(
                    self.contract
                        .top_up_vault(managed_token_id!(token_identifier), nonce),
                )
                .expect(expect.unwrap_or(TxExpect::ok())),
        );
        self
    }
}
