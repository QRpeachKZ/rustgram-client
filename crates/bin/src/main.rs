//! rustgram-client - Telegram CLI Test Client
//!
//! Simple console client for testing the rustgram-client library.
//!
//! # Usage
//!
//! ```bash
//! # Production mode (default)
//! cargo run
//!
//! # Test/Dev mode - loads .env.test
//! cargo run --dev
//!
//! # Or via environment variable
//! RUSTGRAM_TEST=1 cargo run
//! ```

use std::io::{self, Write};
use std::net::IpAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{bail, Context, Result};
use rustgram_auth_manager::AuthManager;
use rustgram_net::{
    set_test_mode, ConnectionPool, DcId, DcOption, DcOptionsSet,
    NetQueryDispatcher, SessionConnectionConfig,
};
use rustgram_storage::{DbConnection, DialogDb};
use tokio::time::sleep;
use tracing::{error, info, warn, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Check CLI args FIRST (before loading .env) to detect flags
    let args: Vec<String> = std::env::args().collect();
    let has_dev_flag = args.iter().any(|a| a == "--dev");

    // Check for test mode via environment variable
    let is_test_env = std::env::var("RUSTGRAM_TEST").is_ok();

    // Load appropriate .env file
    if has_dev_flag || is_test_env {
        dotenv::from_filename(".env.test").ok();
    } else {
        dotenv::dotenv().ok();
    }

    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    // Test mode can be enabled via --dev flag or RUSTGRAM_TEST env var
    let test_dc = has_dev_flag || is_test_env;
    info!("🚀 rustgram-client - Telegram CLI");
    info!("====================================");
    info!("Test mode: {} (--dev: {}, env: {})", test_dc, has_dev_flag, is_test_env);
    if test_dc {
        info!("Using .env.test configuration");
    }

    // Load configuration
    let config = Config::load(test_dc)?;

    if config.api_id == 0 {
        bail!("Missing RUSTGRAM_API_ID. Get it from https://my.telegram.org/apps");
    }

    if config.api_hash.is_empty() {
        bail!("Missing RUSTGRAM_API_HASH. Get it from https://my.telegram.org/apps");
    }

    if config.phone_number.is_empty() {
        bail!("Missing RUSTGRAM_PHONE. Set your phone number (e.g., +1234567890)");
    }

    info!("Configuration loaded:");
    info!("  API ID: {}", config.api_id);
    info!("  Phone: {}", config.phone_number);
    info!("  Data path: {}", config.data_path);
    info!("  RSA Key: {}", config.rsa_key_path.display());
    if let Some(dc2) = &config.dc2_override {
        info!("  DC2 Override: {}", dc2);
    }
    info!(
        "  Mode: {}",
        if config.test_dc { "TEST DC" } else { "PRODUCTION" }
    );

    // Initialize data directory
    let data_path = PathBuf::from(&config.data_path);
    tokio::fs::create_dir_all(&data_path)
        .await
        .context("Failed to create data directory")?;

    info!("Initializing storage...");
    let dialog_db_path = data_path.join("dialogs.db");
    let db_conn = DbConnection::new(dialog_db_path)?;
    let _dialog_db = Arc::new(DialogDb::new(db_conn));
    info!("Storage initialized");

    // Set global test mode
    set_test_mode(config.test_dc);

    // Load RSA key from .pem file
    if let Err(e) = load_rsa_key(&config.rsa_key_path) {
        warn!(
            "Failed to load RSA key from {}: {}, using fallback",
            config.rsa_key_path.display(),
            e
        );
    }

    // Create connection pool
    info!("Creating connection pool...");
    let pool = Arc::new(ConnectionPool::new());

    // Load DC options with DC2 override support and set on pool
    let dc_options = load_dc_options(&config)?;
    info!("Loaded {} DC options", dc_options.get_options().len());
    pool.set_dc_options(dc_options);

    // Create network dispatcher
    let dispatcher = NetQueryDispatcher::new();
    dispatcher.set_main_dc_id(2); // DC 2 is default for new auth
    dispatcher.set_session_pool(pool.clone());
    dispatcher.set_session_config(SessionConnectionConfig::new(DcId::internal(2)));

    info!("Starting authentication flow");

    // Create auth manager
    let auth_manager = AuthManager::new(
        config.api_id as i32,
        config.api_hash.clone(),
        dispatcher,
    );

    // Start authentication flow
    info!("\n📱 Starting authentication flow...");

    // Set phone number (this automatically triggers code sending)
    info!("🔐 Requesting verification code for {}...", config.phone_number);
    if let Err(e) = auth_manager.set_phone_number(config.phone_number.clone()).await {
        error!("Failed to send code request: {}", e);
        bail!("Authentication failed: {}", e);
    }

    // Wait for state transition to WaitCode (with timeout)
    info!("Waiting for server response...");
    let mut wait_count = 0;
    while wait_count < 30 {
        sleep(Duration::from_millis(500)).await;
        wait_count += 1;

        let state = auth_manager.get_state();
        let state_str = format!("{:?}", state);

        if state_str.contains("WaitCode") || state_str.contains("WaitCode") {
            info!("✅ Code sent! Check your phone for the verification code.");
            break;
        }

        if state_str.contains("Ok") {
            info!("✅ Already authorized!");
            return Ok(());
        }

        if wait_count % 4 == 0 {
            info!("Still waiting... (state: {})", state_str);
        }

        if state_str.contains("Error") || state_str.contains("Closed") {
            bail!("Authentication failed with state: {}", state_str);
        }
    }

    // Prompt for code
    print!("Enter verification code: ");
    io::stdout().flush()?;
    let mut code = String::new();
    io::stdin().read_line(&mut code)?;
    let code = code.trim();

    if code.is_empty() {
        bail!("Verification code cannot be empty");
    }

    // Check code with the server
    info!("🔑 Verifying code...");

    match auth_manager
        .check_code(code.to_string(), None)
        .await
    {
        Ok(_) => {
            info!("✅ Code verification successful!");
        }
        Err(e) => {
            error!("Code verification failed: {}", e);
            bail!("Authentication failed: {}", e);
        }
    }

    // Wait for authorization to complete
    info!("Waiting for authorization to complete...");
    let mut auth_wait_count = 0;
    while auth_wait_count < 60 {
        sleep(Duration::from_millis(500)).await;
        auth_wait_count += 1;

        let state = auth_manager.get_state();
        let state_str = format!("{:?}", state);

        if state_str.contains("Ok") {
            info!("\n✅ Authentication successful!");
            info!("You are now logged in as {}", config.phone_number);
            break;
        }

        if state_str.contains("WaitPassword") {
            info!("\n🔒 Two-factor authentication enabled.");
            print!("Enter your password: ");
            io::stdout().flush()?;
            let mut password = String::new();
            io::stdin().read_line(&mut password)?;
            let password = password.trim();

            if let Err(e) = auth_manager.check_password(password.to_string()).await {
                error!("Password verification failed: {}", e);
                bail!("Authentication failed: {}", e);
            }
            continue;
        }

        if auth_wait_count % 4 == 0 {
            info!("Still authenticating... (state: {})", state_str);
        }

        if state_str.contains("Error") || state_str.contains("Closed") {
            bail!("Authentication failed with state: {}", state_str);
        }
    }

    if auth_wait_count >= 60 {
        bail!("Authentication timeout - please try again");
    }

    info!("\n✅ Client setup complete!");
    info!("The client is now ready to use.");

    Ok(())
}

/// Loads Telegram DC options with DC2 override support.
///
/// Returns DC options from test_config module, with optional DC2 IP override
/// from the RUSTGRAM_DC2 environment variable.
fn load_dc_options(config: &Config) -> Result<DcOptionsSet> {
    // Get base DC options for current mode (production or test)
    let mut dc_options = rustgram_net::get_dc_options();

    // Apply DC2 override if specified
    if let Some((ip, port)) = config.parse_dc2_override() {
        info!("Applying DC2 override: {}:{}", ip, port);

        // Remove existing DC2 options
        dc_options.retain(|opt| opt.dc_id != DcId::internal(2));

        // Add new DC2 option with overridden IP
        dc_options.add(DcOption::new(DcId::internal(2), ip, port));
    }

    let mut set = DcOptionsSet::new();
    set.add_options(dc_options);
    Ok(set)
}

/// Loads RSA key from .pem file.
///
/// Reads the PEM file and parses it as an RSA public key. The key can then
/// be passed to RsaKeyManager for use in MTProto authentication.
fn load_rsa_key(path: &PathBuf) -> Result<()> {
    use rustgram_net::RsaPublicKeyWrapper;
    use std::fs;

    let pem_data = fs::read(path)?;
    let _key = RsaPublicKeyWrapper::from_pem(&pem_data)
        .map_err(|e| anyhow::anyhow!("Failed to parse RSA key: {}", e))?;

    // TODO: Pass key to RsaKeyManager when integrated
    info!("RSA key loaded from {}", path.display());
    Ok(())
}

/// Client configuration.
///
/// Configuration loaded from environment variables and CLI flags.
#[derive(Debug, Clone)]
struct Config {
    /// API ID from https://my.telegram.org/apps
    api_id: u32,

    /// API Hash from https://my.telegram.org/apps
    api_hash: String,

    /// Phone number for authentication
    phone_number: String,

    /// Path to data directory
    data_path: String,

    /// Use test DC servers
    test_dc: bool,

    /// Path to RSA public key file (.pem)
    rsa_key_path: PathBuf,

    /// Optional DC2 IP override (format: "IP:PORT" or "IP")
    dc2_override: Option<String>,
}

impl Config {
    /// Loads configuration from environment variables.
    ///
    /// Reads the following environment variables:
    /// - `RUSTGRAM_API_ID`: Telegram API ID
    /// - `RUSTGRAM_API_HASH`: Telegram API hash
    /// - `RUSTGRAM_PHONE`: Phone number for authentication
    /// - `RUSTGRAM_DATA_PATH`: Path to data directory (optional, defaults to ~/.local/share/rustgram-client)
    /// - `RUSTGRAM_RSA_KEY_PATH`: Path to RSA public key file (optional, defaults to ./rsa_public.pem)
    /// - `RUSTGRAM_DC2`: DC2 IP override (optional, format: "IP:PORT" or "IP")
    fn load(test_dc: bool) -> Result<Self> {
        let api_id = std::env::var("RUSTGRAM_API_ID")
            .unwrap_or_default()
            .parse::<u32>()
            .unwrap_or(0);

        let api_hash = std::env::var("RUSTGRAM_API_HASH").unwrap_or_default();

        let phone_number = std::env::var("RUSTGRAM_PHONE").unwrap_or_default();

        let data_path = std::env::var("RUSTGRAM_DATA_PATH").unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_default();
            format!("{}/.local/share/rustgram-client", home)
        });

        // RSA key path from .env or default to ./rsa_public.pem
        let rsa_key_path = std::env::var("RUSTGRAM_RSA_KEY_PATH")
            .ok()
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("./rsa_public.pem"));

        // DC2 override (format: "IP:PORT" or "IP")
        let dc2_override = std::env::var("RUSTGRAM_DC2").ok();

        Ok(Self {
            api_id,
            api_hash,
            phone_number,
            data_path,
            test_dc,
            rsa_key_path,
            dc2_override,
        })
    }

    /// Parses the RUSTGRAM_DC2 override value.
    ///
    /// Supports two formats:
    /// - "IP:PORT" (e.g., "192.168.1.100:443")
    /// - "IP" (defaults to port 443)
    ///
    /// For IPv6 addresses with port, use brackets: "[::1]:443"
    ///
    /// Returns `None` if dc2_override is not set or parsing fails.
    fn parse_dc2_override(&self) -> Option<(IpAddr, u16)> {
        let value = self.dc2_override.as_ref()?;

        // Check for bracketed IPv6 format: "[IP]:PORT"
        if value.starts_with('[') {
            if let Some(bracket_end) = value.find(']') {
                let ip_str = &value[1..bracket_end];
                let port_str = value.get(bracket_end + 1..)?.strip_prefix(':')?;

                let ip = ip_str.parse::<IpAddr>().ok()?;
                let port = port_str.parse::<u16>().ok()?;
                return Some((ip, port));
            }
        }

        // Try to parse as IP:PORT for IPv4
        if value.contains(':') {
            // Count colons to distinguish IPv4:PORT from IPv6
            let colon_count = value.chars().filter(|&c| c == ':').count();

            if colon_count == 1 {
                // IPv4:PORT format
                let parts: Vec<&str> = value.split(':').collect();
                if parts.len() == 2 {
                    let ip = parts[0].parse::<IpAddr>().ok()?;
                    let port = parts[1].parse::<u16>().ok()?;
                    return Some((ip, port));
                }
            }

            // Might be IPv6 address without port (multiple colons)
            // Try to parse as IP only, default port 443
            if let Ok(ip) = value.parse::<IpAddr>() {
                return Some((ip, 443));
            }
        }

        // Format: IP only (default port 443)
        let ip = value.parse::<IpAddr>().ok()?;
        Some((ip, 443))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dc2_override_with_port() {
        let config = Config {
            api_id: 0,
            api_hash: String::new(),
            phone_number: String::new(),
            data_path: String::new(),
            test_dc: false,
            rsa_key_path: PathBuf::new(),
            dc2_override: Some("192.168.1.100:443".to_string()),
        };

        let (ip, port) = config.parse_dc2_override().unwrap();
        assert_eq!(ip, IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 1, 100)));
        assert_eq!(port, 443);
    }

    #[test]
    fn test_parse_dc2_override_without_port() {
        let config = Config {
            api_id: 0,
            api_hash: String::new(),
            phone_number: String::new(),
            data_path: String::new(),
            test_dc: false,
            rsa_key_path: PathBuf::new(),
            dc2_override: Some("192.168.1.100".to_string()),
        };

        let (ip, port) = config.parse_dc2_override().unwrap();
        assert_eq!(ip, IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 1, 100)));
        assert_eq!(port, 443); // Default port
    }

    #[test]
    fn test_parse_dc2_override_none() {
        let config = Config {
            api_id: 0,
            api_hash: String::new(),
            phone_number: String::new(),
            data_path: String::new(),
            test_dc: false,
            rsa_key_path: PathBuf::new(),
            dc2_override: None,
        };

        assert!(config.parse_dc2_override().is_none());
    }

    #[test]
    fn test_parse_dc2_override_invalid() {
        let config = Config {
            api_id: 0,
            api_hash: String::new(),
            phone_number: String::new(),
            data_path: String::new(),
            test_dc: false,
            rsa_key_path: PathBuf::new(),
            dc2_override: Some("invalid".to_string()),
        };

        assert!(config.parse_dc2_override().is_none());
    }

    #[test]
    fn test_parse_dc2_override_ipv6() {
        let config = Config {
            api_id: 0,
            api_hash: String::new(),
            phone_number: String::new(),
            data_path: String::new(),
            test_dc: false,
            rsa_key_path: PathBuf::new(),
            dc2_override: Some("[::1]:443".to_string()),
        };

        let (ip, port) = config.parse_dc2_override().unwrap();
        assert_eq!(ip, IpAddr::V6(std::net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)));
        assert_eq!(port, 443);
    }

    #[test]
    fn test_parse_dc2_override_ipv6_no_port() {
        let config = Config {
            api_id: 0,
            api_hash: String::new(),
            phone_number: String::new(),
            data_path: String::new(),
            test_dc: false,
            rsa_key_path: PathBuf::new(),
            dc2_override: Some("::1".to_string()),
        };

        let (ip, port) = config.parse_dc2_override().unwrap();
        assert_eq!(ip, IpAddr::V6(std::net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)));
        assert_eq!(port, 443); // Default port
    }
}
