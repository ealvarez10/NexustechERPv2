#!/bin/bash
# start_backend.sh — Levanta NexusTech ERP Backend
export DATABASE_URL=postgres:///nexustech_db
export RUST_LOG=warn
export JWT_SECRET=nexustech_erp_v2_jwt_secret_2026_dev_only_change_in_production
export SERVER_PORT=8090
export ENVIRONMENT=development
export DATABASE_POOL_MAX=5

cd /home/ealvarez/workspace/NexustechERPv2
echo "[$(date)] Iniciando NexusTech ERP Backend..." >> /tmp/backend_persistent.log
exec ./target/release/nexustech-erp >> /tmp/backend_persistent.log 2>&1
