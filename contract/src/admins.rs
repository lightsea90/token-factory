use crate::*;

#[near_bindgen]
impl TokenFactory {
    pub fn reset(&mut self) {
        assert!(env::state_exists(), "The contract is not initialized");
        assert!(
            env::current_account_id() == env::signer_account_id(),
            "Function called not from the contract owner itself",
        );
        self.tokens.clear();
    }

    pub fn unregister(&mut self, ft_contract: AccountId) {
        assert!(env::state_exists(), "The contract is not initialized");
        assert!(
            env::current_account_id() == env::signer_account_id(),
            "Function called not from the contract owner itself",
        );
        self.tokens.remove(&ft_contract);
    }
}
