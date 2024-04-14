#[macro_use]
extern crate lazy_static;
mod oracle;
use crate::oracle::{Request, Response};
use axum::{extract, routing::post, Json, Router};
use num_bigint::BigUint;
use num_traits::{Num, One, ToPrimitive, Zero};
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use tracing::debug;

lazy_static! {
    static ref P: BigUint = BigUint::from_str_radix(
        "21888242871839275222246405745257275088696311157297823662689037894645226208583",
        10
    )
    .unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonResult {
    result: Response,
}

fn combine_u256(limb0: u64, limb1: u64, limb2: u64, limb3: u64) -> BigUint {
    let limb0 = BigUint::from(limb0);
    let limb1 = BigUint::from(limb1) << 64; // Use bitwise shift for powers of 2
    let limb2 = BigUint::from(limb2) << 128;
    let limb3 = BigUint::from(limb3) << 192;

    limb0 + limb1 + limb2 + limb3
}

fn split_u256(value: BigUint) -> (u64, u64, u64, u64) {
    // Define the mask for the lower 64 bits
    let mask: BigUint = (BigUint::one() << 64) - BigUint::one();

    // Extract each limb with bit manipulation
    let limb0 = (&value & &mask).to_u64().expect("Conversion to u64 failed");
    let limb1 = ((&value >> 64u32) & &mask)
        .to_u64()
        .expect("Conversion to u64 failed");
    let limb2 = ((&value >> 128u32) & &mask)
        .to_u64()
        .expect("Conversion to u64 failed");
    let limb3 = ((&value >> 192u32) & &mask)
        .to_u64()
        .expect("Conversion to u64 failed");

    (limb0, limb1, limb2, limb3)
}

fn mod_exp(mut base: BigUint, mut exp: BigUint) -> BigUint {
    let mut result = BigUint::one();

    // Perform modulus operation correctly by dereferencing P when needed
    base = &base % &*P; // Dereference P here to get &BigUint from &BigUint

    while exp > BigUint::zero() {
        if &exp % 2u32 == BigUint::one() {
            result = (&result * &base) % &*P; // Dereference P correctly
        }
        exp >>= 1;
        base = (&base * &base) % &*P; // Clone minimized and dereference P correctly
    }

    result
}

async fn mod_exp_handler(extract::Json(payload): extract::Json<Request>) -> Json<JsonResult> {
    debug!("Received payload: {:?}", payload);
    let base = payload.base.as_ref().expect("missing base");
    let exp = payload.exp.as_ref().expect("missing exp");

    let base_combined = combine_u256(base.limb0, base.limb1, base.limb2, base.limb3);
    let exp_combined = combine_u256(exp.limb0, exp.limb1, exp.limb2, exp.limb3);

    let result = mod_exp(base_combined, exp_combined);
    let (limb0, limb1, limb2, limb3) = split_u256(result);

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
        .route("/mod_exp", post(mod_exp_handler))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");
    debug!("Server started on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
