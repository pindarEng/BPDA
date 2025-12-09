#![allow(non_snake_case)]

pub mod config;
pub mod football_renter_proxy;
use football_renter_proxy as proxy;

pub type SlotId = u64;

use config::Config;
use multiversx_sc_snippets::imports::*;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};

const STATE_FILE: &str = "state.toml";

// Note: Removed the CLI main function logic to focus on Testing structure
pub async fn football_renter_cli() { 
    // Kept empty to satisfy compiler if needed, but we focus on the struct below
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    contract_address: Option<Bech32Address>
}

impl State {
    pub fn load_state() -> Self {
        if Path::new(STATE_FILE).exists() {
            let mut file = std::fs::File::open(STATE_FILE).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            toml::from_str(&content).unwrap()
        } else {
            Self::default()
        }
    }

    pub fn set_address(&mut self, address: Bech32Address) {
        self.contract_address = Some(address);
    }

    pub fn current_address(&self) -> &Bech32Address {
        self.contract_address
            .as_ref()
            .expect("no known contract, deploy first")
    }
}

impl Drop for State {
    fn drop(&mut self) {
        let mut file = std::fs::File::create(STATE_FILE).unwrap();
        file.write_all(toml::to_string(self).unwrap().as_bytes())
            .unwrap();
    }
}

pub struct ContractInteract {
    interactor: Interactor,
    wallet_address: Address,     // Default (Alice/Owner)
    second_wallet_address: Address, // Bob
    contract_code: BytesValue,
    state: State
}

impl ContractInteract {
    pub async fn new(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());

        interactor.set_current_dir_from_workspace("football-renter");
        
        // Register Alice (Owner) and Bob (User)
        let wallet_address = interactor.register_wallet(test_wallets::alice()).await;
        let second_wallet_address = interactor.register_wallet(test_wallets::bob()).await;

        interactor.generate_blocks_until_all_activations().await;
        
        let contract_code = BytesValue::interpret_from(
            "mxsc:../output/football-renter.mxsc.json",
            &InterpreterContext::default(),
        );

        ContractInteract {
            interactor,
            wallet_address,
            second_wallet_address,
            contract_code,
            state: State::load_state()
        }
    }

    // Helper to get Alice (Owner)
    pub fn owner_wallet(&self) -> &Address {
        &self.wallet_address
    }

    // Helper to get Bob (User)
    pub fn user_wallet(&self) -> &Address {
        &self.second_wallet_address
    }

    pub fn interactor_mut(&mut self) -> &mut Interactor {
        &mut self.interactor
    }

    pub fn contract_address(&self) -> &Bech32Address {
        self.state.current_address()
    }

    pub async fn deploy(&mut self) {
        let min_deposit_init = BigUint::<StaticApi>::from(500u128); // Initialize with 500

        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(100_000_000u64) // INCREASED GAS HERE
            .typed(proxy::FootballRenterProxy)
            .init(min_deposit_init)
            .code(&self.contract_code)
            .returns(ReturnsNewAddress)
            .run()
            .await;
        
        let new_address_bech32 = new_address.to_bech32_default();
        println!("Deployed at: {new_address_bech32}");
        self.state.set_address(new_address_bech32);
    }

    pub async fn set_minimum_deposit(&mut self, caller: &Address, amount: u128) {
        let amount_bn = BigUint::<StaticApi>::from(amount);

        self.interactor
            .tx()
            .from(caller)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::FootballRenterProxy)
            .set_minimum_deposit(amount_bn)
            .run()
            .await;
    }

    // Modified to accept arguments and return the Slot ID
    pub async fn create_football_slot(&mut self, caller: &Address, start: u64, end: u64, payment: u128) -> u64 {
        let payment_bn = BigUint::<StaticApi>::from(payment);

        let result = self.interactor
            .tx()
            .from(caller)
            .to(self.state.current_address())
            .gas(50_000_000u64)
            .typed(proxy::FootballRenterProxy)
            .create_football_slot(start, end)
            .egld(payment_bn)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
        
        result
    }

    pub async fn participate_football_slot(&mut self, caller: &Address, slot_id: u64, payment: u128) {
        let payment_bn = BigUint::<StaticApi>::from(payment);

        self.interactor
            .tx()
            .from(caller)
            .to(self.state.current_address())
            .gas(50_000_000u64)
            .typed(proxy::FootballRenterProxy)
            .participate_football_slot(slot_id)
            .egld(payment_bn)
            .run()
            .await;
    }

    pub async fn confirm_slot(&mut self, caller: &Address, slot_id: u64) {
        self.interactor
            .tx()
            .from(caller)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::FootballRenterProxy)
            .confirm_slot(slot_id)
            .run()
            .await;
    }

    pub async fn pay_court(&mut self, caller: &Address, slot_id: u64) {
        self.interactor
            .tx()
            .from(caller)
            .to(self.state.current_address())
            .gas(50_000_000u64)
            .typed(proxy::FootballRenterProxy)
            .pay_court(slot_id)
            .run()
            .await;
    }

    pub async fn set_football_court_cost(&mut self, caller: &Address, cost: u128) {
        let cost_bn = BigUint::<StaticApi>::from(cost);
        self.interactor
            .tx()
            .from(caller)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::FootballRenterProxy)
            .set_football_court_cost(cost_bn)
            .run()
            .await;
    }

    // View function to help verify tests
    pub async fn get_slot_status_view(&mut self, slot_id: u64) -> bool {
        let result = self.interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FootballRenterProxy)
            .get_slot_status(slot_id)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
        
        let (_slot, _participants, _amount, confirmed) = result.into_tuple();
        
        confirmed
    }

    pub async fn cancel_football_slot(&mut self, caller: &Address, slot_id: u64) {  
        self.interactor  
            .tx()  
            .from(caller)  
            .to(self.state.current_address())  
            .gas(50_000_000u64)  
            .typed(proxy::FootballRenterProxy)  
            .cancel_football_slot(slot_id)  
            .run()  
            .await;  
    }

    pub async fn set_football_field_manager(&mut self, caller: &Address, new_manager: &Address) {
        // Note: The proxy usually accepts a standard reference to Address
        self.interactor
            .tx()
            .from(caller)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::FootballRenterProxy)
            .set_football_field_manager(new_manager)
            .run()
            .await;
    }

    // pub async fn get_reserved_slot_details(&mut self, slot_id: u64) -> MultiValue4<Slot<StaticApi>, ManagedVec<StaticApi, ManagedAddress<StaticApi>>, BigUint<StaticApi>, bool> {  
    //     self.interactor  
    //         .query()  
    //         .to(self.state.current_address())  
    //         .typed(proxy::FootballRenterProxy)  
    //         .get_reserved_slot_details(slot_id)  
    //         .returns(ReturnsResultUnmanaged)  
    //         .run()  
    //         .await  
    // }

}