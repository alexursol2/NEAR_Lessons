use near_sdk::store::LookupSet;
use near_sdk::{near, PanicOnDefault};

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct StorageExample {
    lookup_set: LookupSet<i32>,
}

#[near]
impl StorageExample {
    #[init]
    pub fn new() -> Self {
        Self {
            lookup_set: LookupSet::new(b"s"),
        }
    }

    pub fn insert_set(&mut self, value: i32) {
        self.lookup_set.insert(value);
    }

    pub fn remove_set(&mut self, value: i32) {
        self.lookup_set.remove(&value);
    }

    pub fn contains_set(&self, value: i32) -> bool {
        self.lookup_set.contains(&value)
    }
}