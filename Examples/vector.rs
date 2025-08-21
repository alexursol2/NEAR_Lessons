// Function 1: Manage whitelist of approved users
    pub fn manage_user_whitelist(&self) -> String {
        // Create empty vector - starts with zero capacity
        let mut whitelist: Vec<String> = Vec::new();
        
        // Add users to whitelist (vector grows automatically)
        whitelist.push("alice.near".to_string());
        whitelist.push("bob.near".to_string());
        whitelist.push("charlie.near".to_string());
        
        // Access elements by index (like arrays)
        let first_user = &whitelist[0]; // "alice.near"
        let access_result = format!("First whitelisted user: {}", first_user);
        
        // Get vector length (changes as we add/remove)
        let total_users = whitelist.len();
        let length_result = format!("Total whitelisted users: {}", total_users);
        
        // Check if vector contains a user
        let has_bob = whitelist.contains(&"bob.near".to_string());
        let contains_result = format!("Bob is whitelisted: {}", has_bob);
        
        // Create vector with initial values using vec! macro
        let admin_list: Vec<String> = vec!["admin.near".to_string(), "moderator.near".to_string()];
        let macro_result = format!("Admin list: {:?}", admin_list);
        
        format!("{} | {} | {} | {}", access_result, length_result, contains_result, macro_result)
    }

    // Function 2: Track and manage transaction history
    pub fn manage_transaction_history(&self) -> String {
        let mut transactions: Vec<u64> = vec![100, 250, 75, 500, 125];
        
        // Remove last transaction (pop returns Option<T>)
        let last_tx = transactions.pop(); // Returns Some(125) or None if empty
        let pop_result = format!("Removed transaction: {}", last_tx.unwrap_or(0));
        
        // Remove specific transaction by index
        let removed_tx = transactions.remove(1); // Removes and returns element at index 1
        let remove_result = format!("Removed tx at index 1: {}", removed_tx);
        
        // Insert transaction at specific position
        transactions.insert(1, 300); // Insert 300 at index 1, shifts others right
        let insert_result = format!("After insert: {:?}", transactions);

        // Find position of specific value
        let search_value = 500;
        let position = transactions.iter().position(|&x| x == search_value);
        let find_result = match position {
            Some(index) => format!("Found {} at index {}", search_value, index),
            None => format!("{} not found", search_value),
        };
        
        // Get total transaction amount (sum all values)
        let total_amount: u64 = transactions.iter().sum();
        let sum_result = format!("Total transaction amount: {}", total_amount);
        
        // Keep only transactions above threshold (filter in place)
        transactions.retain(|&x| x >= 100);
        let filter_result = format!("High-value transactions: {:?}", transactions);
        
        format!(
            "{} | {} | {} | {} | {} | {}", 
            pop_result, remove_result, insert_result, find_result, sum_result, filter_result
        )
    }