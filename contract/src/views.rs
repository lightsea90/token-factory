use crate::*;

#[near_bindgen]
impl TokenFactory {
    pub fn get_token_state(self, ft_contract: AccountId) -> Value {
        let token = self.tokens.get(&ft_contract).unwrap_or_default();
        self.assert_invalid_allocations(ft_contract);
        // assert!(
        //     token.vesting_end_time != 0 && token.total_supply != Some(0),
        //     "Token is not registered",
        // );
        return json!({
            "ft_contract": token.ft_contract,
            "total_supply": WrappedBalance::from(
                token
                    .total_supply
                    .expect("Total supply is None !")
            ),
            "token_name": token.token_name,
            "symbol": token.symbol,
            "decimals": token.decimals,

            "ft_deployer": token.ft_deployer,
            "creator": token.creator,

            // "initial_release": WrappedBalance::from(token.initial_release),
            // "vesting_start_time": WrappedTimestamp::from(token.vesting_start_time),
            // "vesting_end_time": WrappedTimestamp::from(token.vesting_end_time),
            // "vesting_interval": WrappedDuration::from(token.vesting_interval),
            // "treasury_allocation": WrappedBalance::from(token.treasury_allocation),

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
                    "total_supply": WrappedBalance::from(
                        state
                            .total_supply
                            .expect("Total supply is None !")
                    ),
                    "token_name": state.token_name,
                    "symbol": state.symbol,
                    "icon": state.icon,
                    "reference": state.reference,
                    "reference_hash": state.reference_hash,
                    "decimals": state.decimals,

                    "ft_deployer": state.ft_deployer,
                    "creator": state.creator,

                    // "initial_release": WrappedBalance::from(state.initial_release),
                    // "vesting_start_time": WrappedTimestamp::from(state.vesting_start_time),
                    // "vesting_end_time": WrappedTimestamp::from(state.vesting_end_time),
                    // "vesting_interval": WrappedDuration::from(state.vesting_interval),
                    // "treasury_allocation": WrappedBalance::from(state.treasury_allocation),

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
                "total_supply": WrappedBalance::from(
                    state
                        .total_supply
                        .expect("Total supply is None !")
                ),
                "token_name": state.token_name,
                "symbol": state.symbol,
                "icon": state.icon,
                "reference": state.reference,
                "reference_hash": state.reference_hash,
                "decimals": state.decimals,

                "ft_deployer": state.ft_deployer,
                "creator": state.creator,

                // "initial_release": WrappedBalance::from(state.initial_release),
                // "vesting_start_time": WrappedTimestamp::from(state.vesting_start_time),
                // "vesting_end_time": WrappedTimestamp::from(state.vesting_end_time),
                // "vesting_interval": WrappedDuration::from(state.vesting_interval),
                // "treasury_allocation": WrappedBalance::from(state.treasury_allocation),

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
            // assert!(
            //     state.vesting_end_time != 0 && state.total_supply != Some(0),
            //     "Token is not registered",
            // );
            result.as_array_mut().unwrap().push(json!({
                "ft_contract": state.ft_contract,
                "total_supply": WrappedBalance::from(
                    state
                        .total_supply
                        .expect("Total supply is None !")
                ),
                "token_name": state.token_name,
                "symbol": state.symbol,
                "icon": state.icon,
                "reference": state.reference,
                "reference_hash": state.reference_hash,
                "decimals": state.decimals,

                "ft_deployer": state.ft_deployer,
                "creator": state.creator,

                // "initial_release": WrappedBalance::from(state.initial_release),
                // "vesting_start_time": WrappedTimestamp::from(state.vesting_start_time),
                // "vesting_end_time": WrappedTimestamp::from(state.vesting_end_time),
                // "vesting_interval": WrappedDuration::from(state.vesting_interval),
                // "treasury_allocation": WrappedBalance::from(state.treasury_allocation),

                "ft_contract_deployed": state.ft_contract_deployed,
                "deployer_contract_deployed": state.deployer_contract_deployed,
                "ft_issued": state.ft_issued,
                "allocation_initialized": state.allocation_initialized,
            }));
        }
        return result;
    }
}

