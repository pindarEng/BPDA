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
    let mut interact = ContractInteract::new(Config::chain_simulator_config()).await;
    let owner = interact.owner_wallet().clone();
    let gabi = interact.user_wallet().clone();

    
    interact.deploy().await; 

    
    let court_cost = 1000u128; // pret 1000
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

    // collected = 1000 >= court cost ok good
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
    println!("refund logic executed");

    let contract_address = interact.contract_address().to_address();
    
    let contract_account = interact.interactor_mut()
        .get_account(&contract_address)
        .await;

    println!("contract balance: {}", contract_account.balance);  

    let balance: u128 = contract_account.balance.parse().unwrap();  
    assert_eq!(balance, 0u128, "should be 0 money in contract");
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
    assert_eq!(balance, 0u128, "contract should be empty");  
}

#[tokio::test]  
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]  
async fn overlap_failure_test() {  
    let mut interact = ContractInteract::new(Config::chain_simulator_config()).await;  
    let bob = interact.user_wallet().clone();  
    let alice = interact.owner_wallet().clone();  
  
    interact.deploy().await;   
  
    // Bob books 100 -> 200 successfully  
    interact.create_football_slot(&bob, 100, 200, 500).await;  
    println!("Bob booked 100-200");  
  
    let dest_address = interact.contract_address().clone();
    // Alice tries to book overlapping slot 150 -> 250  
    let result = interact.interactor_mut()  
        .tx()  
        .from(&alice)  
        .to(&dest_address)  
        .gas(50_000_000u64)  
        .typed(rust_interact::football_renter_proxy::FootballRenterProxy)  
        .create_football_slot(150u64, 250u64)  
        .egld(BigUint::from(500u128))  
        .returns(ReturnsHandledOrError::new())  
        .run()  
        .await;  
  
    assert!(result.is_err(), "overlap - need to block");  
    println!("overlap blocked !!!");  
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn security_permission_test() {
    let mut interact = ContractInteract::new(Config::chain_simulator_config()).await;
    let bob = interact.user_wallet().clone(); // bob is NOT the manager

    interact.deploy().await; 

    let dest_address = interact.contract_address().clone();

    // bob incearca prostii
    let status_code = interact.interactor_mut()
        .tx()
        .from(&bob)
        .to(&dest_address)
        .gas(30_000_000u64)
        .typed(rust_interact::football_renter_proxy::FootballRenterProxy)
        .set_football_court_cost(BigUint::<StaticApi>::from(5000u128))
        .returns(ReturnsStatus)
        .run()
        .await;

    assert_eq!(status_code, 4, "bob cant do this, bob not the mananger, bad bob");
}