# Assignment for Blockchain protocols and distributed applications course


The smart contract was implemented using the multiversx blockchain.

## Testing

In order to test the endpoints, we use mxpy, inside the folder `football-renter/interact/` run `. devnet.snippets.sh`. The tests will be done on the devnet.

Interactor tests are also available using the chain-simulator.


## General description

Blockchain technology provides a secure and transparent way to record every action related to renting the field.
This ensures that payments, reservations, and cancellations are handled correctly without third-party involvement.

The task:
Implement a smart contract that manages the rental of a football field for specific time intervals.
The contract must allow:

- slot reservation,
- payment of a guarantee,
- adding participants,
- slot cancellation,
- reservation confirmation,
- payment to the field administrator.

All actions must emit events and store relevant data on-chain.

## 7. Detailed Technical Requirements

### 7.1. Deploy contract on testnet

- Contract must be compiled and deployed on chosen testnet/devnet.
- Contract address must be saved in repository.
- Requirements may not cover all flows; students may add extra logic.

---

### 7.2. Storage

The contract defines the following data structures:

- `football_field_manager_address`
- `football_court_cost`
- `participants`
- `reserved_slot: Slot` or optional  
  Structure: `{ start, end, payer, amount, confirmed }`


---

### 7.3. Endpoint: createFootballSlot

#### Behavior

This function lets a user start a new football game session.
The caller sends a small fixed deposit; if no session already exists, the contract records the new slot, registers the caller as the initiator/first participant, and logs that a new session was created.

---

### 7.4. Endpoint: participateToFootballSlot

This allows another user to join an already created session.
They pay the same fixed deposit, and if everything is valid (session exists, user not already joined), they are added to the participant list and the action is logged.

---

### 7.5. Endpoint: cancelFootballSlot

The session creator can cancel the session before it is confirmed.
When this happens, everyone who joined gets their deposit back, and the session data is completely cleared. A cancellation event is recorded.

---

### 7.6. Endpoint: setFootballFieldManager

The contract owner can assign the field manager — the entity who ultimately receives the payment.
Once set, the contract records this assignment and emits an event confirming the change.

---

### 7.7. Endpoint: payCourt

This function transfers the accumulated funds to the previously assigned field manager.
It performs basic checks (manager and price must be set, enough funds collected, slot must exist) and then executes the payment, logging the transfer.

---

### 7.8. Endpoint: setFootballCourtCost

The owner defines the overall field rental cost.
This value is stored and later used when making the final payment to the manager.

---

### 7.9. Endpoint: confirmSlot

The manager (or owner) confirms that the session is valid and approved.
Once confirmed, the slot is marked accordingly so that no further changes (like cancelation or re-confirmation) can occur.

---

### 7.10. Endpoint: getSlotStatus

Returns:

- slot data
- participants
- accumulated amount
- confirmation status

---


Course repository: https://cs-pub-ro.github.io/blockchain-protocols-and-distributed-applications/