use axum::{routing::get, Router};
use std::net::SocketAddr;

async fn screening() -> String {
	"Screening result for 4e3cde86-37ea-48bb-adef-4f820ba70b0f".to_string()
}

#[tokio::main]
async fn main() {
	let app = Router::new().route(
		"/api/sanctions/v1/4e3cde86-37ea-48bb-adef-4f820ba70b0f/screening",
		get(screening),
	);

	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	println!("Server listening on {}", addr);

	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}
