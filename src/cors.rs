use http::Method;
use tower_http::cors::{self, CorsLayer};

pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_origin(cors::Any)
        .allow_headers(cors::Any)
}
