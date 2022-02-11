use crate::*;

#[near_bindgen]
impl TokenFactory {
    pub fn get_token_state(self, ft_contract: AccountId) -> Value {
        let token = self.tokens.get(&ft_contract).unwrap_or_default();
        self.assert_invalid_allocations(ft_contract);

        return json!({
            "ft_contract": token.ft_contract,
            "ft_metadata": token.ft_metadata
                            .expect("Not found ft_metadata"),
            "allocations": token.allocations.to_vec(),

            "ft_deployer": token.ft_deployer,
            "creator": token.creator,

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
                    "ft_metadata": state.ft_metadata
                                    .expect("Not found ft_metadata"),
                    "allocations": state.allocations.to_vec(),

                    "ft_deployer": state.ft_deployer,
                    "creator": state.creator,

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
                "ft_metadata": state.ft_metadata
                                .expect("Not found ft_metadata"),
                "allocations": state.allocations.to_vec(),

                "ft_deployer": state.ft_deployer,
                "creator": state.creator,

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

            result.as_array_mut().unwrap().push(json!({
                "ft_contract": state.ft_contract,
                "ft_deployer": state.ft_deployer,
                "creator": state.creator,

                "ft_deployer": state.ft_deployer,
                "creator": state.creator,

                "ft_contract_deployed": state.ft_contract_deployed,
                "deployer_contract_deployed": state.deployer_contract_deployed,
                "ft_issued": state.ft_issued,
                "allocation_initialized": state.allocation_initialized,
            }));
        }
        return result;
    }
}

