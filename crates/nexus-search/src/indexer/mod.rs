//! nexus-search::indexer — Sincronización ERP → Motor de búsqueda
//!
//! Sincroniza entidades del ERP (productos, partners, ventas) con el
//! motor de búsqueda de ultra alta velocidad en lotes eficientes.

use anyhow::Result;
use rust_decimal::Decimal;
use serde_json::{json, Value};
use sqlx::PgPool;
use tracing::{error, info, warn};

use crate::client::NexusSearchClient;
use crate::indexes;

/// Estadísticas de una operación de sincronización
#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct SyncStats {
    pub total_indexados: u64,
    pub total_errores: u64,
}

/// Sincroniza todos los índices ERP en secuencia
pub async fn sincronizar_todos(
    pool: &PgPool,
    client: &NexusSearchClient,
    company_id: i32,
) -> Result<SyncStats> {
    // Configurar índices
    configurar_indice_productos(client).await?;
    configurar_indice_partners(client).await?;

    let mut stats = SyncStats::default();

    // Productos
    match sincronizar_productos(pool, client, company_id).await {
        Ok(s) => {
            stats.total_indexados += s.total_indexados;
            stats.total_errores += s.total_errores;
        }
        Err(e) => {
            error!("Error sincronizando productos: {}", e);
            stats.total_errores += 1;
        }
    }

    // Partners (clientes y proveedores)
    match sincronizar_partners(pool, client).await {
        Ok(s) => {
            stats.total_indexados += s.total_indexados;
            stats.total_errores += s.total_errores;
        }
        Err(e) => {
            error!("Error sincronizando partners: {}", e);
            stats.total_errores += 1;
        }
    }

    info!(
        "Sincronización completa: {} indexados, {} errores",
        stats.total_indexados, stats.total_errores
    );
    Ok(stats)
}

// ── Configuración de índices ──────────────────────────────────────────────────

/// Crea y configura el índice de productos con atributos de búsqueda óptimos
pub async fn configurar_indice_productos(client: &NexusSearchClient) -> Result<()> {
    // create_index es idempotente — no falla si ya existe
    let _ = client
        .inner
        .create_index(indexes::PRODUCTS, Some("id"))
        .await;

    let index = client.inner.index(indexes::PRODUCTS);

    // Atributos buscables (orden = prioridad de relevancia)
    if let Err(e) = index
        .set_searchable_attributes(&["name", "default_code", "categ_name", "description_sale"])
        .await
    {
        warn!("No se pudo configurar searchable-attributes de {}: {}", indexes::PRODUCTS, e);
    }

    // Atributos filtrables
    if let Err(e) = index
        .set_filterable_attributes(&["active", "company_id", "categ_id", "tipo", "in_stock"])
        .await
    {
        warn!("No se pudo configurar filterable-attributes de {}: {}", indexes::PRODUCTS, e);
    }

    // Atributos ordenables
    if let Err(e) = index
        .set_sortable_attributes(&["name", "list_price", "qty_available"])
        .await
    {
        warn!("No se pudo configurar sortable-attributes de {}: {}", indexes::PRODUCTS, e);
    }

    info!("Índice {} configurado", indexes::PRODUCTS);
    Ok(())
}

/// Crea y configura el índice de partners
pub async fn configurar_indice_partners(client: &NexusSearchClient) -> Result<()> {
    let _ = client
        .inner
        .create_index(indexes::PARTNERS, Some("id"))
        .await;

    let index = client.inner.index(indexes::PARTNERS);

    let _ = index
        .set_searchable_attributes(&["name", "vat", "email", "phone", "city"])
        .await;
    let _ = index
        .set_filterable_attributes(&["is_company", "customer_rank", "supplier_rank"])
        .await;

    info!("Índice {} configurado", indexes::PARTNERS);
    Ok(())
}

// ── Sincronización de productos ───────────────────────────────────────────────

/// Sincroniza todos los productos activos de la empresa al índice de búsqueda.
///
/// Opera en lotes de 500 para evitar saturar la base de datos o el motor.
pub async fn sincronizar_productos(
    pool: &PgPool,
    client: &NexusSearchClient,
    company_id: i32,
) -> Result<SyncStats> {
    let batch_size: i64 = 500;
    let mut offset: i64 = 0;
    let mut stats = SyncStats::default();

    info!("Sincronizando productos, empresa={}", company_id);

    loop {
        let filas = sqlx::query!(
            r#"
            SELECT
                pt.id,
                pt.name,
                pt.default_code,
                pc.name  AS categ_name,
                pt.description_sale,
                pt.list_price,
                pt.active,
                pt.categ_id,
                pt.company_id,
                pt.type,
                0.0::double precision AS "qty_available!"
            FROM product_template pt
            LEFT JOIN product_category pc ON pt.categ_id = pc.id
            WHERE pt.active = true
              AND (pt.company_id = $1 OR pt.company_id IS NULL)
            ORDER BY pt.id
            LIMIT $2 OFFSET $3
            "#,
            company_id,
            batch_size,
            offset
        )
        .fetch_all(pool)
        .await;

        match filas {
            Ok(rows) if rows.is_empty() => break,
            Ok(rows) => {
                let n = rows.len() as u64;
                let docs: Vec<Value> = rows
                    .iter()
                    .map(|r| {
                        let qty: f64 = r.qty_available;
                        json!({
                            "id":               r.id,
                            "name":             r.name,
                            "default_code":     r.default_code,
                            "categ_name":       r.categ_name,
                            "description_sale": r.description_sale,
                            "list_price":       r.list_price,
                            "qty_available":    qty,
                            "active":           r.active.unwrap_or(true),
                            "company_id":       r.company_id,
                            "categ_id":         r.categ_id,
                            "tipo":             r.r#type,
                            "in_stock":         qty > 0.0,
                        })
                    })
                    .collect();

                match indexar_lote(client, indexes::PRODUCTS, &docs).await {
                    Ok(_) => {
                        stats.total_indexados += n;
                        info!("Productos: {} indexados (offset={})", n, offset);
                    }
                    Err(e) => {
                        stats.total_errores += n;
                        error!("Error indexando lote productos offset={}: {}", offset, e);
                    }
                }
                offset += batch_size;
            }
            Err(e) => {
                error!("Error leyendo productos de DB: {}", e);
                stats.total_errores += 1;
                break;
            }
        }
    }

    Ok(stats)
}

// ── Sincronización de partners ────────────────────────────────────────────────

/// Sincroniza clientes, proveedores y contactos al índice de partners.
pub async fn sincronizar_partners(pool: &PgPool, client: &NexusSearchClient) -> Result<SyncStats> {
    let batch_size: i64 = 500;
    let mut offset: i64 = 0;
    let mut stats = SyncStats::default();

    info!("Sincronizando partners...");

    loop {
        let filas = sqlx::query!(
            r#"
            SELECT id, name, vat, email, phone, street,
                   zip, city, is_company, customer_rank, supplier_rank
            FROM res_partner
            WHERE active = true AND name IS NOT NULL
            ORDER BY id
            LIMIT $1 OFFSET $2
            "#,
            batch_size,
            offset,
        )
        .fetch_all(pool)
        .await;

        match filas {
            Ok(rows) if rows.is_empty() => break,
            Ok(rows) => {
                let n = rows.len() as u64;
                let docs: Vec<Value> = rows
                    .iter()
                    .map(|r| {
                        json!({
                            "id":            r.id,
                            "name":          r.name,
                            "vat":           r.vat,
                            "email":         r.email,
                            "phone":         r.phone,
                            "street":        r.street,
                            "zip":           r.zip,
                            "city":          r.city,
                            "is_company":    r.is_company.unwrap_or(false),
                            "customer_rank": r.customer_rank.unwrap_or(0),
                            "supplier_rank": r.supplier_rank.unwrap_or(0),
                        })
                    })
                    .collect();

                match indexar_lote(client, indexes::PARTNERS, &docs).await {
                    Ok(_) => stats.total_indexados += n,
                    Err(e) => {
                        stats.total_errores += n;
                        error!("Error indexando partners offset={}: {}", offset, e);
                    }
                }
                offset += batch_size;
            }
            Err(e) => {
                error!("Error leyendo partners de DB: {}", e);
                stats.total_errores += 1;
                break;
            }
        }
    }

    Ok(stats)
}

// ── Utilidades ────────────────────────────────────────────────────────────────

/// Indexa un lote de documentos JSON en el índice especificado.
async fn indexar_lote(
    client: &NexusSearchClient,
    indice: &str,
    documentos: &[Value],
) -> Result<()> {
    if documentos.is_empty() {
        return Ok(());
    }

    let index = client.inner.index(indice);
    index
        .add_documents(documentos, Some("id"))
        .await
        .map_err(|e| anyhow::anyhow!("Error indexando en {}: {}", indice, e))?;

    Ok(())
}

/// Elimina un documento del índice por su ID.
pub async fn eliminar_documento(
    client: &NexusSearchClient,
    indice: &str,
    id: &str,
) -> Result<()> {
    let index = client.inner.index(indice);
    index
        .delete_document(id)
        .await
        .map_err(|e| anyhow::anyhow!("Error eliminando documento {}: {}", id, e))?;
    Ok(())
}
