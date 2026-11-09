use axum::{
	routing::get,
	Json,
	Router

};
use serde_json::json;
use tokio::net::TcpListener;
use tracing::info;

async fn health_check() -> Json<serde_json::value> {
	Json(
		json!(
		"status": "healthy",
		"service": "cloudrack-api",
		"version": "0.1.0",
		)

	)

}


#[tokio::main]
async fn main () {

	tracing_subcriber::fmt::init();

	let app = Router::new()
		.route("/health", get(health_check));


	let listener = TcpListener::bind("0.0.0.0.8080")
		.await
		.expect("Failed to bind TCP listener");

	info!("CloudRack API listening on port 8080");

	axum::server(listener, app)
		.await
		.expect("Server failed");

}

