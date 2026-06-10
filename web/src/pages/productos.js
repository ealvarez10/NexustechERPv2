import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { fmtMxn, fmt, paginationHtml } from '../ui.js'

const CATS = ['Electrónica','Cómputo','Accesorios','Mobiliario','Consumibles','Redes','Telefonía']
const BASES = ['Monitor 27" 4K','Laptop Core i7','Teclado Mecánico RGB','Silla Ergonómica Pro','Papel A4 Resma 500h','Switch 24 Puertos PoE','Smartphone 5G']
const PREFIJOS = ['EL','PC','AC','MB','CO','RE','TL']

const MOCK = Array.from({length:35}, (_,i) => {
  const idx = i%7
  const stock = Math.round(Math.random()*180)
  const min = Math.round(5+Math.random()*25)
  return {
    sku: `${PREFIJOS[idx]}-${String(1000+i).padStart(4,'0')}`,
    nombre: `${BASES[idx]} ${i > 6 ? `(Ref ${i+1})` : ''}`.trim(),
    categoria: CATS[idx],
    precio: Math.round(400+Math.random()*18000),
    stock,
    minimo: min,
    activo: i%8 !== 0,
    proveedor: ['Dist. Nacional SA','Importadora MX','Tech Supply SA'][i%3],
  }
})

let page = 1
const PER = 12
let query = ''

export async function renderProductos() {
  ensureLayout()
  setBreadcrumb([{label:'Dashboard',href:'dashboard'},{label:'Productos'}])
  page = 1
  renderList()
}

function renderList() {
  const filtered = query
    ? MOCK.filter(p => p.nombre.toLowerCase().includes(query.toLowerCase()) || p.sku.toLowerCase().includes(query.toLowerCase()))
    : MOCK
  const items = filtered.slice((page-1)*PER, page*PER)
  const hasMore = page*PER < filtered.length
  const activos = MOCK.filter(p=>p.activo).length
  const bajosMin = MOCK.filter(p=>p.stock<=p.minimo).length

  setPage(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">Productos</h1>
      <p class="page-subtitle">${MOCK.length} productos en catálogo · ${activos} activos</p>
    </div>
    <div class="page-actions">
      <button class="btn btn-secondary">📥 Importar CSV</button>
      <button class="btn btn-primary">+ Nuevo Producto</button>
    </div>
  </div>

  <div class="kpi-grid anim-2" style="grid-template-columns:repeat(4,1fr)">
    ${[
      {label:'Total Productos', val:fmt(MOCK.length), icon:'📦', color:'indigo'},
      {label:'Activos',         val:fmt(activos),     icon:'✅', color:'emerald'},
      {label:'Bajo Mínimo',     val:fmt(bajosMin),    icon:'⚠️', color:'amber'},
      {label:'Categorías',      val:fmt(CATS.length), icon:'🏷️', color:'violet'},
    ].map(k => `
    <div class="kpi-card kpi-${k.color}">
      <div class="kpi-label"><span>${k.label}</span><div class="kpi-icon-box">${k.icon}</div></div>
      <div class="kpi-value">${k.val}</div>
    </div>`).join('')}
  </div>

  <div class="data-card anim-3">
    <div class="data-card-header">
      <div>
        <div class="data-card-title">Catálogo de Productos</div>
        <div class="data-card-subtitle">Mostrando ${(page-1)*PER+1}–${Math.min(page*PER,filtered.length)} de ${filtered.length}</div>
      </div>
      <div class="filter-group">
        <div class="input-search">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/></svg>
          <input type="text" id="prod-search" placeholder="SKU o nombre..." value="${query}">
        </div>
        <select class="form-control" style="width:auto;padding:6px 10px;font-size:13px" id="cat-filter">
          <option value="">Todas las categorías</option>
          ${CATS.map(c=>`<option>${c}</option>`).join('')}
        </select>
      </div>
    </div>
    <table class="data-table">
      <thead><tr>
        <th>SKU</th><th>Producto</th><th>Categoría</th><th>Precio</th><th>Stock</th><th>Mínimo</th><th>Proveedor</th><th>Estado</th><th></th>
      </tr></thead>
      <tbody>
        ${items.map(p => `
        <tr>
          <td class="td-mono">${p.sku}</td>
          <td class="td-primary">${p.nombre}</td>
          <td><span class="badge badge-indigo">${p.categoria}</span></td>
          <td class="td-amount">${fmtMxn(p.precio)}</td>
          <td>
            <span class="badge badge-${p.stock<=p.minimo?'red':p.stock<=p.minimo*2?'amber':'emerald'}">
              ${fmt(p.stock)}
            </span>
          </td>
          <td style="color:var(--text-400)">${fmt(p.minimo)}</td>
          <td style="font-size:12px;color:var(--text-500)">${p.proveedor}</td>
          <td><span class="badge badge-${p.activo?'emerald':'gray'}">${p.activo?'Activo':'Inactivo'}</span></td>
          <td>
            <div style="display:flex;gap:4px">
              <button class="btn btn-secondary btn-sm">✏️ Editar</button>
            </div>
          </td>
        </tr>`).join('')}
      </tbody>
    </table>
    ${paginationHtml(page, hasMore, (p) => { page=p; renderList() })}
  </div>`)

  // Search handler
  const searchEl = document.getElementById('prod-search')
  if (searchEl) {
    searchEl.addEventListener('input', e => {
      query = e.target.value
      page = 1
      renderList()
    })
  }
}
