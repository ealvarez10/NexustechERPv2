# Problemas Resueltos

> Catálogo de errores encontrados y sus soluciones.
> El agente consulta este archivo ANTES de intentar resolver un problema.

<!-- Ejemplo de entrada:
## Error: EACCES al instalar paquetes npm
**Contexto:** Windows PowerShell sin privilegios de administrador
**Solución:** Ejecutar PowerShell como admin, o usar `npm config set prefix`
**Fecha:** 2026-05-29
-->

## Error: GitHub push 403 — token sin permisos para repo nuevo
**Contexto:** El token del repo nexustecherp no tenía scope 'repo' para crear/escribir NexustechERPv2
**Solución:** El usuario actualizó los permisos del token en GitHub Settings. El mismo token github_pat_11ALBI6WY0... ahora funciona para ambos repos. Guardar en variable de entorno GITHUB_TOKEN.
**Fecha:** 2026-06-09

## Error: nexustech --stop-after-init falla con ModuleNotFoundError: odoo
**Contexto:** Al llamar al binario nexustech-bin directamente sin el PYTHONPATH correcto
**Solución:** Usar bash start.sh (en /workspace/nexustech erp/) en lugar del binario directo. El start.sh configura PYTHONPATH al core correcto y activa el venv. Alternativa: export PYTHONPATH=/home/ealvarez/workspace/nexustech erp/core:$PYTHONPATH antes de llamar nexustech.
**Fecha:** 2026-06-09

## Error: Vista Apps muestra columnas genéricas (Id, Create Uid, Create Date, Write Date, Write Uid, Website)
**Contexto:** El handler get_views en app/src/handlers/web.rs intentaba leer la columna 'arch' de ir_ui_view, pero la tabla no tiene esa columna. La columna real es 'arch_db' (JSONB multilingual: {"en_US": "<list>...</list>"}).
**Solución:** 1) Cambiar todas las queries de ir_ui_view para usar arch_db en lugar de arch. 2) Extraer el XML del JSONB priorizando es_MX > en_US > primer valor disponible. 3) Si el arch resultante tiene menos de 20 chars (vacío), usar los arches hardcoded de Odoo 17 específicos para el modelo. En web.rs: buscar el bloque 'get_views | load_views' y verificar que las queries usen 'arch_db'.
**Fecha:** 2026-06-12

## Error: OwlError: KeyNotFoundError: Cannot find key 'module_list' (o 'module_kanban') in the 'views' registry
**Contexto:** El arch hardcoded para ir.module.module incluía js_class='module_list' y js_class='module_kanban'. Estos son componentes JavaScript personalizados que no están compilados en el bundle web.assets_web.min.js de esta instalación, por lo que Owl no puede resolverlos.
**Solución:** Remover todos los atributos js_class de los arches hardcoded de ir.module.module. Usar los renderers genéricos estándar (list y kanban sin js_class). Las vistas funcionan igual sin el js_class — solo pierden el widget especializado de módulos (que requiere el módulo web_module_installer, ausente aquí).
**Fecha:** 2026-06-12

## Error: OwlError: TypeError en RelationalModel — campos computados retornan null en vez de false
**Contexto:** El frontend Odoo/Owl espera que los campos que no existen en la BD (icon_image, display_name, installed_version) devuelvan false (JSON boolean), no null ni que estén ausentes de la respuesta. Cuando el backend omitía estos campos del JSON, el RelationalModel de Owl fallaba con Cannot read properties of null.
**Solución:** 1) En get_views: inyectar los campos computados al fields_map con sus tipos correctos (icon_image=binary, display_name=char, installed_version=char, botones=char). 2) En web_search_read: después de construir el record con campos SQL, iterar los campos computados pedidos y añadirlos con false si no existen. Los campos de tipo binary (icon_image) deben ir como false (no como string vacía ni null). Ver función dispatch_orm_rust, brazo 'web_search_read'.
**Fecha:** 2026-06-12
