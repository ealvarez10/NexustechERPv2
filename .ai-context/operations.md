# Log de Operaciones

> Registro cronológico de todo lo que se ha hecho en el proyecto.
> El agente actualiza este archivo después de cada operación significativa.

## 2026-06-09 — Sesión inicial
- Memoria del proyecto inicializada con project-memory skill


## Sesión 2026-06-10 — Fundación del proyecto
- **2026-06-09 21:30** — COMPLETADO: Monorepo Cargo creado con 7 crates (nexus-cfdi, nexus-core, nexus-crm, nexus-inventory, nexus-ledger, nexus-pos, nexus-sale) + app principal Axum puerto 4000. Commit fd8c504 en GitHub.
- **2026-06-09 21:30** — COMPLETADO: nexus-core/src/models.rs — 9 structs Rust mapeados 1:1 al schema PostgreSQL de la DB nexus (NexusTech Odoo 19): ResPartner(77), SaleOrder(52), SaleOrderLine(46), ProductTemplate(95 incl x_mercadily_*), AccountMove(88 incl CFDI), AccountMoveLine(62), CrmLead, ResUsers, ResCompany, ProductProduct. Commit 716f477.
- **2026-06-09 21:30** — COMPLETADO: nexus-cfdi — Implementación CFDI 4.0 completa: builder.rs (tipos+builder fluido+cálculo automático de impuestos), cadena_original.rs (Anexo 20 sin XSLT), sellado.rs (RSA-SHA256 con ring + lectura DER del CSD), error.rs (tipos de error), pac/mod.rs (trait Pac), pac/facturama.rs (REST Facturama prod+sandbox), pac/sw_sapien.rs (REST SW Sapien prod+sandbox). cargo check OK. Commit 48eae5a.
- **2026-06-09 21:30** — COMPLETADO: DB demo — 135 módulos NexusTech instalados (sale_management, account, crm, stock, purchase, point_of_sale, contacts, mail, stock_barcode, calendar + enterprise). Binario: /home/ealvarez/.local/bin/nexustech. Usando bash start.sh del repo nexustech erp con PYTHONPATH configurado.
- **2026-06-09 21:36** — INICIANDO: Fase 1 continuación — xml.rs (generador XML CFDI 4.0), rfc.rs (validador RFC), catalogs/ (catálogos SAT embebidos), validacion.rs (pre-flight). Workspace cargo check: 0 errores.
- **2026-06-09 21:42** — COMPLETADO: nexus-cfdi Fase 1.1-1.4 — xml.rs (generador XML CFDI 4.0 con quick-xml, 18 tests), rfc.rs (validador RFC PF/PM/Genérico, NO existía en Rust), 14 catálogos SAT embebidos (c_FormaPago, MetodoPago, UsoCFDI, RegimenFiscal, TipoComprobante, Exportacion, Impuesto, TipoFactor, ObjetoImp, ClaveUnidad, MotivoCancelacion, TipoRelacion, Pais, Periodicidad), validacion.rs (pre-flight validator: RFC+catálogos+PUE/PPD+totales matemáticos). 18/18 tests OK. Push commit 1607fcd.

## 2026-06-13 — Fix vista Apps (ir.module.module)
- **2026-06-12 19:44** — COMPLETADO: Corregido bug crítico en get_views handler. El backend leía columna 'arch' inexistente en ir_ui_view en lugar de 'arch_db' (JSONB multilingual). El fallback tomaba las primeras 6 columnas de la DB (id, create_uid, create_date, write_date, write_uid, website) mostrando números en la vista de Apps.
- **2026-06-12 19:44** — COMPLETADO: Implementados arches hardcoded de Odoo 17 para ir.module.module (list, kanban, search, form) ya que las vistas en DB tienen arch_db vacío. Eliminados js_class='module_list' y js_class='module_kanban' que causaban KeyNotFoundError en el registry de views de Owl JS.
- **2026-06-12 19:45** — COMPLETADO: Inyectados campos computados (icon_image, display_name, installed_version, button_immediate_install, button_immediate_upgrade, button_uninstall_wizard, icon_flag) al fields_map de get_views. Sin estos, el frontend Owl lanza TypeError al parsear el schema de campos. Refactorizado web_search_read para separar campos SQL reales de campos computados, inyectando stubs seguros (false) en la respuesta.