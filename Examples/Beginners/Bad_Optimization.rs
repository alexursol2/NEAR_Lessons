use near_sdk::near;
use near_sdk::collections::Vector;

#[near(contract_state)]
pub struct Contract {
    pub value: u32,
    pub items: Vector<String>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            value: 5,
            items: Vector::new(b"i"),
        }
    }
}

#[near]
impl Contract {
    // 2. ❌ BAD: Single Action Per Transaction (No Batching)
    pub fn insert_item_bad(&mut self, item: String) {
        self.items.push(item);
    }

    // ❌ BAD: O(n) counting via loop instead of using len()
    pub fn get_total_item_count_bad(&self) -> u64 {
        let mut count = 0;
        for _ in self.items.iter() {
            count += 1;
        }
        count
    }
}
