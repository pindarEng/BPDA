// #![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;
pub type SlotId = u64;


#[multiversx_sc::module]
pub trait FootbalEvents{
    
    #[event("create_football_slot")]
    fn emit_create_football_slot_event(
        &self,
        #[indexed] slot_id: SlotId,
        #[indexed] initiator: &ManagedAddress<Self::Api>,
        #[indexed] start: u64,
        #[indexed] end: u64,
        deposit: &BigUint<Self::Api>,
    );
    
    #[event("add_participant")]
    fn emit_add_participant_event(
        &self,
        #[indexed] slot_id: SlotId,
        #[indexed] new_participant: &ManagedAddress<Self::Api>
    );

    #[event("slot_cancelled")]
    fn emit_slot_cancelled_event(
        &self,
        #[indexed] slot_id: SlotId,
        #[indexed] cancelled_by: &ManagedAddress<Self::Api>,
        refund_amount: &BigUint<Self::Api>
    );

    #[event("manager_assigned")]
    fn emit_manager_assigned_event(
        &self,
        #[indexed] old_manager: &ManagedAddress<Self::Api>,
        #[indexed] new_manager: &ManagedAddress<Self::Api>,
        #[indexed] assigned_by: &ManagedAddress<Self::Api>,
    );

    #[event("court_paid")]
    fn emit_court_paid_event(
        &self,
        #[indexed] slot_id: SlotId,
        #[indexed] recipient: &ManagedAddress<Self::Api>,
        payment_amount: &BigUint<Self::Api>
    );

    #[event("slot_confirmed")]
    fn emit_slot_confirmed_event(
        &self,
        #[indexed] slot_id: SlotId,
        #[indexed] confirmed_by: &ManagedAddress<Self::Api>,
    );
}