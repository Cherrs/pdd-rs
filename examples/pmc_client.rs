use futures_util::StreamExt;
use pdd::{
    pmc::{PmcClient, PmcConsumers},
    Config,
};
use tracing::{error, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    loop {
        let client = PmcClient::new(Config::from_env()?);
        let mut s = client.connect().await?;
        msg_handler(&mut s).await;
        error!("连接断开,5秒后重新连接");
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}

async fn msg_handler(s: &mut PmcConsumers) {
    while let Some(msg) = s.next().await {
        if msg.is_err() {
            return;
        }
        let msg = msg.unwrap();
        println!("{:?}", msg);
        s.ack(&msg).await;
    }
}
