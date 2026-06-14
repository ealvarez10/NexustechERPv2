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
    cors::Any,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use nexus_core::config::Config;
use state::AppState;
use pyo3::prelude::PyAnyMethods;

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
    ::_nexus::init_db_pool(db.clone());

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
            Arc::new(nexus_search::NexusSearchClient::from_env().unwrap_or_else(|_| {
                nexus_search::NexusSearchClient::fallback()
            }))
        }
    };

    // ── Fase 3: ORM Dinámico & CPython Embebido (PyO3) ──────────────────────
    tracing::info!("Inicializando entorno CPython Embebido con PyO3...");
    
    // 1. Agregar el módulo nativo _nexus a la tabla de inicialización de Python
    use _nexus::_nexus;
    pyo3::append_to_inittab!(_nexus);

    // 2. Inicializar el runtime de Python
    pyo3::prepare_freethreaded_python();

    let registry_opt = pyo3::Python::with_gil(|py| -> Result<Arc<nexus_orm::registry::Registry>, pyo3::PyErr> {
        let sys = py.import_bound("sys")?;
        let path = sys.getattr("path")?;
        path.call_method1("insert", (0, "/home/ealvarez/workspace/NexustechERPv2/shim"))?;
        path.call_method1("insert", (0, "/home/ealvarez/workspace/NexustechERPv2"))?; // para demo_addons

        tracing::info!("Cargando addons de Odoo en CPython...");
        let odoo_modules = py.import_bound("odoo.modules")?;
        odoo_modules.call_method1("load_addons", (
            vec!["/home/ealvarez/workspace/NexustechERPv2/demo_addons"],
            vec!["sale_mini"]
        ))?;

        // Consolidar el Registry final de Rust desde el estado estático de PyO3
        let reg = ::_nexus::build_registry_from_state()
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Fallo al construir registry: {:?}", e)))?;
        let reg_arc = Arc::new(reg);
        ::_nexus::init_registry(reg_arc.clone());
        Ok(reg_arc)
    });

    let registry_opt = match registry_opt {
        Ok(reg) => {
            tracing::info!("Kernel ORM inicializado con addons de Odoo en CPython ✓");
            Some(reg)
        }
        Err(e) => {
            tracing::error!("Fallo en inicialización de Python/Registry: {:?}", e);
            None
        }
    };

    // ── AppState ────────────────────────────────────────────────────────────
    let state = AppState::nueva(db, config.clone(), redis_conn, pac, search_client, registry_opt);

    // ── Router ─────────────────────────────────────────────────────────────
    // Rutas de autenticación — sin middleware JWT
    let auth_routes = Router::new()
        .route("/auth/login",   post(handlers::auth::login))
        .route("/auth/refresh", post(handlers::auth::refresh))
        .route("/auth/logout",  post(handlers::auth::logout))
        .with_state(state.clone());

    // Rutas protegidas — requieren Bearer token JWT
    let rutas_protegidas = Router::new()
        .route("/partners",             get(handlers::partners::listar).post(handlers::partners::crear))
        .route("/partners/{id}",        get(handlers::partners::obtener))
        .route("/clientes",             get(handlers::partners::clientes))
        .route("/proveedores",          get(handlers::partners::proveedores))
        .route("/productos",            get(handlers::products::listar).post(handlers::products::crear))
        .route("/productos/{id}",       get(handlers::products::obtener))
        .route("/ventas",                          get(handlers::ventas::listar).post(handlers::ventas::crear))
        .route("/ventas/kpis",                     get(handlers::ventas::kpis))
        .route("/ventas/buscar-clientes",          get(handlers::ventas::buscar_clientes))
        .route("/ventas/buscar-productos",         get(handlers::ventas::buscar_productos))
        .route("/ventas/{id}",                     get(handlers::ventas::obtener).put(handlers::ventas::actualizar))
        .route("/ventas/{id}/lineas",              get(handlers::ventas::lineas).post(handlers::ventas::agregar_linea))
        .route("/ventas/{id}/lineas/{lid}",        put(handlers::ventas::actualizar_linea).delete(handlers::ventas::eliminar_linea))
        .route("/ventas/{id}/confirmar",           put(handlers::ventas::confirmar))
        .route("/ventas/{id}/cancelar",            put(handlers::ventas::cancelar))
        .route("/ventas/{id}/enviar",              put(handlers::ventas::enviar))
        .route("/ventas/{id}/bloquear",            put(handlers::ventas::bloquear))
        .route("/ventas/{id}/borrador",            put(handlers::ventas::restaurar_borrador))
        // Flujo Ventas → Facturación
        .route("/ventas/{id}/crear-factura",        post(handlers::ventas::crear_factura))
        .route("/ventas/{id}/facturas",             get(handlers::ventas::facturas_de_venta))
        // Flujo Ventas → Almacén
        .route("/ventas/{id}/picking",              get(handlers::ventas::picking_de_venta))
        .route("/ventas/{id}/entrega",              get(handlers::ventas::entrega_de_venta))
        .route("/ventas/{id}/validar-entrega",      put(handlers::ventas::validar_entrega))
        // Duplicar orden
        .route("/ventas/{id}/duplicar",             post(handlers::ventas::duplicar))
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
        .route("/picking",              get(handlers::stock::listar_pickings))
        .route("/picking/{id}",         get(handlers::stock::obtener_picking))
        .route("/picking/{id}/validar", put(handlers::stock::validar_picking))
        // ── CFDI ─────────────────────────────────────────────────────────────
        .route("/cfdi/timbrar",         post(handlers::cfdi::timbrar))
        .route("/cfdi/cancelar",        post(handlers::cfdi::cancelar))
        .route("/cfdi/{uuid}/pdf",      get(handlers::cfdi::pdf_por_uuid))
        .route("/cfdi/timbrados",       get(handlers::cfdi_timbrados::listar_timbrados))
        .route("/cfdi/timbrados/{uuid}", get(handlers::cfdi_timbrados::obtener_timbrado))
        .route("/cfdi/kpis",            get(handlers::cfdi_timbrados::kpis_cfdi))
        .route("/nomina",               get(handlers::nomina::listar))
        .route("/nomina/kpis",          get(handlers::nomina::kpis))
        .route("/nomina/calcular",      post(handlers::nomina::calcular))
        .route("/nomina/{id}",          get(handlers::nomina::obtener))
        // ── Compras ──────────────────────────────────────────────────────────
        .route("/compras",              get(handlers::compras::listar).post(handlers::compras::crear))
        .route("/compras/kpis",         get(handlers::compras::kpis))
        .route("/compras/{id}",         get(handlers::compras::obtener))
        .route("/compras/{id}/lineas",  get(handlers::compras::lineas))
        .route("/compras/{id}/confirmar", post(handlers::compras::confirmar))
        .route("/compras/{id}/pagar",   post(handlers::compras::pagar))
        // ── Cotizaciones / Sale ───────────────────────────────────────────────
        .route("/cotizaciones",          get(handlers::sale::listar_cotizaciones).post(handlers::sale::crear_cotizacion))
        .route("/cotizaciones/kpis",     get(handlers::sale::kpis_cotizaciones))
        .route("/cotizaciones/{id}",     get(handlers::sale::obtener_cotizacion).put(handlers::sale::actualizar_cotizacion))
        .route("/cotizaciones/{id}/confirmar", put(handlers::sale::confirmar_cotizacion))
        .route("/cotizaciones/{id}/cancelar",  put(handlers::sale::cancelar_cotizacion))
        .route("/cotizaciones/{id}/lineas",    post(handlers::sale::agregar_linea))
        .route("/cotizaciones/{id}/lineas/{linea_id}", delete(handlers::sale::eliminar_linea))
        // ── Contabilidad — Asientos ────────────────────────────────────────────
        .route("/account-moves",                   get(handlers::account::listar).post(handlers::account::crear))
        .route("/account-moves/kpis",              get(handlers::account::kpis))
        .route("/account-moves/{id}",              get(handlers::account::obtener))
        .route("/account-moves/{id}/lineas",       get(handlers::account::lineas))
        .route("/account-moves/{id}/confirmar",    put(handlers::account::confirmar))
        .route("/account-moves/{id}/borrador",     put(handlers::account::borrador))
        .route("/account-moves/{id}/cancelar",     put(handlers::account::cancelar))
        // ── Motor de búsqueda ──────────────────────────────────────────────
        .route("/search/sync",          post(handlers::search::sync))
        .route("/search/status",        get(handlers::search::status))
        // ── Fase 3: ORM Dinámico Universal ──────────────────────────────────
        .route("/orm/{model}/{method}", post(handlers::orm_rpc::orm_rpc))
        // ── App Store ────────────────────────────────────────────────────────
        .route("/apps",                 get(handlers::apps::listar_apps))
        .route("/apps/{id}/install",    post(handlers::apps::instalar_app))
        .route("/apps/{id}/uninstall",  post(handlers::apps::desinstalar_app))
        .route("/ir-views",             get(handlers::ir_views::list_views))
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            middleware::auth_middleware,
        ))
        .with_state(state.clone());

    let api_v1 = auth_routes.merge(rutas_protegidas);

    let app = Router::new()
        .route("/health", get(handlers::health::health))
        
        // Original /web routes
        .route("/web", get(handlers::web::bootstrap))
        .route("/web/login", get(handlers::web::login_page).post(handlers::web::web_login))
        .route("/web/logout", get(handlers::web::web_logout))
        .route("/web/webclient/load_menus", get(handlers::web::load_menus))
        .route("/web/webclient/bootstrap_translations", post(handlers::web::bootstrap_translations))
        .route("/web/webclient/translations", get(handlers::web::translations))
        .route("/web/webclient/version_info", post(handlers::web::version_info))
        .route("/web/dataset/call_kw", post(handlers::web::dispatch_jsonrpc))
        .route("/web/dataset/search_read", post(handlers::web::dispatch_jsonrpc))
        // Odoo 17: /web/dataset/call_kw/{model}/{method} and /web/dataset/call_button
        .route("/web/dataset/call_kw/{model}/{method}", post(handlers::web::dispatch_jsonrpc))
        .route("/web/dataset/call_button", post(handlers::web::dispatch_jsonrpc))
        .route("/web/dataset/call_button/{model}/{method}", post(handlers::web::dispatch_jsonrpc))
        .route("/web/action/load", post(handlers::web::action_load))
        .route("/web/action/run", post(handlers::web::action_run))
        .route("/web/action/load_breadcrumbs", post(handlers::web::action_load_breadcrumbs))
        .route("/web/assets/{*path}", get(handlers::web::serve_attachment))
        .route("/web/content/{*path}", get(handlers::web::serve_attachment))
        .route("/web/image/{*path}", get(handlers::web::serve_attachment))
        .route("/web/image", get(handlers::web::serve_attachment))
        .route("/web/bundle/{bundle_name}", get(handlers::web::serve_bundle))
        
        // Rebranded /nexustech routes
        .route("/nexustech", get(handlers::web::bootstrap))
        .route("/nexustech/{*path}", get(handlers::web::bootstrap))
        .route("/nexustech/login", get(handlers::web::login_page).post(handlers::web::web_login))
        .route("/nexustech/logout", get(handlers::web::web_logout))
        .route("/nexustech/webclient/load_menus", get(handlers::web::load_menus))
        .route("/nexustech/webclient/bootstrap_translations", post(handlers::web::bootstrap_translations))
        .route("/nexustech/webclient/translations", get(handlers::web::translations))
        .route("/nexustech/webclient/version_info", post(handlers::web::version_info))
        .route("/nexustech/dataset/call_kw", post(handlers::web::dispatch_jsonrpc))
        .route("/nexustech/dataset/search_read", post(handlers::web::dispatch_jsonrpc))
        // Odoo 17: /nexustech/dataset/call_kw/{model}/{method} and call_button
        .route("/nexustech/dataset/call_kw/{model}/{method}", post(handlers::web::dispatch_jsonrpc))
        .route("/nexustech/dataset/call_button", post(handlers::web::dispatch_jsonrpc))
        .route("/nexustech/dataset/call_button/{model}/{method}", post(handlers::web::dispatch_jsonrpc))
        .route("/nexustech/action/load", post(handlers::web::action_load))
        .route("/nexustech/action/run", post(handlers::web::action_run))
        .route("/nexustech/action/load_breadcrumbs", post(handlers::web::action_load_breadcrumbs))
        .route("/nexustech/assets/{*path}", get(handlers::web::serve_attachment))
        .route("/nexustech/content/{*path}", get(handlers::web::serve_attachment))
        .route("/nexustech/image/{*path}", get(handlers::web::serve_attachment))
        .route("/nexustech/image", get(handlers::web::serve_attachment))
        .route("/nexustech/bundle/{bundle_name}", get(handlers::web::serve_bundle))
        
        // Static routes
        .route("/{addon}/static/{*path}", get(handlers::web::serve_static))

        // ── Mail bus / longpolling (causa principal de 'Connection lost') ──
        .route("/mail/data",                 post(handlers::web::mail_data))
        .route("/nexustech/mail/data",       post(handlers::web::mail_data))
        .route("/mail/message/fetch",        post(handlers::web::mail_data))
        .route("/nexustech/mail/message/fetch", post(handlers::web::mail_data))
        .route("/mail/thread/messages",      post(handlers::web::mail_data))
        .route("/nexustech/mail/thread/messages", post(handlers::web::mail_data))
        .route("/mail/thread/data",          post(handlers::web::mail_data))
        .route("/nexustech/mail/thread/data", post(handlers::web::mail_data))

        // ── PWA / Service Worker ──────────────────────────────────────────
        .route("/web/manifest.webmanifest",        get(handlers::web::serve_manifest))
        .route("/nexustech/manifest.webmanifest",  get(handlers::web::serve_manifest))
        .route("/web/service-worker.js",           get(handlers::web::serve_service_worker))
        .route("/nexustech/service-worker.js",     get(handlers::web::serve_service_worker))

        // Bus / Websocket routes
        .route("/websocket", get(handlers::web::serve_websocket))
        .route("/websocket/health", get(handlers::web::websocket_health))
        .route("/websocket/peek_notifications", post(handlers::web::websocket_peek_notifications))
        .route("/websocket/on_closed", post(handlers::web::websocket_on_closed))
        .route("/bus/websocket_worker_bundle", get(handlers::web::serve_websocket_worker_bundle))
        
        .route("/nexustech/websocket", get(handlers::web::serve_websocket))
        .route("/nexustech/websocket/health", get(handlers::web::websocket_health))
        .route("/nexustech/websocket/peek_notifications", post(handlers::web::websocket_peek_notifications))
        .route("/nexustech/websocket/on_closed", post(handlers::web::websocket_on_closed))
        .route("/nexustech/bus/websocket_worker_bundle", get(handlers::web::serve_websocket_worker_bundle))
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
