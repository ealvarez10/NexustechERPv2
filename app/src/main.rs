//! NexusTech ERP v2 — Servidor principal
//!
//! Arranca el servidor Axum con:
//! - Pool PostgreSQL (SQLx)
//! - Redis (opcional)
//! - Middleware JWT
//! - Router REST v1
//! - PAC timbrado CFDI
//! - Motor de búsqueda integrado

mod api;
mod handlers;
mod middleware;
mod state;

use axum::{
    middleware as axum_middleware,
    routing::{delete, get, post, put},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
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

    // ── PAC — Proveedor de Certificación (timbrado CFDI) ──────────────────
    let pac_mode = std::env::var("PAC_MODE").unwrap_or_else(|_| "sandbox".to_string());
    let pac_user = std::env::var("PAC_USER").unwrap_or_default();
    let pac_password = std::env::var("PAC_PASSWORD").unwrap_or_default();

    let pac: Arc<dyn nexus_cfdi::Pac> = if pac_mode == "produccion" {
        tracing::info!("PAC: modo producción");
        Arc::new(nexus_cfdi::FacturamaPac::produccion(pac_user, pac_password))
    } else {
        tracing::info!("PAC: modo sandbox (pruebas)");
        Arc::new(nexus_cfdi::FacturamaPac::sandbox(pac_user, pac_password))
    };

    // ── Motor de búsqueda integrado ────────────────────────────────────────
    let search_client = match nexus_search::NexusSearchClient::from_env() {
        Ok(client) => {
            tracing::info!("Motor de búsqueda conectado ✓");
            Arc::new(client)
        }
        Err(e) => {
            tracing::warn!("Motor de búsqueda no disponible: {}", e);
            // Crear cliente con valores por defecto — fallará en runtime pero no bloquea el arranque
            Arc::new(nexus_search::NexusSearchClient::from_env().unwrap_or_else(|_| {
                nexus_search::NexusSearchClient::fallback()
            }))
        }
    };

    // ── AppState ────────────────────────────────────────────────────────────
    let state = AppState::nueva(db, config.clone(), redis_conn, pac, search_client);

    // ── Router ─────────────────────────────────────────────────────────────
    // Rutas de autenticación — sin middleware JWT
    let auth_routes = Router::new()
        .route("/auth/login",   post(handlers::auth::login))
        .route("/auth/refresh", post(handlers::auth::refresh))
        .route("/auth/logout",  post(handlers::auth::logout))
        .with_state(state.clone());

    // Rutas protegidas — requieren Bearer token JWT
    let rutas_protegidas = Router::new()
        .route("/partners",             get(handlers::partners::listar))
        .route("/partners/{id}",        get(handlers::partners::obtener))
        .route("/clientes",             get(handlers::partners::clientes))
        .route("/proveedores",          get(handlers::partners::proveedores))
        .route("/productos",            get(handlers::products::listar))
        .route("/productos/{id}",       get(handlers::products::obtener))
        .route("/ventas",               get(handlers::ventas::listar).post(handlers::ventas::crear))
        .route("/ventas/kpis",          get(handlers::ventas::kpis))
        .route("/ventas/{id}",          get(handlers::ventas::obtener))
        .route("/ventas/{id}/lineas",   get(handlers::ventas::lineas))
        .route("/ventas/{id}/confirmar", put(handlers::ventas::confirmar))
        .route("/ventas/{id}/cancelar",  put(handlers::ventas::cancelar))
        // ── Facturas ─────────────────────────────────────────────────────────
        .route("/facturas",             get(handlers::facturas::listar).post(handlers::facturas::crear))
        .route("/facturas/kpis",        get(handlers::facturas::kpis))
        .route("/facturas/por-cobrar",  get(handlers::facturas::por_cobrar))
        .route("/facturas/{id}",        get(handlers::facturas::obtener))
        .route("/facturas/{id}/lineas",    get(handlers::facturas::lineas))
        .route("/facturas/{id}/confirmar", put(handlers::facturas::confirmar))
        .route("/facturas/{id}/pago",      post(handlers::facturas::registrar_pago))
        .route("/facturas/{id}/cancelar",  put(handlers::facturas::cancelar))
        // ── Dashboard ────────────────────────────────────────────────────────
        .route("/dashboard",            get(handlers::dashboard::kpis))
        // ── Stock ────────────────────────────────────────────────────────────
        .route("/stock",                get(handlers::stock::listar))
        .route("/stock/kpis",           get(handlers::stock::kpis))
        .route("/stock/bajo",           get(handlers::stock::bajo))
        .route("/stock/producto/{id}",  get(handlers::stock::por_producto))
        // ── CFDI ─────────────────────────────────────────────────────────────
        .route("/cfdi/timbrar",         post(handlers::cfdi::timbrar))
        .route("/cfdi/cancelar",        post(handlers::cfdi::cancelar))
        .route("/cfdi/{uuid}/pdf",      get(handlers::cfdi::pdf_por_uuid))
        .route("/cfdi/timbrados",       get(handlers::cfdi_timbrados::listar_timbrados))
        .route("/cfdi/timbrados/{uuid}", get(handlers::cfdi_timbrados::obtener_timbrado))
        .route("/cfdi/kpis",            get(handlers::cfdi_timbrados::kpis_cfdi))
        .route("/nomina",               get(handlers::nomina::listar))
        .route("/nomina/kpis",          get(handlers::nomina::kpis))
        .route("/nomina/{id}",          get(handlers::nomina::obtener))
        // ── Compras ──────────────────────────────────────────────────────────
        .route("/compras",              get(handlers::compras::listar))
        .route("/compras/kpis",         get(handlers::compras::kpis))
        .route("/compras/{id}",         get(handlers::compras::obtener))
        .route("/compras/{id}/lineas",  get(handlers::compras::lineas))
        // ── Cotizaciones / Sale ───────────────────────────────────────────────
        .route("/cotizaciones",          get(handlers::sale::listar_cotizaciones).post(handlers::sale::crear_cotizacion))
        .route("/cotizaciones/kpis",     get(handlers::sale::kpis_cotizaciones))
        .route("/cotizaciones/{id}",     get(handlers::sale::obtener_cotizacion).put(handlers::sale::actualizar_cotizacion))
        .route("/cotizaciones/{id}/confirmar", put(handlers::sale::confirmar_cotizacion))
        .route("/cotizaciones/{id}/cancelar",  put(handlers::sale::cancelar_cotizacion))
        .route("/cotizaciones/{id}/lineas",    post(handlers::sale::agregar_linea))
        .route("/cotizaciones/{id}/lineas/{linea_id}", delete(handlers::sale::eliminar_linea))
        // ── Motor de búsqueda ──────────────────────────────────────────────
        .route("/search/sync",          post(handlers::search::sync))
        .route("/search/status",        get(handlers::search::status))
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            middleware::auth_middleware,
        ))
        .with_state(state.clone());

    let api_v1 = auth_routes.merge(rutas_protegidas);

    let app = Router::new()
        .route("/health", get(handlers::health::health))
        .nest("/api/v1", api_v1.merge(
            Router::new()
                .route("/health", get(handlers::health::health))
                .with_state(state.clone())
        ))
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
