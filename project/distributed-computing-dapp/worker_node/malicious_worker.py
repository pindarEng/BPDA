import argparse
import subprocess
import hashlib
import sys
import os
import time

# Configuration
PROXY = "https://devnet-api.multiversx.com"
CONTRACT_ADDRESS = "erd1qqqqqqqqqqqqqpgqskctlvmpx5kk2573h0vevgr3hmmplr0ync0sxx26qx"

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

def submit_result(task_id, result_hash, pem_file):
    """Submits the result to the smart contract."""
    print(f"[*] Submitting MALICIOUS result for Task #{task_id}...")
    
    cmd = (
        f"mxpy contract call {CONTRACT_ADDRESS} "
        f"--pem={pem_file} "
        f"--gas-limit=10000000 "
        f"--function=submitResult "
        f"--arguments {task_id} str:{result_hash} "
        f"--proxy={PROXY} "
        f"--send"
    )
    
    output = run_command(cmd)
    if output:
        print(f"[+] Malicious result submitted! Tx Hash: {output}")
    else:
        print("[-] Submission failed.")

def main():
    parser = argparse.ArgumentParser(description='Malicious Worker Node')
    parser.add_argument('--task-id', type=int, required=True, help='The ID of the task')
    parser.add_argument('--wallet', type=str, required=True, help='Path to the wallet PEM file')
    
    args = parser.parse_args()

    if not os.path.exists(args.wallet):
        print(f"Error: Wallet file not found at {args.wallet}")
        sys.exit(1)

    print(f"--- Starting MALICIOUS Worker for Task #{args.task_id} ---")
    print(f"--- Wallet: {args.wallet} ---")
    
    # Generate a fake result
    # We don't even run the docker image. We just fabricate a result.
    fake_data = f"WRONG_ANSWER_{time.time()}"
    print(f"[*] Generated fake data: {fake_data}")
    
    # Hash it
    result_hash = hashlib.sha256(fake_data.encode()).hexdigest()
    print(f"[*] Fake Hash: {result_hash}")
    
    # Submit
    submit_result(args.task_id, result_hash, args.wallet)

if __name__ == "__main__":
    main()
