use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_types::region::Region;

use std::env;
use std::fs;
use std::process::exit;
use tracing_subscriber::FmtSubscriber;
use tracing::Level;

use clap::Parser;

use serde_derive::Deserialize;

use anyhow::anyhow;

//======================================== TRACING
pub fn configure_tracing(level: Level) {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(level)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
}
//======================================== END TRACING

//======================================== AWS
pub async fn configure_aws(s: String) -> aws_config::SdkConfig {
    let provider = RegionProviderChain::first_try(env::var("AWS_DEFAULT_REGION").ok().map(Region::new))
        .or_default_provider()
        .or_else(Region::new(s));

    aws_config::defaults(BehaviorVersion::latest())
        .region(provider)
        .load()
        .await
}
//======================================== END AWS
//======================================== CONFIG FILE PARSING
#[derive(Deserialize)]
pub struct FileConfig {
    pub account_id: Option<String>,
    pub role: Option<String>,
    pub session_name: Option<String>
}

// FIX: handle the lack of config file and lack of parameters
impl FileConfig {
    pub fn load_config(filename: String) -> Result<Self, anyhow::Error> {
        let _contents: String = match fs::read_to_string(filename) {
            Ok(c) => {
                let config: FileConfig = toml::from_str::<FileConfig>(&c).unwrap();
                return Ok(config);

            }
            Err(e) => {
                eprintln!("Could not read config file! {}",e);
                exit(1);
            }
        };

    }
}
//======================================== END CONFIG FILE PARSING
//======================================== ARGUMENT PARSING
#[derive(Parser, Default)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub account: Option<String>,
    #[arg(short, long)]
    pub role: Option<String>,
    #[arg(short, long)]
    pub session_name: Option<String>
}
//======================================== END ARGUMENT PARSING
//======================================== CONFIG PARSING
pub struct Config {
    pub account_id: String,
    pub role: String,
    pub session_name: String
}
pub fn parse_config() -> Result<Config, anyhow::Error> {
    // parse arguments
    let arguments = Args::parse();
    // parse configuration
    let pristup_config = FileConfig::load_config("pristup.toml".to_string())?;
    let role = if arguments.role.is_some() {
        arguments.role
    } else {
        pristup_config.role
    }.ok_or_else(||anyhow!("Unable to parse the role. Either add it as a parameter, or make sure you have the config file set up."))?;

    let account_id = if arguments.account.is_some() {
        arguments.account
    } else {
        pristup_config.account_id
    }.ok_or_else(||anyhow!("Unable to parse the account id. Either add it as a parameter, or make sure you have the config file set up."))?;

    let session_name = if arguments.session_name.is_some() {
        arguments.session_name
    } else {
        pristup_config.session_name
    }.ok_or_else(||anyhow!("Unable to parse the session name. Either add it as a parameter, or make sure you have the config file set up."))?;

    Ok(Config {
        account_id,
        role,
        session_name,
    })
}
//======================================== END CONFIG PARSING
