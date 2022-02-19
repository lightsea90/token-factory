use crate::*;

#[near_bindgen]
impl TokenFactory {
    pub fn get_token_state(self, ft_contract: AccountId) -> WrappedState {
        let token = self.tokens.get(&ft_contract).unwrap_or_default();
        assert!(
            token.ft_contract != String::from("__default_value__"),
            "Token is not registered",
        );
        WrappedState::from(token)
    }

    pub fn list_my_tokens(&self, account_id: AccountId) -> Vec<WrappedState> {
        assert!(env::state_exists(), "The contract is not initialized");

        let token_list = self.tokens.keys_as_vector();
        let mut result = vec![];

        for token in token_list.iter() {
            let state = self.tokens.get(&token).unwrap_or_default();
            if state.creator.eq(&account_id) {
                // let e = json!({state.ft_contract});
                let e = WrappedState::from(state);
                result.push(e);
            }
        }

        return result;
    }

    pub fn list_all_tokens(&self) -> Vec<WrappedState> {
        assert!(env::state_exists(), "The contract is not initialized");

        let token_list = self.tokens.keys_as_vector();
        let mut result = vec![];

        for token in token_list.iter() {
            let state = self.tokens.get(&token).unwrap_or_default();
            // let e = json!({state.ft_contract});
            let e = WrappedState::from(state);
            result.push(e);
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

    pub fn list_token_states(&self, token_contracts: Vec<AccountId>) -> Vec<WrappedState> {
        assert!(env::state_exists(), "The contract is not initialized");
        let mut result = vec![];
        for token in token_contracts.iter() {
            let state = self.tokens.get(&token).unwrap_or_default();

            result.push(WrappedState::from(state));
        }
        return result;
    }
}

