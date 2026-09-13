use axum::{Router, routing::get};
use http::Method;
use tower::ServiceBuilder;
use tower_http::cors::{self, CorsLayer};
use vercel_runtime::axum::VercelLayer;

#[tokio::main]
async fn main() -> Result<(), vercel_runtime::Error> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(cors::Any)
        .allow_headers(cors::Any);

    let router = Router::new().fallback(get(backend::api::library::handler));

    let app = ServiceBuilder::new()
        .layer(VercelLayer::new())
        .layer(cors)
        .service(router);

    vercel_runtime::run(app).await
}
