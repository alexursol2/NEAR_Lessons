use near_sdk::{log, near, env, AccountId, serde_json};
use near_sdk::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CounterUpdateEvent {
    old_value: u32,
    new_value: u32,
    updated_by: AccountId,
}

#[near(contract_state)]
pub struct Contract {
    counter: u32,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            counter: 0,
        }
    }
}

#[near]
impl Contract {
    // Simple function with basic logging
    pub fn increment(&mut self) {
        self.counter += 1;
        log!("Counter updated to: {}", self.counter);
    }
    
    // Function with structured event
    pub fn update_counter(&mut self) {
        let old_value = self.counter;
        self.counter += 1;
        
        let event = CounterUpdateEvent {
            old_value,
            new_value: self.counter,
            updated_by: env::predecessor_account_id(),
        };
        
        log!("EVENT_JSON:{}", serde_json::to_string(&event).unwrap());
    }
}
