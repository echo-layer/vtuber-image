use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, RwLock};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelEntry {
    pub model_id: String,
    pub hash: String,
    pub license: String,
    pub allow_nsfw: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersonaAssets {
    pub image_registry: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersonaConfig {
    pub assets: PersonaAssets,
}

#[derive(Debug, Clone)]
pub struct GuardCache {
    model_cache: Arc<RwLock<HashMap<String, ModelEntry>>>,
    persona_cache: Arc<RwLock<HashMap<String, PersonaConfig>>>,
}

impl Default for GuardCache {
    fn default() -> Self {
        Self::new()
    }
}

impl GuardCache {
    pub fn new() -> Self {
        Self {
            model_cache: Arc::new(RwLock::new(HashMap::new())),
            persona_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn load_from_file<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let content = fs::read_to_string(path)?;
        let entries: Vec<ModelEntry> = serde_json::from_str(&content)?;

        let mut cache = self
            .model_cache
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire write lock"))?;
        cache.clear();
        for entry in entries {
            cache.insert(entry.model_id.clone(), entry);
        }

        Ok(())
    }

    pub fn get_model(&self, model_id: &str) -> Option<ModelEntry> {
        let cache = self.model_cache.read().ok()?;
        cache.get(model_id).cloned()
    }

    pub fn get_persona(&self, persona_id: &str) -> Option<PersonaConfig> {
        let cache = self.persona_cache.read().ok()?;
        cache.get(persona_id).cloned()
    }

    pub fn insert_persona(&self, persona_id: String, config: PersonaConfig) {
        if let Ok(mut cache) = self.persona_cache.write() {
            cache.insert(persona_id, config);
        }
    }
}
