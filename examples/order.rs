use pdd::{requests::PddOrderListGet, Client};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::from_env()?;
    let req = PddOrderListGet {
        start_confirm_at: Some(1689250974),
        end_confirm_at: Some(1689260974),
        use_has_next: Some(true),
        order_status: Some(1),
        // refund_status: Some(5),
        ..Default::default()
    };
    client.send(req).await?;
    Ok(())
}
