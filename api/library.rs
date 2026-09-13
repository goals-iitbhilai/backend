use axum::{Router, routing::get};
use http::Method;
use tower::ServiceBuilder;
use tower_http::cors::{self, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;
use vercel_runtime::axum::VercelLayer;

#[tokio::main]
async fn main() -> Result<(), vercel_runtime::Error> {
    backend::telemetry::init();

    info!("starting library handler");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(cors::Any)
        .allow_headers(cors::Any);

    let router = Router::new()
        .fallback(get(backend::api::library::handler))
        .layer(TraceLayer::new_for_http());

    let app = ServiceBuilder::new()
        .layer(VercelLayer::new())
        .layer(cors)
        .service(router);

    vercel_runtime::run(app).await
}
