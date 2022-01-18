/*
Functions:

 */


// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde_json::{json, Value};
use near_sdk::{env, near_bindgen, ext_contract, PanicOnDefault};
use near_sdk::collections::UnorderedMap;
use near_sdk::{AccountId, Balance, Timestamp, Duration, Gas};
use near_sdk::{Promise, PromiseResult};
use near_sdk::json_types::{WrappedBalance, WrappedDuration, WrappedTimestamp, Base64VecU8};

near_sdk::setup_alloc!();

const DEFAULT_GAS_FEE: Gas = 20_000_000_000_000;
const TOKENHUB_TREASURY: &str = "treasury.tokenhub.testnet";
const FT_WASM_CODE: &[u8] = include_bytes!("../../static/fungible_token.wasm");
const DEPLOYER_WASM_CODE: &[u8] = include_bytes!("../../static/token_deployer.wasm");

#[derive(BorshDeserialize, BorshSerialize)]
pub struct State {
    // token info
    ft_contract: AccountId,
    total_supply: Balance,
    token_name: String,
    symbol: String,
    icon: Option<String>,
    reference: Option<String>,
    reference_hash: Option<Base64VecU8>,
    decimals: u8,

    // creator and deployer
    ft_deployer: AccountId,
    creator: AccountId,

    // tokenomics
    initial_release: Balance,
    vesting_start_time: Timestamp,
    vesting_end_time: Timestamp,
    vesting_interval: Duration,
    treasury_allocation: Balance,

    // issuance states
    ft_contract_deployed: u8,
    deployer_contract_deployed: u8,
    ft_issued: u8,
    allocation_initialized: u8,
}

impl Default for State {
    fn default() -> Self {
        Self {
            ft_contract: String::from("__default_value__"),
            total_supply: 0,
            token_name: String::from("__default_value__"),
            symbol: String::from("__default_value__"),
            icon: Some(String::from("__default_value__")),
            reference: None,
            reference_hash: None,
            decimals: 0,

            ft_deployer: String::from("__default_value__"),
            creator: String::from("__default_value__"),

            initial_release: 0,
            vesting_start_time: 0,
            vesting_end_time: 0,
            vesting_interval: 0,
            treasury_allocation: 0,

            ft_contract_deployed: 0,
            deployer_contract_deployed: 0,
            ft_issued: 0,
            allocation_initialized: 0,
        }
    }
}

#[ext_contract(ext_self)]
pub trait ExtTokenFactory {
    fn on_ft_contract_deployed(&mut self, ft_contract: AccountId) -> bool;
    fn on_ft_deployer_deployed(&mut self, ft_contract: AccountId) -> bool;
    fn on_token_issued(&mut self, ft_contract: AccountId) -> bool;
    fn on_allocation_init(&mut self, ft_contract: AccountId) -> bool;
}

#[near_bindgen]
impl TokenFactory {
    #[private]
    pub fn on_ft_contract_deployed(&mut self, ft_contract: AccountId) -> bool {
        env::log(
            format!("promise_result_count = {}", env::promise_results_count())
            .as_bytes()
        );
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                let mut token = self.tokens.remove(&ft_contract).unwrap_or_default();
                assert!(
                    token.vesting_end_time != 0 && token.total_supply != 0,
                    "Token is not registered",
                );
                assert!(
                    token.ft_contract_deployed == 0,
                    "State ft_contract_deployed is invalid",
                );
                token.ft_contract_deployed = 1;
                self.tokens.insert(&ft_contract, &token);
                true
            },
            _ => false
        }
    }

    #[private]
    pub fn on_ft_deployer_deployed(&mut self, ft_contract: AccountId) -> bool {
        env::log(
            format!("promise_result_count = {}", env::promise_results_count())
            .as_bytes()
        );
        // format!("fasfas");
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                let mut token = self.tokens.remove(&ft_contract).unwrap_or_default();
                assert!(
                    token.vesting_end_time != 0 && token.total_supply != 0,
                    "Token is not registered",
                );
                assert!(
                    token.deployer_contract_deployed == 0,
                    "State deployer_contract_deployed is invalid",
                );
                token.deployer_contract_deployed = 1;
                self.tokens.insert(&ft_contract, &token);
                true
            },
            _ => false
        }
    }

    #[private]
    pub fn on_token_issued(&mut self, ft_contract: AccountId) -> bool {
        env::log(
            format!("promise_result_count = {}", env::promise_results_count())
            .as_bytes()
        );
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                let mut token = self.tokens.remove(&ft_contract).unwrap_or_default();
                assert!(
                    token.vesting_end_time != 0 && token.total_supply != 0,
                    "Token is not registered",
                );
                assert!(
                    token.ft_issued == 0,
                    "State ft_issued is invalid",
                );
                token.ft_issued = 1;
                self.tokens.insert(&ft_contract, &token);
                true
            },
            _ => false
        }
    }

    #[private]
    pub fn on_allocation_init(&mut self, ft_contract: AccountId) -> bool {
        env::log(
            format!("promise_result_count = {}", env::promise_results_count())
            .as_bytes()
        );
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                let mut token = self.tokens.remove(&ft_contract).unwrap_or_default();
                assert!(
                    token.vesting_end_time != 0 && token.total_supply != 0,
                    "Token is not registered",
                );
                assert!(
                    token.allocation_initialized == 0,
                    "State allocation_initialized is invalid",
                );
                token.allocation_initialized = 1;
                self.tokens.insert(&ft_contract, &token);
                true
            },
            _ => false
        }
    }

    pub fn reset(&mut self) {
        assert!(env::state_exists(), "The contract is not initialized");
        assert!(
            env::current_account_id() == env::signer_account_id(), 
            "Function called not from the contract owner itself",
        );
        self.tokens.clear();
    }

    pub fn unregister(&mut self, ft_contract: AccountId) {
        assert!(env::state_exists(), "The contract is not initialized");
        assert!(
            env::current_account_id() == env::signer_account_id(), 
            "Function called not from the contract owner itself",
        );
        self.tokens.remove(&ft_contract);
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
        decimals: u8,
        
        initial_release: WrappedBalance,
        vesting_start_time: WrappedTimestamp,
        vesting_end_time: WrappedTimestamp,
        vesting_interval: WrappedDuration,
        treasury_allocation: WrappedBalance,
    ) {
        let mut token: State = self.tokens.get(&ft_contract).unwrap_or_default();
        assert!(
            env::attached_deposit() >= 4_000_000_000_000_000_000_000_000,
            "Minimum deposit is 4 NEAR",
        );
        assert!(
            token.vesting_end_time == 0 && token.total_supply == 0,
            "Token is already registered",
        );

        token = State {
            ft_contract: ft_contract.clone(),
            total_supply: total_supply.into(),
            token_name: token_name,
            symbol: symbol,
            icon: icon,
            reference: reference,
            reference_hash: reference_hash,
            decimals: decimals,

            ft_deployer: deployer_contract,
            creator: env::signer_account_id(),

            initial_release: initial_release.into(),
            vesting_start_time: vesting_start_time.into(),
            vesting_end_time: vesting_end_time.into(),
            vesting_interval: vesting_interval.into(),
            treasury_allocation: treasury_allocation.into(),
            
            ft_contract_deployed: 0,
            deployer_contract_deployed: 0,
            ft_issued: 0,
            allocation_initialized: 0,
        };

        assert!(
            token.total_supply > 0,
            "total_supply must be greater than 0",
        );
        // assert!(
        //     !env::is_valid_account_id(token.ft_contract.as_bytes()),
        //     "ft_contract must not existed",
        // );
        // assert!(
        //     !env::is_valid_account_id(token.ft_deployer.as_bytes()),
        //     "ft_deployer already existed",
        // );
        assert!(
            token.vesting_interval <= token.vesting_end_time - token.vesting_start_time,
            "Vesting interval is larger than vesting time",
        );
        assert!(
            token.initial_release + token.treasury_allocation <= token.total_supply,
            "Total allocation is more than total supply",
        );
        // TODO: validate more?

        self.tokens.insert(&ft_contract, &token);
    }

    pub fn create_ft_contract(&mut self, ft_contract: AccountId) -> Promise {
        let token = self.tokens.get(&ft_contract).unwrap_or_default();
        assert!(
            token.vesting_end_time != 0 && token.total_supply != 0,
            "Token is not registered",
        );
        assert!(
            env::signer_account_id() == token.creator,
            "Only creator is allowed to execute the function",
        );

        return Promise::new(ft_contract.parse().unwrap())
            .create_account()
            // .add_full_access_key(env::signer_account_pk())
            .transfer(4_000_000_000_000_000_000_000_000)
            .deploy_contract(FT_WASM_CODE.to_vec())
            .then(
                ext_self::on_ft_contract_deployed(
                    ft_contract,
                    &env::current_account_id(),
                    0, DEFAULT_GAS_FEE,
                )
            );
    }

    pub fn create_deployer_contract(&mut self, ft_contract: AccountId) -> Promise {
        let token = self.tokens.get(&ft_contract).unwrap_or_default();
        assert!(
            token.vesting_end_time != 0 && token.total_supply != 0,
            "Token is not registered",
        );
        assert!(
            env::signer_account_id() == token.creator,
            "Only creator is allowed to execute the function",
        );

        return Promise::new(token.ft_deployer.parse().unwrap())
            .create_account()
            // .add_full_access_key(env::signer_account_pk())
            .transfer(4_000_000_000_000_000_000_000_000)
            .deploy_contract(DEPLOYER_WASM_CODE.to_vec())
            .then(
                ext_self::on_ft_deployer_deployed(
                    ft_contract,
                    &env::current_account_id(),
                    0, DEFAULT_GAS_FEE,
                )
            );
    }

    pub fn issue_ft(&mut self, ft_contract: AccountId) -> Promise {
        let token = self.tokens.get(&ft_contract).unwrap_or_default();
        assert!(
            token.vesting_end_time != 0 && token.total_supply != 0,
            "Token is not registered",
        );
        assert!(
            env::signer_account_id() == token.creator,
            "Only creator is allowed to execute the function",
        );

        return Promise::new(ft_contract.parse().unwrap())
            .function_call(
                b"new".to_vec(),
                json!({
                    "owner_id": token.ft_deployer,
                    "total_supply": WrappedBalance::from(token.total_supply),
                    "metadata": {
                        "spec": "ft-1.0.0",
                        "name": token.token_name,
                        "symbol": token.symbol,
                        "icon": token.icon,
                        "reference": token.reference,
                        "reference_hash": token.reference,
                        "decimals": token.decimals,
                    }
                }).to_string().as_bytes().to_vec(),
                0, DEFAULT_GAS_FEE,
            )
            .then(
                ext_self::on_token_issued(
                    ft_contract,
                    &env::current_account_id(),
                    0, DEFAULT_GAS_FEE,
                )
            );
    }

    pub fn init_token_allocation(&mut self, ft_contract: AccountId) -> Promise {
        let token = self.tokens.get(&ft_contract).unwrap_or_default();
        assert!(
            token.vesting_end_time != 0 && token.total_supply != 0,
            "Token is not registered",
        );
        assert!(
            env::signer_account_id() == token.creator,
            "Only creator is allowed to execute the function",
        );

        return Promise::new(token.ft_deployer.parse().unwrap())
            .function_call(
                b"new".to_vec(),
                json!({
                    "ft_contract_name": ft_contract,
                    "total_supply": WrappedBalance::from(token.total_supply), 
                    "allocations": {
                        &token.creator: {
                            "allocated_num": WrappedBalance::from(
                                token.total_supply - token.treasury_allocation),
                            "initial_release": WrappedBalance::from(token.initial_release),
                            "vesting_start_time": WrappedTimestamp::from(token.vesting_start_time),
                            "vesting_end_time": WrappedTimestamp::from(token.vesting_end_time),
                            "vesting_interval": WrappedDuration::from(token.vesting_interval),
                        },
                        TOKENHUB_TREASURY: {
                            "allocated_num": WrappedBalance::from(token.treasury_allocation),
                            "initial_release": "0",
                            "vesting_start_time": WrappedTimestamp::from(token.vesting_start_time),
                            "vesting_end_time": WrappedTimestamp::from(token.vesting_end_time),
                            "vesting_interval": WrappedDuration::from(token.vesting_interval),
                        }
                    }
                }).to_string().as_bytes().to_vec(),
                0, DEFAULT_GAS_FEE,
            )
            .then(
                ext_self::on_allocation_init(
                    ft_contract,
                    &env::current_account_id(),
                    0, DEFAULT_GAS_FEE,
                )
            );
    }

    pub fn get_token_state(self, ft_contract: AccountId) -> Value {
        let token = self.tokens.get(&ft_contract).unwrap_or_default();
        assert!(
            token.vesting_end_time != 0 && token.total_supply != 0,
            "Token is not registered",
        );
        return json!({
            "ft_contract": token.ft_contract,
            "total_supply": WrappedBalance::from(token.total_supply),
            "token_name": token.token_name,
            "symbol": token.symbol,
            "decimals": token.decimals,

            "ft_deployer": token.ft_deployer,
            "creator": token.creator,

            "initial_release": WrappedBalance::from(token.initial_release),
            "vesting_start_time": WrappedTimestamp::from(token.vesting_start_time),
            "vesting_end_time": WrappedTimestamp::from(token.vesting_end_time),
            "vesting_interval": WrappedDuration::from(token.vesting_interval),
            "treasury_allocation": WrappedBalance::from(token.treasury_allocation),

            "ft_contract_deployed": token.ft_contract_deployed,
            "deployer_contract_deployed": token.deployer_contract_deployed,
            "ft_issued": token.ft_issued,
            "allocation_initialized": token.allocation_initialized,
        });
    }

    pub fn list_my_tokens(&self, account_id: AccountId) -> Value {
        assert!(env::state_exists(), "The contract is not initialized");

        let token_list = self.tokens.keys_as_vector();
        let mut result: Value = json!([]);

        for token in token_list.iter() {
            let state = self.tokens.get(&token).unwrap_or_default();
            if state.creator.eq(&account_id) {
                // let e = json!({state.ft_contract});
                let e = json!({
                    "ft_contract": state.ft_contract,
                    "total_supply": WrappedBalance::from(state.total_supply),
                    "token_name": state.token_name,
                    "symbol": state.symbol,
                    "icon": state.icon,
                    "reference": state.reference,
                    "reference_hash": state.reference_hash,
                    "decimals": state.decimals,
        
                    "ft_deployer": state.ft_deployer,
                    "creator": state.creator,
        
                    "initial_release": WrappedBalance::from(state.initial_release),
                    "vesting_start_time": WrappedTimestamp::from(state.vesting_start_time),
                    "vesting_end_time": WrappedTimestamp::from(state.vesting_end_time),
                    "vesting_interval": WrappedDuration::from(state.vesting_interval),
                    "treasury_allocation": WrappedBalance::from(state.treasury_allocation),
        
                    "ft_contract_deployed": state.ft_contract_deployed,
                    "deployer_contract_deployed": state.deployer_contract_deployed,
                    "ft_issued": state.ft_issued,
                    "allocation_initialized": state.allocation_initialized,
                });
                result.as_array_mut().unwrap().push(e);
            }
        }

        return result;
    }

    pub fn list_all_tokens(&self) -> Value {
        assert!(env::state_exists(), "The contract is not initialized");

        let token_list = self.tokens.keys_as_vector();
        let mut result: Value = json!([]);

        for token in token_list.iter() {
            let state = self.tokens.get(&token).unwrap_or_default();
            let e = json!({
                "ft_contract": state.ft_contract,
                "total_supply": WrappedBalance::from(state.total_supply),
                "token_name": state.token_name,
                "symbol": state.symbol,
                "icon": state.icon,
                "reference": state.reference,
                "reference_hash": state.reference_hash,
                "decimals": state.decimals,
    
                "ft_deployer": state.ft_deployer,
                "creator": state.creator,
    
                "initial_release": WrappedBalance::from(state.initial_release),
                "vesting_start_time": WrappedTimestamp::from(state.vesting_start_time),
                "vesting_end_time": WrappedTimestamp::from(state.vesting_end_time),
                "vesting_interval": WrappedDuration::from(state.vesting_interval),
                "treasury_allocation": WrappedBalance::from(state.treasury_allocation),
    
                "ft_contract_deployed": state.ft_contract_deployed,
                "deployer_contract_deployed": state.deployer_contract_deployed,
                "ft_issued": state.ft_issued,
                "allocation_initialized": state.allocation_initialized,
            });
            result.as_array_mut().unwrap().push(e);
        }

        return result;
    }

    pub fn list_all_token_contracts(self) -> Value {
        assert!(env::state_exists(), "The contract is not initialized");

        let token_list = self.tokens.keys_as_vector();
        let mut result: Value = json!([]);

        for token in token_list.iter() {
            result.as_array_mut().unwrap().push(json!(token));
        }
        return result;
    }

    pub fn list_token_states(&self, token_contracts: Vec<AccountId>) -> Value {
        assert!(env::state_exists(), "The contract is not initialized");
        let mut result: Value = json!([]);
        for token in token_contracts.iter() {
            let state = self.tokens.get(&token).unwrap_or_default();
            assert!(
                state.vesting_end_time != 0 && state.total_supply != 0,
                "Token is not registered",
            );
            result.as_array_mut().unwrap().push(json!({
                "ft_contract": state.ft_contract,
                "total_supply": WrappedBalance::from(state.total_supply),
                "token_name": state.token_name,
                "symbol": state.symbol,
                "icon": state.icon,
                "reference": state.reference,
                "reference_hash": state.reference_hash,
                "decimals": state.decimals,
    
                "ft_deployer": state.ft_deployer,
                "creator": state.creator,
    
                "initial_release": WrappedBalance::from(state.initial_release),
                "vesting_start_time": WrappedTimestamp::from(state.vesting_start_time),
                "vesting_end_time": WrappedTimestamp::from(state.vesting_end_time),
                "vesting_interval": WrappedDuration::from(state.vesting_interval),
                "treasury_allocation": WrappedBalance::from(state.treasury_allocation),
    
                "ft_contract_deployed": state.ft_contract_deployed,
                "deployer_contract_deployed": state.deployer_contract_deployed,
                "ft_issued": state.ft_issued,
                "allocation_initialized": state.allocation_initialized,
            }));
        }
        return result;
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