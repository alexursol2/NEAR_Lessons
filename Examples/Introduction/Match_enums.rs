use near_sdk::near;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum UserType{
    Basic,
    Premium,
    VIP
}

#[near(contract_state)]
pub struct Contract {
    user_type: UserType,
    balance: u64,
}

impl Default for Contract {
    fn default() -> Self{
        Self{
            user_type: UserType::Basic,
            balance: 20,
        }
    }
}

#[near]
impl Contract {
    pub fn get_current_type(&self) -> UserType {
        self.user_type.clone()
    }

    pub fn upgrade_to_premium(&mut self) -> UserType {
        self.user_type = UserType::Premium;
        self.get_current_type()
    }

    pub fn predict_fee(&self) -> u64 {
        match self.user_type {
            UserType::Basic => self.balance / 10 + 5,
            UserType::VIP => 5,
            _ => 0,
        }
    }
}