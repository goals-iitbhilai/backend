use axum::{Router, routing::post};
use http::Method;
use tower::ServiceBuilder;
use tower_http::cors::{self, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;
use vercel_runtime::axum::VercelLayer;

#[tokio::main]
async fn main() -> Result<(), vercel_runtime::Error> {
    backend::telemetry::init();

    info!("starting contact handler");

    let cors = CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_origin(cors::Any)
        .allow_headers(cors::Any);

    let router = Router::new()
        .fallback(post(backend::api::contact::handler))
        .layer(TraceLayer::new_for_http());

    let app = ServiceBuilder::new()
        .layer(VercelLayer::new())
        .layer(cors)
        .service(router);

    vercel_runtime::run(app).await
}
