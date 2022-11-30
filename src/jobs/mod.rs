pub mod browser;

#[poise::async_trait]
pub trait Job {
    async fn start(self) -> anyhow::Result<()>;
}
