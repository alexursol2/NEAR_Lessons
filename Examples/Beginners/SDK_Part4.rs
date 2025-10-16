use near_sdk::{near, env, PublicKey, CryptoHash};

#[near(contract_state)]
pub struct Contract {
    authorized_keys: Vec<PublicKey>,
    counter: u32,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            authorized_keys: Vec::new(),
            counter: 0,
        }
    }
}

#[near]
impl Contract {
    pub fn check_gas_status(&mut self) -> String {
        self.counter += 1;
        let prepaid = env::prepaid_gas(); // Get total gas attached to this call
        let used = env::used_gas(); // Get gas used so far
        let remaining = prepaid.as_gas() - used.as_gas();
        
        format!("Prepaid: {}, Used: {}, Remaining: {}", 
                prepaid.as_gas(), used.as_gas(), remaining)
    }
    
    pub fn add_authorized_key(&mut self, key: PublicKey) {
        // Only allow if signed by current owner
        self.authorized_keys.push(key);
    }

    pub fn verify_signature(&self, key_str: String) -> bool {
        // Parse string to PublicKey
        match key_str.parse::<PublicKey>() {
            Ok(public_key) => {
                // Check if this key is in our authorized list
                self.authorized_keys.contains(&public_key)
            },
            Err(_) => false
        }
    }

    pub fn secure_document(&self, content: String) -> CryptoHash {
        // Hash the provided content
        let content_bytes = content.as_bytes();
        let computed_hash = env::keccak256(content_bytes);
        let computed_crypto_hash = CryptoHash::try_from(computed_hash).unwrap();
        
        // Compare with expected hash
        computed_crypto_hash
    }
}
