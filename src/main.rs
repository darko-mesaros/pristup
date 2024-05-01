pub mod constants;
pub mod utils;

use pristup::{assume_role, get_caller_identity};
use tracing::Level;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // configure tracing
    utils::configure_tracing(Level::WARN);

    let pristup_config = utils::parse_config()?;

    // configure aws
    let config = utils::configure_aws("us-west-2".into()).await;
    // setup the bedrock-runtime client
    let sts_client = aws_sdk_sts::Client::new(&config);

    println!("----------------------------------------");
    println!("🔑| Assuming role as:");
    println!("👤| {}", get_caller_identity(&sts_client).await?);
    println!("🌍| Generating the console URL...");
    println!();

    // TODO: Handle the printing here
    assume_role(
        pristup_config.role,
        pristup_config.account_id,
        pristup_config.session_name,
        sts_client,
    )
    .await?;

    Ok(())
}
