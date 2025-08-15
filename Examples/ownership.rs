use near_sdk::near;

#[near(contract_state)]
pub struct Contract{
    counter: u32,
    owner: String
}

impl Default for Contract {
    fn default() -> Self{
        Self{
            counter: 0,
            owner: "test.near"
        }
    }
}

#[near]
impl Contract{
    pub fn transfer_ownership() {
        let message = String::from("Hello NEAR");
        let new_message = message;
    }

    pub fn mistake1(&self) {
        let name = self.owner.clone();
        some_function(name.clone());
    }

    pub fn mistake2(&mut self) {
        {
            let counter1 = &mut self.counter;
        }
        let counter2 = &mut self.counter;
    }

    pub fn get_owner(&self) -> String{
        self.owner.clone()
    }

    pub fn tip1(&self, account: &str) -> bool{
        self.owner == account
    }
}