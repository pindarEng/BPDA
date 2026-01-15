import sys
import json
import argparse
import urllib.request

def clean_data(data):
    """
    Expects data to be a list of dictionaries or a dictionary with lists.
    We will implement a generic cleaner for a list of records.
    """
    cleaned = []
    seen = set()
    
    # If the input is wrapped in a key like {"data": [...]}, extract it
    records = data
    if isinstance(data, dict):
        # heuristics to find the list
        for k, v in data.items():
            if isinstance(v, list):
                records = v
                break
    
    if not isinstance(records, list):
        return {"error": "Input data must be a list of records or contain a list"}

    for item in records:
        if not isinstance(item, dict):
            continue
            
        # 1. Prune Nulls (remove keys with null values? or remove record if any value is null?)
        # Let's say we remove the record if ANY critical field is null, 
        # or just remove keys that are null.
        # The user said "prune them", usually means removing the observation.
        # Let's remove observations with ANY None/null values for simplicity.
        if any(v is None for v in item.values()):
            continue
            
        # 2. Prune Duplicates
        # To hash a dict, we convert to sorted tuple of items
        # JSON types: str, int, float, bool, None. All hashable except lists/dicts.
        try:
            item_hash = tuple(sorted((k, str(v)) for k, v in item.items()))
        except Exception:
            # If nested structures exist, skip complex dedup for this simple script
            # and just keep it.
            cleaned.append(item)
            continue
            
        if item_hash in seen:
            continue
            
        seen.add(item_hash)
        cleaned.append(item)
        
    return cleaned

def fetch_data(uri):
    if uri.startswith("http://") or uri.startswith("https://"):
        with urllib.request.urlopen(uri) as response:
            return json.loads(response.read().decode())
    else:
        # Assume it's a raw JSON string
        return json.loads(uri)

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    # We accept input either as a direct JSON string arg OR we might need to fetch it 
    # if the container is run with a URL argument.
    # However, for simplicity, let's assume the caller (worker) has handled the fetching
    # OR we handle it here if passed a URL.
    parser.add_argument("input_source", help="JSON string or URL")
    args = parser.parse_args()
    
    try:
        data = fetch_data(args.input_source)
        result = clean_data(data)
        print(json.dumps(result))
    except Exception as e:
        print(json.dumps({"error": str(e)}))
