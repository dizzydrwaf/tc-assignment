use std::env;
use tower_http::cors::CorsLayer;
use http::{HeaderValue, Method, header};

pub fn dev() -> CorsLayer {
    CorsLayer::new()
        .allow_origin([
            HeaderValue::from_static("http://localhost:5173"),
            HeaderValue::from_static("http://127.0.0.1:5173"),
        ])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
        ])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
        ])
        .allow_credentials(true)
}

pub fn prod() -> CorsLayer {
    let origins: Vec<HeaderValue> = env::var("CORS_ORIGINS")
        .expect("CORS_ORIGINS must be set")
        .split(',')
        .map(|s| s.parse().expect("invalid origin"))
        .collect();

    CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
        ])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
        ])
        .allow_credentials(true)
}

