import requests
import json

url = "http://localhost:8090/web/dataset/call_kw"
payload = {
    "jsonrpc": "2.0",
    "method": "call",
    "params": {
        "model": "slide.channel",
        "method": "get_views",
        "args": [],
        "kwargs": {
            "views": [[None, "list"], [None, "form"]],
            "options": {}
        }
    },
    "id": 1
}

print(f"Sending POST to {url}...")
try:
    response = requests.post(url, json=payload, headers={"Content-Type": "application/json"})
    print(f"Status Code: {response.status_code}")
    res_data = response.json()
    if "error" in res_data:
        print("Error response:")
        print(json.dumps(res_data["error"], indent=2))
    elif "result" in res_data:
        print("Success! Fields list:")
        result = res_data["result"]
        views = result.get("views", {})
        models = result.get("models", {})
        
        print(f"Models returned: {list(models.keys())}")
        fields = models.get("slide.channel", {}).get("fields", {})
        print(f"Number of fields for slide.channel: {len(fields)}")
        print(f"Sample fields: {list(fields.keys())[:10]}")
        
        for vtype, vdata in views.items():
            print(f"\n--- View Type: {vtype} ---")
            print(f"ID: {vdata.get('id')}")
            print(f"Name: {vdata.get('name')}")
            print("Architecture:")
            print(vdata.get("arch"))
    else:
        print("Unknown format:", res_data)
except Exception as e:
    print("Request failed:", e)
