use crate::*;

pub type TokenAllocationInput = HashMap<AccountId, WrappedTokenAllocation>;

#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct WrappedTokenAllocation {
    pub allocated_percent: u64,
    pub initial_release: u64,
    pub vesting_start_time: WrappedTimestamp,
    pub vesting_end_time: WrappedTimestamp,
    pub vesting_interval: WrappedDuration,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenAllocation {
    pub allocated_percent: u64, // Decimal: 2
    pub initial_release: u64,
    pub vesting_start_time: Timestamp,
    pub vesting_end_time: Timestamp,
    pub vesting_interval: Duration,
    pub claimed: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FTMetadata {
    pub total_supply: Balance,
    pub token_name: String,
    pub symbol: String,
    pub icon: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<Base64VecU8>,
    pub decimals: u8,
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
    pub ft_contract: AccountId,
    pub ft_metadata: Option<FTMetadata>,

    // creator and deployer
    pub ft_deployer: AccountId,
    pub creator: AccountId,

    // Multiple tokenomics
    pub allocations: UnorderedMap<AccountId, TokenAllocation>, // => None after deploy token

    // issuance states
    pub ft_contract_deployed: u8,
    pub deployer_contract_deployed: u8,
    pub ft_issued: u8,
    pub allocation_initialized: u8,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct WrappedState {
    // token info
    ft_contract: AccountId,
    pub ft_metadata: Option<WrappedFTMetadata>,

    // creator and deployer
    ft_deployer: AccountId,
    creator: AccountId,

    // Multiple tokenomics
    allocations: Vec<(AccountId, TokenAllocation)>, // => None after deploy token

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
