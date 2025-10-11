use near_sdk::{AccountId, PanicOnDefault, near, env};

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {
    owner: AccountId,          // The account that owns this contract
    current_price: u128,       // Current price from oracle
}

#[near]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            owner: env::predecessor_account_id(),  // Set deployer as owner
            current_price: 0,                 // Initialize price to 0
        }
    }

    pub fn get_contract_info(&self) -> String {
        let contract_id = env::current_account_id();  // Get this contract's address
        format!("This contract is deployed at: {}", contract_id)
    }

    pub fn call_by_owner(&mut self, price: u128) {
        let caller = env::predecessor_account_id();  // Get who called this function directly
        if caller != owner { // Check if caller is trusted owner
            panic!("Only owner can call this");
        }
        self.current_price = price;  // Update the price from oracle
    }
}
