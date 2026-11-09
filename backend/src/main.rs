use std::{env, error::Error};

use axum::{Json, Router, extract::State, http::StatusCode, routing::get};

use sqlx::{
    PgPool, 
    postgres::PgPoolOptions
};

use serde_json::{json, Value};
use tokio::net::TcpListener;
use tracing::{info, warn};

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "cloudrack-api",
        "version": "0.1.0"
    }))
}

async fn readiness_check(
    State(pool): State<PgPool>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&pool)
        .await
    {
        Ok(_) => Ok(Json(json!({
            "status": "ready",
            "database": "connected"
        }))),

        Err(error) => {
            warn!("Database readiness check failed: {error}");

            Err((
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({
                    "status": "ready",
                    "database": "unavailable"
                })),
            ))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    info!("Connected to Postgres");

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/ready", get(readiness_check))
        .with_state(pool);

    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind TCP listener");

    info!("CloudRack API listening on port 8080");

    axum::serve(listener, app).await.expect("Server failed");

    Ok(())
}
