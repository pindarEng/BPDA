import json
from http.server import BaseHTTPRequestHandler, HTTPServer
import argparse
import os
import sys
import subprocess
import hashlib

# Configuration
PROXY = "https://devnet-api.multiversx.com"
CONTRACT_ADDRESS = "erd1qqqqqqqqqqqqqpgqfgayg3ykmn6jluazdfhka02y3q9vjc8wnc0syn2dwx"
PORT = 5005

# Global wallet path
WALLET_PATH = ""

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

def run_docker_processor(image_name, input_data):
    """Runs the docker container to process data."""
    print(f"[*] Processing task with image '{image_name}'...")

    try:
        # 1. Attempt to pull the image
        # We try to pull to ensure we have the image or get the latest version.
        print(f"[*] Pulling image: {image_name}...")
        try:
            subprocess.run(
                ["docker", "pull", image_name], 
                check=True, 
                stdout=subprocess.PIPE, 
                stderr=subprocess.PIPE
            )
        except subprocess.CalledProcessError:
            print(f"[*] Pull failed or image is local. Proceeding to execution...")

        # 2. Run the container
        # --rm removes the container instance after it exits
        command = ["docker", "run", "--rm", image_name]
        
        if input_data and input_data.strip():
            command.append(input_data)
            
        print(f"[*] Executing container...")
        
        result = subprocess.run(
            command, 
            check=True, 
            stdout=subprocess.PIPE, 
            stderr=subprocess.PIPE,
            text=True
        )
        
        output = result.stdout.strip()
        print(f"[*] Container Output: {output}")

        # 3. Remove the image to save space
        # We don't check=True here because if it's in use or fails, we don't want to crash the worker response
        print(f"[*] Cleaning up image: {image_name}...")
        subprocess.run(
            ["docker", "rmi", image_name], 
            check=False, 
            stdout=subprocess.DEVNULL, 
            stderr=subprocess.DEVNULL
        )

        return output
        
    except subprocess.CalledProcessError as e:
        print(f"[!] Docker execution failed: {e.stderr}")
        # Attempt cleanup even on failure
        subprocess.run(["docker", "rmi", image_name], check=False, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        return None
    except FileNotFoundError:
        print("[!] Error: Docker executable not found. Is Docker installed?")
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
        return output
    else:
        print("[-] Submission failed.")
        return None

class WorkerHandler(BaseHTTPRequestHandler):
    def do_OPTIONS(self):
        self.send_response(200)
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        self.end_headers()

    def do_POST(self):
        if self.path == '/process_task':
            content_length = int(self.headers['Content-Length'])
            post_data = self.rfile.read(content_length)
            
            try:
                data = json.loads(post_data)
                task_id = data.get('taskId')
                image = data.get('image')
                input_data = data.get('inputData')
                
                print(f"\n[Request] Processing Task #{task_id}")
                
                # 1. Run Processor
                result = run_docker_processor(image, input_data)
                
                if result:
                    # 2. Hash Result
                    result_hash = hashlib.sha256(result.encode()).hexdigest()
                    
                    # 3. Submit Transaction
                    tx_hash = submit_result(task_id, result_hash, WALLET_PATH)
                    
                    if tx_hash:
                        self.send_response(200)
                        self.send_header('Access-Control-Allow-Origin', '*')
                        self.send_header('Content-Type', 'application/json')
                        self.end_headers()
                        self.wfile.write(json.dumps({"status": "success", "txHash": tx_hash}).encode())
                        return

                self.send_error(500, "Processing failed")
                
            except Exception as e:
                print(f"Error: {e}")
                self.send_error(500, str(e))
        else:
            self.send_error(404)

def main():
    global WALLET_PATH
    parser = argparse.ArgumentParser(description='On-Demand Worker Server')
    parser.add_argument('--wallet', type=str, required=True, help='Path to the worker wallet PEM file')
    args = parser.parse_args()

    if not os.path.exists(args.wallet):
        print(f"Error: Wallet file not found at {args.wallet}")
        sys.exit(1)
        
    WALLET_PATH = args.wallet

    print(f"--- Starting Worker Server on port {PORT} ---")
    print(f"--- Wallet: {WALLET_PATH} ---")
    
    server = HTTPServer(('localhost', PORT), WorkerHandler)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    server.server_close()

if __name__ == "__main__":
    main()
