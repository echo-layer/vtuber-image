import requests
import json
import uuid
import boto3
import os
from dotenv import load_dotenv

class ComfyClient:
    def __init__(self, server_address="http://localhost:8188"):
        load_dotenv()
        self.server_address = server_address
        self.client_id = str(uuid.uuid4())
        self.s3 = boto3.client('s3')

    def fetch_template(self, bucket, key):
        response = self.s3.get_object(Bucket=bucket, Key=key)
        return json.loads(response['Body'].read().decode('utf-8'))

    def inject_overrides(self, workflow_json, overrides):
        for node_id, node in workflow_json.items():
            class_type = node.get('class_type', '')
            title = node.get('_meta', {}).get('title', '')
            
            # Check if the node is a prompt or text node based on title or class
            if any(term in class_type for term in ["Prompt", "Text"]) or \
               any(term in title for term in ["Prompt", "Text"]):
                
                if 'inputs' in node:
                    for input_key, input_value in node['inputs'].items():
                        if isinstance(input_value, str):
                            for key, value in overrides.items():
                                placeholder = f"{{{{{key}}}}}"
                                if placeholder in input_value:
                                    input_value = input_value.replace(placeholder, value)
                            node['inputs'][input_key] = input_value
        return workflow_json

    def queue_prompt(self, prompt):
        p = {"prompt": prompt, "client_id": self.client_id}
        data = json.dumps(p).encode('utf-8')
        response = requests.post(f"{self.server_address}/prompt", data=data)
        return response.json()

if __name__ == "__main__":
    client = ComfyClient()
    print(f"Client initialized with ID: {client.client_id}")
