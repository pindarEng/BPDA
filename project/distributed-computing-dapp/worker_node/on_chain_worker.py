import time
import subprocess
import json
import hashlib
import sys
import os
import argparse
import requests

# Configuration
PROXY = "https://devnet-api.multiversx.com"
CONTRACT_ADDRESS = "erd1qqqqqqqqqqqqqpgqfgayg3ykmn6jluazdfhka02y3q9vjc8wnc0syn2dwx"
POLL_INTERVAL = 5  # Seconds

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
        return None

def get_task_status(task_id):
    """Queries the contract for task status."""
    # Using mxpy for simplicity as it handles encoding
    cmd = (
        f"mxpy contract query {CONTRACT_ADDRESS} "
        f"--function=getTaskStatus "
        f"--arguments {task_id} "
        f"--proxy={PROXY}"
    )
    output = run_command(cmd)
    if output:
        # Parse the output. It usually returns a JSON-like structure or raw values.
        # For enum, it might return the index or name.
        # Let's assume we look for "Open" or "0" in the output.
        if "Open" in output or "0" in output: # simplistic check
            return "Open"
    return "Unknown"

def get_task_details(task_id):
    """Queries the contract for task details."""
    cmd = (
        f"mxpy contract query {CONTRACT_ADDRESS} "
        f"--function=getTask "
        f"--arguments {task_id} "
        f"--proxy={PROXY}"
    )
    output = run_command(cmd)
    # Parsing mxpy query output can be tricky without a proper parser.
    # For this MVP, we will assume the input data is passed as an argument to the script
    # or we parse the JSON output if mxpy returns JSON.
    return output

def run_docker_processor(image_name, input_data):
    """Runs the docker container to process data."""
    print(f"[*] Running Docker image '{image_name}' with input: {input_data}")
    
    # Ensure the image exists (build it if not - for this demo we assume it's built)
    # cmd = f"docker run --rm {image_name} '{input_data}'"
    
    # For the sake of this environment where I cannot run actual docker, 
    # I will simulate the docker run by calling the python script directly if image is 'local_processor'
    # In a real scenario, uncomment the docker command above.
    
    if image_name == "simple-processor":
        # Simulate docker run by running the script directly
        # Check if we are in the worker_node directory or root
        script_path = "processor.py" if os.path.exists("processor.py") else "worker_node/processor.py"
        cmd = f"python3 {script_path} '{input_data}'"
    else:
        cmd = f"docker run --rm {image_name} '{input_data}'"

    output = run_command(cmd)
    if output:
        print(f"[*] Container Output: {output}")
        return output
    else:
        print("[!] Docker execution failed")
        return None

def submit_result(task_id, result_hash, pem_file):
    """Submits the result to the smart contract."""
    print(f"[*] Submitting result for Task #{task_id}...")
    
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
        print(f"[+] Result submitted! Tx Hash: {output}")
    else:
        print("[-] Submission failed.")

def main():
    parser = argparse.ArgumentParser(description='On-Chain Signal Worker Node')
    parser.add_argument('--wallet', type=str, required=True, help='Path to the worker wallet PEM file')
    parser.add_argument('--image', type=str, default="simple-processor", help='Docker image to run')
    args = parser.parse_args()

    if not os.path.exists(args.wallet):
        print(f"Error: Wallet file not found at {args.wallet}")
        sys.exit(1)

    print(f"--- Starting Worker Node (Wallet: {args.wallet}) ---")
    print(f"--- Listening for Open tasks on {CONTRACT_ADDRESS} ---")

    # Keep track of processed tasks to avoid double submission
    processed_tasks = set()

    while True:
        # In a real app, we would listen to events. 
        # Here we poll a range of tasks (e.g., 0 to 10) for demonstration.
        for task_id in range(0, 10):
            if task_id in processed_tasks:
                continue

            # Check status
            # Note: This is inefficient (polling). Better to use an indexer or websocket.
            status = get_task_status(task_id)
            
            if status == "Open":
                print(f"[!] Found Open Task #{task_id}")
                
                # Fetch details (Simulated: We assume input data is standard for this demo)
                # In reality, we would parse the 'getTask' output to get 'input_data_uri'
                # For this demo, let's use a hardcoded input that matches our processor
                input_data = json.dumps({"a": [1, 2, 3], "b": [4, 5, 6]})
                
                # Run Processor
                result = run_docker_processor(args.image, input_data)
                
                if result:
                    # Hash the result
                    result_hash = hashlib.sha256(result.encode()).hexdigest()
                    
                    # Submit
                    submit_result(task_id, result_hash, args.wallet)
                    
                    processed_tasks.add(task_id)
            
            time.sleep(0.5) # Small delay between checks
        
        print("[*] Polling cycle complete. Waiting...")
        time.sleep(POLL_INTERVAL)

if __name__ == "__main__":
    main()
