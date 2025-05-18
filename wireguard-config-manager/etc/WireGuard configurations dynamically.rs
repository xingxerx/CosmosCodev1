use std::process::Command;

fn update_wireguard_peer(peer: &str, priority: u32) {
    let cmd = format!("wg set {} peer {} persistent-keepalive {}", "wg0", peer, priority);
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("Failed to update WireGuard peer");

    println!("WireGuard Updated: {:?}", output);
}