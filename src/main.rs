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

    // Better error handling from the assume_role function
    match assume_role(
        pristup_config.role,
        pristup_config.account_id,
        pristup_config.session_name,
        pristup_config.timeout,
        sts_client)
    .await {
        Ok(url) => {

            // If we set the parameter to use the clipboard, set the URL in the clipboard
            if pristup_config.use_clipboard.unwrap_or(false) {
                // TODO: see if you can avoid a clone()
                if let Err(e) = utils::set_into_clipboard(url.clone()) {
                    eprintln!("Error setting clipboard: {}",e);
                }
            }

            // Print out the URL
            println!("{}", url);
        },
        Err(e) => eprintln!("Something went wrong while generating the presigned URL: {}", e),
    }
    Ok(())
}
