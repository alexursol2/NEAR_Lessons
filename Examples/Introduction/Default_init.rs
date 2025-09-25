use near_sdk::near;

#[near(contract_state)]
pub struct Contract{
    counter: u32
}

impl Default for Contract {
    fn default() -> Self{
        Self{
            counter: 0
        }
    }
}

#[near]
impl Contract{
    #[init]
    pub fn new(counter: u32) -> Self {
        Self {
            counter
        }
}
}
