use axum::{routing::{get, post}, Router, Json, extract::State};
use tower_http::services::ServeDir;
use sqlx::{sqlite::{SqlitePoolOptions, SqliteConnectOptions}, SqlitePool, FromRow};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Serialize, Debug, FromRow)]
struct Telemetry {
    battery_level: i64,
    temperature_c: f64,
    status: String,
}

#[derive(Deserialize, Debug)]
struct CommandPayload {
    action: String,
    speed: Option<i64>,
}

// Handlers
async fn get_telemetry(State(pool): State<SqlitePool>) -> Json<Telemetry> {
    let record: Telemetry = sqlx::query_as(
        "SELECT battery_level, temperature_c, status FROM telemetry ORDER BY id DESC LIMIT 1"
    )
    .fetch_one(&pool)
    .await
    .unwrap(); 

    Json(record)
}

async fn send_command(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CommandPayload>,
) -> Json<Telemetry> {
    
    let mut current: Telemetry = sqlx::query_as(
        "SELECT battery_level, temperature_c, status FROM telemetry ORDER BY id DESC LIMIT 1"
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    if payload.action == "MOVE_FORWARD" {
        current.status = format!("Moving at speed {}", payload.speed.unwrap_or(10));
        current.battery_level -= 1; 
    } else if payload.action == "STOP" {
        current.status = "Idle".to_string();
    }

    sqlx::query("INSERT INTO telemetry (battery_level, temperature_c, status) VALUES (?, ?, ?)")
        .bind(current.battery_level)
        .bind(current.temperature_c)
        .bind(&current.status)
        .execute(&pool)
        .await
        .unwrap();

    Json(current)
}

// Main Routing and DB Setup
#[tokio::main]
async fn main() {
    let options = SqliteConnectOptions::from_str("sqlite://robase.db")
        .unwrap()
        .create_if_missing(true);
        
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS telemetry (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            battery_level INTEGER,
            temperature_c REAL,
            status TEXT
        )"
    ).execute(&pool).await.unwrap();

    sqlx::query("INSERT INTO telemetry (battery_level, temperature_c, status) SELECT 100, 24.5, 'Idle' WHERE NOT EXISTS (SELECT 1 FROM telemetry)")
        .execute(&pool).await.unwrap();

    let app = Router::new()
        .nest_service("/", ServeDir::new("public"))
        .route("/api/telemetry", get(get_telemetry))
        .route("/api/command", post(send_command))
        .with_state(pool); 

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("RoBase API running on port 3000 with SQLite!");
    axum::serve(listener, app).await.unwrap();
}
