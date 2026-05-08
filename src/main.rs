use notify::{Event, RecursiveMode, Watcher};
use std::io::Write;
use std::path::Path;
use std::process::Stdio;
use tonic::{transport::Server, Request, Response, Status};
use vtuber_image::v1::image_generator_service_server::{
    ImageGeneratorService, ImageGeneratorServiceServer,
};
use vtuber_image::v1::{GenerateRequest, GenerateResponse};

pub mod guard;

pub mod vtuber_image {
    pub mod v1 {
        tonic::include_proto!("vtuber_image.v1");
    }
}

pub struct MyImageGeneratorService {
    pub guard_cache: guard::cache::GuardCache,
}

#[tonic::async_trait]
impl ImageGeneratorService for MyImageGeneratorService {
    async fn generate(
        &self,
        request: Request<GenerateRequest>,
    ) -> Result<Response<GenerateResponse>, Status> {
        let req = request.into_inner();
        println!("Received request for persona: {}", req.persona_id);

        // Task 2: Rust gRPC Enforcement
        // Placeholder check: Is the persona_id in the allowlist cache?
        if self.guard_cache.get_model(&req.persona_id).is_none() {
            return Err(Status::permission_denied(format!(
                "Requested configuration (persona: {}) is not in the allowlist",
                req.persona_id
            )));
        }

        let input_payload = serde_json::json!({
            "template_bucket": std::env::var("S3_BUCKET_TEMPLATES").unwrap_or_else(|_| "templates".to_string()),
            "template_key": format!("{}.json", req.persona_id),
            "overrides": {
                "hair_style": req.overrides.as_ref().map(|o| o.hair_style.clone()).unwrap_or_default(),
                "eye_color": req.overrides.as_ref().map(|o| o.eye_color.clone()).unwrap_or_default(),
                "outfit": req.overrides.as_ref().map(|o| o.outfit.clone()).unwrap_or_default(),
            },
            "output_bucket": std::env::var("S3_BUCKET_OUTPUTS").unwrap_or_else(|_| "outputs".to_string()),
            "output_key": format!("{}.png", uuid::Uuid::new_v4()),
        });

        let mut child = std::process::Command::new("python3")
            .arg("python/comfy_client.py")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| Status::internal(format!("Failed to spawn python worker: {}", e)))?;

        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        std::thread::spawn(move || {
            stdin
                .write_all(input_payload.to_string().as_bytes())
                .expect("Failed to write to stdin");
        });

        let output = child
            .wait_with_output()
            .map_err(|e| Status::internal(format!("Failed to wait for python worker: {}", e)))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let last_line = stdout.lines().last().unwrap_or_default();

        let reply = GenerateResponse {
            image_url: last_line.to_string(),
            metadata: std::collections::HashMap::new(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:8083".parse()?;

    let guard_cache = guard::cache::GuardCache::new();

    let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config".to_string());
    let allowlist_path = Path::new(&config_path).join("allowlist.json");

    // Ensure config directory exists
    if let Some(parent) = allowlist_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Initial load if exists
    if allowlist_path.exists() {
        println!("Loading initial allowlist from {:?}", allowlist_path);
        if let Err(e) = guard_cache.load_from_file(&allowlist_path) {
            eprintln!("Failed to load initial allowlist: {}", e);
        }
    } else {
        println!(
            "Allowlist file not found at {:?}, starting with empty cache",
            allowlist_path
        );
    }

    let cache_clone = guard_cache.clone();
    let allowlist_path_clone = allowlist_path.clone();

    let (tx, mut rx) = tokio::sync::mpsc::channel(1);

    let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| {
        if let Ok(event) = res {
            if event.kind.is_modify() {
                let _ = tx.blocking_send(());
            }
        }
    })?;

    if allowlist_path.exists() {
        watcher.watch(&allowlist_path, RecursiveMode::NonRecursive)?;
    } else if let Some(parent) = allowlist_path.parent() {
        // Watch the parent directory if the file doesn't exist yet
        watcher.watch(parent, RecursiveMode::NonRecursive)?;
    }

    tokio::spawn(async move {
        while let Some(_) = rx.recv().await {
            println!("Allowlist file change detected, reloading...");
            if let Err(e) = cache_clone.load_from_file(&allowlist_path_clone) {
                eprintln!("Failed to reload allowlist: {}", e);
            }
        }
    });

    let generator = MyImageGeneratorService { guard_cache };

    println!("ImageGeneratorService server listening on {}", addr);

    Server::builder()
        .add_service(ImageGeneratorServiceServer::new(generator))
        .serve(addr)
        .await?;

    Ok(())
}
