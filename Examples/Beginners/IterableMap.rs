use near_sdk::store::IterableMap;
use near_sdk::{near, PanicOnDefault};

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {
    users: IterableMap<String, u8>,
}

#[near]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            users: IterableMap::new(b"u"),
        }
    }

    pub fn set_user(&mut self, name: String, age: u8) -> Option<u8> {
        self.users.insert(name, age)
    }

    pub fn get_user(&self, name: String) -> Option<u8> {
        self.users.get(&name).copied()
    }

    pub fn remove_user(&mut self, name: String) {
        self.users.remove(&name);
    }

    pub fn clear_all(&mut self) {
        self.users.clear();
    }

    pub fn get_total_age(&self) -> u64 {
        self.users
            .values()
            .map(|&age| age as u64)
            .sum()
    }

    pub fn get_users_above_age(&self, min_age: u8) -> Vec<(String, u8)> {
        self.users
            .iter()                                  
            .filter(|(_name, &age)| age >= min_age)  
            .map(|(name, &age)| (name.clone(), age)) 
            .collect()                               
    }
}