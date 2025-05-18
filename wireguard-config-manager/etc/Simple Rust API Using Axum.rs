use axum::{routing::get, Router, Json};
use std::net::SocketAddr;
use serde_json::json;

async fn network_health() -> Json<serde_json::Value> {
    Json(json!({
        "latency": "20ms",
        "bandwidth_usage": "1.5GB/s",
        "alerts": ["High congestion on Node B"]
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/network-health", get(network_health));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    println!("Server running on http://{}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}