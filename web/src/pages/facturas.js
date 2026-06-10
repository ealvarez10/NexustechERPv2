import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { fmtMxn, fmtDate, paginationHtml } from '../ui.js'

const ESTADOS = [
  {key:'posted',     lbl:'Publicada',  color:'emerald'},
  {key:'draft',      lbl:'Borrador',   color:'gray'},
  {key:'in_payment', lbl:'En cobro',   color:'violet'},
  {key:'paid',       lbl:'Pagada',     color:'emerald'},
  {key:'cancel',     lbl:'Cancelada',  color:'red'},
]

const CLIENTES = [
  {nombre:'Constructora Azteca SA de CV', rfc:'CAAZ850101AAA'},
  {nombre:'Farmacia San Rafael SA',       rfc:'FASA920605BBB'},
  {nombre:'Hotel Gran Turismo',           rfc:'HOTG780312CCC'},
  {nombre:'Tech Solutions México',        rfc:'TSMX930415DDD'},
]

const MOCK = Array.from({length:30}, (_,i) => {
  const e = ESTADOS[i%5]
  const c = CLIENTES[i%4]
  const sub = Math.round(5000+Math.random()*250000)
  const iva = Math.round(sub*0.16)
  return {
    folio: `NXTE-${String(1000+i).padStart(4,'0')}`,
    uuid: `${Math.random().toString(36).slice(2,8).toUpperCase()}-${Math.random().toString(36).slice(2,6).toUpperCase()}-CFDI`,
    cliente: c.nombre,
    rfc: c.rfc,
    fecha: new Date(Date.now()-i*86400000*1.1).toISOString().split('T')[0],
    subtotal: sub,
    iva: iva,
    total: sub+iva,
    estado: e.key,
    estado_label: e.lbl,
    estado_color: e.color,
  }
})

let page = 1
const PER = 10

export async function renderFacturas() {
  ensureLayout()
  setBreadcrumb([{label:'Dashboard',href:'dashboard'},{label:'Facturación'}])
  page = 1
  renderList()
}

function renderList() {
  const items = MOCK.slice((page-1)*PER, page*PER)
  const hasMore = page*PER < MOCK.length
  const totalCobrar = MOCK.filter(f => f.estado==='posted'||f.estado==='in_payment').reduce((s,f)=>s+f.total,0)
  const totalCobrado = MOCK.filter(f => f.estado==='paid').reduce((s,f)=>s+f.total,0)

  setPage(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">Facturación</h1>
      <p class="page-subtitle">CFDI 4.0 · ${MOCK.length} facturas en el sistema</p>
    </div>
    <div class="page-actions">
      <button class="btn btn-secondary">📥 Exportar XML</button>
      <button class="btn btn-primary">🧾 Nueva Factura</button>
    </div>
  </div>

  <div class="kpi-grid anim-2" style="grid-template-columns:repeat(4,1fr)">
    ${[
      {label:'Emitidas (mes)', val:'127',              icon:'📄', color:'indigo'},
      {label:'Por cobrar',     val:fmtMxn(totalCobrar), icon:'⏳', color:'amber'},
      {label:'Cobradas',       val:fmtMxn(totalCobrado),icon:'✅', color:'emerald'},
      {label:'Canceladas',     val:'3',                icon:'❌', color:'sky'},
    ].map(k => `
    <div class="kpi-card kpi-${k.color}">
      <div class="kpi-label"><span>${k.label}</span><div class="kpi-icon-box">${k.icon}</div></div>
      <div class="kpi-value">${k.val}</div>
    </div>`).join('')}
  </div>

  <div class="data-card anim-3">
    <div class="data-card-header">
      <div>
        <div class="data-card-title">Facturas CFDI 4.0</div>
        <div class="data-card-subtitle">Mostrando ${(page-1)*PER+1}–${Math.min(page*PER,MOCK.length)} de ${MOCK.length}</div>
      </div>
      <div class="filter-group">
        <div class="input-search">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/></svg>
          <input type="text" placeholder="RFC, folio o cliente...">
        </div>
        <select class="form-control" style="width:auto;padding:6px 10px;font-size:13px">
          <option>Todos los estados</option>
          ${ESTADOS.map(e=>`<option>${e.lbl}</option>`).join('')}
        </select>
      </div>
    </div>
    <table class="data-table">
      <thead><tr>
        <th>Folio</th><th>UUID CFDI</th><th>Cliente</th><th>RFC</th><th>Fecha</th>
        <th>Subtotal</th><th>IVA</th><th>Total</th><th>Estado</th><th></th>
      </tr></thead>
      <tbody>
        ${items.map(f => `
        <tr>
          <td class="td-mono td-primary">${f.folio}</td>
          <td class="td-mono" style="font-size:10.5px;color:var(--text-400);max-width:100px;overflow:hidden;text-overflow:ellipsis">${f.uuid}</td>
          <td class="td-primary" style="max-width:140px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">${f.cliente}</td>
          <td class="td-mono">${f.rfc}</td>
          <td>${fmtDate(f.fecha)}</td>
          <td>${fmtMxn(f.subtotal)}</td>
          <td>${fmtMxn(f.iva)}</td>
          <td class="td-amount">${fmtMxn(f.total)}</td>
          <td><span class="badge badge-${f.estado_color} badge-dot">${f.estado_label}</span></td>
          <td>
            <div style="display:flex;gap:4px">
              <button class="btn btn-secondary btn-sm">📄 PDF</button>
              <button class="btn btn-secondary btn-sm">📦 XML</button>
            </div>
          </td>
        </tr>`).join('')}
      </tbody>
    </table>
    ${paginationHtml(page, hasMore, (p) => { page=p; renderList() })}
  </div>`)
}
