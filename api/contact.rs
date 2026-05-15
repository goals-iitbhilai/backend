use axum::{Router, routing::post};
use backend::cors::cors_layer;
use tower::ServiceBuilder;
use vercel_runtime::axum::VercelLayer;

#[tokio::main]
async fn main() -> Result<(), vercel_runtime::Error> {
    let router = Router::new().fallback(post(backend::api::contact::handler));

    let app = ServiceBuilder::new()
        .layer(VercelLayer::new())
        .layer(cors_layer())
        .service(router);

    vercel_runtime::run(app).await
}
