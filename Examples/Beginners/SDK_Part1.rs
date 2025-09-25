use near_sdk::{Timestamp, NearToken, near, env};

#[near(contract_state)]
pub struct Contract {
    total_deposited: NearToken,     // Total NEAR tokens deposited
    auction_end_time: Timestamp,    // When auction ends
    minimum_deposit: NearToken,     // Minimum required deposit
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            total_deposited: NearToken::from_yoctonear(0),
            auction_end_time: 1754582400000000000, // August 5, 2025 00:00:00 UTC in nanoseconds
            minimum_deposit: NearToken::from_near(1), // 1 NEAR
        }
    }
}

#[near]
impl Contract {
    pub fn buy_item(&mut self) {
        let deposit = env::attached_deposit(); // Get attached NEAR amount (NearToken)
        
        if deposit < self.minimum_deposit {
            panic!("Insufficient deposit. Minimum: {} NEAR", self.minimum_deposit.as_near());
        }
        
        self.total_deposited = self.total_deposited.saturating_add(deposit); // Add to total
    }

    pub fn is_auction_ended(&self) -> bool {
        let current_time = env::block_timestamp();
        current_time >= self.auction_end_time // Compare timestamps
    }
}
