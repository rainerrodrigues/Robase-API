use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use axum::{routing::{get, post}, Router, Json, extract::State};
use tower_http::services::ServeDir;

type SharedState = Arc<Mutex<Telemetry>>;

#[derive(Clone, Serialize, Debug)]
struct Telemetry {
    battery_level: u8,
    temperature_c: f32,
    status: String,
}

#[derive(Deserialize, Debug)]
struct CommandPayload {
    action: String, // e.g., "MOVE_FORWARD", "STOP"
    speed: Option<u8>,
}

async fn get_telemetry(State(state): State<SharedState>) -> Json<Telemetry> {
    let data = state.lock().await;
    Json(data.clone())
}

async fn send_command(
    State(state): State<SharedState>,
    Json(payload): Json<CommandPayload>,
) -> Json<Telemetry> {
    let mut data = state.lock().await;

    // Simulate the robot reacting to a command
    if payload.action == "MOVE_FORWARD" {
        data.status = format!("Moving at speed {:?}", payload.speed.unwrap_or(10));
        data.battery_level -= 1; // Simulate battery drain
    } else if payload.action == "STOP" {
        data.status = "Idle".to_string();
    }

    Json(data.clone())
}

#[tokio::main]
async fn main() {
    // Initialize starting telemetry
    let initial_state = Arc::new(Mutex::new(Telemetry {
        battery_level: 100,
        temperature_c: 24.5,
        status: "Idle".to_string(),
    }));

    // Build the router
    let app = Router::new()
        .nest_service("/", ServeDir::new("public"))
        .route("/api/telemetry", get(get_telemetry))
        .route("/api/command", post(send_command))
        .with_state(initial_state); // Pass state to handlers [00:35:34]

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("RoBase API running on port 3000");
    axum::serve(listener, app).await.unwrap();
}
