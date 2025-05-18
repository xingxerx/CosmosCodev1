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