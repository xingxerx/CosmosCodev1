use std::process::Command;
use std::str;

fn scan_wifi_networks() {
    println!("Scanning for Wi-Fi networks...");

    // This uses `nmcli`, which is common on Linux systems with NetworkManager.
    let output_result = Command::new("nmcli")
        .arg("dev")
        .arg("wifi")
        .arg("list")
        .output();

    match output_result {
        Ok(output) => {
            if output.status.success() {
                let stdout = str::from_utf8(&output.stdout).unwrap_or("Failed to parse stdout from nmcli");
                println!("Available Wi-Fi Networks:\n{}", stdout);
                // For a more user-friendly display, you'd parse this output.
            } else {
                let stderr = str::from_utf8(&output.stderr).unwrap_or("Failed to parse stderr from nmcli");
                eprintln!("Error scanning for Wi-Fi networks (nmcli exit code: {}).", output.status);
                eprintln!("Details: {}", stderr);
                println!("Please ensure 'nmcli' (NetworkManager) is installed and your Wi-Fi device is enabled and not blocked.");
            }
        }
        Err(e) => {
            eprintln!("Failed to execute nmcli: {}", e);
            println!("Please ensure 'nmcli' (NetworkManager) is installed and in your system's PATH.");
        }
    }
}

fn main() {
    println!("WireGuard Config Manager - Wi-Fi Scanner Feature");
    scan_wifi_networks();
}
