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

        let state = self
            .tokens
            .get(&ft_contract)
            .expect("ft_contract not found!");

        for allocator in state.allocations.keys_as_vector().iter() {
            let mut user_tokens = self
                .user_token_map
                .get(&allocator)
                .expect("user_tokens not found");
            user_tokens.remove(&ft_contract);
            self.user_token_map.insert(&allocator, &user_tokens);
        }
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
        let old_state: OldTokenFactory = env::state_read().expect("failed");
        let new_state = Self {
            owner_id: old_state.owner_id,
            admins: old_state.admins,
            tokens: old_state.tokens,
            user_token_map: LookupMap::new(b"tokenmap".to_vec()),
        };

        new_state
    }

    pub fn migrate_data(&mut self, from_index: u64, limit: u64) {
        self.assert_admin();
        let contract_ids = self.tokens.keys_as_vector();
        for index in from_index..std::cmp::min(from_index + limit, contract_ids.len()) {
            let contract_id = contract_ids.get(index).unwrap();
            let state = self.tokens.get(&contract_id).unwrap();
            for (allocator, _) in state.allocations.to_vec() {
                let mut tokens = self
                    .user_token_map
                    .get(&allocator)
                    .unwrap_or(UnorderedSet::new(
                        format!("{}#{}", allocator, env::block_timestamp()).as_bytes(),
                    ));

                tokens.insert(&contract_id);
                env::log(
                    format!("account_id: {:#?} tokens {:#?}", allocator, tokens.to_vec())
                        .as_bytes(),
                );
                self.user_token_map.insert(&allocator, &tokens);
            }
        }
    }

    pub fn internal_add_user_token(&mut self, account_id: AccountId, token_id: TokenId) {
        let mut tokens = self
            .user_token_map
            .get(&account_id)
            .unwrap_or(UnorderedSet::new(
                format!("{}#{}", account_id, env::block_timestamp()).as_bytes(),
            ));

        tokens.insert(&token_id);
        env::log(
            format!(
                "account_id: {:#?} tokens {:#?}",
                account_id,
                tokens.to_vec()
            )
            .as_bytes(),
        );
        self.user_token_map.insert(&account_id, &tokens);
    }
}
