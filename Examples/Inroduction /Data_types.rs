// Library
use near_sdk::near;

#[near(contract_state)]
pub struct Contract{
    // Your data goes here
    owner: String, 
    is_active: bool, 
    decimals: u8,
    small_id: u16,
    minicounter: u32,
    created_at: u64,
    large_number: u128
}

#[near]
impl Contract{
    // Your functions go here
}
