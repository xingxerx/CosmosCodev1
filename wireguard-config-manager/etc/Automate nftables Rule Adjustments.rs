use std::process::Command;

fn adjust_nftables(priority: u32, target_ip: &str) {
    let cmd = format!("nft add rule ip filter forward ip daddr {} priority {}", target_ip, priority);
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("Failed to update nftables");

    println!("Updated nftables rule for {} with priority {}", target_ip, priority);
}