use axum::extract::{Extension, Path};
use axum::{routing::{get}, Json, Router, extract};


use serde_json::{json, Value};
use sha1::{Digest, Sha1};
use std::net::SocketAddr;

use std::sync::Arc;
use axum::routing::post;
use serde_derive::Deserialize;
use structopt::StructOpt;
use tokio::signal::unix::{signal, SignalKind};
use easypwned_bloom::bloom::{bloom_get, EasyBloom};

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Opt {
    #[structopt(long = "bloomfile", default_value = "easypwned.bloom")]
    bloomfile: String,
    #[structopt(long = "bind", default_value = "0.0.0.0:3342")]
    bind: String,
}

#[tokio::main]
async fn main() -> ::anyhow::Result<(), ::anyhow::Error> {

    let opt: Opt = Opt::from_args();
    tracing_subscriber::fmt::init();

    println!("reading bloom filter file {}", &opt.bloomfile);
    let bloom = match bloom_get(&opt.bloomfile) {
        Ok(b) => b,
        Err(e) => {
            println!("could not get bloom {}", e);
            panic!();
        }
    };
    println!("finished reading bloom filter file {}", &opt.bloomfile);

    let bloom = bloom.to_bloom();

    let checks = vec![
        "0000000CAEF405439D57847A8657218C618160B2",
        "0000000CAEF405439D57847A8657218C618160BX",
    ];

    for check in checks {
        println!(
            "check: {} -> {:?}",
            check,
            bloom.check(&check.as_bytes().to_vec())
        );
    }

    let bloom_ext = Arc::new(bloom);
    let app = Router::new()
        .route("/hash/:hash", get(handler_hash))
        .route("/pw/:pw", get(handler_pw))
        .route("/check", post(handler_check))
        .layer(Extension(bloom_ext));

    let addr = opt.bind.parse::<SocketAddr>().expect("");
    println!("listening on {}", addr);
    let axum_handle = axum::Server::bind(&addr)
        .serve(app.into_make_service());

    let mut sig_quit = signal(SignalKind::quit())?;
    let mut sig_term = signal(SignalKind::terminate())?;

    ::tokio::select! {
        axum = axum_handle => {
            axum?;
            panic!("axum quitted")
        },
        _ = sig_quit.recv() => {
            println!("Signal quit, quit.");
        },
        _ = sig_term.recv() => {
            println!("Signal term, quit.");
        }
        _ = ::tokio::signal::ctrl_c() => {
            println!("Signal ctrl_c, quit.");
        }
    };

    Ok(())
}

async fn handler_hash(
    Extension(bloom): Extension<Arc<EasyBloom>>,
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
    Extension(bloom): Extension<Arc<EasyBloom>>,
    extract::Json(payload): extract::Json<CheckRequestBody>,
) -> Json<Value> {
    let check = bloom.check(&payload.hash.as_bytes().to_vec());
    Json(json!({
        "hash": payload.hash,
        "secure": !check,
    }))
}

async fn handler_pw(
    Extension(bloom): Extension<Arc<EasyBloom>>,
    Path(pw): Path<String>,
) -> Json<Value> {
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
