import sys
import json
import argparse

def process_data(input_data):
    try:
        # Expecting input_data to be a JSON string like '{"a": [1, 2], "b": [3, 4]}'
        data = json.loads(input_data)
        list_a = data.get('a', [])
        list_b = data.get('b', [])
        
        if len(list_a) != len(list_b):
            return json.dumps({"error": "Lists must be of same length"})
            
        # Element-wise multiplication
        result = [a * b for a, b in zip(list_a, list_b)]
        return json.dumps({"c": result})
    except Exception as e:
        return json.dumps({"error": str(e)})

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("input_data", help="JSON input string")
    args = parser.parse_args()
    
    result = process_data(args.input_data)
    print(result)
