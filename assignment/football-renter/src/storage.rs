
// use multiversx_sc::imports::*;  
// pub type SlotId = u64;

// #[multiversx_sc::module]
// pub trait FootbalStorage{
//     #[storage_mapper("footballFieldManagerAddress")]
//         fn field_manager_address(&self) -> SingleValueMapper<ManagedAddress<Self::Api>>;

//         #[storage_mapper("courtCost")]
//         fn court_cost(&self) -> SingleValueMapper<BigUint<Self::Api>>;

//         #[storage_mapper("minimumDeposit")]
//         fn minimum_deposit(&self) -> SingleValueMapper<BigUint<Self::Api>>;

//         #[storage_mapper("nextSlotId")]
//         fn next_slot_id(&self) -> SingleValueMapper<SlotId>;

//         #[storage_mapper("reservedSlot")]
//         fn reserved_slots(&self, slot_id: SlotId) -> SingleValueMapper<Slot<Self::Api>>;

//         #[storage_mapper("participants")]
//         fn participants(&self, slot_id: SlotId) -> SetMapper<ManagedAddress<Self::Api>>;
// }