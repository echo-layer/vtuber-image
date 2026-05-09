use anyhow::Context;
use oci_distribution::{client::Client, Reference};
use std::path::PathBuf;
use tokio::fs;

pub struct OCIClient {
    client: Client,
    cache_dir: PathBuf,
}

impl OCIClient {
    pub fn new(cache_dir: PathBuf) -> Self {
        Self {
            client: Client::default(),
            cache_dir,
        }
    }

    pub async fn pull_workflow(&self, image_url: &str) -> anyhow::Result<String> {
        let reference: Reference = image_url.parse().context("Failed to parse image URL")?;

        // Smart Cache Logic: Use sanitized image_url as filename
        let cache_filename = format!("{}.json", image_url.replace("/", "_").replace(":", "_"));
        let cache_path = self.cache_dir.join(cache_filename);

        if cache_path.exists() {
            println!("Cache hit for workflow: {}", image_url);
            return fs::read_to_string(cache_path)
                .await
                .context("Failed to read cached workflow");
        }

        println!(
            "Cache miss for workflow: {}, pulling from registry...",
            image_url
        );

        let auth = oci_distribution::secrets::RegistryAuth::Anonymous;

        // Pull the image data
        // For simplicity, we pull the manifest and then the blobs (layers)
        let image_data = self
            .client
            .pull(
                &reference,
                &auth,
                vec![
                    "application/vnd.oci.image.layer.v1.tar+gzip",
                    "application/vnd.docker.image.rootfs.diff.tar.gzip",
                ],
            )
            .await
            .context("Failed to pull image from OCI registry")?;

        let mut workflow_content = None;
        for layer in image_data.layers {
            if let Ok(content) = String::from_utf8(layer.data) {
                if content.trim().starts_with('{') {
                    workflow_content = Some(content);
                    break;
                }
            }
        }

        let content = workflow_content
            .ok_or_else(|| anyhow::anyhow!("workflow.json not found in OCI image layers"))?;

        // Save to cache
        if let Some(parent) = cache_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        fs::write(&cache_path, &content).await?;

        Ok(content)
    }
}
