PROXY=https://devnet-api.multiversx.com
PROJECT="../output/distributed-computing.wasm"
ADDRESS=$(mxpy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(mxpy data load --key=deployTransaction-devnet)


GABI="/home/pindar/wallet.pem"


WORKER_1="WORKERS_WALLETS/new_WORKER_1.pem"
WORKER_2="WORKERS_WALLETS/new_WORKER_2.pem"
WORKER_3="WORKERS_WALLETS/new_WORKER_3.pem"
WORKER_4="WORKERS_WALLETS/new_WORKER_4.pem"
WORKER_5="WORKERS_WALLETS/new_WORKER_5.pem"



DOCKER_IMAGE_URI="TEST_DOCKER_IMAGE"
INPUT_DATA_URI="TEST_INPUT_DATA"
MAX_WORKERS=5




deploy() {
    mxpy --verbose contract deploy --bytecode=${PROJECT} --pem=${GABI} --gas-limit 50000000 --send --outfile="deploy-devnet.interaction.json" --proxy=${PROXY} || return

    TRANSACTION=$(mxpy data parse --file="deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(mxpy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")

    mxpy data store --key=address-devnet --value=${ADDRESS}
    mxpy data store --key=deployTransaction-devnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

post_task(){
    read -p "Enter reward amount (in wei): " EGLD_AMOUNT  
    mxpy --verbose contract call ${ADDRESS} --pem=${GABI} --gas-limit 10000000 --function="postTask" --arguments "str:${DOCKER_IMAGE_URI}" "str:${INPUT_DATA_URI}" ${MAX_WORKERS} --value=${EGLD_AMOUNT} --proxy=${PROXY} --send  

}

submit_result() {
    TASK_ID=$1
    WORKER_PEM=$2
    RESULT_HASH=$3

    mxpy --verbose contract call ${ADDRESS} \
        --pem=${WORKER_PEM} \
        --gas-limit 10000000 \
        --function="submitResult" \
        --arguments ${TASK_ID} "str:${RESULT_HASH}" \
        --proxy=${PROXY} \
        --send
}


check_task() {
    read -p "Enter task id: " TASK_ID  

    mxpy --verbose contract query ${ADDRESS} \
        --function="getTask" \
        --arguments ${TASK_ID} \
        --proxy=${PROXY}
}


task_status(){

    read -p "Enter task id: " TASK_ID  

    mxpy contract query ${ADDRESS} \
    --function="getTaskStatus" \
    --arguments ${TASK_ID} \
    --proxy ${PROXY}
}

simulate_task() {
    echo "Creating task..."
    post_task

    read -p "Enter Task ID (usually 0): " TASK_ID

    echo "Submitting results..."
    submit_result ${TASK_ID} ${WORKER_1} "resultA"
    submit_result ${TASK_ID} ${WORKER_2} "resultA"
    submit_result ${TASK_ID} ${WORKER_3} "resultB"
    submit_result ${TASK_ID} ${WORKER_4} "resultA"
    submit_result ${TASK_ID} ${WORKER_5} "resultA"

    echo "Checking final task status..."
    task_status ${TASK_ID}
}


