use near_sdk::{env, near, AccountId, require};
use near_sdk::store::IterableMap;

// ✅ The Secure Contract (Optimized)
#[near(contract_state)]
pub struct Contract {
    pub owner: AccountId,
    pub balances: IterableMap<AccountId, u32>,
    pub total_supply: u32,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            owner: env::predecessor_account_id(),
            balances: IterableMap::new(b"b"), 
            total_supply: 100,
        }
    }
}

#[near]
impl Contract {
    // ✅ Rule 1: Access Control
    pub fn set_owner(&mut self, new_owner: AccountId) {
        require!(
            env::predecessor_account_id() == self.owner,
            "Only the current owner can set a new owner."
        );
        self.owner = new_owner;
    }

    // ✅ Rule 2: Preventing Panics (Fail Fast)
    pub fn withdraw(&mut self, amount: u32) {
        let caller = env::predecessor_account_id();
        
        // FIX 1: Dereference (*) the result and provide a reference (&0) to unwrap
        let current_balance = *self.balances.get(&caller).unwrap_or(&0);
        
        require!(current_balance >= amount, "Insufficient balance to withdraw.");
        
        // FIX 2: Remove the borrow (&) from caller. insert() needs ownership.
        self.balances.insert(caller, current_balance - amount);
    }
    
    // ✅ Rule 3: Integer Overflow (Use safe math)
    pub fn add_supply_safe(&mut self, amount: u32) {
        self.total_supply = self.total_supply.saturating_add(amount);
    }
}