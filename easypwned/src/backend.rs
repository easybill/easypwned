use std::net::SocketAddr;
use dioxus::prelude::*;

use std::sync::Arc;
use axum::{Extension, Json};
use dioxus_fullstack::axum_adapter::DioxusRouterExt;
use dioxus_fullstack::prelude::{ServeConfig, ServeConfigBuilder};
use serde_derive::Deserialize;
use serde_json::{json, Value};
use sha1::{Sha1, Digest};
use tokio::signal::unix::{signal, SignalKind};
use easypwned_bloom::bloom::{bloom_get, EasyBloom};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Opt {
    #[structopt(long = "bloomfile", default_value = "easypwned.bloom")]
    bloomfile: String,
    #[structopt(long = "bind", default_value = "0.0.0.0:3342")]
    bind: String,
}

pub(crate) async fn setup(server_config: ServeConfigBuilder<()>) -> ::anyhow::Result<(), ::anyhow::Error> {
    let opt: Opt = Opt::from_args();
    let bloom_ext = load_bloom(&opt.bloomfile).await;
    init_axum(opt.bind, bloom_ext, server_config).await;

    Ok(())
}

async fn init_axum(bind: String, bloom_ext: Arc<EasyBloom>, server_config: ServeConfigBuilder<()>) -> ::anyhow::Result<(), ::anyhow::Error> {
    let app = axum::Router::new()
        .route("/hash/:hash", axum::routing::get(handler_hash))
        .route("/pw/:pw", axum::routing::get(handler_pw))
        .route("/check", axum::routing::post(handler_check))
        .serve_dioxus_application("", server_config)
        .layer(Extension(bloom_ext));

    let addr = bind.parse::<SocketAddr>().expect("");
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

async fn load_bloom(bloomfile: &str) -> Arc<EasyBloom> {
    println!("reading bloom filter file {}", bloomfile);
    let bloom = match bloom_get(bloomfile) {
        Ok(b) => b,
        Err(e) => {
            println!("could not get bloom {}", e);
            panic!();
        }
    };
    println!("finished reading bloom filter file {}", bloomfile);

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

    Arc::new(bloom)
}

async fn handler_hash(
    Extension(bloom): Extension<Arc<EasyBloom>>,
    axum::extract::Path(hash): axum::extract::Path<String>,
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
    Json(payload): Json<CheckRequestBody>,
) -> Json<Value> {
    let check = bloom.check(&payload.hash.as_bytes().to_vec());
    Json(json!({
        "hash": payload.hash,
        "secure": !check,
    }))
}


async fn handler_pw(
    Extension(bloom): Extension<Arc<EasyBloom>>,
    axum::extract::Path(pw): axum::extract::Path<String>,
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
