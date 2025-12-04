// use multiversx_sc_snippets::imports::*;
// use rust_interact::{config::Config, ContractInteract};
// use rust_interact::football_renter_proxy::FootballRenterProxy;

// // Simple deploy test that runs using the chain simulator configuration.
// // In order for this test to work, make sure that the `config.toml` file contains the chain simulator config (or choose it manually)
// // The chain simulator should already be installed and running before attempting to run this test.
// // The chain-simulator-tests feature should be present in Cargo.toml.
// // Can be run with `sc-meta test -c`.
// #[tokio::test]
// #[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
// async fn deploy_test_football_renter_cs() {
//     // let mut interactor = ContractInteract::new(Config::chain_simulator_config()).await;

//     interactor.deploy().await;
// }

// #[tokio::test]
// #[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
// async fn test_create_slot() {
//     let mut interactor = ContractInteract::new(Config::chain_simulator_config()).await;

//     // deploy contract
//     interactor.deploy().await;

//     // set min deposit to 1 EGLD
//     let one = BigUint::<StaticApi>::from(1u64);

//     interactor
//         .interactor_mut()                           // ✔ use getter
//         .tx()
//         .from(interactor.wallet())                  // ✔ use getter
//         .to(interactor.contract_address())          // ✔ use getter
//         .gas(30_000_000u64)
//         .typed(FootballRenterProxy)
//         .set_minimum_deposit(one.clone())
//         .run()
//         .await;

//     // create slot
//     let result = interactor
//         .interactor_mut()                           // ✔ use getter
//         .tx()
//         .from(interactor.wallet())                  // ✔ use getter
//         .to(interactor.contract_address())          // ✔ use getter
//         .gas(30_000_000)
//         .typed(FootballRenterProxy)
//         .create_football_slot(10u64, 20u64)
//         .egld(one.clone())
//         .returns(ReturnsResultUnmanaged)
//         .run()
//         .await;

//     println!("result: {result:?}");
// }