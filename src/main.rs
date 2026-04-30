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
