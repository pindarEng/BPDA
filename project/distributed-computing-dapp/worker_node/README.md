# Distributed Computing Worker Node

This folder contains the worker node implementation that listens to the blockchain for tasks and processes them using Docker.

## Prerequisites

- Python 3.x
- Docker installed and running
- `mxpy` installed and configured

## Setup

1. **Build the Docker Image**
   The worker expects a docker image named `simple-processor`.
   ```bash
   cd worker_node
   docker build -t simple-processor .
   ```

2. **Prepare Wallet**
   Ensure you have a wallet PEM file (e.g., `wallet.pem`).

## Running the Worker

### Option 1: Auto-Pilot (Polls for tasks)

Run the worker script. It will poll the Devnet for Open tasks (IDs 0-10) and process them.

```bash
python3 on_chain_worker.py --wallet ../wallet/new_WORKER_1.pem
```

### Option 2: On-Demand Server (Triggered from UI)

Start the worker server. This allows you to click "Run Worker" in the dApp dashboard to trigger processing on your machine.

```bash
python3 worker_server.py --wallet ../wallet/new_WORKER_1.pem
```

3. **Malicious**

Run this:
```bash 
python3 worker_node/malicious_worker.py --task-id {YOU CHOOSE THIS} --wallet wallet/new_WORKER_{YOU CHOOSE THIS}.pem
```
The server runs on `http://localhost:5005`.

## How it works

1. The script polls the smart contract for tasks with status `Open`.
2. When found, it runs the `simple-processor` Docker container (or simulates it if Docker isn't available).
3. The processor multiplies two lists (hardcoded input for this demo).
4. The result is hashed and submitted back to the smart contract via `submitResult`.
