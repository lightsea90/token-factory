/*
Functions:

 */

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, UnorderedSet};
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
const MAX_SUPPLY_PERCENT: u64 = 10000; // Decimal: 2

pub type TokenAllocationInput = HashMap<AccountId, WrappedTokenAllocation>;

#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct WrappedTokenAllocation {
    allocated_percent: u64,
    initial_release: u64,
    vesting_start_time: WrappedTimestamp,
    vesting_end_time: WrappedTimestamp,
    vesting_interval: WrappedDuration,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenAllocation {
    allocated_percent: u64, // Decimal: 2
    initial_release: u64, 
    vesting_start_time: Timestamp,
    vesting_end_time: Timestamp,
    vesting_interval: Duration,
    claimed: u64, 
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FTMetadata {
    total_supply: Balance,
    token_name: String,
    symbol: String,
    icon: Option<String>,
    reference: Option<String>,
    reference_hash: Option<Base64VecU8>,
    decimals: u8,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct WrappedFTMetadata {
    total_supply: WrappedBalance,
    token_name: String,
    symbol: String,
    icon: Option<String>,
    reference: Option<String>,
    reference_hash: Option<Base64VecU8>,
    decimals: u8,
}

impl From<FTMetadata> for WrappedFTMetadata {
   fn from(ft_metadata: FTMetadata) -> Self {
       WrappedFTMetadata {
        total_supply: WrappedBalance::from(ft_metadata.total_supply),
        token_name: ft_metadata.token_name,
        symbol: ft_metadata.symbol,
        icon: ft_metadata.icon,
        reference: ft_metadata.reference,
        reference_hash: ft_metadata.reference_hash,
        decimals: ft_metadata.decimals,
       }
   }
}


#[derive(BorshDeserialize, BorshSerialize)]
pub struct State {
    // token info
    ft_contract: AccountId,
    ft_metadata: Option<FTMetadata>,

    // creator and deployer
    ft_deployer: AccountId,
    creator: AccountId,

    // Multiple tokenomics
    allocations: UnorderedMap<AccountId, TokenAllocation>, // => None after deploy token

    // issuance states
    ft_contract_deployed: u8,
    deployer_contract_deployed: u8,
    ft_issued: u8,
    allocation_initialized: u8,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct WrappedState {
    // token info
    ft_contract: AccountId,
    ft_metadata: Option<WrappedFTMetadata>,

    // creator and deployer
    ft_deployer: AccountId,
    creator: AccountId,

    // Multiple tokenomics
    allocations: Vec<(AccountId, TokenAllocation)>,// => None after deploy token

    // issuance states
    ft_contract_deployed: u8,
    deployer_contract_deployed: u8,
    ft_issued: u8,
    allocation_initialized: u8,
}

impl From<State> for WrappedState {
   fn from(state: State) -> Self {
       WrappedState {
        ft_contract: state.ft_contract,
        ft_metadata: match state.ft_metadata {
            None => None,
            Some(d) => Some(WrappedFTMetadata::from(d)),
        },
        // Some(WrappedFTMetadata::from(state.ft_metadata.expect("ft metadata not found!"))),

        // creator and deployer
        ft_deployer: state.ft_deployer,
        creator: state.creator,

        // Multiple tokenomics
        allocations: state.allocations.to_vec(), // => None after deploy token

        // issuance states
        ft_contract_deployed: state.ft_contract_deployed,
        deployer_contract_deployed: state.deployer_contract_deployed,
        ft_issued: state.ft_issued,
        allocation_initialized: state.allocation_initialized,
       }
   }
}

impl Default for State {
    fn default() -> Self {
        let default_string_value = String::from("__default_value__");
        Self {
            ft_contract: default_string_value.clone(),
            ft_metadata: Some(FTMetadata {
                total_supply: 0,
                token_name: default_string_value.clone(),
                symbol: default_string_value.clone(),
                icon: Some(default_string_value.clone()),
                reference: None,
                reference_hash: None,
                decimals: 0,
            }),

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
    owner_id: AccountId,
    admins: UnorderedSet<AccountId>,
    tokens: UnorderedMap<AccountId, State>,
}

#[near_bindgen]
impl TokenFactory {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        return Self {
            owner_id: String::from(owner_id),
            admins: UnorderedSet::new(b"admins".to_vec()),
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

        let mut allocation_prefix = Vec::with_capacity(33);
        // Adding unique prefix.
        allocation_prefix.push(b'a');
        // Adding the hash of the account_id (key of the outer map) to the prefix.
        // This is needed to differentiate across accounts.
        allocation_prefix.extend(
            env::sha256(format!("{}@{}", ft_contract, env::block_timestamp()).as_bytes())
        );
        
        let mut state_allocations = UnorderedMap::new(allocation_prefix);

        let mut treasury_exist = false;

        for (account_id, alloc) in &allocations {
            let a = TokenAllocation {
                allocated_percent: alloc.allocated_percent.into(),
                initial_release: alloc.initial_release,
                vesting_start_time: alloc.vesting_start_time.into(),
                vesting_end_time: alloc.vesting_end_time.into(),
                vesting_interval: alloc.vesting_interval.into(),
                claimed: 0,
            };

            if account_id == TOKENHUB_TREASURY && a.allocated_percent > 0 {
                treasury_exist = true;
            }

            self.assert_invalid_allocation(a.clone());

            let total_allocs: u64 = state_allocations 
                .values()
                .map(|v: TokenAllocation| v.allocated_percent)
                .sum();

            assert!(
                total_allocs + a.allocated_percent <= MAX_SUPPLY_PERCENT,
                "Total allocations is greater than total supply"
            );
            state_allocations.insert(account_id, &a);
        }

        assert!(
            treasury_exist, 
            "Treasury allocation must exist!"
        );
        
        let token = State {
            ft_contract: ft_contract.clone(),
            ft_metadata: Some(FTMetadata {
                total_supply: total_supply.into(),
                token_name,
                symbol,
                icon,
                reference,
                reference_hash,
                decimals,
                }),

            ft_deployer: deployer_contract,
            creator: env::signer_account_id(),

            allocations: state_allocations, 

            ft_contract_deployed: 0,
            deployer_contract_deployed: 0,
            ft_issued: 0,
            allocation_initialized: 0,
        };

        assert!(
            token.ft_metadata
                .as_ref()
                .expect("Not found ft_metadata")
                .total_supply > 0,
            "total_supply must be greater than 0",
        );
        assert!(
            env::is_valid_account_id(token.ft_contract.as_bytes()),
            "ft_contract is not valid",
        );
        assert!(
            env::is_valid_account_id(token.ft_deployer.as_bytes()),
            "ft_deployer is not valid",
        );
        if let Some(_) = self.tokens.get(&token.ft_contract) {
            std::panic!("ft_contract already registered");
        }

        // TODO: validate more?
        self.tokens.insert(&ft_contract, &token);
    }

    pub fn create_ft_contract(&mut self, ft_contract: AccountId) -> Promise {
        let token = self.tokens.get(&ft_contract.clone()).unwrap_or_default();
        self.assert_invalid_allocations(ft_contract.clone());
        self.assert_creator(token.creator);

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
        self.assert_creator(token.creator);

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
        self.assert_creator(token.creator);

        let ft_metadata = token.ft_metadata
                            .expect("Not found ft_metadata");
        

        return Promise::new(ft_contract.parse().unwrap())
            .function_call(
                b"new".to_vec(),
                json!({
                    "owner_id": token.ft_deployer,
                    "total_supply": WrappedBalance::from(
                            ft_metadata
                            .total_supply
                    ),
                    "metadata": {
                        "spec": "ft-1.0.0",
                        "name": ft_metadata.token_name,
                        "symbol": ft_metadata.symbol,
                        "icon": ft_metadata.icon,
                        "reference": ft_metadata.reference,
                        "reference_hash": ft_metadata.reference,
                        "decimals": ft_metadata.decimals,
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
        self.assert_creator(token.creator);

        let mut allocations: HashMap<AccountId, WrappedTokenAllocation> = HashMap::new();

        for k in token.allocations.keys() {
            allocations.insert(
                k.clone(),
                token.allocations
                .get(&k.clone())
                .map(|v| WrappedTokenAllocation {
                        allocated_percent: v.allocated_percent,
                        initial_release: v.initial_release,
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
                            .ft_metadata
                            .expect("Not found ft_metadata")
                            .total_supply
                    ),
                    "allocations": allocations
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

        env::log(format!(
            "total supply: {}, allocation length: {}",
            token.ft_metadata.as_ref().expect("fadfa").total_supply,
            token.allocations.values_as_vector().len(),
        ).as_bytes());

        assert!(
            token
                .ft_metadata
                .as_ref()
                .expect("Not found ft_metadata")
                .total_supply > 0 
            && token
                .allocations
                .values_as_vector()
                .len() > 0,
            "Token is not register"
        );

        let total_allocations: u64 = token.allocations 
                .values()
                .map(|a| {
                    self.assert_invalid_allocation(a.clone());
                    a.allocated_percent
                })
                .sum();
        
        assert!(
            total_allocations == MAX_SUPPLY_PERCENT,
            "Total allocations is not equal to total supply"
        );
    }

    fn assert_invalid_allocation(
        &self, 
        allocation: TokenAllocation 
    ) {
        //TODO: Allocation > 0
            assert!(
                allocation.allocated_percent >= allocation.initial_release + allocation.claimed,
                "Allocation is smaller than the total claimable",
            );
            assert!(
                allocation.vesting_interval <= allocation.vesting_end_time - allocation.vesting_start_time,
                "Vesting interval is larger than vesting time",
            );
    }

    fn assert_creator(
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
