use near_sdk::near;
use std::collections::HashMap;

#[near(contract_state)]
pub struct Contract {

}

#[near]
impl Contract {
    // Function 1: Manage user token balances
    pub fn manage_user_balances() -> String {
        // Create HashMap to store user balances (account_id -> balance)
        let mut user_balances: HashMap<String, u64> = HashMap::new();
        
        // Insert initial balances for users
        user_balances.insert("alice.near".to_string(), 1000);
        user_balances.insert("bob.near".to_string(), 500);
        user_balances.insert("charlie.near".to_string(), 750);
        
        // Get a user's balance - returns Option<&u64>
        let alice_balance = user_balances.get("alice.near").unwrap_or(&0);
        let balance_result = format!("Alice's balance: {}", alice_balance);
        
        // Update existing balance
        user_balances.insert("alice.near".to_string(), 1200); // Overwrites previous value
        let updated_result = format!("Alice's updated balance: {}", user_balances.get("alice.near").unwrap());
        
        // Check if user exists in our system
        let has_bob = user_balances.contains_key("bob.near");
        let contains_result = format!("Bob exists: {}", has_bob);
        
        format!("{} | {} | {}", balance_result, updated_result, contains_result)
    }

    // Function 2: Track NFT ownership and marketplace operations
    pub fn manage_nft() -> String {
        let mut nft_owners: HashMap<u32, String> = HashMap::new();
        
        // Mint new NFTs to users
        nft_owners.insert(1, "alice.near".to_string());
        nft_owners.insert(2, "bob.near".to_string());
        nft_owners.insert(3, "alice.near".to_string());
        nft_owners.insert(4, "charlie.near".to_string());
        
        // Count total NFTs in collection
        let total_nfts = nft_owners.len();
        
        // Remove NFT (burn operation)
        let burned_nft = nft_owners.remove(&2); // Returns Some("bob.near") or None
        let burn_result = format!("Burned NFT #2 from: {}", burned_nft.unwrap_or("none".to_string()));
        
        format!(
            "Total: {} | {}", 
            total_nfts, burn_result
        )
    }
}