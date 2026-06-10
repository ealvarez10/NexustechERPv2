# NexusTech ERP v2

**El primer ERP nativo en Rust del mundo** — 100x más rápido que Odoo, con CFDI 4.0 mexicano integrado.

## Estructura

```
NexustechERPv2/
├── app/                    # Aplicación principal (Axum)
├── crates/
│   ├── nexus-cfdi/         # 🎯 CFDI 4.0 — Primer crate del mundo en Rust
│   ├── nexus-core/         # Tipos compartidos, auth, multi-tenant
│   ├── nexus-crm/          # CRM — leads, pipeline, contactos
│   ├── nexus-inventory/    # Inventario — movimientos, reservas, ubicaciones
│   ├── nexus-ledger/       # Contabilidad doble entrada + DIOT + SAT
│   ├── nexus-pos/          # POS offline-first (WASM)
│   └── nexus-sale/         # Ventas — cotización → orden → factura
└── docs/                   # Documentación técnica
```

## Tech Stack

- **Runtime:** Axum 0.8 + Tokio
- **DB:** PostgreSQL + SQLx
- **Cache:** Redis + Moka
- **Search:** Meilisearch
- **CFDI:** `nexus-cfdi` (propio) + PAC REST
- **Contabilidad:** `cala-ledger` (GaloyMoney) extendido
- **UI:** HTMX + Askama

## Performance vs Odoo

| Métrica | Odoo | NexusTech ERP v2 |
|---|---|---|
| Requests/seg | ~1,000 | ~100,000 |
| RAM | 2-4 GB | 50-150 MB |
| Hosting | $100-500/mes | $5-20/mes |
| Latencia p99 | 500ms-2s | 5-20ms |

## Roadmap

- [ ] Fase 1: `nexus-cfdi` — CFDI 4.0 + timbrado PAC
- [ ] Fase 2: CRM + Ventas + Contactos
- [ ] Fase 3: Inventario + Código de barras
- [ ] Fase 4: Contabilidad completa + DIOT
- [ ] Fase 5: POS offline-first (WASM)
