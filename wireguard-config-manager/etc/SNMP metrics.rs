use snmp::{SyncSession, Value};

fn get_snmp_metric(host: &str, oid: &str) -> Option<String> {
    let session = SyncSession::new(host, "public", Some(2), 1000).ok()?;
    match session.get(oid) {
        Ok((_, Value::Integer(value))) => Some(value.to_string()),
        Ok((_, Value::OctetString(value))) => Some(String::from_utf8_lossy(&value).to_string()),
        _ => None,
    }
}

fn main() {
    let snmp_metrics = vec![
        ("Latency", "1.3.6.1.2.1.2.2.1.10"),
        ("Bandwidth Usage", "1.3.6.1.2.1.31.1.1.1.6"),
        ("Packet Loss", "1.3.6.1.2.1.5.6")
    ];

    for (name, oid) in snmp_metrics {
        if let Some(value) = get_snmp_metric("localhost", oid) {
            println!("{}: {}", name, value);
        } else {
            println!("Failed to retrieve {}", name);
        }
    }
}