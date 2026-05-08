use std::io::Write;
use std::process::Stdio;
use tonic::{transport::Server, Request, Response, Status};
use vtuber_image::v1::image_generator_service_server::{
    ImageGeneratorService, ImageGeneratorServiceServer,
};
use vtuber_image::v1::{GenerateRequest, GenerateResponse};

pub mod vtuber_image {
    pub mod v1 {
        tonic::include_proto!("vtuber_image.v1");
    }
}

#[derive(Default)]
pub struct MyImageGeneratorService {}

#[tonic::async_trait]
impl ImageGeneratorService for MyImageGeneratorService {
    async fn generate(
        &self,
        request: Request<GenerateRequest>,
    ) -> Result<Response<GenerateResponse>, Status> {
        let req = request.into_inner();
        println!("Received request for persona: {}", req.persona_id);

        let overrides = req.overrides.unwrap_or_default();
        let input_json = serde_json::json!({
            "template_bucket": std::env::var("S3_BUCKET_TEMPLATES").unwrap_or_else(|_| "templates".to_string()),
            "template_key": format!("{}.json", req.persona_id),
            "overrides": {
                "hair_style": overrides.hair_style,
                "eye_color": overrides.eye_color,
                "outfit": overrides.outfit,
            },
            "output_bucket": std::env::var("S3_BUCKET_OUTPUTS").unwrap_or_else(|_| "outputs".to_string()),
            "output_key": format!("{}/base.png", req.persona_id),
        });

        // Bridge to Python worker with stdin
        let mut child = std::process::Command::new("python3")
            .arg("python/comfy_client.py")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| Status::internal(format!("Failed to spawn python worker: {}", e)))?;

        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| Status::internal("Failed to open stdin"))?;

        let input_str = input_json.to_string();
        stdin
            .write_all(input_str.as_bytes())
            .map_err(|e| Status::internal(format!("Failed to write to stdin: {}", e)))?;
        drop(stdin);

        let output = child
            .wait_with_output()
            .map_err(|e| Status::internal(format!("Failed to wait for python worker: {}", e)))?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(Status::internal(format!("Python worker failed: {}", err)));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let image_url = stdout.lines().last().unwrap_or("").trim().to_string();

        println!("Generated image URL: {}", image_url);

        let reply = GenerateResponse {
            image_url,
            metadata: std::collections::HashMap::new(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:8083".parse()?;
    let generator = MyImageGeneratorService::default();

    println!("ImageGeneratorService server listening on {}", addr);

    Server::builder()
        .add_service(ImageGeneratorServiceServer::new(generator))
        .serve(addr)
        .await?;

    Ok(())
}
