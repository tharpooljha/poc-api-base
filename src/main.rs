use axum::{
	extract::Path, response::Html, response::IntoResponse, routing::get, Json,
	Router,
};
use std::net::SocketAddr;

async fn screening(
	Path((api_version, uuid)): Path<(String, String)>,
) -> Html<String> {
	let content = format!("<p>{}</p><p>{}</p>", api_version, uuid);
	Html(content)
}

async fn provider(
	Path((api_version, uuid)): Path<(String, String)>,
) -> Html<String> {
	let content = format!("<p>{}</p><p>{}</p>", api_version, uuid);
	Html(content)
}

async fn index() -> Html<&'static str> {
	Html(
		"<h1>poc-api-base!</h1> \n <p>API for POC</p>\n   <p>Version 0.2.0</p>\n
	<p>Test Screening:</p>\n  <p><a href='/api/poc-api-base/v1/4e3cde86-37ea-48bb-adef-4f820ba70b0f/screening'>Screening API</p>\n
	<p>Test Providers:</p>\n  <p><a href='/api/poc-api-base/v1/4e3cde86-37ea-48bb-adef-4f820ba70b0f/providers'>Providers API</p>",
	)
}

pub async fn health_check_handler() -> impl IntoResponse {
	const MESSAGE: &str = "API Services";

	let json_response = serde_json::json!({
		"status": "ok",
		"message": MESSAGE
	});

	Json(json_response)
}

#[tokio::main]
async fn main() {
	let app = Router::new()
		.route("/api/poc-api-base", get(index))
		.route(
			"/api/poc-api-base/:api_version/:uuid/screening",
			get(screening),
		)
		.route(
			"/api/poc-api-base/:api_version/:uuid/providers",
			get(provider),
		)
		.route("/health/liveness", get(health_check_handler))
		.route("/health/readiness", get(health_check_handler));

	let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
	println!("Server listening on {}", addr);

	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}
