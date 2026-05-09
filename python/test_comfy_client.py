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

    @patch('requests.get')
    def test_verify_model_success(self, mock_get):
        mock_response = MagicMock()
        mock_response.status_code = 200
        mock_response.json.return_value = {
            "nsfw": False,
            "modelVersions": [
                {
                    "files": [
                        {
                            "hashes": {
                                "SHA256": "ABCDEF123456"
                            }
                        }
                    ]
                }
            ]
        }
        mock_get.return_value = mock_response
        
        result = self.client.verify_model("123", "abcdef123456", False)
        self.assertTrue(result)

    @patch('requests.get')
    def test_verify_model_nsfw_rejected(self, mock_get):
        mock_response = MagicMock()
        mock_response.status_code = 200
        mock_response.json.return_value = {
            "nsfw": True,
            "modelVersions": []
        }
        mock_get.return_value = mock_response
        
        with self.assertRaisesRegex(Exception, "Model 123 is marked as NSFW"):
            self.client.verify_model("123", "hash", False)

    @patch('requests.get')
    def test_verify_model_hash_mismatch(self, mock_get):
        mock_response = MagicMock()
        mock_response.status_code = 200
        mock_response.json.return_value = {
            "nsfw": False,
            "modelVersions": [
                {
                    "files": [
                        {
                            "hashes": {
                                "SHA256": "WRONGHASH"
                            }
                        }
                    ]
                }
            ]
        }
        mock_get.return_value = mock_response
        
        with self.assertRaisesRegex(Exception, "SHA256 hash mismatch"):
            self.client.verify_model("123", "expectedhash", False)

if __name__ == '__main__':
    unittest.main()
