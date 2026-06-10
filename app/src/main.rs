//! NexusTech ERP v2 — Servidor principal
//!
//! Arranca el servidor Axum con:
//! - Pool PostgreSQL (SQLx)
//! - Redis (opcional)
//! - Middleware JWT
//! - Router REST v1

mod api;
mod handlers;
mod middleware;
mod state;

use axum::{
    middleware as axum_middleware,
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use nexus_core::config::Config;
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ── Tracing / Logging ──────────────────────────────────────────────────
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,nexustech_erp=debug".into()))
        .with(tracing_subscriber::fmt::layer().with_target(true))
        .init();

    tracing::info!("NexusTech ERP v2 iniciando...");

    // ── Configuración ──────────────────────────────────────────────────────
    let config = Config::from_env()?;
    tracing::info!(
        "Entorno: {} | Puerto: {}",
        config.environment,
        config.server_port
    );

    // ── Base de datos ──────────────────────────────────────────────────────
    tracing::info!("Conectando a la base de datos...");
    let db = PgPoolOptions::new()
        .max_connections(config.database_pool_max)
        .acquire_timeout(Duration::from_secs(config.database_connect_timeout_secs))
        .connect(&config.database_url)
        .await?;
    tracing::info!("Conexión a la base de datos establecida ✓");

    // ── Redis (opcional) ────────────────────────────────────────────────────
    let redis_conn = match redis::Client::open(config.redis_url.as_str()) {
        Ok(client) => match client.get_multiplexed_async_connection().await {
            Ok(conn) => {
                tracing::info!("Redis conectado ✓");
                Some(conn)
            }
            Err(e) => {
                tracing::warn!("Redis no disponible (sin caché): {}", e);
                None
            }
        },
        Err(e) => {
            tracing::warn!("Redis mal configurado: {}", e);
            None
        }
    };

    // ── AppState ────────────────────────────────────────────────────────────
    let state = AppState::nueva(db, config.clone(), redis_conn);

    // ── Router ─────────────────────────────────────────────────────────────
    let api_v1 = Router::new()
        // Auth (sin middleware JWT)
        .route("/auth/login",   post(handlers::auth::login))
        .route("/auth/refresh", post(handlers::auth::refresh))
        .route("/auth/logout",  post(handlers::auth::logout))
        // Recursos protegidos con JWT
        .route("/partners",    get(handlers::partners::listar))
        .route("/partners/:id", get(handlers::partners::obtener))
        .route("/clientes",    get(handlers::partners::clientes))
        .route("/proveedores", get(handlers::partners::proveedores))
        .route("/productos",   get(handlers::products::listar))
        .route("/productos/:id", get(handlers::products::obtener))
        .route("/ventas",      get(handlers::ventas::listar))
        .route("/ventas/kpis", get(handlers::ventas::kpis))
        .route("/ventas/:id",  get(handlers::ventas::obtener))
        .route("/facturas",         get(handlers::facturas::listar))
        .route("/facturas/kpis",    get(handlers::facturas::kpis))
        .route("/facturas/por-cobrar", get(handlers::facturas::por_cobrar))
        .route("/facturas/:id",     get(handlers::facturas::obtener))
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            middleware::auth_middleware,
        ))
        .with_state(state.clone());

    let app = Router::new()
        .route("/health", get(handlers::health::health))
        .nest("/api/v1", api_v1)
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // ── Arrancar ────────────────────────────────────────────────────────────
    let addr = config.server_addr();
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("🚀 NexusTech ERP v2 escuchando en http://{}", addr);
    tracing::info!("   Health: http://{}/health", addr);
    tracing::info!("   API:    http://{}/api/v1/", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
