use snmp::{SyncSession, Value};

fn get_snmp_metric(oid: &str) -> Option<String> {
    let session = SyncSession::new("localhost", "public", Some(2), 1000).ok()?;
    match session.get(oid) {
        Ok((_, Value::Integer(value))) => Some(value.to_string()),
        Ok((_, Value::OctetString(value))) => Some(String::from_utf8_lossy(&value).to_string()),
        _ => None,
    }
}

fn main() {
    if let Some(latency) = get_snmp_metric("1.3.6.1.2.1.2.2.1.10") {
        println!("Latency: {} ms", latency);
    } else {
        println!("Failed to retrieve SNMP data.");
    }
}