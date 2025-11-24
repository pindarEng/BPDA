ALICE="/home/pindar/wallet.pem"
ADDRESS=$(mxpy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(mxpy data load --key=deployTransaction-devnet)
PROXY=https://devnet-api.multiversx.com
PROJECT="../output/football-renter.wasm"

deploy() {
    mxpy --verbose contract deploy --bytecode=${PROJECT} --pem=${ALICE} --gas-limit 5000000 --send --outfile="deploy-devnet.interaction.json" --proxy=${PROXY} || return

    TRANSACTION=$(mxpy data parse --file="deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(mxpy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")

    mxpy data store --key=address-devnet --value=${ADDRESS}
    mxpy data store --key=deployTransaction-devnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}
set_minimum_deposit() {  
    read -p "Enter minimum deposit (in wei): " AMOUNT  
    mxpy --verbose contract call ${ADDRESS} --pem=${ALICE} --gas-limit 5000000 --function="setMinDeposit" --arguments ${AMOUNT} --proxy=${PROXY} --send  
}  
  
# Test create_football_slot endpoint  
create_slot() {  
    read -p "Enter start time (unix timestamp): " START_TIME  
    read -p "Enter end time (unix timestamp): " END_TIME  
    read -p "Enter EGLD amount to send (in wei): " EGLD_AMOUNT  
      
    mxpy --verbose contract call ${ADDRESS} --pem=${ALICE} --gas-limit 5000000 --function="create_football_slot" --arguments ${START_TIME} ${END_TIME} --value=${EGLD_AMOUNT} --gas-limit=10000000  --proxy=${PROXY} --send  
}  
  
# Query a slot by ID  
get_slot() {  
    read -p "Enter slot ID: " SLOT_ID  
    mxpy --verbose contract query ${ADDRESS} --function="getReservedSlot" --arguments ${SLOT_ID} --proxy=${PROXY}  
}