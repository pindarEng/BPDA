import argparse
import subprocess
import time
import hashlib
import sys
import os

# Configuration
PROXY = "https://devnet-api.multiversx.com"
# Default contract address from the dApp config
DEFAULT_CONTRACT_ADDRESS = "erd1qqqqqqqqqqqqqpgqfgayg3ykmn6jluazdfhka02y3q9vjc8wnc0syn2dwx"

def run_command(command):
    """Runs a shell command and returns the output."""
    try:
        result = subprocess.run(
            command, 
            shell=True, 
            check=True, 
            stdout=subprocess.PIPE, 
            stderr=subprocess.PIPE,
            text=True
        )
        return result.stdout.strip()
    except subprocess.CalledProcessError as e:
        print(f"Error running command: {command}")
        print(e.stderr)
        sys.exit(1)

def simulate_docker_execution(image_uri, input_data):
    """
    Simulates fetching a docker image and running it.
    In a real scenario, this would use the 'docker' python library.
    """
    print(f"[*] Pulling Docker image: {image_uri}...")
    time.sleep(1) # Simulate network delay
    
    print(f"[*] Running container with input: {input_data}...")
    time.sleep(2) # Simulate processing time
    
    # Simulate a result based on input
    result_data = f"Processed_{input_data}_with_{image_uri}"
    
    # Hash the result (simulating the proof/output hash)
    result_hash = hashlib.sha256(result_data.encode()).hexdigest()
    
    print(f"[*] Execution complete. Result Hash: {result_hash}")
    return result_hash

def submit_result(task_id, result_hash, pem_file, contract_address):
    """Submits the result to the smart contract using mxpy."""
    print(f"[*] Submitting result to contract {contract_address}...")
    
    cmd = (
        f"mxpy contract call {contract_address} "
        f"--pem={pem_file} "
        f"--gas-limit=10000000 "
        f"--function=submitResult "
        f"--arguments {task_id} str:{result_hash} "
        f"--proxy={PROXY} "
        f"--send"
    )
    
    output = run_command(cmd)
    print("[+] Transaction sent!")
    print(output)

def main():
    parser = argparse.ArgumentParser(description='Distributed Computing Worker Node')
    parser.add_argument('--task-id', type=int, required=True, help='The ID of the task to solve')
    parser.add_argument('--wallet', type=str, required=True, help='Path to the worker wallet PEM file')
    parser.add_argument('--contract', type=str, default=DEFAULT_CONTRACT_ADDRESS, help='Smart contract address')
    
    # In a real node, these might be fetched from the chain using the task_id
    parser.add_argument('--image', type=str, default="ubuntu:latest", help='Docker image URI (simulated)')
    parser.add_argument('--input', type=str, default="default_input", help='Input data URI (simulated)')

    args = parser.parse_args()

    if not os.path.exists(args.wallet):
        print(f"Error: Wallet file not found at {args.wallet}")
        sys.exit(1)

    print(f"--- Starting Worker for Task #{args.task_id} ---")
    
    # 1. Execute Task
    result_hash = simulate_docker_execution(args.image, args.input)
    
    # 2. Submit Result
    submit_result(args.task_id, result_hash, args.wallet, args.contract)

if __name__ == "__main__":
    main()
