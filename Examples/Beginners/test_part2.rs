use anyhow::Result;
use near_workspaces::Account;
use near_workspaces::types::{NearToken, AccountId, SecretKey, KeyType}; 
use serde_json::json;

const INITIAL_BALANCE: NearToken = NearToken::from_near(5);

#[tokio::test]
async fn test_counter_logic_and_donations() -> Result<()> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("./").await?;

    let root = sandbox.root_account()?;

    let account_id: AccountId = "alice.near".parse().unwrap();
    let secret_key = SecretKey::from_random(KeyType::ED25519);
    let alice = Account::from_secret_key(account_id, secret_key, &sandbox);

    let bob = root.create_subaccount("bob").initial_balance(INITIAL_BALANCE).transact().await?.unwrap();

    let contract = bob.deploy(&contract_wasm).await?.unwrap();

    bob.call(contract.id(), "increment")
        .args_json(json!({})) 
        .transact()
        .await?
        .into_result()?;

    let incremented_value: i8 = contract
        .view("get_num")
        .await?
        .json()?;
    assert_eq!(incremented_value, 1);
    println!("✅ Test 1 Passed: Increment successful (value is 1).");

    bob.call(contract.id(), "decrement")
        .args_json(json!({
            "number": 5
        }))
        .transact()
        .await?
        .into_result()?;

    let final_value: i8 = contract
        .view("get_num")
        .await?
        .json()?;

    assert_eq!(final_value, -4);
    println!("✅ Test 2 Passed: Decrement successful (value is -4).");

    let underflow_check = bob
        .call(contract.id(), "decrement")
        .args_json(json!({
            "number": 127
        }))
        .transact()
        .await?
        .into_result();
    assert!(!underflow_check.is_ok());
    println!("✅ Test 3 Passed: Underflow check succeeded.");

    let donation_amount = NearToken::from_millinear(10);
    let donation_result = bob
        .call(contract.id(), "donate_to_smart_contract")
        .args_json(json!({}))
        .deposit(donation_amount)
        .transact()
        .await?
        .into_result();
    assert!(donation_result.is_ok());
    println!("✅ Test 4 Passed: Donation from Bob succeeded.");

    let failure_result = alice
        .call(contract.id(), "donate_to_smart_contract")
        .args_json(json!({}))
        .deposit(donation_amount)
        .transact()
        .await;
    assert!(!failure_result.is_ok());
    println!("✅ Test 5 Passed: 'alice.near' was correctly blocked.");

    Ok(())

}
