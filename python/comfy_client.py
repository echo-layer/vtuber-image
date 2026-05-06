import requests
import json
import uuid
import boto3
import os
import time
from dotenv import load_dotenv

class ComfyClient:
    def __init__(self, server_address="http://localhost:8188"):
        load_dotenv()
        self.server_address = server_address
        self.client_id = str(uuid.uuid4())
        
        # S3 / SeaweedFS Configuration
        s3_endpoint = os.getenv("S3_ENDPOINT_URL", "http://localhost:8333")
        s3_access_key = os.getenv("S3_ACCESS_KEY", "any")
        s3_secret_key = os.getenv("S3_SECRET_KEY", "any")
        
        self.s3 = boto3.client(
            's3',
            endpoint_url=s3_endpoint,
            aws_access_key_id=s3_access_key,
            aws_secret_access_key=s3_secret_key
        )

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

    def wait_for_completion(self, prompt_id):
        while True:
            response = requests.get(f"{self.server_address}/history/{prompt_id}")
            history = response.json()
            if prompt_id in history:
                outputs = history[prompt_id].get("outputs", {})
                for node_id in outputs:
                    node_output = outputs[node_id]
                    if "images" in node_output:
                        return node_output["images"][0]["filename"]
            time.sleep(1)

    def upload_result(self, local_filename, target_bucket, target_key):
        # Fetch image bytes from ComfyUI
        response = requests.get(f"{self.server_address}/view?filename={local_filename}")
        image_bytes = response.content
        
        # Upload to S3/SeaweedFS
        self.s3.put_object(
            Bucket=target_bucket,
            Key=target_key,
            Body=image_bytes,
            ContentType='image/png'
        )
        return f"s3://{target_bucket}/{target_key}"

if __name__ == "__main__":
    client = ComfyClient()
    print(f"Client initialized with ID: {client.client_id}")
