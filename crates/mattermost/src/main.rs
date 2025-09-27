#[tokio::main]
pub async fn main() -> Result<(), mattermost::Error> {
    mattermost::initialize().await?;

    mattermost::run().await?;

    Ok(())
}
