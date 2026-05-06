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

        // Simple bridge to Python worker
        let output = std::process::Command::new("python3")
            .arg("python/comfy_client.py")
            .output()
            .map_err(|e| Status::internal(format!("Failed to execute python worker: {}", e)))?;

        println!("Python output: {:?}", String::from_utf8_lossy(&output.stdout));

        let reply = GenerateResponse {
            image_url: "http://placeholder.com/image.png".to_string(),
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
