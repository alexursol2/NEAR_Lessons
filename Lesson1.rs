use near_sdk::near;

#[near(contract_state)]
pub struct SimpleToken{
    total_supply: u128,
    owner: String,
    name: String,
    decimals: u8,
    is_paused: bool
}

#[near]
impl SimpleToken{

    #[init]
    pub fn new(owner: String, name: String, total_supply: u128) -> Self{
        Self{
            total_supply,
            owner,
            name,
            decimals: 18,
            is_paused: false,
        }
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    fn is_valid_amount(&self, amount: u128) -> bool{
        amount > 0 && amount < 1000000
    }

    pub fn transfer(&mut self, to: String, amount: u128) -> bool{
        true
    }

    // borrowed string
    pub fn set_owner(&mut self, new_owner: &str){
        self.owner = new_owner.to_string();
    }

    pub fn pause(&mut self){
        self.is_paused = true;
    }
}
