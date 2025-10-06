use near_sdk::{Timestamp, near, env, NearToken, PanicOnDefault};

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {
    total_deposited: NearToken,
    auction_end_time: Timestamp,  
    minimum_deposit: NearToken,
}

#[near]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            total_deposited: NearToken::from_yoctonear(0),
            auction_end_time: 1754582400000000000, // August 5, 2025
            minimum_deposit: NearToken::from_near(1),
        }
    }

    // This function CAN accept NEAR payments
    #[payable]
    pub fn buy_item(&mut self) {
        let deposit = env::attached_deposit();
        if deposit < self.minimum_deposit {
            panic!("Insufficient deposit. Minimum: {} NEAR", self.minimum_deposit.as_near());
        }
        
        // Fix: Add the yoctoNEAR values, then create a new NearToken
        self.total_deposited = self.total_deposited.saturating_add(deposit);

        // Call private function internally
        self.update_auction_status();
    }
    
    // This function CANNOT accept NEAR payments  
    pub fn is_auction_ended(&self) -> bool {
        let current_time = env::block_timestamp();
        current_time >= self.auction_end_time
    }

    // Only this contract can call this function
    #[private]
    pub fn update_auction_status(&mut self) {
        // Internal logic that shouldn't be called externally
        if self.is_auction_ended() {
            self.auction_end_time += 1440000000000000; // + 1 day
        }
    }
}
