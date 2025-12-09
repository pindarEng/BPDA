use multiversx_sc_snippets::imports::*;
use rust_interact::{config::Config, ContractInteract};
use rust_interact::football_renter_proxy::FootballRenterProxy;

// Simple deploy test that runs using the chain simulator configuration.
// In order for this test to work, make sure that the `config.toml` file contains the chain simulator config (or choose it manually)
// The chain simulator should already be installed and running before attempting to run this test.
// The chain-simulator-tests feature should be present in Cargo.toml.
// Can be run with `sc-meta test -c`.
#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn deploy_test_football_renter_cs() {
    let mut interactor = ContractInteract::new(Config::chain_simulator_config()).await;

    interactor.deploy().await;
}


#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn full_football_renter_scenario() {
    // 1. Setup Interactor
    let mut interact = ContractInteract::new(Config::chain_simulator_config()).await;
    let owner = interact.owner_wallet().clone();
    let gabi = interact.user_wallet().clone();

    // 2. Owner
    interact.deploy().await; 

    // 3. Admin Setup: Set Court Cost
    let court_cost = 1000u128;
    interact.set_football_court_cost(&owner, court_cost).await;

    // start=100, end=200, pay=500
    let slot_id = interact.create_football_slot(&gabi, 100, 200, 500).await;
    
    assert_eq!(slot_id, 1, "First slot ID should be 1");
    println!("Slot created successfully by Bob");

    interact.participate_football_slot(&owner, slot_id, 500).await;
    println!("Owner participated in the slot");

    interact.confirm_slot(&owner, slot_id).await;
    println!("Slot confirmed by Admin");

    let is_confirmed = interact.get_slot_status_view(slot_id).await;
    assert!(is_confirmed, "Slot should be confirmed now");

    // 8. Pay Court (Triggered by Manager/Owner)
    // Total collected (1000) >= Court Cost (1000). Payment should succeed.
    interact.pay_court(&owner, slot_id).await;
    println!("Court paid successfully");


}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn refund_scenario_test() {
    let mut interact = ContractInteract::new(Config::chain_simulator_config()).await;
    let owner = interact.owner_wallet().clone();
    let bob = interact.user_wallet().clone();

    interact.deploy().await; 
    interact.set_football_court_cost(&owner, 1000u128).await;

    // bob plateste 500 dar costa 1000
    let slot_id = interact.create_football_slot(&bob, 100, 200, 500).await;
    interact.confirm_slot(&owner, slot_id).await;

    interact.pay_court(&owner, slot_id).await;
    println!("Refund logic executed");

    let contract_address = interact.contract_address().to_address();
    
    let contract_account = interact.interactor_mut()
        .get_account(&contract_address)
        .await;

    println!("Contract balance: {}", contract_account.balance);  

    let balance: u128 = contract_account.balance.parse().unwrap();  
    assert_eq!(balance, 0u128, "Contract should have 0 balance after refund");
}

#[tokio::test]  
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]  
async fn cancel_slot_test() {  
    let mut interact = ContractInteract::new(Config::chain_simulator_config()).await;  
    let owner = interact.owner_wallet().clone();  
    let bob = interact.user_wallet().clone();  
  
    interact.deploy().await;  
      
    let slot_id = interact.create_football_slot(&bob, 100, 200, 500).await;  
      
    interact.participate_football_slot(&owner, slot_id, 500).await;  
      
    interact.cancel_football_slot(&bob, slot_id).await;  
      
    let contract_address = interact.contract_address().to_address();  
    let contract_account = interact.interactor_mut()  
        .get_account(&contract_address)  
        .await;  
    let balance: u128 = contract_account.balance.parse().unwrap();  
    assert_eq!(balance, 0u128, "Contract should be empty after cancellation");  
}


