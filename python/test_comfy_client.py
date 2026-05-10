import unittest
from unittest.mock import MagicMock, patch, mock_open
from comfy_client import ComfyClient, SecurityVerificationError, compute_sha256
import json
import os

class TestComfyClient(unittest.TestCase):
    def setUp(self):
        with patch('boto3.client'):
            self.client = ComfyClient()

    def test_compute_sha256(self):
        content = b"hello world"
        # hash of "hello world" is 
        # b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
        expected = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        
        with patch("builtins.open", mock_open(read_data=content)):
            result = compute_sha256("dummy_path")
            self.assertEqual(result, expected)

    @patch('requests.get')
    @patch('os.path.exists')
    @patch('comfy_client.compute_sha256')
    def test_verify_model_success(self, mock_hash, mock_exists, mock_get):
        # 1. Mock Civitai response
        mock_response = MagicMock()
        mock_response.status_code = 200
        mock_response.json.return_value = {
            "nsfw": False,
            "modelVersions": [
                {
                    "files": [
                        {
                            "name": "model.safetensors",
                            "hashes": {
                                "SHA256": "ABCDEF123456"
                            }
                        }
                    ]
                }
            ]
        }
        mock_get.return_value = mock_response
        
        # 2. Mock Disk checks
        mock_exists.return_value = True
        mock_hash.return_value = "abcdef123456"
        
        result = self.client.verify_model("123", "abcdef123456", False)
        self.assertTrue(result)
        mock_exists.assert_called_with(os.path.join("models", "checkpoints", "model.safetensors"))

    @patch('requests.get')
    def test_verify_model_nsfw_rejected(self, mock_get):
        mock_response = MagicMock()
        mock_response.status_code = 200
        mock_response.json.return_value = {
            "nsfw": True,
            "modelVersions": []
        }
        mock_get.return_value = mock_response
        
        with self.assertRaises(SecurityVerificationError) as cm:
            self.client.verify_model("123", "hash", False)
        self.assertEqual(cm.exception.reason, "SEC_FAIL_NSFW")

    @patch('requests.get')
    def test_verify_model_wrong_format(self, mock_get):
        mock_response = MagicMock()
        mock_response.status_code = 200
        mock_response.json.return_value = {
            "nsfw": False,
            "modelVersions": [
                {
                    "files": [
                        {
                            "name": "model.pt",
                            "hashes": {
                                "SHA256": "ABCDEF123456"
                            }
                        }
                    ]
                }
            ]
        }
        mock_get.return_value = mock_response
        
        with self.assertRaises(SecurityVerificationError) as cm:
            self.client.verify_model("123", "abcdef123456", False)
        self.assertEqual(cm.exception.reason, "SEC_FAIL_FORMAT")

    @patch('requests.get')
    @patch('os.path.exists')
    @patch('comfy_client.compute_sha256')
    def test_verify_model_disk_hash_mismatch(self, mock_hash, mock_exists, mock_get):
        mock_response = MagicMock()
        mock_response.status_code = 200
        mock_response.json.return_value = {
            "nsfw": False,
            "modelVersions": [
                {
                    "files": [
                        {
                            "name": "model.safetensors",
                            "hashes": {
                                "SHA256": "ABCDEF123456"
                            }
                        }
                    ]
                }
            ]
        }
        mock_get.return_value = mock_response
        mock_exists.return_value = True
        mock_hash.return_value = "WRONG_DISK_HASH"
        
        with self.assertRaises(SecurityVerificationError) as cm:
            self.client.verify_model("123", "abcdef123456", False)
        self.assertEqual(cm.exception.reason, "SEC_FAIL_HASH")

    @patch('requests.get')
    @patch('os.path.exists')
    def test_verify_model_missing_file(self, mock_exists, mock_get):
        mock_response = MagicMock()
        mock_response.status_code = 200
        mock_response.json.return_value = {
            "nsfw": False,
            "modelVersions": [
                {
                    "files": [
                        {
                            "name": "model.safetensors",
                            "hashes": {
                                "SHA256": "ABCDEF123456"
                            }
                        }
                    ]
                }
            ]
        }
        mock_get.return_value = mock_response
        mock_exists.return_value = False
        
        with self.assertRaises(SecurityVerificationError) as cm:
            self.client.verify_model("123", "abcdef123456", False)
        self.assertEqual(cm.exception.reason, "SEC_FAIL_MISSING")

    # (Other existing tests kept and adapted if necessary)
    @patch('requests.get')
    def test_wait_for_completion(self, mock_get):
        prompt_id = "test_prompt_id"
        mock_history = {
            prompt_id: {
                "outputs": {"9": {"images": [{"filename": "test_image.png"}]}}
            }
        }
        mock_get.return_value.json.return_value = mock_history
        filename = self.client.wait_for_completion(prompt_id)
        self.assertEqual(filename, "test_image.png")

    @patch('requests.get')
    def test_upload_result(self, mock_get):
        mock_get.return_value.content = b"fake_image_data"
        self.client.s3.put_object = MagicMock()
        result = self.client.upload_result("test_image.png", "test_bucket", "test_key")
        self.assertEqual(result, "s3://test_bucket/test_key")

if __name__ == '__main__':
    unittest.main()
