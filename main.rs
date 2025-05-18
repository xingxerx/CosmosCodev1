use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct WireGuardConfig {
    Interface: InterfaceConfig,
    Peer: Vec<PeerConfig>,
}

#[derive(Debug, Deserialize)]
struct InterfaceConfig {
    Address: String,
    PrivateKey: String,
}

#[derive(Debug, Deserialize)]
struct PeerConfig {
    PublicKey: String,
    Endpoint: String,
}

fn main() {
    let config_content = fs::read_to_string("wg0.conf").expect("Failed to read file");
    let config: WireGuardConfig = toml::from_str(&config_content).expect("Failed to parse");
    println!("{:?}", config);
}

// ... (previous code) ...
        }
    }
}
#[derive(Debug, Deserialize)] // This is line 94
struct WireGuardConfig {      // This is line 95 - the duplicate!
    #[serde(rename = "Interface")]
    interface: InterfaceConfig,
    #[serde(rename = "Peer")]
    peer: Vec<PeerConfig>, // This expects a list/array of peers
}
