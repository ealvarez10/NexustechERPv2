#!/usr/bin/env bash
# NexusTech ERP — Servidor de desarrollo persistente
# Uso: ./scripts/start-erp.sh [start|stop|restart|status|log]

PIDFILE="/tmp/nexustech-erp.pid"
LOGFILE="/tmp/nexustech-erp.log"
BINARY="/home/ealvarez/workspace/NexustechERPv2/target/debug/nexustech-erp"
WORKDIR="/home/ealvarez/workspace/NexustechERPv2"

start() {
  if [ -f "$PIDFILE" ] && kill -0 "$(cat $PIDFILE)" 2>/dev/null; then
    echo "✅ ERP ya está corriendo (PID $(cat $PIDFILE))"
    return
  fi
  cd "$WORKDIR" || exit 1
  echo "" >> "$LOGFILE"
  echo "=== START $(date) ===" >> "$LOGFILE"
  RUST_LOG=error,nexustech_erp=info \
    setsid "$BINARY" >> "$LOGFILE" 2>&1 &
  echo $! > "$PIDFILE"
  sleep 3
  if kill -0 "$(cat $PIDFILE)" 2>/dev/null; then
    echo "🚀 ERP arrancado — PID $(cat $PIDFILE) — Puerto 8090"
    curl -s http://localhost:8090/api/v1/health
    echo ""
  else
    echo "❌ ERP no arrancó. Ver: $LOGFILE"
    tail -20 "$LOGFILE"
  fi
}

stop() {
  if [ -f "$PIDFILE" ]; then
    kill "$(cat $PIDFILE)" 2>/dev/null && echo "⏹ ERP detenido" || echo "Ya estaba parado"
    rm -f "$PIDFILE"
  else
    pkill -f "nexustech-erp" 2>/dev/null && echo "⏹ ERP detenido" || echo "No estaba corriendo"
  fi
}

status() {
  if [ -f "$PIDFILE" ] && kill -0 "$(cat $PIDFILE)" 2>/dev/null; then
    echo "✅ ERP corriendo — PID $(cat $PIDFILE)"
    curl -s http://localhost:8090/api/v1/health
    echo ""
  else
    echo "❌ ERP no está corriendo"
  fi
}

case "${1:-start}" in
  start)   start ;;
  stop)    stop ;;
  restart) stop; sleep 1; start ;;
  status)  status ;;
  log)     tail -f "$LOGFILE" ;;
  *)       echo "Uso: $0 {start|stop|restart|status|log}" ;;
esac
