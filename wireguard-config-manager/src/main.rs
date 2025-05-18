use tokio::process::Command;
use tokio::time::{sleep, Duration};
use reqwest::Client;
use serde::Serialize;
use std::error::Error;
use std::env;

#[derive(Serialize)]
struct TelegramMessage<'a> {
    chat_id: &'a str,
    text: &'a str,
}

async fn send_telegram_message(
    client: &Client,
    bot_token: &str,
    chat_id: &str,
    message: &str,
) -> Result<(), reqwest::Error> {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
    let payload = TelegramMessage { chat_id, text: message };
    let response = client.post(&url).json(&payload).send().await?;
    println!("Telegram response: {:?}", response);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load Telegram credentials from environment variables.
    let bot_token = env::var("TG_BOT_TOKEN")
        .expect("TG_BOT_TOKEN environment variable not set");
    let chat_id = env::var("TG_CHAT_ID")
        .expect("TG_CHAT_ID environment variable not set");
        
    let client = Client::new();

    loop {
        // Run the "wg show" command asynchronously
        let output = Command::new("wg")
            .arg("show")
            .output()
            .await?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("WireGuard status:\n{}", stdout);

            // Demonstration condition: if the output does not contain "peer:"
            if !stdout.contains("peer:") {
                println!("No peers detected! Sending Telegram notification...");
                send_telegram_message(
                    &client,
                    &bot_token,
                    &chat_id,
                    "Alert: No active peers detected on the WireGuard interface!",
                )
                .await?;
            }
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Error running wg show: {}", stderr);

            // Optionally, notify about errors as well.
            send_telegram_message(
                &client,
                &bot_token,
                &chat_id,
                &format!("Error running wg show: {}", stderr),
            )
            .await?;
        }
        
        // Sleep for 10 seconds asynchronously before checking again.
        sleep(Duration::from_secs(10)).await;
    }
}