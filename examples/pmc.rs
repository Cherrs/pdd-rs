use pdd::{requests::PddPmcUserPermit, Client};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::from_env()?;
    let req = PddPmcUserPermit {
        ..Default::default()
    };
    client.exec(req).await?;
    Ok(())
}
