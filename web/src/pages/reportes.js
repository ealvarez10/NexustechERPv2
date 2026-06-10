import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { fmtMxn, fmtDate, toast } from '../ui.js'
import { api } from '../api.js'

export async function renderReportes() {
  ensureLayout()
  setBreadcrumb([{label:'Dashboard',href:'dashboard'},{label:'Reportes'}])
  await loadReportes()
}

async function loadReportes() {
  setPage(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">📈 Reportes</h1>
      <p class="page-subtitle">Análisis ejecutivos e inteligencia de negocio</p>
    </div>
    <div class="page-actions">
      <button class="btn btn-secondary" onclick="window._exportReporte()">📥 Exportar</button>
    </div>
  </div>

  <!-- Cards de reportes disponibles -->
  <div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(280px,1fr));gap:16px;margin-bottom:24px" class="anim-2">
    ${[
      { icon:'💰', titulo:'Reporte de Ventas', sub:'Órdenes, tendencias y proyecciones mensuales', color:'indigo', key:'ventas' },
      { icon:'🧾', titulo:'Reporte de Facturación', sub:'CFDIs emitidos, cancelados y saldo pendiente', color:'emerald', key:'facturas' },
      { icon:'🏭', titulo:'Reporte de Inventario', sub:'Stock actual, valor, y alertas de reorden', color:'violet', key:'inventario' },
      { icon:'🛒', titulo:'Reporte de Compras', sub:'Órdenes de compra, proveedores y gasto', color:'amber', key:'compras' },
      { icon:'👥', titulo:'Reporte de Clientes', sub:'Top clientes, retención y geografía', color:'sky', key:'clientes' },
      { icon:'👔', titulo:'Reporte de Nómina', sub:'Plantilla activa, costo mensual y IMSS', color:'rose', key:'nomina' },
    ].map(r => `
    <div class="data-card" style="padding:20px;cursor:pointer;transition:transform 0.15s,box-shadow 0.15s"
      onmouseover="this.style.transform='translateY(-3px)';this.style.boxShadow='0 8px 24px rgba(0,0,0,.12)'"
      onmouseout="this.style.transform='';this.style.boxShadow=''"
      onclick="window._verReporte('${r.key}')">
      <div style="width:46px;height:46px;border-radius:12px;background:var(--${r.color==='indigo'?'primary':r.color}-light,var(--primary-light));
        display:flex;align-items:center;justify-content:center;font-size:22px;margin-bottom:12px">
        ${r.icon}
      </div>
      <div style="font-size:14px;font-weight:700;color:var(--text-800);margin-bottom:4px">${r.titulo}</div>
      <div style="font-size:12px;color:var(--text-400)">${r.sub}</div>
      <div style="margin-top:12px">
        <span style="font-size:11px;color:var(--primary);font-weight:600">Ver reporte →</span>
      </div>
    </div>`).join('')}
  </div>

  <!-- Reporte principal — Resumen ejecutivo -->
  <div class="data-card anim-3">
    <div class="data-card-header">
      <div class="data-card-title">📊 Resumen Ejecutivo</div>
      <div style="font-size:12px;color:var(--text-400)" id="rep-fecha"></div>
    </div>
    <div id="rep-contenido">
      <div style="display:grid;grid-template-columns:repeat(3,1fr);gap:16px;padding:8px 0" id="rep-kpis">
        ${[1,2,3].map(() => `<div class="skeleton" style="height:80px;border-radius:12px"></div>`).join('')}
      </div>
    </div>
  </div>`)

  window._verReporte = (key) => {
    toast('Reporte seleccionado', `Generando reporte de ${key}…`, 'info')
    loadDetalleReporte(key)
  }

  window._exportReporte = () => {
    toast('Exportar', 'Función de exportación CSV/PDF — próximamente', 'info')
  }

  // Cargar resumen ejecutivo
  await loadResumenEjecutivo()
}

async function loadResumenEjecutivo() {
  const fechaEl = document.getElementById('rep-fecha')
  if (fechaEl) fechaEl.textContent = new Date().toLocaleDateString('es-MX', { day:'2-digit', month:'long', year:'numeric' })

  try {
    const [ventasR, factR, stockR, comprasR] = await Promise.allSettled([
      api.ventaKpis(),
      api.factKpis(),
      api.stockKpis(),
      api.comprasKpis(),
    ])

    const v = ventasR.value?.data || {}
    const f = factR.value?.data || {}
    const s = stockR.value?.data || {}
    const c = comprasR.value?.data || {}

    const kpisEl = document.getElementById('rep-kpis')
    if (kpisEl) {
      kpisEl.innerHTML = `
      ${[
        { label: 'Ventas confirmadas', val: v.ordenes_confirmadas ?? 0, tipo:'num', desc: `$${parseFloat(v.total_facturado||0).toLocaleString('es-MX',{minimumFractionDigits:2})} este mes` },
        { label: 'Facturación total',  val: fmtMxn(parseFloat(f.monto_total||0)), tipo:'txt', desc: `${f.total_facturas ?? 0} comprobantes emitidos` },
        { label: 'Valor inventario',   val: fmtMxn(parseFloat(s.valor_inventario||0)), tipo:'txt', desc: `${s.alertas_stock_bajo ?? 0} alertas de stock bajo` },
      ].map(k => `
      <div style="padding:16px;background:var(--bg);border-radius:12px;border:1px solid var(--border)">
        <div style="font-size:11px;color:var(--text-400);font-weight:700;text-transform:uppercase;margin-bottom:6px">${k.label}</div>
        <div style="font-size:24px;font-weight:800;color:var(--text-900);margin-bottom:4px">
          ${k.tipo === 'num' ? Number(k.val).toLocaleString('es-MX') : k.val}
        </div>
        <div style="font-size:11px;color:var(--text-500)">${k.desc}</div>
      </div>`).join('')}

      <div style="grid-column:1/-1;margin-top:8px">
        <div style="font-size:12px;font-weight:700;color:var(--text-600);margin-bottom:10px">COMPRAS</div>
        <div style="display:grid;grid-template-columns:repeat(3,1fr);gap:12px">
          ${[
            { label: 'Total OC',     val: c.total      ?? 0 },
            { label: 'Confirmadas',  val: c.confirmadas ?? 0 },
            { label: 'Monto compras',val: fmtMxn(parseFloat(c.monto_total||0)) },
          ].map(k => `
          <div style="padding:12px;background:var(--bg);border-radius:10px;border:1px solid var(--border)">
            <div style="font-size:10px;color:var(--text-400);font-weight:700;text-transform:uppercase;margin-bottom:4px">${k.label}</div>
            <div style="font-size:18px;font-weight:800;color:var(--text-900)">${k.val}</div>
          </div>`).join('')}
        </div>
      </div>`
    }

  } catch (err) {
    console.error(err)
  }
}

async function loadDetalleReporte(key) {
  const kpisEl = document.getElementById('rep-kpis')
  const header = document.querySelector('.data-card-title')
  if (header) {
    const TITULOS = {ventas:'💰 Reporte de Ventas',facturas:'🧾 Facturación',inventario:'🏭 Inventario',compras:'🛒 Compras',clientes:'👥 Clientes',nomina:'👔 Nómina'}
    header.textContent = TITULOS[key] || 'Reporte'
  }
  if (kpisEl) kpisEl.innerHTML = `<div class="skeleton" style="height:120px;border-radius:12px;grid-column:1/-1"></div>`
  await loadResumenEjecutivo()
}
