use tokio::process::Command;
use tokio::time::{sleep, Duration};
use reqwest::Client;
use serde::Serialize;
use std::env;
use std::error::Error;

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
    // Ensure these environment variables are set:
    let bot_token = env::var("TG_BOT_TOKEN")
        .expect("TG_BOT_TOKEN environment variable not set");
    let chat_id = env::var("TG_CHAT_ID")
        .expect("TG_CHAT_ID environment variable not set");

    let client = Client::new();

    loop {
        // This example runs the "wg show" command every 10 seconds.
        let output = Command::new("wg")
            .arg("show")
            .output()
            .await?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("WireGuard status:\n{}", stdout);

            // Simulated condition: if "peer:" is not found, notify.
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
            send_telegram_message(
                &client,
                &bot_token,
                &chat_id,
                &format!("Error running wg show: {}", stderr),
            )
            .await?;
        }

        sleep(Duration::from_secs(10)).await;
    }
}