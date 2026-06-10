import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { fmt, fmtMxn, paginationHtml } from '../ui.js'

const MOCK_PARTNERS = Array.from({length:20}, (_,i) => ({
  id: 1000+i,
  nombre: ['Constructora Azteca SA','Farmacia San Rafael','Hotel Gran Turismo','Tech Solutions MX','Restaurantes La Villa','Distribuidora Norte','Mueblería Central','Servicios Integral'][i%8],
  rfc: `RFC${String(i).padStart(6,'0')}MEX`,
  ciudad: ['Ciudad de México','Monterrey','Guadalajara','Puebla','Querétaro'][i%5],
  ventas: Math.round(50000+Math.random()*2000000),
  facturas: Math.round(3+Math.random()*80),
  saldo: Math.round(Math.random()*500000),
  tipo: i%3===0?'proveedor':'cliente',
}))

let page = 1, tipo = ''
const PER = 10

export async function renderPartners() {
  ensureLayout()
  setBreadcrumb([{label:'Dashboard',href:'dashboard'},{label:'Clientes / Proveedores'}])
  page = 1; tipo = ''
  renderList()
}

function renderList() {
  const filtered = tipo ? MOCK_PARTNERS.filter(p=>p.tipo===tipo) : MOCK_PARTNERS
  const items = filtered.slice((page-1)*PER, page*PER)
  const hasMore = page*PER < filtered.length

  setPage(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">Clientes & Proveedores</h1>
      <p class="page-subtitle">${MOCK_PARTNERS.length} partners registrados en el sistema</p>
    </div>
    <div class="page-actions">
      <button class="btn btn-secondary">📥 Exportar</button>
      <button class="btn btn-primary">+ Nuevo Partner</button>
    </div>
  </div>

  <div class="kpi-grid anim-2" style="grid-template-columns:repeat(4,1fr)">
    ${[
      {label:'Total Partners',  val:fmt(MOCK_PARTNERS.length), icon:'👥', color:'indigo'},
      {label:'Clientes',        val:fmt(MOCK_PARTNERS.filter(p=>p.tipo==='cliente').length), icon:'🏢', color:'emerald'},
      {label:'Proveedores',     val:fmt(MOCK_PARTNERS.filter(p=>p.tipo==='proveedor').length), icon:'🚚', color:'amber'},
      {label:'Saldo Por Cobrar',val:fmtMxn(MOCK_PARTNERS.reduce((s,p)=>s+p.saldo,0)), icon:'💳', color:'violet'},
    ].map(k => `
    <div class="kpi-card kpi-${k.color}">
      <div class="kpi-label"><span>${k.label}</span><div class="kpi-icon-box">${k.icon}</div></div>
      <div class="kpi-value">${k.val}</div>
    </div>`).join('')}
  </div>

  <div class="data-card anim-3">
    <div class="data-card-header">
      <div>
        <div class="data-card-title">Directorio de Partners</div>
      </div>
      <div class="filter-group">
        <div class="input-search">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/></svg>
          <input type="text" placeholder="Nombre o RFC...">
        </div>
        <div style="display:flex;gap:4px">
          <button class="btn ${tipo===''?'btn-primary':'btn-secondary'} btn-sm" onclick="window.__setTipo('')">Todos</button>
          <button class="btn ${tipo==='cliente'?'btn-primary':'btn-secondary'} btn-sm" onclick="window.__setTipo('cliente')">Clientes</button>
          <button class="btn ${tipo==='proveedor'?'btn-primary':'btn-secondary'} btn-sm" onclick="window.__setTipo('proveedor')">Proveedores</button>
        </div>
      </div>
    </div>
    <table class="data-table">
      <thead><tr>
        <th>#</th><th>Nombre/Razón Social</th><th>RFC</th><th>Ciudad</th><th>Facturas</th><th>Ventas Totales</th><th>Saldo</th><th>Tipo</th><th></th>
      </tr></thead>
      <tbody>
        ${items.map(p => `
        <tr>
          <td class="td-mono">${p.id}</td>
          <td class="td-primary">${p.nombre}</td>
          <td class="td-mono">${p.rfc}</td>
          <td>${p.ciudad}</td>
          <td style="text-align:center">${p.facturas}</td>
          <td class="td-amount">${fmtMxn(p.ventas)}</td>
          <td class="${p.saldo>0?'td-amount':''}" style="${p.saldo>0?'color:var(--warning)':'color:var(--text-400)'}">${p.saldo>0?fmtMxn(p.saldo):'—'}</td>
          <td><span class="badge badge-${p.tipo==='cliente'?'indigo':'amber'}">${p.tipo==='cliente'?'Cliente':'Proveedor'}</span></td>
          <td>
            <div style="display:flex;gap:4px">
              <button class="btn btn-secondary btn-sm">👁 Ver</button>
              <button class="btn btn-secondary btn-sm">✏️</button>
            </div>
          </td>
        </tr>`).join('')}
      </tbody>
    </table>
    ${paginationHtml(page, hasMore, (p) => { page=p; renderList() })}
  </div>`)

  window.__setTipo = (t) => { tipo=t; page=1; renderList() }
}
