#[tokio::main]
pub async fn main() -> Result<(), photo_viewer::Error> {
    photo_viewer::initialize().await?;
    photo_viewer::run().await?;
    Ok(())
}
