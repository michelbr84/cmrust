//! CM Server - HTTP server for CM game.

use std::net::SocketAddr;

use axum::Router;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

use cm_api::routes::create_router;
use cm_telemetry::tracing::init_tracing;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing(EnvFilter::from_default_env().add_directive("info".parse()?));

    let app = Router::new()
        .merge(create_router())
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
