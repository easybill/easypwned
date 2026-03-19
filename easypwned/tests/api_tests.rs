use std::sync::Arc;

use axum::body::Body;
use bloomfilter::Bloom;
use http::Request;
use http_body_util::BodyExt;
use tower::ServiceExt;

fn create_test_app() -> axum::Router {
    let seed = [1u8; 32];
    let mut bloom: Bloom<[u8]> = Bloom::new_for_fp_rate_with_seed(1000, 0.01, &seed).unwrap();
    // SHA1("password") = 5BAA61E4C9B93F3F0682250B6CF8331B7EE68FD8
    bloom.set(b"5BAA61E4C9B93F3F0682250B6CF8331B7EE68FD8");
    bloom.set(b"0000000CAEF405439D57847A8657218C618160B2");
    easypwned::create_router(Arc::new(bloom))
}

async fn get_json(app: axum::Router, uri: &str) -> serde_json::Value {
    let response = app
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), http::StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&body).unwrap()
}

#[tokio::test]
async fn test_hash_found() {
    let json = get_json(
        create_test_app(),
        "/hash/0000000CAEF405439D57847A8657218C618160B2",
    )
    .await;
    assert_eq!(json["hash"], "0000000CAEF405439D57847A8657218C618160B2");
    assert_eq!(json["secure"], false);
}

#[tokio::test]
async fn test_hash_not_found() {
    let json = get_json(
        create_test_app(),
        "/hash/FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
    )
    .await;
    assert_eq!(json["hash"], "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
    assert_eq!(json["secure"], true);
}

#[tokio::test]
async fn test_pw_known() {
    let json = get_json(create_test_app(), "/pw/password").await;
    assert_eq!(json["pw"], "password");
    assert_eq!(json["hash"], "5BAA61E4C9B93F3F0682250B6CF8331B7EE68FD8");
    assert_eq!(json["secure"], false);
}

#[tokio::test]
async fn test_pw_unknown() {
    let json = get_json(create_test_app(), "/pw/this_is_a_very_unique_safe_pw_12345").await;
    assert_eq!(json["secure"], true);
}

#[tokio::test]
async fn test_check_found() {
    let app = create_test_app();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/check")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"hash":"0000000CAEF405439D57847A8657218C618160B2"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), http::StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["hash"], "0000000CAEF405439D57847A8657218C618160B2");
    assert_eq!(json["secure"], false);
}

#[tokio::test]
async fn test_check_not_found() {
    let app = create_test_app();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/check")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"hash":"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), http::StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["secure"], true);
}

#[tokio::test]
async fn test_check_invalid_body() {
    let app = create_test_app();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/check")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"not_a_hash": 123}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_nonexistent_route() {
    let app = create_test_app();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/nonexistent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), http::StatusCode::NOT_FOUND);
}
