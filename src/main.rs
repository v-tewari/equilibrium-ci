use std::net::SocketAddr;

use axum::{Router, routing::{get}};
use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod cmd;
mod health;


#[derive(Parser, Debug)]
#[command(name = "equilibrium")]
#[command(version = "0.1")]
struct Args {
    #[arg(short = 'p', long = "port", )]
    port: u16
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = cmd::cli().get_matches();
    let port: u16 = *args
        .get_one("port")
        .expect("port is a required field");

    tracing::debug!("port set to {}", port);
    tracing::debug!("initializing the server");

    let app = Router::new()
        .route("/health", get(health::health_endpoint));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}