use serde_json::json;

let network_data = json!({
    "latency": get_snmp_metric("localhost", "1.3.6.1.2.1.2.2.1.10"),
    "bandwidth_usage": get_snmp_metric("localhost", "1.3.6.1.2.1.31.1.1.1.6"),
    "packet_loss": get_snmp_metric("localhost", "1.3.6.1.2.1.5.6")
});

println!("{}", network_data);