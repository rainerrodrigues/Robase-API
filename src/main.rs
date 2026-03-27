use serde::{Deserialize, Serialize};

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
