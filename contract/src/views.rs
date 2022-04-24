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

    pub fn list_my_tokens(&self, account_id: AccountId) -> Vec<TokenId> {
        assert!(env::state_exists(), "The contract is not initialized");

        if let Some(token_ids) = self.user_tokens_map.get(&account_id) {
            return token_ids.to_vec();
        }
        vec![]
    }

    pub fn list_token_contracts(&self, from_index: u64, limit: u64) -> Vec<WrappedState> {
        assert!(env::state_exists(), "The contract is not initialized");

        let token_list = self.tokens.keys_as_vector();
        (from_index..std::cmp::min(from_index + limit, token_list.len()))
            .map(|index| {
                let key = token_list.get(index).unwrap();
                let state = self.tokens.get(&key).unwrap();
                WrappedState::from(state)
            })
            .collect()
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
