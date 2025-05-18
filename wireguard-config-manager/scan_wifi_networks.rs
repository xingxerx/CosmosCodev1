// Around line 7
#[derive(Debug, Deserialize)]
struct WireGuardConfig {
    #[serde(rename = "Interface")]
    interface: InterfaceConfig,
    #[serde(rename = "Peer")]
    peer: Vec<PeerConfig>,
}

// Around line 15
#[derive(Debug, Deserialize)]
struct InterfaceConfig {
    #[serde(rename = "Address")]
    address: String,
    #[serde(rename = "PrivateKey")]
    private_key: String,
}

// ... scan_wifi_networks ...

// Around line 61 (this was the original placement from a previous diff)
#[derive(Debug, Deserialize)]
struct PeerConfig {
    #[serde(rename = "PublicKey")]
    public_key: String,
    #[serde(rename = "Endpoint")]
    endpoint: String,
}
