use axum::{routing::post, Json, Router};
use serde_json::json;
use std::process::Command;

async fn update_wireguard(Json(payload): Json<serde_json::Value>) -> Json<serde_json::Value> {
    let peer = payload["peer"].as_str().unwrap_or("");
    let priority = payload["priority"].as_u64().unwrap_or(0);

    let cmd = format!("wg set wg0 peer {} persistent-keepalive {}", peer, priority);
    Command::new("sh").arg("-c").arg(cmd).output().expect("Failed to update WireGuard");

    Json(json!({ "status": "WireGuard updated", "peer": peer, "priority": priority }))
}

fn main() {
    let app = Router::new().route("/update_wireguard", post(update_wireguard));
    println!("API Running...");
}