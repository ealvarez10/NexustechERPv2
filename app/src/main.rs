//! NexusTech ERP v2 — El ERP más rápido del mundo
//! Construido en Rust con Axum + PostgreSQL + Meilisearch

use axum::{Router, routing::get};
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health));

    let addr = SocketAddr::from(([0, 0, 0, 0], 4000));
    info!("NexusTech ERP v2 escuchando en {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn root() -> &'static str {
    "NexusTech ERP v2 — Initializing..."
}

async fn health() -> &'static str {
    "OK"
}
