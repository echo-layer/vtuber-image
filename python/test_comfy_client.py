import unittest
from unittest.mock import MagicMock, patch
from comfy_client import ComfyClient
import json

class TestComfyClient(unittest.TestCase):
    def setUp(self):
        with patch('boto3.client'):
            self.client = ComfyClient()

    @patch('requests.get')
    def test_wait_for_completion(self, mock_get):
        # Mock history response
        prompt_id = "test_prompt_id"
        mock_history = {
            prompt_id: {
                "outputs": {
                    "9": {
                        "images": [{"filename": "test_image.png"}]
                    }
                }
            }
        }
        mock_get.return_value.json.return_value = mock_history
        
        filename = self.client.wait_for_completion(prompt_id)
        self.assertEqual(filename, "test_image.png")
        mock_get.assert_called_with(f"{self.client.server_address}/history/{prompt_id}")

    @patch('requests.get')
    def test_upload_result(self, mock_get):
        # Mock view response
        mock_get.return_value.content = b"fake_image_data"
        
        # Mock S3 put_object
        self.client.s3.put_object = MagicMock()
        
        result = self.client.upload_result("test_image.png", "test_bucket", "test_key")
        
        self.assertEqual(result, "s3://test_bucket/test_key")
        mock_get.assert_called_with(f"{self.client.server_address}/view?filename=test_image.png")
        self.client.s3.put_object.assert_called_once()
        args, kwargs = self.client.s3.put_object.call_args
        self.assertEqual(kwargs['Bucket'], "test_bucket")
        self.assertEqual(kwargs['Key'], "test_key")
        self.assertEqual(kwargs['Body'], b"fake_image_data")

if __name__ == '__main__':
    unittest.main()
