use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use axum::{routing::{get, post}, Router, Json, extract::State};

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
