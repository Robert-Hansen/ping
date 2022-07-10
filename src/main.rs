use axum::{extract::Extension, routing::get, Router, Server};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
use tower::ServiceBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	env::set_var("RUST_LOG", "debug");
	tracing_subscriber::fmt::init();

	dotenv::dotenv().ok();
	let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
	let host = env::var("HOST").expect("HOST is not set in .env file");
	let port = env::var("PORT").expect("PORT is not set in .env file");
	let server_url = format!("{}:{}", host, port);

	let conn = Database::connect(db_url)
		.await
		.expect("Database connection failed");

	Migrator::up(&conn, None).await.unwrap();

	let app = Router::new()
		.route("/", get(|| async { "Hello, World!" }))
		.layer(ServiceBuilder::new().layer(Extension(conn)));

	let addr = SocketAddr::from_str(&server_url).unwrap();
	Server::bind(&addr).serve(app.into_make_service()).await?;

	Ok(())
}
