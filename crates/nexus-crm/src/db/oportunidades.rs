//! Oportunidades y leads del pipeline CRM

use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::error::CrmError;

/// Oportunidad / Lead CRM — columnas reales de crm_lead
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Oportunidad {
    pub id: i32,
    pub name: String,
    /// Discriminador: 'opportunity' | 'lead'
    #[sqlx(rename = "type")]
    pub tipo: Option<String>,
    /// Prioridad: '0' normal, '1' alta, '2' muy alta, '3' crítica
    pub priority: Option<String>,
    pub stage_id: Option<i32>,
    pub partner_id: Option<i32>,
    pub partner_name: Option<String>,
    pub company_id: Option<i32>,
    pub user_id: Option<i32>,
    pub team_id: Option<i32>,
    pub contact_name: Option<String>,
    pub email_from: Option<String>,
    pub phone: Option<String>,
    pub expected_revenue: Option<Decimal>,
    pub prorated_revenue: Option<Decimal>,
    pub probability: Option<f64>,
    /// Estado ganado/perdido: 'won' | 'lost' | null
    pub won_status: Option<String>,
    pub active: Option<bool>,
    pub create_date: Option<NaiveDateTime>,
    pub date_closed: Option<NaiveDateTime>,
    pub date_deadline: Option<chrono::NaiveDate>,
}

/// KPIs del pipeline CRM
#[derive(Debug, Serialize)]
pub struct KpisCrm {
    pub total_oportunidades: i64,
    pub total_leads: i64,
    pub revenue_esperado: Decimal,
    pub probabilidad_promedio: f64,
    pub ganadas: i64,
    pub perdidas: i64,
}

/// Fila interna para agregaciones de KPIs
#[derive(sqlx::FromRow)]
struct KpisRow {
    total_oportunidades: i64,
    total_leads: i64,
    revenue_esperado: Option<Decimal>,
    probabilidad_promedio: Option<f64>,
    ganadas: i64,
    perdidas: i64,
}

/// Fila interna para la consulta por etapa
#[derive(sqlx::FromRow)]
struct EtapaRow {
    nombre_etapa: Option<String>,
    cantidad: i64,
    revenue_total: Option<Decimal>,
}

/// Lista oportunidades (type = 'opportunity') paginado
pub async fn listar_oportunidades(
    pool: &PgPool,
    company_id: i32,
    pagina: i64,
    por_pagina: i64,
) -> Result<Vec<Oportunidad>, CrmError> {
    let offset = (pagina.saturating_sub(1)) * por_pagina;
    let registros = sqlx::query_as::<_, Oportunidad>(
        r#"
        SELECT
            id,
            name,
            type,
            priority,
            stage_id,
            partner_id,
            partner_name,
            company_id,
            user_id,
            team_id,
            contact_name,
            email_from,
            phone,
            expected_revenue,
            prorated_revenue,
            probability,
            won_status,
            active,
            create_date,
            date_closed,
            date_deadline
        FROM crm_lead
        WHERE type = 'opportunity'
          AND (company_id = $1 OR company_id IS NULL)
          AND active = TRUE
        ORDER BY id DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(company_id)
    .bind(por_pagina)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(registros)
}

/// Lista leads (type = 'lead') paginado
pub async fn listar_leads(
    pool: &PgPool,
    company_id: i32,
    pagina: i64,
    por_pagina: i64,
) -> Result<Vec<Oportunidad>, CrmError> {
    let offset = (pagina.saturating_sub(1)) * por_pagina;
    let registros = sqlx::query_as::<_, Oportunidad>(
        r#"
        SELECT
            id,
            name,
            type,
            priority,
            stage_id,
            partner_id,
            partner_name,
            company_id,
            user_id,
            team_id,
            contact_name,
            email_from,
            phone,
            expected_revenue,
            prorated_revenue,
            probability,
            won_status,
            active,
            create_date,
            date_closed,
            date_deadline
        FROM crm_lead
        WHERE type = 'lead'
          AND (company_id = $1 OR company_id IS NULL)
          AND active = TRUE
        ORDER BY id DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(company_id)
    .bind(por_pagina)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(registros)
}

/// KPIs del pipeline CRM para una empresa
pub async fn kpis(pool: &PgPool, company_id: i32) -> Result<KpisCrm, CrmError> {
    let row = sqlx::query_as::<_, KpisRow>(
        r#"
        SELECT
            COUNT(*) FILTER (WHERE type = 'opportunity') AS total_oportunidades,
            COUNT(*) FILTER (WHERE type = 'lead')        AS total_leads,
            SUM(expected_revenue) FILTER (WHERE type = 'opportunity' AND active = TRUE)
                AS revenue_esperado,
            AVG(probability) FILTER (WHERE type = 'opportunity' AND active = TRUE)
                AS probabilidad_promedio,
            COUNT(*) FILTER (WHERE won_status = 'won')   AS ganadas,
            COUNT(*) FILTER (WHERE won_status = 'lost')  AS perdidas
        FROM crm_lead
        WHERE (company_id = $1 OR company_id IS NULL)
        "#,
    )
    .bind(company_id)
    .fetch_one(pool)
    .await?;

    Ok(KpisCrm {
        total_oportunidades: row.total_oportunidades,
        total_leads: row.total_leads,
        revenue_esperado: row.revenue_esperado.unwrap_or(Decimal::ZERO),
        probabilidad_promedio: row.probabilidad_promedio.unwrap_or(0.0),
        ganadas: row.ganadas,
        perdidas: row.perdidas,
    })
}

/// Oportunidades agrupadas por etapa (para vista kanban)
/// Retorna: `(nombre_etapa, cantidad, revenue_total)`
pub async fn por_etapa(
    pool: &PgPool,
    company_id: i32,
) -> Result<Vec<(String, i64, Decimal)>, CrmError> {
    let filas = sqlx::query_as::<_, EtapaRow>(
        r#"
        SELECT
            cs.name::text              AS nombre_etapa,
            COUNT(cl.id)               AS cantidad,
            COALESCE(SUM(cl.expected_revenue), 0) AS revenue_total
        FROM crm_lead cl
        LEFT JOIN crm_stage cs ON cs.id = cl.stage_id
        WHERE cl.type = 'opportunity'
          AND cl.active = TRUE
          AND (cl.company_id = $1 OR cl.company_id IS NULL)
        GROUP BY cs.id, cs.name, cs.sequence
        ORDER BY cs.sequence ASC NULLS LAST
        "#,
    )
    .bind(company_id)
    .fetch_all(pool)
    .await?;

    let resultado = filas
        .into_iter()
        .map(|f| {
            (
                f.nombre_etapa.unwrap_or_else(|| "Sin etapa".to_string()),
                f.cantidad,
                f.revenue_total.unwrap_or(Decimal::ZERO),
            )
        })
        .collect();

    Ok(resultado)
}

// ── Tests unitarios (sin DB real, con datos mock) ─────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    fn oportunidad_mock(id: i32, tipo: &str, won_status: Option<&str>) -> Oportunidad {
        Oportunidad {
            id,
            name: format!("Registro {id}"),
            tipo: Some(tipo.to_string()),
            priority: Some("1".to_string()),
            stage_id: Some(1),
            partner_id: Some(10),
            partner_name: Some("Cliente SA".to_string()),
            company_id: Some(1),
            user_id: Some(2),
            team_id: Some(1),
            contact_name: Some("Juan Pérez".to_string()),
            email_from: Some("juan@ejemplo.com".to_string()),
            phone: Some("+52 55 0000 0000".to_string()),
            expected_revenue: Some(Decimal::new(150_000, 2)), // 1500.00
            prorated_revenue: Some(Decimal::new(75_000, 2)),
            probability: Some(60.0),
            won_status: won_status.map(|s| s.to_string()),
            active: Some(true),
            create_date: None,
            date_closed: None,
            date_deadline: None,
        }
    }

    #[test]
    fn test_oportunidad_mock_campos_basicos() {
        let op = oportunidad_mock(42, "opportunity", None);
        assert_eq!(op.id, 42);
        assert_eq!(op.tipo.as_deref(), Some("opportunity"));
        assert_eq!(op.priority.as_deref(), Some("1"));
        assert!(op.active.unwrap_or(false));
    }

    #[test]
    fn test_kpis_calculo_local() {
        let lista = vec![
            oportunidad_mock(1, "opportunity", Some("won")),
            oportunidad_mock(2, "opportunity", Some("lost")),
            oportunidad_mock(3, "opportunity", None),
            oportunidad_mock(4, "lead", None),
        ];

        let total_ops = lista.iter().filter(|o| o.tipo.as_deref() == Some("opportunity")).count();
        let total_leads = lista.iter().filter(|o| o.tipo.as_deref() == Some("lead")).count();
        let ganadas = lista.iter().filter(|o| o.won_status.as_deref() == Some("won")).count();
        let perdidas = lista.iter().filter(|o| o.won_status.as_deref() == Some("lost")).count();

        assert_eq!(total_ops, 3);
        assert_eq!(total_leads, 1);
        assert_eq!(ganadas, 1);
        assert_eq!(perdidas, 1);

        let revenue_total: Decimal = lista
            .iter()
            .filter_map(|o| o.expected_revenue)
            .fold(Decimal::ZERO, |acc, v| acc + v);

        assert!(revenue_total > Decimal::ZERO);
    }

    #[test]
    fn test_paginacion_offset() {
        let pagina: i64 = 3;
        let por_pagina: i64 = 20;
        let offset = (pagina.saturating_sub(1)) * por_pagina;
        assert_eq!(offset, 40);
    }

    #[test]
    fn test_paginacion_primera_pagina() {
        let pagina: i64 = 1;
        let por_pagina: i64 = 50;
        let offset = (pagina.saturating_sub(1)) * por_pagina;
        assert_eq!(offset, 0);
    }
}
