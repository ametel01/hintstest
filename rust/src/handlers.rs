use crate::crypto::mod_exp;
use crate::models::{JsonResult, Request, Response};
use crate::utils::{combine_u256, split_u256};
use axum::{extract, Json};
use tracing::debug;

pub async fn mod_exp_handler(extract::Json(payload): extract::Json<Request>) -> Json<JsonResult> {
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
