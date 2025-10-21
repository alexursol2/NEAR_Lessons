use near_sdk::{env, near, AccountId, require};

// 1. Don't create many structs & 2. Use gas-efficient data types
#[near(contract_state)]
pub struct Contract {
    // ✅ Uses parallel vectors instead of a large struct (Optimization 1)
    // ✅ Uses efficient fixed-size types (Optimization 2)
    listing_owners: Vec<AccountId>, // AccountId is highly efficient
    listing_prices: Vec<u64>,       // u64 instead of u128 (50% storage saving)
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            listing_owners: Vec::new(),
            listing_prices: Vec::new(),
        }
    }
}

#[near]
impl Contract {
    // 5. Batching operations (combined with 3 & 4)
    pub fn create_listings(&mut self, prices: Vec<u64>) {
        // 4. Early returns (Check length FIRST - Fail Fast)
        require!(!prices.is_empty(), "Price list cannot be empty.");

        // 3. Caching Environment Calls (Call once, reuse in the loop)
        let caller = env::predecessor_account_id();
        
        // 5. Process all in one transaction (Avoids fixed overhead per item)
        for price in prices.into_iter() {
            // 4. Early return / Validation check for each element
            require!(price > 0, "Price must be positive."); 

            // ✅ No env calls inside the loop (Optimization 3 maintained)
            self.listing_owners.push(caller.clone());
            self.listing_prices.push(price);
        }
    }
}