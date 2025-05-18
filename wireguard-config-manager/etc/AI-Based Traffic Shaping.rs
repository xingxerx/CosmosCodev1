use std::process::Command;

fn update_nftables(priority: u32) {
    let cmd = format!("nft add rule ip filter forward priority {}", priority);
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("Failed to update nftables");

    println!("nftables Updated: {:?}", output);
}