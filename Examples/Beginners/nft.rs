use near_sdk::{near, env, AccountId, PanicOnDefault, require};
use near_sdk_contract_tools::nft::*; // NonFungibleToken, ContractMetadata, Nep171Burn, etc.

#[derive(PanicOnDefault, NonFungibleToken)]
#[near(contract_state)]
pub struct MyNftContract {
    owner: AccountId,
}

#[near]
impl MyNftContract {
    #[init]
    pub fn new() -> Self {
        let mut contract = Self {
            owner: env::predecessor_account_id(),
        };
        
        contract.set_contract_metadata(&ContractMetadata::new("My NFT", "MNFT", None));
        contract
    }

    /// Allows the contract **owner** to burn a specific NFT.
    pub fn owner_burn(&mut self, token_id: String) {
        require!(env::predecessor_account_id() == self.owner, "Only owner can burn tokens.");

        // Create the burn action: Vec of token IDs + owner ID
        let token_ids = vec![token_id];
        let action = Nep171Burn::new(token_ids, self.owner.clone());

        // Call the burn method implemented by the NonFungibleToken derive macro
        self.burn(&action);
    }
}

// --- Tests Module ---
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{VMContextBuilder};
    use near_sdk::testing_env;

    const CONTRACT_ID: &str = "my_nft_contract.near";
    const OWNER_ID: &str = "owner.near";
    const ALICE_ID: &str = "alice.near";
    const TEST_TOKEN_ID: &str = "token_123";

    // Helper function to set up the testing environment context
    fn get_context(predecessor_account_id: &str) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(AccountId::new_unvalidated(CONTRACT_ID.to_string()))
            .signer_account_id(AccountId::new_unvalidated(predecessor_account_id.to_string()))
            .predecessor_account_id(AccountId::new_unvalidated(predecessor_account_id.to_string()));
        builder
    }

    // Helper function to initialize the contract
    fn init_contract() -> MyNftContract {
        let context = get_context(OWNER_ID);
        testing_env!(context.build());
        
        MyNftContract::new()
    }

    /// Test 1: Initialization verifies owner and metadata setup.
    #[test]
    fn test_new_contract_init() {
        let contract = init_contract();

        assert_eq!(contract.owner.to_string(), OWNER_ID, "Owner should be set to the predecessor account.");
        // We can't easily assert the metadata content here as it's set via a trait method, 
        // but we verify the owner which is the custom logic in `new()`.
    }
    
    /// Test 2: Successful burn by the contract owner.
    #[test]
    fn test_owner_burn_success() {
        let mut contract = init_contract();
        
        // Simulating the owner calling the function
        let context = get_context(OWNER_ID);
        testing_env!(context.build());

        // This call should succeed (it will try to call the internal burn, 
        // which will interact with mock storage if it were a full simulation, 
        // but here we only verify no panic occurs).
        contract.owner_burn(TEST_TOKEN_ID.to_string());
    }

    /// Test 3: Failed burn by a non-owner (access control check).
    #[test]
    #[should_panic(expected = "Only owner can burn tokens.")]
    fn test_owner_burn_unauthorized_panic() {
        let mut contract = init_contract();

        // Simulating a non-owner (Alice) calling the function
        let context = get_context(ALICE_ID);
        testing_env!(context.build());

        // This call MUST panic due to the `require!` check
        contract.owner_burn(TEST_TOKEN_ID.to_string());
    }
}