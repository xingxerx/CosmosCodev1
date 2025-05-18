use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    loop {
        // Run the "wg show" command
        let output = Command::new("wg").arg("show").output();

        match output {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("WireGuard status:\n{}", stdout);
                // TODO: Parse stdout or analyze stats as needed
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Error running wg show: {}", stderr);
            }
            Err(err) => {
                eprintln!("Failed to execute wg show: {}", err);
            }
        }

        // Sleep for 10 seconds before polling again.
        thread::sleep(Duration::from_secs(10));
    }
}