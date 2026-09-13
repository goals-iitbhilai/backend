use axum::{Router, routing::post};
use http::Method;
use tower::ServiceBuilder;
use tower_http::cors::{self, CorsLayer};
use vercel_runtime::axum::VercelLayer;

#[tokio::main]
async fn main() -> Result<(), vercel_runtime::Error> {
    let cors = CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_origin(cors::Any)
        .allow_headers(cors::Any);

    let router = Router::new().fallback(post(backend::api::contact::handler));

    let app = ServiceBuilder::new()
        .layer(VercelLayer::new())
        .layer(cors)
        .service(router);

    vercel_runtime::run(app).await
}
