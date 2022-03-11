use crate::bloom_create::{bloom_create, bloom_get};
use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use axum::extract::Extension;

pub mod bloom_create;

#[tokio::main]
async fn main() {


    match bloom_create() {
        Ok(()) => {},
        Err(e) => println!("could not create bloom: {}", e),
    };

    let bloom = match bloom_get() {
        Ok(b) => b,
        Err(e) => {
            println!("could not get bloom {}", e);
            panic!();
        }
    };

    let checks = vec![
        "0000000CAEF405439D57847A8657218C618160B2"
    ];

    for check in checks {
        println!("check: {} -> {:?}", check, bloom.check(&check.as_bytes().to_vec()));
    }

    panic!("ok!");


    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        // .layer(Extension())
        .route("/", get(root));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}