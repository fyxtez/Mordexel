use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};

// TODO: When adding auth, replace Any with explicit origins and enable allow_credentials(true)
pub fn build_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
        ])
        .allow_headers([
            "Content-Type".parse().unwrap(),
            "Authorization".parse().unwrap(),
        ])
}
