use core_mx_life_bonding_sc::{
    admin::ProxyTrait as _,
    config::{ProxyTrait as _, State},
    storage::PenaltyType,
    storage::ProxyTrait as _,
    views::ProxyTrait as _,
};

use core_mx_life_bonding_sc::ProxyTrait as _;
use multiversx_sc::{
    codec::multi_types::{MultiValue2, OptionalValue},
    storage::mappers::SingleValue,
    types::{Address, MultiValueEncoded, TokenIdentifier},
};
use multiversx_sc_scenario::{
    api::StaticApi,
    managed_address, managed_biguint, managed_token_id,
    num_bigint::BigUint,
    scenario_model::{
        Account, AddressValue, BytesKey, BytesValue, ScCallStep, ScDeployStep, ScQueryStep,
        SetStateStep, TxExpect,
    },
    ContractInfo, ScenarioWorld,
};

pub const BONDING_CONTRACT_PATH: &str = "mxsc:output/core-mx-life-bonding-sc.msxc.json";
pub const BONDING_CONTRACT_ADDRESS_EXPR: &str = "sc:core-mx-life-bonding-sc";

pub const OWNER_BONDING_CONTRACT_ADDRESS_EXPR: &str = "address:owner";

pub const ITHEUM_TOKEN_IDENTIFIER_EXPR: &str = "str:ITHEUM-fce905";
pub const ITHEUM_TOKEN_IDENTIFIER: &[u8] = b"ITHEUM-fce905";

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
                .call(self.contract.set_lock_periods_with_bonds(arg))
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

    pub fn mock_bond_and_compensation_storage(
        &mut self,
        id: u64,
        compensation_accumulated_amount: u64,
        compensation_proof_amount: u64,
        compensation_end_date: u64,
        bond_lock_period: u64,
        bond_timestamp: u64,
        unbound_timestamp: u64,
        bond_amount: u64,
        remaining_amount: u64,
    ) -> Account {
        let bonding_contract_code = self.world.code_expression(BONDING_CONTRACT_PATH);

        let mut acc = Account::new().code(bonding_contract_code);

        acc.storage.insert(
            BytesKey::from(b"contract_state".to_vec()),
            BytesValue::from(vec![0u8]),
        );

        acc.storage.insert(
            BytesKey::from(b"administrator".to_vec()),
            BytesValue::from(self.admin.as_bytes().to_vec()),
        );

        acc.storage.insert(
            BytesKey::from(b"accepted_callers".to_vec()),
            BytesValue::from(self.admin.as_bytes().to_vec()),
        );

        acc.storage.insert(
            BytesKey::from(b"bond_payment_token".to_vec()),
            BytesValue::from(ITHEUM_TOKEN_IDENTIFIER.to_vec()),
        );

        acc.storage.insert(
            BytesKey::from(b"lock_periods".to_vec()),
            BytesValue::from(bond_lock_period.to_be_bytes().to_vec()),
        );

        acc.storage.insert(
            create_bytes_key("lock_period_bond_amount", 10u64),
            BytesValue::from(BigUint::from(bond_amount).to_bytes_be().to_vec()),
        );

        acc.storage.insert(
            BytesKey::from(b"minimum_penalty".to_vec()),
            BytesValue::from(500u64.to_be_bytes().to_vec()),
        );

        acc.storage.insert(
            BytesKey::from(b"maximum_penalty".to_vec()),
            BytesValue::from(10_000u64.to_be_bytes().to_vec()),
        );

        acc.storage.insert(
            BytesKey::from(b"withdraw_penalty".to_vec()),
            BytesValue::from(8_000u64.to_be_bytes().to_vec()),
        );

        // compensation storage with id

        acc.storage.insert(
            create_bytes_key("compensation_token_identifer", id),
            BytesValue::from(DATA_NFT_IDENTIFIER.to_vec()),
        );

        acc.storage.insert(
            create_bytes_key("compensation_nonce", id),
            BytesValue::from(1u64.to_be_bytes().to_vec()),
        );

        acc.storage.insert(
            create_bytes_key("compensation_accumulated_amount", id),
            BytesValue::from(
                BigUint::from(compensation_accumulated_amount)
                    .to_bytes_be()
                    .to_vec(),
            ),
        );

        acc.storage.insert(
            create_bytes_key("compensation_proof_amount", id),
            BytesValue::from(
                BigUint::from(compensation_proof_amount)
                    .to_bytes_be()
                    .to_vec(),
            ),
        );

        acc.storage.insert(
            create_bytes_key("compensation_end_date", id),
            BytesValue::from(compensation_end_date.to_be_bytes().to_vec()),
        );

        acc.storage.insert(
            BytesKey::from(b"compensations_idsid".to_vec()),
            BytesValue::from(1u64.to_be_bytes().to_vec()),
        );

        acc.storage.insert(
            BytesKey::from(b"compensations_idsobject".to_vec()),
            BytesValue::from(
                DATA_NFT_IDENTIFIER
                    .to_vec()
                    .extend_from_slice(&1u64.to_le_bytes()),
            ),
        );

        // bond storage with id

        acc.storage.insert(
            create_bytes_key("bond_address", id),
            BytesValue::from(self.first_user_address.as_bytes().to_vec()),
        );

        acc.storage.insert(
            create_bytes_key("bond_token_identifier", id),
            BytesValue::from(DATA_NFT_IDENTIFIER.to_vec()),
        );

        acc.storage.insert(
            create_bytes_key("bond_nonce", id),
            BytesValue::from(1u64.to_be_bytes().to_vec()),
        );

        acc.storage.insert(
            create_bytes_key("bond_lock_period", id),
            BytesValue::from(bond_lock_period.to_be_bytes().to_vec()),
        );

        acc.storage.insert(
            create_bytes_key("bond_timestamp", id),
            BytesValue::from(bond_timestamp.to_be_bytes().to_vec()),
        );

        acc.storage.insert(
            create_bytes_key("unbound_timestamp", id),
            BytesValue::from(unbound_timestamp.to_be_bytes().to_vec()),
        );

        acc.storage.insert(
            create_bytes_key("bond_amount", id),
            BytesValue::from(BigUint::from(bond_amount).to_bytes_be().to_vec()),
        );

        acc.storage.insert(
            create_bytes_key("remaining_amount", id),
            BytesValue::from(BigUint::from(remaining_amount).to_bytes_be().to_vec()),
        );

        acc.storage.insert(
            BytesKey::from(b"bonds_idsid".to_vec()),
            BytesValue::from(1u64.to_be_bytes().to_vec()),
        );

        acc.storage.insert(
            BytesKey::from(b"bonds_idsobject".to_vec()),
            BytesValue::from(
                DATA_NFT_IDENTIFIER
                    .to_vec()
                    .extend_from_slice(&1u64.to_le_bytes()),
            ),
        );

        let mut key_vec = b"address_bonds".to_vec();
        key_vec.extend_from_slice(self.first_user_address.as_array());

        acc.storage.insert(
            BytesKey::from(key_vec),
            BytesValue::from(1u64.to_be_bytes().to_vec()),
        );

        acc.storage.insert(
            BytesKey::from(b"bonds".to_vec()),
            BytesValue::from(1u64.to_be_bytes().to_vec()),
        );

        acc
    }

    pub fn deploy_mock_account(&mut self, acc: &mut Account) -> &mut Self {
        acc.owner = Option::Some(AddressValue::from(OWNER_BONDING_CONTRACT_ADDRESS_EXPR));
        self.world.set_state_step(
            SetStateStep::new()
                .new_token_identifier(ITHEUM_TOKEN_IDENTIFIER_EXPR)
                .new_token_identifier(DATA_NFT_IDENTIFIER_EXPR)
                .put_account(BONDING_CONTRACT_ADDRESS_EXPR, acc.clone()),
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

    pub fn claim_refund(
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

fn create_bytes_key<T: Into<u64>>(string: &str, value: T) -> BytesKey {
    let value_u64 = value.into();
    let bytes = value_u64.to_be_bytes().to_vec();

    let mut storage_mapper = Vec::new();

    storage_mapper.extend_from_slice(string.as_bytes());
    storage_mapper.extend_from_slice(&bytes);

    BytesKey::from(storage_mapper)
}
