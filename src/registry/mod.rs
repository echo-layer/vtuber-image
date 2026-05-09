pub mod client;

pub use client::OCIClient;

pub struct SmartCache {
    pub cache_dir: std::path::PathBuf,
}

impl SmartCache {
    pub fn new(cache_dir: std::path::PathBuf) -> Self {
        Self { cache_dir }
    }

    pub fn get_cache_path(&self, image_url: &str) -> std::path::PathBuf {
        let cache_filename = format!("{}.json", image_url.replace("/", "_").replace(":", "_"));
        self.cache_dir.join(cache_filename)
    }
}
