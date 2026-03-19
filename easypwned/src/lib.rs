use std::sync::Arc;

use axum::extract::{Path, State};
use axum::{
    routing::{get, post},
    Json, Router,
};
use serde_derive::Deserialize;
use serde_json::{json, Value};
use sha1::{Digest, Sha1};

use easypwned_bloom::bloom::EasyBloom;

pub fn create_router(bloom: Arc<EasyBloom>) -> Router {
    Router::new()
        .route("/hash/{hash}", get(handler_hash))
        .route("/pw/{pw}", get(handler_pw))
        .route("/check", post(handler_check))
        .with_state(bloom)
}

async fn handler_hash(
    State(bloom): State<Arc<EasyBloom>>,
    Path(hash): Path<String>,
) -> Json<Value> {
    let check = bloom.check(&hash.as_bytes().to_vec());
    Json(json!({
        "hash": hash,
        "secure": !check,
    }))
}

#[derive(Deserialize)]
struct CheckRequestBody {
    hash: String,
}

async fn handler_check(
    State(bloom): State<Arc<EasyBloom>>,
    Json(payload): Json<CheckRequestBody>,
) -> Json<Value> {
    let check = bloom.check(&payload.hash.as_bytes().to_vec());
    Json(json!({
        "hash": payload.hash,
        "secure": !check,
    }))
}

async fn handler_pw(State(bloom): State<Arc<EasyBloom>>, Path(pw): Path<String>) -> Json<Value> {
    let mut hasher = Sha1::new();
    hasher.update(pw.as_bytes());
    let hash_raw = &hasher.finalize();
    let hash = base16ct::lower::encode_string(hash_raw).to_uppercase();

    let check = bloom.check(&hash.as_bytes().to_vec());
    Json(json!({
        "pw": pw,
        "hash": hash,
        "secure": !check,
    }))
}
