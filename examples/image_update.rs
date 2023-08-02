use pdd::{requests::PddGoodsFilespaceImageUpload, Client, PddFile};
use tracing::trace;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let client = Client::from_env()?;
    let req = PddGoodsFilespaceImageUpload {
        file: Some(PddFile::from_file("examples/1.jpg").await?),
    };
    let rsp = client.file_upload(req).await?;
    trace!("{:?}", rsp);
    Ok(())
}
