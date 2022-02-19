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
}
