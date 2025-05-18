use std::fs;
use std::process::Command;
use std::str;
use serde::Deserialize; // Added for TOML parsing

// Structs for WireGuard configuration
#[derive(Debug, Deserialize)]
struct WireGuardConfig {
    #[serde(rename = "Interface")]
    interface: InterfaceConfig,
    #[serde(rename = "Peer")]
    peer: Vec<PeerConfig>,
}

#[derive(Debug, Deserialize)]
struct InterfaceConfig {
    #[serde(rename = "Address")]
    address: String,
    #[serde(rename = "PrivateKey")]
    private_key: String,
    // We can add ListenPort, DNS, etc. here later if needed
}

fn scan_wifi_networks() {
    println!("Scanning for Wi-Fi networks...");

    // This uses `nmcli`, which is common on Linux systems with NetworkManager.
    let output_result = Command::new("nmcli")
        .arg("dev")
        .arg("wifi")
        .arg("list")
        .output();

    match output_result {
        Ok(output) => {
            if output.status.success() {
                let stdout_str = str::from_utf8(&output.stdout).unwrap_or("Failed to parse stdout from nmcli");
                if stdout_str.trim().is_empty() {
                    println!("No Wi-Fi networks found or Wi-Fi might be disabled.");
                    println!("If you are running this in WSL (Windows Subsystem for Linux),");
                    println!("note that direct Wi-Fi scanning via 'nmcli' can be problematic as it may not have access to the host's Wi-Fi hardware.");
                    println!("Ensure your Wi-Fi adapter is enabled on the host and, if applicable, NetworkManager is active within your Linux environment.");
                } else {
                    println!("Available Wi-Fi Networks:\n{}", stdout_str);
                    // For a more user-friendly display, you might want to parse this output further.
                }
            } else {
                let stderr = str::from_utf8(&output.stderr).unwrap_or("Failed to parse stderr from nmcli");
                eprintln!("Error scanning for Wi-Fi networks (nmcli exit code: {}).", output.status);
                eprintln!("Details: {}", stderr);
                println!("Please ensure 'nmcli' (NetworkManager) is installed and your Wi-Fi device is enabled and not blocked.");
            }
        }
        Err(e) => {
            eprintln!("Failed to execute nmcli: {}", e);
            println!("Please ensure 'nmcli' (NetworkManager) is installed and in your system's PATH.");
        }
    }
}

#[derive(Debug, Deserialize)]
struct PeerConfig {
    #[serde(rename = "PublicKey")]
    public_key: String,
    #[serde(rename = "Endpoint")]
    endpoint: String,
    // We can add AllowedIPs, PresharedKey, PersistentKeepalive here later
}

fn load_wireguard_config(file_path: &str) -> Result<WireGuardConfig, Box<dyn std::error::Error>> {
    println!("\nLoading WireGuard configuration from {}...", file_path);
    let config_content = fs::read_to_string(file_path)?;
    let config: WireGuardConfig = toml::from_str(&config_content)?;
    Ok(config)
}

fn main() {
    println!("WireGuard Config Manager - Wi-Fi Scanner Feature");
    scan_wifi_networks();

    // Attempt to load and parse a WireGuard config file
    // For now, let's assume it's named "wg0.conf" in the current directory
    match load_wireguard_config("wg0.conf") {
        Ok(config) => {
            println!("Successfully loaded WireGuard configuration:");
            println!("{:#?}", config); // Pretty print the config
        }
        Err(e) => {
            eprintln!("Error loading or parsing WireGuard config: {}", e);
            eprintln!("Please ensure 'wg0.conf' exists in the current directory and is a valid TOML WireGuard configuration.");
        }
    }
}
