use near_sdk::near;

#[near(contract_state)]
pub struct Contract{
    counter: u32,
    mindays: u64,
    tier: String,
}

impl Default for Contract {
    fn default() -> Self{
        Self{
            counter: 0,
            mindays: 5,
            tier: "Bronze".to_string(),
        }
    }
}

#[near]
impl Contract{
    pub fn set_staking_tier(&mut self, amount: u64) -> String {
        // == checks if two values are exactly equal.
        // != checks if two values are different.
        // > checks if left value is bigger than right value.
        // < checks if left value is smaller than right value.
        // >= checks if left value is bigger than or equal to right value.
        // <= checks if left value is smaller than or equal to right value.
        if amount >= 1000 {
            self.tier = "Diamond".to_string();
        } else if amount >= 500 {
            self.tier = "Gold".to_string();
        } else if amount >= 100{
            self.tier = "Silver".to_string();
        } else {
            self.tier = "Bronze".to_string();
        }
        self.tier.clone()
    }
    
    pub fn validate_transaction(&mut self, days: u64, is_verified: bool) {
        // AND operator (&&) - Both conditions must be true
        if days >= self.mindays && is_verified {
            self.counter += 1;
        }
        
        // OR operator (||) - At least one condition must be true  
        if days >= self.mindays || is_verified {
            self.counter += 1;
        }
        
        // Combining AND and OR with parentheses
        if days >= 100 || (is_verified && self.mindays == 5) {
            self.counter += 1;
        }
    }

}
