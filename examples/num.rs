use pdd::{requests::PddLogisticsCompaniesGet, Client};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::from_env()?;
    let req = PddLogisticsCompaniesGet {};
    client.exec(req).await?;
    Ok(())
}
