fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(
            &["proto/vtuber_image/v1/image.proto"],
            &["proto"],
        )?;
    Ok(())
}
