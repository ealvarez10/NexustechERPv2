# APIs del Proyecto

> ⚠️ NUNCA guardar API keys, tokens o passwords aquí.
> Solo registrar el nombre de la variable de entorno o la ruta del archivo.

<!-- Ejemplo de entrada:
## Nombre de la API
- **Endpoint base:** https://api.ejemplo.com/v1
- **Autenticación:** Bearer token — key en variable NOMBRE_API_KEY en .env
- **Rate limit:** 100 req/s
- **Notas:** Solo disponible en producción
-->

## Facturama PAC (Multiemisor)
- **Endpoint base:** https://api.facturama.mx (producción) / https://apisandbox.facturama.mx (sandbox)
- **Autenticación:** Basic Auth — credenciales en variables FACTURAMA_USER y FACTURAMA_SECRET
- **Rate limit:** No documentado, uso razonable

## SW Sapien PAC
- **Endpoint base:** https://services.sw.com.mx (producción) / https://services.test.sw.com.mx (sandbox)
- **Autenticación:** Bearer token — en variable SW_TOKEN
- **Rate limit:** No documentado

## PostgreSQL NexusTech (local dev)
- **Endpoint base:** postgresql://ealvarez@localhost/demo (dev) | postgresql://ealvarez@localhost/nexus (staging)
- **Autenticación:** Sin contraseña en local. Producción: ver .env en servidor
- **Rate limit:** N/A

## NexusSearch (Meilisearch compatible)
- **Endpoint base:** http://127.0.0.1:7700 (local dev) — en prod: ver .env MEILI_URL
- **Autenticación:** Bearer token — key en variable MEILI_MASTER_KEY (local dev: nexustech_dev_key_2026)
- **Rate limit:** Sin límite explícito, instancia propia
