// CONFIGURATION FILES
pub static CONFIG_DIR_NAME: &str = "pristup";
pub static CONFIG_FILE_NAME: &str = "pristup.toml";

// UPDATED: 2024-03-31
pub static CONFIG_FILE_CONTENTS: &str = r#"[config]
account_id = "%ACCOUNT_ID%"
role = "%ROLE%"
session_name = "Pristup"
timeout = %TIMEOUT%
use_clipboard = False
"#;
