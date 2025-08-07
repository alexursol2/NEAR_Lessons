use near_sdk::near;

#[near(contract_state)]
pub struct Contract{
    counter: u32
}

#[near]
impl Contract{
    let mut counter = 0;
    counter = 5;

    pub fn increment(&mut self) {
        self.counter += 1;
    }
}
