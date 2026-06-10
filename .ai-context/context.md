# Proyecto: NexusTech ERP v2
**Última actualización:** 2026-06-09
**Stack:** Rust, Axum 0.8, SQLx, PostgreSQL (schema Odoo 19), Redis, Meilisearch, ring (criptografía), async-trait, quick-xml, CFDI 4.0
**Ruta raíz:** /home/ealvarez/workspace/NexustechERPv2

## Arquitectura
<!-- Describe cómo está estructurado el proyecto -->

## Rutas importantes
<!-- Ejemplo:
- Config: /home/ealvarez/workspace/NexustechERPv2\config\
- Build output: /home/ealvarez/workspace/NexustechERPv2\dist\
- Variables de entorno: archivo .env (NUNCA copiar valores aquí)
-->

## Cómo ejecutar
<!-- Comandos para dev, build, test -->

## Decisiones técnicas importantes
<!-- Por qué se eligió X en vez de Y, trade-offs conocidos -->

## Estado actual
<!-- Qué está funcionando, qué está pendiente -->


## Visión y Objetivo
El primer ERP nativo en Rust del mundo. 100x más rápido que Odoo.
KILLER FEATURE: Compatible DROP-IN con bases de datos NexusTech (Odoo 19 Enterprise) — sin migración, sin scripts, solo conectar y funcionar.
Objetivo de mercado: clientes actuales de Odoo que quieren velocidad + clientes nuevos atraídos por el performance.
Monetización: open-source (nexus-cfdi en crates.io) + ERP enterprise de pago.


## Arquitectura del Monorepo
Workspace Cargo en /home/ealvarez/workspace/NexustechERPv2/
Estructura:
  app/                     → Binario principal Axum (puerto 4000)
  crates/nexus-cfdi/       → CFDI 4.0 completo (PRIORIDAD #1 — EN PROGRESO)
  crates/nexus-core/       → Tipos, structs DB, auth, multi-tenant
  crates/nexus-crm/        → CRM — leads, pipeline
  crates/nexus-inventory/  → Inventario — movimientos, reservas, ubicaciones
  crates/nexus-ledger/     → Contabilidad doble entrada + DIOT + SAT
  crates/nexus-pos/        → POS offline-first (WASM)
  crates/nexus-sale/       → Ventas: cotización → orden → factura

GitHub: https://github.com/ealvarez10/NexustechERPv2
Git remote: usa token en env (ver nexustech erp repo para referencia)
Rama principal: main


## Base de Datos — Schema Odoo 19 Compatible
DB principal de desarrollo: nexus (PostgreSQL local, usuario ealvarez)
DB demo (con módulos instalados): demo
  → 135 módulos instalados: sale_management, account, crm, stock, purchase, point_of_sale, contacts, mail, stock_barcode, calendar + enterprise

Estrategia de compatibilidad:
  - Los structs Rust (SQLx FromRow) usan exactamente los mismos nombres de tabla y columna que Odoo 19
  - Un cliente puede apuntar su DB de NexusTech a este ERP sin migración
  - Los campos JSONB de Odoo se mapean como serde_json::Value

Tablas mapeadas en nexus-core/src/models.rs:
  - res_partner        (77 cols)
  - sale_order         (52 cols)
  - sale_order_line    (46 cols)
  - product_template   (95 cols, incluye x_mercadily_*)
  - account_move       (88 cols, incluye CFDI México l10n_mx_*)
  - account_move_line  (62 cols)
  - crm_lead           (no existe en DB demo/nexus básica, solo en instancias con CRM)
  - res_users, res_company, product_product

Binario NexusTech ERP: /home/ealvarez/.local/bin/nexustech (v19.0+e)
  Instalar módulos: bash start.sh -d <db> -i <modulos> --stop-after-init --log-level=warn
  PYTHONPATH requerido: /home/ealvarez/workspace/nexustech erp/core
  Nota: start.sh del repo nexustech erp ya lo configura automáticamente


## Entorno de Desarrollo
OS: Linux (local)
Rust: instalado con rustup
PostgreSQL: local, usuario ealvarez sin contraseña
Meilisearch: corriendo en 127.0.0.1:7700 (proceso PID 1751)
NexusTech Storefront (Rust): corriendo en target/release (PID 1752)
Repo nexustech erp: /home/ealvarez/workspace/nexustech erp/
Repo NexustechERPv2: /home/ealvarez/workspace/NexustechERPv2/

Comandos útiles:
  cargo check -p nexus-cfdi     → verificar compilación del crate
  cargo check                   → verificar todo el workspace
  psql -d demo                  → acceder a DB demo
  psql -d nexus                 → acceder a DB producción local


## NexusSearch — Motor de Búsqueda
NexusSearch es la instancia Meilisearch propia de NexusTech.
URL local dev: http://127.0.0.1:7700
Key local: variable MEILI_MASTER_KEY (ver .env del storefront)
Proceso PID 1751: /home/ealvarez/.local/bin/meilisearch --master-key=nexustech_dev_key_2026 --db-path=/home/ealvarez/workspace/nexustech-storefront/meili_data --no-analytics --http-addr=127.0.0.1:7700

Índices existentes (del storefront):
  - products      → productos Odoo
  - cva_products  → productos CVA Computación

Crate Rust para Meilisearch: meilisearch-sdk = '0.27' (ya en workspace dependencies)
También disponible vía reqwest HTTP directo (más eficiente con timeout controlado)

Uso en ERP v2:
  - Búsqueda global de productos, contactos, órdenes, leads
  - Auto-complete en POS
  - Búsqueda full-text de clientes para CRM
  - Filtros avanzados en inventario

Índices que hay que crear para ERP v2:
  - erp_products   → product_template + product_product (sincronizado desde PostgreSQL)
  - erp_partners   → res_partner (clientes y proveedores)
  - erp_orders     → sale_order (búsqueda por folio, cliente, estado)
  - erp_leads      → crm_lead (búsqueda full-text en pipeline)
  - erp_pos        → productos para autocompletado en POS

Sincronización:
  - Al crear/modificar un registro en PostgreSQL → indexar en NexusSearch
  - Usar tareas Tokio en background para no bloquear la respuesta HTTP
