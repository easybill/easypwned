use crate::bloom_create::{bloom_create, bloom_get, EasyBloom};
use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use axum::extract::{Extension, Path};
use bloomfilter::Bloom;
use serde_json::{json, Value};
use sha1::{Sha1, Digest};
use structopt::StructOpt;

pub mod bloom_create;


#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Opt {
    /// Files to process
    #[structopt(long = "bloomfile", default_value = "easypwned.bloom")]
    bloomfile: String,

    /// Files to process
    #[structopt(long = "create_bloom_file_from_file")]
    create_bloom_file_from_file: Option<String>,
}

#[tokio::main]
async fn main() {

    let opt : Opt = Opt::from_args();

    println!("{:?}", opt);

    match &opt.create_bloom_file_from_file {
        Some(password_file) => match bloom_create(&opt.bloomfile, password_file.as_str()) {
            Ok(b) => {},
            Err(e) => {
                println!("could not create bloom: {}", e);
                panic!();
            },
        },
        None => {},
    };

    let bloom = match bloom_get(&opt.bloomfile) {
        Ok(b) => b,
        Err(e) => {
            println!("could not get bloom {}", e);
            panic!();
        }
    };

    let bloom = bloom.to_bloom();

    let checks = vec![
        "0000000CAEF405439D57847A8657218C618160B2",
        "0000000CAEF405439D57847A8657218C618160BX"
    ];

    for check in checks {
        println!("check: {} -> {:?}", check, bloom.check(&check.as_bytes().to_vec()));
    }

    // initialize tracing
    tracing_subscriber::fmt::init();

    let bloom_ext = Arc::new(bloom);

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/hash/:hash", get(handler_hash))
        .route("/pw/:pw", get(handler_pw))
        .layer(Extension(bloom_ext));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3342));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn handler_hash(Extension(bloom): Extension<Arc<EasyBloom>>, Path(hash): Path<String>) -> Json<Value> {
    let check = bloom.check(&hash.as_bytes().to_vec());
    Json(json!({
        "hash": hash,
        "secure": !check,
    }))
}

async fn handler_pw(Extension(bloom): Extension<Arc<EasyBloom>>, Path(pw): Path<String>) -> Json<Value> {
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