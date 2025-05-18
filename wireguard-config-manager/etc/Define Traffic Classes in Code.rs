#[derive(Debug)]
enum TrafficClass {
    HighPriority,   // VPN, SNMP, AI monitoring
    MediumPriority, // Web browsing, interactive apps
    LowPriority,    // Background downloads, non-essential traffic
}

fn classify_traffic(service: &str) -> TrafficClass {
    match service {
        "WireGuard VPN" | "SNMP Monitoring" | "AI Anomaly Detection" => TrafficClass::HighPriority,
        "Web Browsing" | "Interactive Apps" => TrafficClass::MediumPriority,
        "Background Downloads" => TrafficClass::LowPriority,
        _ => TrafficClass::LowPriority, // Default if unknown
    }
}