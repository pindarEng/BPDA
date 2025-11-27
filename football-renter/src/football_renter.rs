#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;
use multiversx_sc::derive_imports::*;

pub type SlotId = u64;

mod events;
mod storage;

#[type_abi]
#[derive(TopEncode, TopDecode,NestedEncode,NestedDecode, Debug)]
pub struct Slot<M: ManagedTypeApi>{
    pub start: u64,
    pub end: u64,
    pub payer_address: ManagedAddress<M>,
    pub amount: BigUint<M>,
    pub confirmed: bool, // true if admin says confirmed
    pub initiator_address: ManagedAddress<M>,
}

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait FootballRenter: events::FootbalEvents{
    // have to do smth here i guess
    #[init]
    fn init(&self) {
        self.next_slot_id().set(1);
    }

    #[upgrade]
    fn upgrade(&self) {}

// 7.2 storages
    #[storage_mapper("footballFieldManagerAddress")]
    fn field_manager_address(&self) -> SingleValueMapper<ManagedAddress<Self::Api>>;

    #[storage_mapper("courtCost")]
    fn court_cost(&self) -> SingleValueMapper<BigUint<Self::Api>>;

    #[storage_mapper("minimumDeposit")]
    fn minimum_deposit(&self) -> SingleValueMapper<BigUint<Self::Api>>;

    #[storage_mapper("nextSlotId")]
    fn next_slot_id(&self) -> SingleValueMapper<SlotId>;

    #[storage_mapper("reservedSlot")]
    fn reserved_slots(&self, slot_id: SlotId) -> SingleValueMapper<Slot<Self::Api>>;

    #[storage_mapper("participants")]
    fn participants(&self, slot_id: SlotId) -> SetMapper<ManagedAddress<Self::Api>>;

    // TODO: add this to the init not here ...
    #[endpoint(setMinDeposit)]
    fn set_minimum_deposit(&self, amount: BigUint){
        
        self.minimum_deposit().set(amount);  
    }
// 7.3 
    #[payable("EGLD")]
    #[endpoint]
    fn create_football_slot(&self, start_time: u64, end_time: u64) -> SlotId {
        let deposit_amount = self.call_value().egld();
        let caller = self.blockchain().get_caller();
        
        let minimum_deposit = self.minimum_deposit().get();

        require!(
            *deposit_amount >= minimum_deposit,
            "the deposit must be at least equal or bigger than the deposit requiremt."
        );

        require!(
            start_time < end_time,
            "yo ahh aint a time traveler 6/7 not 7/6"
        );

        let current_slot_id = self.next_slot_id().get();
        let next_slot_id = current_slot_id + 1;
        self.next_slot_id().set(next_slot_id);

        let new_slot = Slot {
            start: start_time,
            end: end_time,
            payer_address: caller.clone(),
            amount: deposit_amount.clone_value(),
            confirmed: false,
            initiator_address: caller.clone(),
        };

        self.reserved_slots(current_slot_id).set(&new_slot);

        self.participants(current_slot_id).insert(caller.clone());

        self.emit_create_football_slot_event(current_slot_id, &caller, start_time, end_time, &deposit_amount);
        
        current_slot_id
    }

// 7.4 participare
    #[payable("EGLD")]
    #[endpoint]
    fn participate_football_slot(&self, slot_id: SlotId) {
        // participant
        let caller = self.blockchain().get_caller();
        let deposit_amount = self.call_value().egld();
    
        let minimum_deposit = self.minimum_deposit().get();

        require!(
            *deposit_amount >= minimum_deposit,
            "the deposit must be at least equal or bigger than the deposit requiremt."
        );

        require!(
            !self.reserved_slots(slot_id).is_empty(),
            "the slot doesnt exist"
        );


        let mut slot: Slot<<Self as ContractBase>::Api> = self.reserved_slots(slot_id).get();

        require!(
            !slot.confirmed,
            "slot is confirmed cant join anymore"
        );

        let mut participants_mapper = self.participants(slot_id);

        require!(
            !participants_mapper.contains(&caller),
            "you are already a participant in this slot"
        );

        participants_mapper.insert(caller.clone());
        slot.amount += deposit_amount.clone_value();
        
        self.reserved_slots(slot_id).set(&slot);
        
    }

// 7.5 cancel slot
    #[endpoint]
    fn cancel_football_slot(&self, slot_id: SlotId) {
        let caller = self.blockchain().get_caller();

        require!(
            !self.reserved_slots(slot_id).is_empty(),
            "the slot doesnt exit"
        );

        let slot = self.reserved_slots(slot_id).get();

        require!(
            caller == slot.initiator_address,
            "only the slot creator can cancel slots"
        );

        require!(
            !slot.confirmed,
            "slot has been confirmed already cannot cancel"
        );


        let min_deposit = self.minimum_deposit().get();
        let refund_amount = slot.amount.clone();
        self.send().direct_egld(&slot.initiator_address, &refund_amount);



    }


// 7.6 setare football manager
    #[endpoint(setFootballFieldManager)]
    fn set_football_field_manager(&self, new_manager: ManagedAddress){
        let caller = self.blockchain().get_caller();
        let previous_manager = self.field_manager_address().get();

        require!(
            caller == previous_manager,
            "the caller must be the previous manager only he can change the manager; old manager(caller) -> new manager "
        );

        self.field_manager_address().set(new_manager.clone());

        //event emit manager assigned event
    }

    #[view(getReservedSlot)]  
    fn get_reserved_slot(&self, slot_id: SlotId) -> Slot<Self::Api> {  
        self.reserved_slots(slot_id).get()  
    }


// 7.7 payCourt - min deposit de la toti participanti? whatabout full cost?
// payment endpoint     
}
