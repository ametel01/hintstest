mod oracle;
use crate::oracle::{Request, Response};
use axum::{extract, routing::post, Json, Router};
use primitive_types::U256;
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use tracing::debug;

// Assuming the Request and Response structures from your message

#[derive(Debug, Serialize, Deserialize)]
struct JsonResult {
    result: Response,
}

fn combine_u256(limb0: u64, limb1: u64, limb2: u64, limb3: u64) -> U256 {
    let mut value = U256::from(limb0);
    value += U256::from(limb1) * U256::from(2).pow(U256::from(64));
    value += U256::from(limb2) * U256::from(2).pow(U256::from(128));
    value += U256::from(limb3) * U256::from(2).pow(U256::from(192));
    value
}

fn split_u256(value: U256) -> (u64, u64, u64, u64) {
    (
        (value % U256::from(2).pow(U256::from(64))).as_u64(),
        ((value / U256::from(2).pow(U256::from(64))) % U256::from(2).pow(U256::from(64))).as_u64(),
        ((value / U256::from(2).pow(U256::from(128))) % U256::from(2).pow(U256::from(64))).as_u64(),
        ((value / U256::from(2).pow(U256::from(192))) % U256::from(2).pow(U256::from(64))).as_u64(),
    )
}

async fn root(extract::Json(payload): extract::Json<Request>) -> Json<JsonResult> {
    debug!("received payload {payload:?}");
    let n = combine_u256(payload.limb0, payload.limb1, payload.limb2, payload.limb3);

    let sqrt_n = n.integer_sqrt();
    println!("sqrt_n: {:?}", sqrt_n);

    let (limb0, limb1, limb2, limb3) = split_u256(sqrt_n);
    Json(JsonResult {
        result: Response {
            limb0,
            limb1,
            limb2,
            limb3,
        },
    })
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = Router::new()
        .route("/sqrt", post(root))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");
    debug!("Server started on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
