use near_sdk::{env, near, AccountId};

// ❌ WRONG: Separate, complex struct (Anti-pattern 1)
pub struct ListingData {
    owner: String,       // ❌ String instead of AccountId (Anti-pattern 2)
    price: u128,         // ❌ u128 instead of u64 (Anti-pattern 2)
    // Removed is_active
}

// ❌ WRONG: Contract stores complex structs (Anti-pattern 1)
#[near(contract_state)]
pub struct Contract {
    listings: Vector<ListingData>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            listings: Vector::new(b"l"),
        }
    }
}

#[near]
impl Contract {
    // ❌ WRONG FUNCTION 1: Demonstrates Anti-patterns 3, 4, 5 with dual inputs
    pub fn create_listings_owner_only(&mut self, prices: Vec<u128>) {
        // No input validation here (Anti-pattern 4)
        
        for price in prices {
            // ❌ Repeated Environment Calls INSIDE the loop (Anti-pattern 3)
            let caller_id = env::predecessor_account_id().to_string(); 

            // ❌ No Batching: Pushing complex struct elements one by one
            self.listings.push(&ListingData {
                owner: caller_id.clone(), // Clone() is also costly
                price: 0,
            });
        }
    }
    
    // ❌ WRONG FUNCTION 2: Demonstrates Anti-pattern 3 (Wasting gas on fixed data)
    pub fn create_listings_price_only(&mut self, prices: Vec<u128>) {
        // No input validation here (Anti-pattern 4)
        
        for price in prices {            
            // ❌ Wastes gas repeating the call and clone inside the loop
            self.listings.push(&ListingData {
                owner: 0,
                price,
            });

            // ❌ Late Validation
            if price == 0 {
                break;
            }
        }
    }
}