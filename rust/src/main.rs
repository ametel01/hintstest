#[macro_use]
extern crate lazy_static;
mod config;
mod crypto;
mod handlers;
mod models;
mod utils;

use axum::{routing::post, Router};
use handlers::mod_exp_handler;
use tower_http::trace::TraceLayer;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = Router::new()
        .route("/mod_exp", post(mod_exp_handler))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");
    println!("Server started on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
