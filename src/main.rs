use axum::{routing::get, Router};
use sqlx::mysql::MySqlPoolOptions;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod cmd;
mod ctx;
mod health;

mod user;

use ctx::Context;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let matches = cmd::cli().get_matches();
    let port: u16 = *matches.get_one("port").expect("port is a required field");

    tracing::debug!("port set to {}", port);
    tracing::debug!("initializing the server");

    tracing::info!("setting up database connection pool");
    // let db_url: &str = *matches
    //     .get_one("conn_str")
    //     .expect("connection string is required");

    let max_connections: u32 = *matches
        .get_one("max_connections")
        .expect("max_connections required");

    let db_url = "mysql://root:@localhost/equilibrium_dev";

    let pool = MySqlPoolOptions::new()
        .max_connections(max_connections)
        .connect(db_url)
        .await
        .unwrap();

    let context = Context::new(pool);

    let app = Router::new()
        .route("/health", get(health::health_endpoint))
        .route("/user", get(user::get_all_users))
        .with_state(context);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
