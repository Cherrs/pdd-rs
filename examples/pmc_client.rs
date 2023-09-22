use futures_util::StreamExt;
use pdd::{
    pmc::{CommandType, PmcClient},
    Config,
};
use tracing::{trace, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let client = PmcClient::new(Config::from_env()?);
    let mut s = client.connect().await?;
    // let aaa = Arc::new(Mutex::new(s));

    while let Some(msg) = s.next().await {
        let msg = msg.unwrap();
        println!("{:?}", msg);
        s.ack(&msg).await;
    }
    Ok(())
}
