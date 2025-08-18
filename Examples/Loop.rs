use near_sdk::near;

#[near(contract_state)]
pub struct Contract{

}

#[near]
impl Contract{
    pub fn calculate_compound_interest(years: u32) -> u64 {
        let mut amount = 1000; // Starting amount
        
        for _year in (1..=years).step_by(2)  {
            amount = amount + (amount / 10); // 10% interest per 2 years
            if amount >= 1200{
                break
            }
        }
        
        amount
    }

    pub fn distribute_rewards(mut available_funds: u64) -> u64 {
        let reward_per_user = 50;
        let mut users_rewarded = 0;
        
        while available_funds >= reward_per_user && users_rewarded < 10{
            available_funds -= reward_per_user;
            users_rewarded += 1;
        }
        
        users_rewarded // Return amount of users that were funded
    }
}
