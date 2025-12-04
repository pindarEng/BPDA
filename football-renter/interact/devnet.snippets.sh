GABI_1="/home/pindar/wallet.pem"
GABI_2="/home/pindar/model_exam_wallet.pem"
GABI_field_manager="/home/pindar/field_manager.pem"
GABI_field_manager_address="erd1lmglp68k99jy2e35ssgm2d332zfekeq0q3d8y6fmjk8rc9pjjlpsuyy9hf"

# GABI="/home/pindar/sim-wallet.pem"
# GABI="/home/pindar/model_exam_wallet.pem"
ADDRESS=$(mxpy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(mxpy data load --key=deployTransaction-devnet)
PROXY=https://devnet-api.multiversx.com
# PROXY=http://localhost:8085
PROJECT="../output/football-renter.wasm"


deploy() {
    mxpy --verbose contract deploy --bytecode=${PROJECT} --pem=${GABI_1} --arguments 100 --gas-limit 50000000 --send --outfile="deploy-devnet.interaction.json" --proxy=${PROXY} || return

    TRANSACTION=$(mxpy data parse --file="deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(mxpy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")

    mxpy data store --key=address-devnet --value=${ADDRESS}
    mxpy data store --key=deployTransaction-devnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}
# min deposit
set_minimum_deposit() {  
    read -p "Enter minimum deposit (in wei): " AMOUNT  
    mxpy --verbose contract call ${ADDRESS} --pem=${GABI_1} --gas-limit 10000000 --function="setMinDeposit" --arguments ${AMOUNT} --proxy=${PROXY} --send  
}  
  
# create football slot  
create_slot() {  
    read -p "Enter start time (unix timestamp): " START_TIME  
    read -p "Enter end time (unix timestamp): " END_TIME  
    read -p "Enter EGLD amount to send (in wei): " EGLD_AMOUNT  
      
    mxpy --verbose contract call ${ADDRESS} --pem=${GABI_1} --gas-limit 10000000 --function="create_football_slot" --arguments ${START_TIME} ${END_TIME} --value=${EGLD_AMOUNT} --gas-limit=10000000  --proxy=${PROXY} --send  
}  
  
# we participate with another wallet, pentru ca primu wallet participa si el deja fiind field_manager
participate_slot(){
    read -p "Slot ID: " SLOT_ID
    read -p "Deposit to send (in wei): " AMOUNT
    mxpy --verbose contract call ${ADDRESS} --pem=${GABI_2} --function="participate_football_slot" --arguments ${SLOT_ID} --value=${AMOUNT} --gas-limit 10000000 --proxy=${PROXY} --send
}

# self explanatory
cancel_slot() {
    read -p "Slot ID: " SLOT_ID

    mxpy --verbose contract call ${ADDRESS} --pem=${GABI_1} --function="cancel_football_slot" --arguments ${SLOT_ID} --gas-limit 10000000 --proxy=${PROXY} --send
}

# iti dai seama
set_manager() {
    # read -p "New manager bech32 address: " NEW_MANAGER

    mxpy --verbose contract call ${ADDRESS} --pem=${GABI_1} --function="setFootballFieldManager" --arguments ${GABI_field_manager_address} --gas-limit 10000000 --proxy=${PROXY} --send
}

set_court_cost() {
    read -p "Enter court cost (wei): " COST

    mxpy --verbose contract call ${ADDRESS} --pem=${GABI_field_manager} --function="setFootballCourtCost" --arguments ${COST} --gas-limit 10000000 --proxy=${PROXY} --send
}

confirm_slot() {
    read -p "Slot ID: " SLOT_ID

    mxpy --verbose contract call ${ADDRESS} --pem=${GABI_field_manager} --function="confirmSlot" --arguments ${SLOT_ID} --gas-limit 10000000 --proxy=${PROXY} --send
}

pay_court() {
    read -p "Slot ID: " SLOT_ID

    mxpy --verbose contract call ${ADDRESS} --pem=${GABI_field_manager} --function="payCourt" --arguments ${SLOT_ID} --gas-limit 10000000 --proxy=${PROXY} --send
}

# bruh
get_slot_status() {
    read -p "Slot ID: " SLOT_ID

    mxpy --verbose contract query ${ADDRESS} \
        --function="getSlotStatus" \
        --arguments ${SLOT_ID} \
        --proxy=${PROXY}
}


# Query a slot by ID  
# get_slot() {  
#     read -p "Enter slot ID: " SLOT_ID  
#     mxpy --verbose contract query ${ADDRESS} --function="getReservedSlot" --arguments ${SLOT_ID} --proxy=${PROXY}  
# }


get_slot() {  
    read -p "Enter slot ID: " SLOT_ID  
    mxpy --verbose contract query ${ADDRESS} --function="getReservedSlotDetails" --arguments ${SLOT_ID} --proxy=${PROXY}  
}