use near_sdk::near;

#[near(contract_state)]
pub struct Contract{
    counter: u32
}

#[near]
impl Contract{
    pub fn get_counter(&self, add: u32) -> u32{
        self.counter + add
    }

    fn private_counter(&self) -> u32{
        1
    }
}
