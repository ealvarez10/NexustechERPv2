#!/bin/bash
# start-backend.sh — Levanta el servidor NexusTech ERP v2
# Uso: ./start-backend.sh [start|stop|restart|status|logs]

BINARY="/home/ealvarez/workspace/NexustechERPv2/target/debug/nexustech-erp"
PIDFILE="/tmp/nexus-erp.pid"
LOGFILE="/tmp/nexus-backend.log"
ENVFILE="/home/ealvarez/workspace/NexustechERPv2/.env"
PORT=8090

# Cargar variables de entorno desde .env
if [ -f "$ENVFILE" ]; then
  set -a
  source "$ENVFILE"
  set +a
fi

case "${1:-start}" in
  start)
    if [ -f "$PIDFILE" ] && kill -0 "$(cat $PIDFILE)" 2>/dev/null; then
      echo "✅ Backend ya está corriendo (PID $(cat $PIDFILE))"
    else
      nohup "$BINARY" > "$LOGFILE" 2>&1 &
      echo $! > "$PIDFILE"
      sleep 2
      if curl -s "http://localhost:$PORT/health" > /dev/null 2>&1; then
        echo "🚀 Backend iniciado en http://localhost:$PORT (PID $(cat $PIDFILE))"
      else
        echo "⚠️  Servidor no respondió — revisa: tail -f $LOGFILE"
      fi
    fi
    ;;
  stop)
    if [ -f "$PIDFILE" ]; then
      kill "$(cat $PIDFILE)" 2>/dev/null && rm "$PIDFILE"
      echo "🛑 Backend detenido"
    else
      pkill -f nexustech-erp 2>/dev/null && echo "🛑 Backend detenido"
    fi
    ;;
  restart)
    $0 stop; sleep 1; $0 start
    ;;
  status)
    if curl -s "http://localhost:$PORT/health" | python3 -m json.tool 2>/dev/null; then
      echo "✅ Backend ACTIVO en puerto $PORT"
    else
      echo "❌ Backend NO responde"
    fi
    ;;
  logs)
    tail -f "$LOGFILE"
    ;;
  *)
    echo "Uso: $0 [start|stop|restart|status|logs]"
    ;;
esac
