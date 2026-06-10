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
