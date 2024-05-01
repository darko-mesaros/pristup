use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_types::region::Region;

use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use clap::Parser;
use colored::*;
use regex::Regex;

use serde_derive::Deserialize;

use anyhow::anyhow;
use dirs::home_dir;

use crate::constants;

//======================================== TRACING
pub fn configure_tracing(level: Level) {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(level)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
//======================================== END TRACING

//======================================== AWS
pub async fn configure_aws(s: String) -> aws_config::SdkConfig {
    let provider =
        RegionProviderChain::first_try(env::var("AWS_DEFAULT_REGION").ok().map(Region::new))
            .or_default_provider()
            .or_else(Region::new(s));

    aws_config::defaults(BehaviorVersion::latest())
        .region(provider)
        .load()
        .await
}
//======================================== END AWS
// function that checks if there are any configuration files present
pub fn check_for_config() -> Result<bool, anyhow::Error> {
    let home_dir = home_dir().expect("Failed to get HOME directory");
    let config_dir = home_dir.join(format!(".config/{}", constants::CONFIG_DIR_NAME));
    let config_file_path = config_dir.join(constants::CONFIG_FILE_NAME);

    if !config_file_path.exists() {
        Ok(false)
    } else {
        Ok(true)
    }
}

// function that creates the configuration files during the `init` command
pub fn initialize_config() -> Result<(), anyhow::Error> {
    // account id loop
    let account_id_regex = Regex::new(r"^\d{12}$")?;
    let mut account_id = String::new();
    loop {
        print!("Enter AWS account id (12 digit number): ");
        io::stdout().flush()?; // so the answers are typed on the same line as above
        io::stdin().read_line(&mut account_id)?;

        if account_id_regex.is_match(account_id.trim()) {
            // we're good
            break;
        } else {
            println!("Invalid account ID. Please entre a 12 digit number.")
        }
    }

    // role loop
    let mut role_name = String::new();
    loop {
        print!("\nEnter the name of the Role you wish to assume: ");
        io::stdout().flush()?; // so the answers are typed on the same line as above
        io::stdin().read_line(&mut role_name)?;

        if role_name.trim().is_empty() {
            println!("The role name cannot be blank. Please try again.")
        } else {
            // we're good
            break;
        }
    }

    let mut config_replacements = std::collections::HashMap::new();
    config_replacements.insert("%ACCOUNT_ID%", account_id.trim());
    config_replacements.insert("%ROLE%", role_name.trim());

    let home_dir = home_dir().expect("Failed to get HOME directory");
    let config_dir = home_dir.join(format!(".config/{}", constants::CONFIG_DIR_NAME));
    fs::create_dir_all(&config_dir)?;

    let config_file_path = config_dir.join(constants::CONFIG_FILE_NAME);
    let mut config_content = constants::CONFIG_FILE_CONTENTS.to_string();
    for (placeholder, value) in &config_replacements {
        config_content = config_content.replace(placeholder, value);
    }
    fs::write(&config_file_path, config_content)?;
    println!(
        "⏳| Pristup configuration file created at: {:?}",
        config_file_path
    );
    println!("👆| This file is used to store configuration items for the pristup application.");
    println!("✅| Pristup configuration has been initialized in ~/.config/pristup. You may now use it as normal.");
    Ok(())
}
//======================================== CONFIG FILE PARSING
#[derive(Deserialize)]
pub struct FileConfig {
    pub config: TomlConfig,
}

#[derive(Deserialize)]
pub struct TomlConfig {
    pub account_id: Option<String>,
    pub role: Option<String>,
    pub session_name: Option<String>,
}

impl FileConfig {
    pub fn load_config(filename: PathBuf) -> Result<Self, anyhow::Error> {
        let _contents: String = match fs::read_to_string(filename) {
            Ok(c) => {
                let config: FileConfig = toml::from_str::<FileConfig>(&c).unwrap();
                return Ok(config);
            }
            Err(e) => {
                eprintln!("Could not read config file! {}", e);
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
    pub session_name: Option<String>,
    #[arg(long, conflicts_with_all(["account", "role", "session_name"]))]
    pub init: bool,
}
//======================================== END ARGUMENT PARSING
//======================================== CONFIG PARSING
pub struct Config {
    pub account_id: String,
    pub role: String,
    pub session_name: String,
}
pub fn parse_config() -> Result<Config, anyhow::Error> {
    // parse arguments
    let arguments = Args::parse();

    // check for init
    if arguments.init {
        // init is passed let's create the configuration files
        if check_for_config()? {
            print_warning("****************************************");
            print_warning("WARNING:");
            println!("You are trying to initialize the Pristup configuration");
            println!("This will overwrite your configuration files in $HOME/.config/pristup/");
            print!("ARE YOU SURE YOU WANT DO TO THIS? Y/N: ");
            io::stdout().flush()?; // so the answers are typed on the same line as above

            let mut confirmation = String::new();
            io::stdin().read_line(&mut confirmation)?;
            if confirmation.trim().eq_ignore_ascii_case("y") {
                print_warning("I ask AGAIN");
                print!("ARE YOU SURE YOU WANT DO TO THIS? Y/N: ");
                io::stdout().flush()?; // so the answers are typed on the same line as above

                let mut confirmation = String::new();
                io::stdin().read_line(&mut confirmation)?;

                if confirmation.trim().eq_ignore_ascii_case("y") {
                    println!("----------------------------------------");
                    println!("📜| Initializing Pristup configuration.");
                    initialize_config()?;
                }
            }
        } else {
            println!("----------------------------------------");
            println!("📜| Initializing Pristup configuration.");
            initialize_config()?;
        }
        print_warning("Bedrust will now exit");
        std::process::exit(0);
    }

    // check if all arguments are preset
    if arguments.account.is_some() && arguments.session_name.is_some() && arguments.role.is_some() {
        // all arguments are present return them
        Ok(Config {
            account_id: arguments
                .account
                .ok_or_else(|| anyhow!("Unable to parse account ID from arguments"))?,
            role: arguments
                .role
                .ok_or_else(|| anyhow!("Unable to parse role from arguments"))?,
            session_name: arguments
                .session_name
                .ok_or_else(|| anyhow!("Unable to parse session from arguments"))?,
        })
    } else {
        // if not check for config file
        if check_for_config()? {
            // parse configuration
            // FIX: Handle the error of not having all configuration items present
            let home_dir = home_dir().expect("Failed to get HOME directory");
            let config_dir = home_dir.join(format!(".config/{}", constants::CONFIG_DIR_NAME));
            let config_file_path = config_dir.join(constants::CONFIG_FILE_NAME);
            let pristup_config = FileConfig::load_config(config_file_path)?;

            let role = if arguments.role.is_some() {
                arguments.role
            } else {
                pristup_config.config.role
            }.ok_or_else(||anyhow!("Unable to parse the role. Either add it as a parameter, or make sure you have the config file set up."))?;

            let account_id = if arguments.account.is_some() {
                arguments.account
            } else {
                pristup_config.config.account_id
            }.ok_or_else(||anyhow!("Unable to parse the account id. Either add it as a parameter, or make sure you have the config file set up."))?;

            let session_name = if arguments.session_name.is_some() {
                arguments.session_name
            } else {
                pristup_config.config.session_name
            }.ok_or_else(||anyhow!("Unable to parse the session name. Either add it as a parameter, or make sure you have the config file set up."))?;

            Ok(Config {
                account_id,
                role,
                session_name,
            })
        } else {
            // if the config file is not present
            // fail and notify of init
            print_warning("****************************************");
            print_warning("WARNING:");
            println!("Your Pristup configuration files are not set up correctly.");
            println!("To use this application you need the appropriate `pristup.toml` in your $HOME/.config/pristup/ directory.");
            println!("You can configure the application by running `pristup --init`");
            print_warning("****************************************");
            print_warning("Pristup will now exit");
            Err(anyhow!("Configuration file not present"))
        }
    }
}
//======================================== END CONFIG PARSING
pub fn print_warning(s: &str) {
    println!("{}", s.yellow());
}
