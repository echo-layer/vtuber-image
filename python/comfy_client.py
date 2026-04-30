import requests
import json
import uuid

class ComfyClient:
    def __init__(self, server_address="http://localhost:8188"):
        self.server_address = server_address
        self.client_id = str(uuid.uuid4())

    def queue_prompt(self, prompt):
        p = {"prompt": prompt, "client_id": self.client_id}
        data = json.dumps(p).encode('utf-8')
        response = requests.post(f"{self.server_address}/prompt", data=data)
        return response.json()

if __name__ == "__main__":
    client = ComfyClient()
    print(f"Client initialized with ID: {client.client_id}")
