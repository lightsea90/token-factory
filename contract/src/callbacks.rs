use near_sdk::ext_contract;

use crate::*;

#[ext_contract(ext_self)]
pub trait ExtTokenFactory {
    fn on_ft_contract_deployed(&mut self, ft_contract: AccountId) -> bool;
    fn on_ft_deployer_deployed(&mut self, ft_contract: AccountId) -> bool;
    fn on_token_issued(&mut self, ft_contract: AccountId) -> bool;
    fn on_allocation_init(&mut self, ft_contract: AccountId) -> bool;
}

#[near_bindgen]
impl TokenFactory {
    #[private]
    pub fn on_ft_contract_deployed(&mut self, ft_contract: AccountId) -> bool {
        env::log(format!("promise_result_count = {}", env::promise_results_count()).as_bytes());
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                self.assert_invalid_allocations(ft_contract.clone());
                let mut token = self.tokens.remove(&ft_contract).unwrap_or_default();
                assert!(
                    token.ft_contract_deployed == 0,
                    "State ft_contract_deployed is invalid",
                );
                token.ft_contract_deployed = 1;
                self.tokens.insert(&ft_contract, &token);
                true
            }
            _ => false,
        }
    }

    #[private]
    pub fn on_ft_deployer_deployed(&mut self, ft_contract: AccountId) -> bool {
        env::log(format!("promise_result_count = {}", env::promise_results_count()).as_bytes());
        // format!("fasfas");
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                self.assert_invalid_allocations(ft_contract.clone());
                let mut token = self.tokens.remove(&ft_contract).unwrap_or_default();
                assert!(
                    token.deployer_contract_deployed == 0,
                    "State deployer_contract_deployed is invalid",
                );
                token.deployer_contract_deployed = 1;
                self.tokens.insert(&ft_contract, &token);
                true
            }
            _ => false,
        }
    }

    #[private]
    pub fn on_token_issued(&mut self, ft_contract: AccountId) -> bool {
        env::log(format!("promise_result_count = {}", env::promise_results_count()).as_bytes());
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                self.assert_invalid_allocations(ft_contract.clone());
                let mut token = self.tokens.remove(&ft_contract).unwrap_or_default();
                assert!(token.ft_issued == 0, "State ft_issued is invalid",);
                token.ft_issued = 1;
                self.tokens.insert(&ft_contract, &token);
                true
            }
            _ => false,
        }
    }

    #[private]
    pub fn on_allocation_init(&mut self, ft_contract: AccountId) -> bool {
        env::log(format!("promise_result_count = {}", env::promise_results_count()).as_bytes());
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                self.assert_invalid_allocations(ft_contract.clone());
                let mut token = self.tokens.remove(&ft_contract).unwrap_or_default();
                assert!(
                    token.allocation_initialized == 0,
                    "State allocation_initialized is invalid",
                );
                token.allocation_initialized = 1;
                token.ft_metadata = None;
                token.allocations.clear();
                self.tokens.insert(&ft_contract, &token);
                true
            }
            _ => false,
        }
    }
}

