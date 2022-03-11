use crate::bloom_create::{bloom_create, bloom_get, EasyBloom};
use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use axum::extract::{Extension, Path};
use bloomfilter::Bloom;
use serde_json::{json, Value};

pub mod bloom_create;

#[tokio::main]
async fn main() {


    /*
    let bloom = match bloom_create() {
        Ok(b) => b,
        Err(e) => {
            println!("could not create bloom: {}", e);
            panic!();
        },
    };
     */

    let bloom = match bloom_get() {
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
        .route("/hash/:hash", get(root))
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
async fn root(Extension(bloom): Extension<Arc<EasyBloom>>, Path(hash): Path<String>) -> Json<Value> {
    let check = bloom.check(&hash.as_bytes().to_vec());
    Json(json!({
        "hash": hash,
        "secure": !check,
    }))
}