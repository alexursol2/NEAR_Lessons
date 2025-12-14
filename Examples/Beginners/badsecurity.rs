use near_sdk::{env, near, AccountId, require};
use near_sdk::collections::UnorderedMap;

// ❌ The Insecure Contract (Anti-Patterns)
#[near(contract_state)]
pub struct Contract {
    pub owner: AccountId,
    // Using u32 directly
    pub balances: UnorderedMap<AccountId, u32>,
    pub total_supply: u32,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            owner: env::predecessor_account_id(),
            balances: UnorderedMap::new(b"b"), 
            total_supply: 100,
        }
    }
}

#[near]
impl Contract {
    // ❌ Anti-Pattern 1: No access control
    pub fn set_owner(&mut self, new_owner: AccountId) {
        self.owner = new_owner;
    }

    // ❌ Anti-Pattern 2: Potential Panic
    pub fn withdraw(&mut self, amount: u32) {
        let caller = env::predecessor_account_id();
        let current_balance = self.balances.get(&caller).unwrap_or(0);
        
        self.balances.insert(&caller, &(current_balance - amount)); 
    }
    
    // ❌ Anti-Pattern 3: Integer Overflow
    pub fn add_supply_unsafe(&mut self, amount: u32) {
        self.total_supply = self.total_supply + amount;
    }
}