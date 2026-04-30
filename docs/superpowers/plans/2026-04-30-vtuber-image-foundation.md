# vtuber-image Core Foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the foundation of vtuber-image: a gRPC service (Rust) that triggers image generation via ComfyUI (Python client) and stores results in SeaweedFS.

**Architecture:** A Rust frontend (`tonic`) receives gRPC requests, selects a workflow template, and invokes a Python worker/client to talk to ComfyUI. The final image is uploaded to SeaweedFS (S3-compatible) and a URL is returned.

**Tech Stack:** Rust (tonic, tokio), Python (requests/httpx), SeaweedFS, ComfyUI REST API.

---

### Task 1: Define gRPC Interface

**Files:**
- Create: `proto/vtuber_image/v1/image.proto`

- [ ] **Step 1: Create the proto file**

```proto
syntax = "proto3";
package vtuber_image.v1;

service ImageGenerator {
  rpc Generate(GenerationRequest) returns (GenerationResponse);
}

message GenerationRequest {
  string persona_id = 1;
  PersonaOverrides overrides = 2;
}

message PersonaOverrides {
  string hair_style = 1;
  string eye_color = 2;
  string outfit = 3;
}

message GenerationResponse {
  string image_url = 1;
  map<string, string> metadata = 2;
}
```

- [ ] **Step 2: Commit**

```bash
git add proto/vtuber_image/v1/image.proto
git commit -m "feat: define gRPC interface for image generation"
```

---

### Task 2: Initialize Rust Project

**Files:**
- Create: `Cargo.toml`
- Create: `build.rs`
- Create: `src/main.rs`

- [ ] **Step 1: Create Cargo.toml**

```toml
[package]
name = "vtuber-image"
version = "0.1.0"
edition = "2021"

[dependencies]
tonic = "0.11"
prost = "0.12"
tokio = { version = "1.0", features = ["full"] }
tokio-stream = "0.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"

[build-dependencies]
tonic-build = "0.11"
```

- [ ] **Step 2: Create build.rs to compile proto**

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(
            &["proto/vtuber_image/v1/image.proto"],
            &["proto"],
        )?;
    Ok(())
}
```

- [ ] **Step 3: Create minimal src/main.rs**

```rust
use tonic::{transport::Server, Request, Response, Status};
use vtuber_image::v1::image_generator_server::{ImageGenerator, ImageGeneratorServer};
use vtuber_image::v1::{GenerationRequest, GenerationResponse};

pub mod vtuber_image {
    pub mod v1 {
        tonic::include_proto!("vtuber_image.v1");
    }
}

#[derive(Default)]
pub struct MyImageGenerator {}

#[tonic::async_trait]
impl ImageGenerator for MyImageGenerator {
    async fn generate(
        &self,
        request: Request<GenerationRequest>,
    ) -> Result<Response<GenerationResponse>, Status> {
        let req = request.into_inner();
        println!("Received request for persona: {}", req.persona_id);

        let reply = GenerationResponse {
            image_url: "http://placeholder.com/image.png".to_string(),
            metadata: std::collections::HashMap::new(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:8083".parse()?;
    let generator = MyImageGenerator::default();

    println!("ImageGenerator server listening on {}", addr);

    Server::builder()
        .add_service(ImageGeneratorServer::new(generator))
        .serve(addr)
        .await?;

    Ok(())
}
```

- [ ] **Step 4: Verify build**

Run: `cargo build`
Expected: Successful compilation.

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml build.rs src/main.rs
git commit -m "feat: initialize Rust gRPC server scaffolding"
```

---

### Task 3: Initialize Python ComfyUI Client

**Files:**
- Create: `python/requirements.txt`
- Create: `python/comfy_client.py`

- [ ] **Step 1: Create requirements.txt**

```text
requests==2.31.0
websocket-client==1.7.0
```

- [ ] **Step 2: Create comfy_client.py skeleton**

```python
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
```

- [ ] **Step 3: Commit**

```bash
git add python/requirements.txt python/comfy_client.py
git commit -m "feat: add Python ComfyUI client skeleton"
```

---

### Task 4: SeaweedFS Local Setup (Podman)

**Files:**
- Create: `docker-compose.yml`

- [ ] **Step 1: Create docker-compose.yml for SeaweedFS**

```yaml
version: '3'

services:
  master:
    image: chrislusf/seaweedfs
    ports:
      - 9333:9333
    command: "master -ip=master"
  volume:
    image: chrislusf/seaweedfs
    ports:
      - 8080:8080
    command: "volume -mserver=master:9333 -port=8080"
  s3:
    image: chrislusf/seaweedfs
    ports:
      - 8333:8333
    command: "s3 -master=master:9333"
```

- [ ] **Step 2: Verify start (optional if podman-compose is available)**

Run: `podman-compose up -d`
Note: If not available, just commit the file for later use.

- [ ] **Step 3: Commit**

```bash
git add docker-compose.yml
git commit -m "chore: add docker-compose for SeaweedFS"
```

---

### Task 5: Integration - Rust calling Python

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Update main.rs to use Command to call Python (simple first step)**

```rust
// ... inside generate implementation ...
let output = std::process::Command::new("python3")
    .arg("python/comfy_client.py")
    .output()
    .expect("failed to execute process");

println!("Python output: {:?}", String::from_utf8_lossy(&output.stdout));
```

- [ ] **Step 2: Commit**

```bash
git add src/main.rs
git commit -m "feat: simple bridge from Rust to Python worker"
```
