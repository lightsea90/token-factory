use crate::*;

#[near_bindgen]
impl TokenFactory {
    pub fn reset(&mut self) {
        assert!(env::state_exists(), "The contract is not initialized");
        self.assert_admin();
        self.tokens.clear();
    }

    pub fn unregister(&mut self, ft_contract: AccountId) {
        assert!(env::state_exists(), "The contract is not initialized");
        self.assert_admin();
        self.tokens.remove(&ft_contract);
    }

    pub fn clear_metadata(&mut self, ft_contract: AccountId) {
        assert!(env::state_exists(), "The contract is not initialized");
        self.assert_admin();
        let mut token = self.tokens.remove(&ft_contract).unwrap();
        token.ft_metadata = None;
        token.allocations.clear();
        self.tokens.insert(&ft_contract, &token);
    }

    fn assert_admin(&self) {
        assert!(
            self.admins.contains(&env::predecessor_account_id()),
            "Function called not from the contract admin",
        );
    }

    fn assert_owner_id(&self) {
        assert!(
            env::predecessor_account_id() == self.owner_id,
            "Function called not from the contract owner",
        );
    }

    pub fn add_admin(&mut self, account_id: AccountId) {
        self.assert_owner_id();
        self.admins.insert(&account_id);
    }

    pub fn remove_admin(&mut self, account_id: AccountId) {
        self.assert_owner_id();
        self.admins.remove(&account_id);
    }

    //Update user_tokens_map for existing tokens
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let old_state: TokenFactory = env::state_read().expect("failed");
        let new_state = Self {
            owner_id: old_state.owner_id,
            admins: old_state.admins,
            tokens: old_state.tokens,
            user_tokens_map: LookupMap::new(b"usertokens".to_vec()),
        };

        new_state
    }

    pub fn migrate_data(&mut self) {
        self.assert_owner_id();
        for (token_id, state) in self.tokens.to_vec() {
            //Add allocators to list
            for (allocator, _) in state.allocations.to_vec() {
                self.internal_add_user_token(allocator, token_id.clone());
            }
        }
    }

    pub fn internal_add_user_token(&mut self, account_id: AccountId, token_id: TokenId) {
        let mut tokens = self
            .user_tokens_map
            .get(&account_id)
            .unwrap_or(UnorderedSet::new(account_id.as_bytes()));

        tokens.insert(&token_id);
        self.user_tokens_map.insert(&account_id, &tokens);
    }
}
