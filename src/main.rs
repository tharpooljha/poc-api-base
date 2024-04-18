use axum::{extract::Path, routing::get, Router};
use std::net::SocketAddr;

async fn screening(
	Path(uuid): Path<String>,
	Path(api_version): Path<String>,
) -> String {
	format!("Screening result for {} {}", api_version, uuid)
}

async fn provider(
	Path(uuid): Path<String>,
	Path(api_version): Path<String>,
) -> String {
	format!("Screening result for {} {}", api_version, uuid)
}

#[tokio::main]
async fn main() {
	let app = Router::new()
		.route(
			"/api/poc-api-base/:api_version/:uuid/screening",
			get(screening),
		)
		.route(
			"/api/poc-api-base/:api_version/:uuid/provider",
			get(provider),
		);

	let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
	println!("Server listening on {}", addr);

	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}
