use near_sdk::near;
use near_sdk::collections::Vector;

#[near(contract_state)]
pub struct Contract {
    pub value: u32,
    pub items: Vector<String>,
    item_count: u64, 
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            value: 5,
            items: Vector::new(b"i"),
            item_count: 0,
        }
    }
}

#[near]
impl Contract {
    pub fn insert_item_batch(&mut self, new_items: Vec<String>) {
        self.item_count += new_items.len() as u64; 
        self.items.extend(new_items);
    }

    pub fn get_total_item_count_opt(&self) -> u64 {
        self.item_count
    }
}
