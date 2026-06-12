import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { api } from '../api.js'
import { toast, fmtMxn } from '../ui.js'

let _periodo = 'year', _ventas = []

export async function renderReportesVentas(params = {}) {
  ensureLayout()
  setBreadcrumb([{ label:'Ventas', href:'#ventas' }, { label:'Reportes' }])
  setPage(`<div class="nx-module-page">
    <div class="o-cp">
      <div class="o-cp-left">
        <div style="display:flex;gap:4px">
          <button id="rp-mes" class="o-btn-filter ${_periodo==='month'?'active':''}" onclick="window._rperiodo('month')">Mes actual</button>
          <button id="rp-tri" class="o-btn-filter ${_periodo==='quarter'?'active':''}" onclick="window._rperiodo('quarter')">Trimestre</button>
          <button id="rp-ano" class="o-btn-filter ${_periodo==='year'?'active':''}" onclick="window._rperiodo('year')">Este año</button>
        </div>
      </div>
      <div class="o-cp-right">
        <button class="o-btn-secondary o-btn-sm" onclick="window.print()">Exportar</button>
      </div>
    </div>
    <div id="rv-content" style="padding:24px">
      <div style="text-align:center;padding:60px;color:var(--text-400)">⏳ Cargando reportes...</div>
    </div>
  </div>`)

  window._rperiodo = (p) => { _periodo = p; _renderReportes() }

  let kpis = {}
  try {
    const res = await api.get('/ventas/kpis')
    kpis = res?.data || res || {}
  } catch {}
  try {
    const res = await api.get('/ventas?limite=200')
    _ventas = res?.data || []
  } catch { _ventas = [] }

  window._rvKpis = kpis
  _renderReportes()
}

function _renderReportes() {
  const c = document.getElementById('rv-content')
  if (!c) return
  const kpis = window._rvKpis || {}

  // Filtrar ventas por periodo
  const now = new Date()
  const ventas = _ventas.filter(v => {
    if (!v.date_order) return true
    const d = new Date(v.date_order)
    if (_periodo === 'month') return d.getMonth() === now.getMonth() && d.getFullYear() === now.getFullYear()
    if (_periodo === 'quarter') {
      const q = Math.floor(now.getMonth()/3)
      return Math.floor(d.getMonth()/3) === q && d.getFullYear() === now.getFullYear()
    }
    return d.getFullYear() === now.getFullYear()
  })

  // Agrupar por mes
  const byMonth = {}
  ventas.forEach(v => {
    if (!v.date_order) return
    const k = v.date_order.slice(0,7)
    if (!byMonth[k]) byMonth[k] = { mes: k, count: 0, total: 0 }
    byMonth[k].count++
    byMonth[k].total += parseFloat(v.amount_total || 0)
  })
  const meses = Object.values(byMonth).sort((a,b) => a.mes.localeCompare(b.mes))
  const maxTotal = Math.max(...meses.map(m => m.total), 1)

  const totalVentas = kpis.total_ventas ?? ventas.reduce((s,v)=>s+parseFloat(v.amount_total||0),0)
  const pedidosConf = kpis.pedidos_confirmados ?? ventas.filter(v=>v.state==='sale'||v.state==='done').length
  const ticket = pedidosConf > 0 ? (totalVentas / pedidosConf) : 0
  const cotizaciones = kpis.cotizaciones_enviadas ?? ventas.filter(v=>v.state==='sent').length

  const meses_es = ['Ene','Feb','Mar','Abr','May','Jun','Jul','Ago','Sep','Oct','Nov','Dic']
  const fmt_mes = (k) => { const [y,m] = k.split('-'); return `${meses_es[parseInt(m)-1]} ${y}` }

  c.innerHTML = `
    <!-- KPIs -->
    <div style="display:grid;grid-template-columns:repeat(4,1fr);gap:16px;margin-bottom:28px">
      <div style="background:var(--bg-card);border:1px solid var(--border);border-radius:12px;padding:20px;border-top:3px solid var(--primary)">
        <div style="font-size:11px;font-weight:700;text-transform:uppercase;color:var(--text-400);margin-bottom:6px">Total Ventas</div>
        <div style="font-size:24px;font-weight:800;color:var(--primary)">${fmtMxn(totalVentas)}</div>
      </div>
      <div style="background:var(--bg-card);border:1px solid var(--border);border-radius:12px;padding:20px;border-top:3px solid #059669">
        <div style="font-size:11px;font-weight:700;text-transform:uppercase;color:var(--text-400);margin-bottom:6px">Pedidos Confirmados</div>
        <div style="font-size:24px;font-weight:800;color:#059669">${pedidosConf}</div>
      </div>
      <div style="background:var(--bg-card);border:1px solid var(--border);border-radius:12px;padding:20px;border-top:3px solid #7C3AED">
        <div style="font-size:11px;font-weight:700;text-transform:uppercase;color:var(--text-400);margin-bottom:6px">Ticket Promedio</div>
        <div style="font-size:24px;font-weight:800;color:#7C3AED">${fmtMxn(ticket)}</div>
      </div>
      <div style="background:var(--bg-card);border:1px solid var(--border);border-radius:12px;padding:20px;border-top:3px solid #F59E0B">
        <div style="font-size:11px;font-weight:700;text-transform:uppercase;color:var(--text-400);margin-bottom:6px">Cotizaciones</div>
        <div style="font-size:24px;font-weight:800;color:#F59E0B">${cotizaciones}</div>
      </div>
    </div>

    <!-- Gráfica barras CSS -->
    <div style="background:var(--bg-card);border:1px solid var(--border);border-radius:12px;padding:24px;margin-bottom:24px">
      <div style="font-size:14px;font-weight:700;margin-bottom:16px">Ventas por Mes</div>
      ${meses.length === 0 ? '<div style="text-align:center;padding:40px;color:var(--text-400)">Sin datos en el periodo seleccionado</div>' : `
      <div style="display:flex;align-items:flex-end;gap:8px;height:180px;border-bottom:2px solid var(--border);padding-bottom:8px">
        ${meses.map(m => `
          <div style="flex:1;display:flex;flex-direction:column;align-items:center;gap:4px">
            <div style="font-size:10px;color:var(--text-400);white-space:nowrap">${fmtMxn(m.total)}</div>
            <div style="width:100%;background:var(--primary);border-radius:4px 4px 0 0;min-height:4px;height:${Math.max(4,Math.round((m.total/maxTotal)*140))}px;transition:height .3s"></div>
          </div>`).join('')}
      </div>
      <div style="display:flex;gap:8px;padding-top:8px">
        ${meses.map(m => `<div style="flex:1;text-align:center;font-size:10px;color:var(--text-400)">${fmt_mes(m.mes)}</div>`).join('')}
      </div>`}
    </div>

    <!-- Tabla pivot -->
    <div style="background:var(--bg-card);border:1px solid var(--border);border-radius:12px;overflow:hidden">
      <div style="padding:16px 20px;border-bottom:1px solid var(--border);font-size:14px;font-weight:700">Análisis por Mes</div>
      <table class="o-list-table">
        <thead><tr><th>Mes</th><th style="text-align:right">Órdenes</th><th style="text-align:right">Total MXN</th><th style="text-align:right">Ticket Promedio</th></tr></thead>
        <tbody>
          ${meses.length === 0 ? '<tr><td colspan="4" style="text-align:center;padding:20px;color:var(--text-400)">Sin datos</td></tr>' :
            meses.map(m => `<tr class="o-list-row"><td>${fmt_mes(m.mes)}</td><td style="text-align:right">${m.count}</td><td style="text-align:right;font-weight:700;color:var(--primary)">${fmtMxn(m.total)}</td><td style="text-align:right">${fmtMxn(m.count>0?m.total/m.count:0)}</td></tr>`).join('')}
          <tr style="background:var(--primary-light);font-weight:700"><td>TOTAL</td><td style="text-align:right">${ventas.length}</td><td style="text-align:right;color:var(--primary)">${fmtMxn(totalVentas)}</td><td style="text-align:right">${fmtMxn(ticket)}</td></tr>
        </tbody>
      </table>
    </div>`
}
