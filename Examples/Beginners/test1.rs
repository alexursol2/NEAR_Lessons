// Find all our documentation at https://docs.near.org
use near_sdk::{log, near, env, Promise, require, AccountId};

// Define the contract structure
#[near(contract_state)]
#[derive(Default)]
pub struct Counter {
    val: i8,
}

// Implement the contract structure
#[near]
impl Counter {
    #[payable]
    pub fn donate_to_smart_contract(&mut self) {
        // FIX: Explicitly parse "alice.near" into AccountId to resolve type ambiguity (E0283)
        require!(
            env::predecessor_account_id() != "alice.near".parse::<AccountId>().unwrap(), 
            "You are not allowed to donate"
        );
        let contract_id = env::current_account_id();
        let deposit = env::attached_deposit();
        // NOTE: In production code, you might want to transfer to a separate treasury,
        // but transferring to self is valid for absorbing the donation.
        Promise::new(contract_id).transfer(deposit);
    }

    // Public read-only method: Returns the counter value.
    pub fn get_num(&self) -> i8 {
        return self.val;
    }

    // Public method: Increment the counter.
    pub fn increment(&mut self, number: Option<i8>) {
        self.val += number.unwrap_or(1);
        log!("Increased number to {}", self.val);
    }

    // Public method: Decrement the counter.
    pub fn decrement(&mut self, number: Option<i8>) {
        self.val -= number.unwrap_or(1);
        log!("Decreased number to {}", self.val);
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * to run these, the command will be: `cargo test`
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;
    // Import necessary testing utilities
    use near_sdk::test_utils::{VMContextBuilder};
    use near_sdk::{testing_env, AccountId, NearToken};

    // Helper function to get a mocked context for the tests
    fn get_context(predecessor: AccountId, attached_deposit: NearToken) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor)
               .attached_deposit(attached_deposit);
        // Set up the default context for the contract (current account ID, etc.)
        builder.current_account_id("counter_contract.near".parse().unwrap());
        builder
    }

    #[test]
    fn increment() {
        // instantiate a contract variable with the counter at zero
        let mut contract = Counter { val: 0 };
        contract.increment(None);
        assert_eq!(1, contract.get_num());
    }

    #[test]
    fn increment_with_points() {
        // instantiate a contract variable with the counter at zero
        let mut contract = Counter { val: 0 };
        contract.increment(Some(10));
        assert_eq!(10, contract.get_num());
    }

    #[test]
    fn decrement() {
        let mut contract = Counter { val: 0 };
        contract.decrement(None);
        assert_eq!(-1, contract.get_num());
    }

    #[test]
    fn decrement_with_points() {
        // instantiate a contract variable with the counter at zero
        let mut contract = Counter { val: 0 };
        contract.decrement(Some(10));
        assert_eq!(-10, contract.get_num());
    }

    #[test]
    #[should_panic]
    fn panics_on_overflow() {
        let mut contract = Counter { val: 127 };
        contract.increment(None);
    }

    #[test]
    #[should_panic]
    fn panics_on_underflow() {
        let mut contract = Counter { val: -128 };
        contract.decrement(None);
    }

    // --- New tests for donate_to_smart_contract ---

    #[test]
    fn successful_donation() {
        // Arrange
        // Using 1 NEAR in yoctonear (10^24) for deposit
        let deposit_amount: NearToken = NearToken::from_yoctonear(1_000_000_000_000_000_000_000_000); 
        let predecessor_id: AccountId = "bob.near".parse().unwrap();
        
        // Setup the mock environment
        let context = get_context(predecessor_id, deposit_amount);
        testing_env!(context.build());
        let mut contract = Counter::default();

        // Act
        // This test ensures the function executes without panicking and creates the transfer promise.
        contract.donate_to_smart_contract();

        // Assert: The call completed successfully.
    }

    #[test]
    #[should_panic(expected = "You are not allowed to donate")]
    fn donation_fails_for_alice() {
        // Arrange
        let deposit_amount: NearToken = NearToken::from_yoctonear(1_000_000_000_000_000_000_000_000); 
        let predecessor_id: AccountId = "alice.near".parse().unwrap();
        
        // Setup the mock environment
        let context = get_context(predecessor_id, deposit_amount);
        testing_env!(context.build());
        let mut contract = Counter::default();

        // Act & Assert (Should Panic)
        contract.donate_to_smart_contract();
    }
}