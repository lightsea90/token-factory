
// let payout_promise = Promise::new(self.ft_contract_name.clone()).function_call(
//     b"ft_transfer".to_vec(), 
//     json!({
//         "receiver_id": account_id,
//         "amount": WrappedBalance::from(amount_to_redeem),
//     }).to_string().as_bytes().to_vec(), 
//     1, DEFAULT_GAS_TO_PAY,
// );

pub mod token_factory {
    use near_sdk::{env, Promise};
    use near_sdk::{AccountId, Balance, Timestamp, Duration, Gas};
    use near_sdk::json_types::{U128, WrappedBalance, WrappedDuration};
    use near_sdk::serde_json::json;
    const DEFAULT_GAS_TO_PAY: Gas = 20_000_000_000_000;

    const FT_WASM_CODE: &[u8] = include_bytes!("../../out/fungible_token.wasm");

    pub fn deploy_ft(
        contract_name: AccountId, 
        owner_id: AccountId,
        total_supply: WrappedBalance,
        token_name: String,
        symbol: String,
        decimals: u8,
    ) -> Promise {
        let deploy = Promise::new(contract_name.parse().unwrap())
            .create_account()
            .add_full_access_key(env::signer_account_pk())
            .transfer(3_000_000_000_000_000_000_000_000) // 3e24yN, 3N
            .deploy_contract(FT_WASM_CODE.to_vec());
        let create = Promise::new(contract_name.parse().unwrap())
            .function_call(
                b"new".to_vec(),
                json!({
                    "owner_id": owner_id,
                    "total_supply": total_supply,
                    "metadata": {
                        "spec": "ft-1.0.0",
                        "name": token_name,
                        "symbol": symbol,
                        "decimals": decimals,
                    }
                }).to_string().as_bytes().to_vec(),
                0, DEFAULT_GAS_TO_PAY,
            );
        return deploy.then(create);
    }
    // pub fn apply_to_vec(vec: Vec<i32>, function: fn (i32) -> i32) -> Vec<i32> {
    //     return vec.iter().map(|elem: &i32| function(*elem)).collect()
    // }
}
