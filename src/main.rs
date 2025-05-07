mod config;
mod database;
mod routes;
mod handlers;
mod models;
mod services;
// mod lib;
mod middlewares;

use axum::serve;
use config::constant::EnvConfig;
use database::connection::establish_connection;
use routes::video_routes::create_video_routes;
use tokio::net::TcpListener;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let config = EnvConfig::init();

    // DB connection
    let db = establish_connection().await;

    // Router setup
    let app = create_video_routes(db);

    // Start server
    let addr: SocketAddr = format!("0.0.0.0:{}", config.port).parse().expect("Invalid address");
    let listener = TcpListener::bind(&addr).await.unwrap();

    println!("🚀 Server starting at http://{}", addr);

    serve(listener, app.into_make_service())
        .await
        .unwrap();

    // This line will not be reached unless the server shuts down.
    // If you want to confirm the server is running, the above println is sufficient.
}
