/*
Functions:

 */

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::{Base64VecU8, WrappedBalance, WrappedDuration, WrappedTimestamp};
use near_sdk::serde_json::{json, Value};
use near_sdk::{env, near_bindgen, PanicOnDefault};
use near_sdk::{AccountId, Balance, Duration, Gas, Timestamp};
use near_sdk::{Promise, PromiseResult};
use near_sdk::serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::callbacks::ext_self;

mod admins;
mod callbacks;
mod views;

near_sdk::setup_alloc!();

const DEFAULT_GAS_FEE: Gas = 20_000_000_000_000;
const TOKENHUB_TREASURY: &str = "treasury.tokenhub.testnet";
const FT_WASM_CODE: &[u8] = include_bytes!("../../static/fungible_token.wasm");
const DEPLOYER_WASM_CODE: &[u8] = include_bytes!("../../static/token_deployer.wasm");

pub type TokenAllocationInput = HashMap<AccountId, WrappedTokenAllocation>;

#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct WrappedTokenAllocation {
    allocated_num: WrappedBalance,
    initial_release: WrappedBalance,
    vesting_start_time: WrappedTimestamp,
    vesting_end_time: WrappedTimestamp,
    vesting_interval: WrappedDuration,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenAllocation {
    allocated_num: Balance,
    initial_release: Balance,
    vesting_start_time: Timestamp,
    vesting_end_time: Timestamp,
    vesting_interval: Duration,
    claimed: Balance,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct State {
    // token info
    ft_contract: AccountId,
    total_supply: Option<Balance>,
    token_name: Option<String>,
    symbol: Option<String>,
    icon: Option<String>,
    reference: Option<String>,
    reference_hash: Option<Base64VecU8>,
    decimals: Option<u8>,

    // creator and deployer
    ft_deployer: AccountId,
    creator: AccountId,

    // Multiple tokenomics
    allocations: UnorderedMap<AccountId, TokenAllocation>,

    // issuance states
    ft_contract_deployed: u8,
    deployer_contract_deployed: u8,
    ft_issued: u8,
    allocation_initialized: u8,
}

impl Default for State {
    fn default() -> Self {
        let default_string_value = String::from("__default_value__");
        Self {
            ft_contract: default_string_value.clone(),
            total_supply: Some(0),
            token_name: Some(default_string_value.clone()),
            symbol: Some(default_string_value.clone()),
            icon: Some(default_string_value.clone()),
            reference: None,
            reference_hash: None,
            decimals: Some(0),

            ft_deployer: default_string_value.clone(),
            creator: default_string_value.clone(),

            allocations: UnorderedMap::new(b"tokennomics".to_vec()),

            ft_contract_deployed: 0,
            deployer_contract_deployed: 0,
            ft_issued: 0,
            allocation_initialized: 0,
        }
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct TokenFactory {
    tokens: UnorderedMap<AccountId, State>,
}

#[near_bindgen]
impl TokenFactory {
    #[init]
    pub fn new() -> Self {
        return Self {
            tokens: UnorderedMap::new(b"tokenspec".to_vec()),
        };
    }

    #[payable]
    pub fn register(
        &mut self,
        ft_contract: AccountId,
        deployer_contract: AccountId,
        total_supply: WrappedBalance,
        token_name: String,
        symbol: String,
        icon: Option<String>,
        reference: Option<String>,
        reference_hash: Option<Base64VecU8>,
        allocations: TokenAllocationInput,
        decimals: u8,
    ) {
        assert!(
            env::attached_deposit() >= 4_000_000_000_000_000_000_000_000,
            "Minimum deposit is 4 NEAR",
        );
        
        let mut state_allocations = UnorderedMap::new(b"allocations".to_vec());

        for (account_id, alloc) in &allocations {
            let a = TokenAllocation {
                allocated_num: alloc.allocated_num.into(),
                initial_release: alloc.initial_release.into(),
                vesting_start_time: alloc.vesting_start_time.into(),
                vesting_end_time: alloc.vesting_end_time.into(),
                vesting_interval: alloc.vesting_interval.into(),
                claimed: 0,
            };

            self.assert_invalid_allocation(a.clone());

            let total_allocs: u128 = state_allocations 
                .values()
                .map(|v: TokenAllocation| v.allocated_num)
                .sum();

            assert!(
                total_allocs + a.allocated_num <= total_supply.into(),
                "Total allocations is greater than total supply"
            );
            state_allocations.insert(account_id, &a);
        }
        
        let token = State {
            ft_contract: ft_contract.clone(),
            total_supply: Some(total_supply.into()),
            token_name: Some(token_name),
            symbol: Some(symbol),
            icon,
            reference,
            reference_hash,
            decimals: Some(decimals),

            ft_deployer: deployer_contract,
            creator: env::signer_account_id(),

            allocations: state_allocations, 

            ft_contract_deployed: 0,
            deployer_contract_deployed: 0,
            ft_issued: 0,
            allocation_initialized: 0,
        };

        assert!(
            token.total_supply > Some(0),
            "total_supply must be greater than 0",
        );
        assert!(
            !env::is_valid_account_id(token.ft_contract.as_bytes()),
            "ft_contract must not existed",
        );
        assert!(
            !env::is_valid_account_id(token.ft_deployer.as_bytes()),
            "ft_deployer already existed",
        );

        // TODO: validate more?
        self.tokens.insert(&ft_contract, &token);
    }

    pub fn create_ft_contract(&mut self, ft_contract: AccountId) -> Promise {
        let token = self.tokens.get(&ft_contract.clone()).unwrap_or_default();
        self.assert_invalid_allocations(ft_contract.clone());
        self.assert_singer_account(token.creator);

        return Promise::new(ft_contract.parse().unwrap())
            .create_account()
            // .add_full_access_key(env::signer_account_pk())
            .transfer(4_000_000_000_000_000_000_000_000)
            .deploy_contract(FT_WASM_CODE.to_vec())
            .then(ext_self::on_ft_contract_deployed(
                ft_contract,
                &env::current_account_id(),
                0,
                DEFAULT_GAS_FEE,
            ));
    }

    pub fn create_deployer_contract(&mut self, ft_contract: AccountId) -> Promise {
        let token = self.tokens.get(&ft_contract).unwrap_or_default();
        self.assert_invalid_allocations(ft_contract.clone());
        self.assert_singer_account(token.creator);

        return Promise::new(token.ft_deployer.parse().unwrap())
            .create_account()
            // .add_full_access_key(env::signer_account_pk())
            .transfer(4_000_000_000_000_000_000_000_000)
            .deploy_contract(DEPLOYER_WASM_CODE.to_vec())
            .then(ext_self::on_ft_deployer_deployed(
                ft_contract,
                &env::current_account_id(),
                0,
                DEFAULT_GAS_FEE,
            ));
    }

    pub fn issue_ft(&mut self, ft_contract: AccountId) -> Promise {
        let token = self.tokens.get(&ft_contract).unwrap_or_default();
        self.assert_invalid_allocations(ft_contract.clone());
        self.assert_singer_account(token.creator);

        return Promise::new(ft_contract.parse().unwrap())
            .function_call(
                b"new".to_vec(),
                json!({
                    "owner_id": token.ft_deployer,
                    "total_supply": WrappedBalance::from(
                        token
                            .total_supply
                            .expect("Total supply is None !")
                    ),
                    "metadata": {
                        "spec": "ft-1.0.0",
                        "name": token.token_name,
                        "symbol": token.symbol,
                        "icon": token.icon,
                        "reference": token.reference,
                        "reference_hash": token.reference,
                        "decimals": token.decimals,
                    }
                })
                .to_string()
                .as_bytes()
                .to_vec(),
                0,
                DEFAULT_GAS_FEE,
            )
            .then(ext_self::on_token_issued(
                ft_contract,
                &env::current_account_id(),
                0,
                DEFAULT_GAS_FEE,
            ));
    }

    pub fn init_token_allocation(&mut self, ft_contract: AccountId) -> Promise {
        let token = self.tokens.get(&ft_contract).unwrap_or_default();
        self.assert_invalid_allocations(ft_contract.clone());
        self.assert_singer_account(token.creator);

        let mut alloctions: HashMap<AccountId, WrappedTokenAllocation> = HashMap::new();

        for k in token.allocations.keys() {
            alloctions.insert(
                k.clone(),
                token.allocations
                .get(&k.clone())
                .map(|v| WrappedTokenAllocation {
                        allocated_num: WrappedBalance::from(v.allocated_num),
                        initial_release: WrappedBalance::from(v.initial_release),
                        vesting_start_time: WrappedTimestamp::from(v.vesting_start_time),
                        vesting_end_time: WrappedTimestamp::from(v.vesting_end_time),
                        vesting_interval: WrappedTimestamp::from(v.vesting_interval)
                    })
                .expect("Allocation not found")
            );
        }

        return Promise::new(token.ft_deployer.parse().unwrap())
            .function_call(
                b"new".to_vec(),
                json!({
                    "ft_contract_name": ft_contract,
                    "total_supply": WrappedBalance::from(
                        token
                            .total_supply
                            .expect("Total supply is None !")
                    ),
                    "alloctions": alloctions
                })
                .to_string()
                .as_bytes()
                .to_vec(),
                0,
                DEFAULT_GAS_FEE,
            )
            .then(ext_self::on_allocation_init(
                ft_contract,
                &env::current_account_id(),
                0,
                DEFAULT_GAS_FEE,
            ));
    }

    /// Utils
    //Get total allocations
    pub fn assert_invalid_allocations(
        &self,
        ft_contract: AccountId
    ) {
        let token = self.tokens.get(&ft_contract).unwrap_or_default();

        assert!(
            token.total_supply != Some(0) && token.allocations.values_as_vector().len() != 0,
            "Token is not register"
        );

        let total_allocations: u128 = token.allocations 
                .values()
                .map(|a| {
                    self.assert_invalid_allocation(a.clone());
                    a.allocated_num
                })
                .sum();
        
        assert!(
            total_allocations == token.total_supply.unwrap_or(0),
            "Total alloctions is not equal to total supply"
        );
    }

    fn assert_invalid_allocation(
        &self, 
        allocation: TokenAllocation 
    ) {
            assert!(
                allocation.allocated_num >= allocation.initial_release + allocation.claimed,
                "Allocation is smaller than the total claimable",
            );
            assert!(
                allocation.vesting_interval <= allocation.vesting_end_time - allocation.vesting_start_time,
                "Vesting interval is larger than vesting time",
            );
    }

    fn assert_singer_account(
        &self,
        creator: AccountId
    ) {
        assert!(
            env::signer_account_id() == creator,
            "Only creator is allowed to execute the function",
        );

    }

}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 *
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "tokensale_near".to_string(),
            signer_account_id: "harrynguyen_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "harrynguyen_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 1_000_000_000_000_000_000_000_000,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }
}
