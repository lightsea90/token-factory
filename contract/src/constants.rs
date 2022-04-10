use crate::*;

pub const DEFAULT_GAS_FEE: Gas = 20_000_000_000_000;
pub const NEAR_DECIMAL: u128 = 1_000_000_000_000_000_000_000_000;
pub const TOKENHUB_TREASURY: &str = "treasury.tokenhub.testnet";
pub const FT_WASM_CODE: &[u8] = include_bytes!("../../static/fungible_token.wasm");
pub const DEPLOYER_WASM_CODE: &[u8] = include_bytes!("../../static/token_deployer.wasm");
pub const EXTRA_BYTES: usize = 10000;
pub const MAX_SUPPLY_PERCENT: u64 = 10000; // Decimal: 2
