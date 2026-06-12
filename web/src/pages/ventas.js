/**
 * Módulo Ventas — Réplica Odoo Sales
 * Flujo: Cotización (draft/sent) → Pedido de Venta (sale) → A Facturar → Realizado (done)
 * Vistas: Lista · Kanban · Formulario completo editable (idéntico a Odoo)
 */
import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { api } from '../api.js'
import { toast, fmtMxn, fmtDate, skeletonTable, paginationHtml } from '../ui.js'

// ─── Estado global ─────────────────────────────────────────────────────────
let _view    = 'list'
let _records = []
let _total   = 0
let _page    = 1
let _filter  = ''
let _search  = ''
let _selIDs  = new Set()
// Estado formulario
let _currentOrder  = null
let _currentLineas = []
let _editMode      = false
// Inline product picker
let _inlinePickerTimer = null

// ─── Configuración del módulo (localStorage: nexus_config_ventas) ──────────
const CFG_DEFAULTS = {
  firma_online: false,
  pago_online: false,
  descuentos: true,
  margenes: false,
  advertencias: false,
  bloquear_confirmado: true,
  validez_cotizacion: 30,
  plantillas_presupuesto: false,
  compra_online: false,
  notas_cierre: false,
  politica_facturacion: 'cantidad_pedida',
  costos_envio: false,
  fecha_entrega: false,
  aviso_stock: false,
  listas_precios: false,
  descuento_precio: false,
  variantes: false,
  unidades_medida: false,
  empaquetado: false,
  terminos: '',
}

function getCfg() {
  try {
    return { ...CFG_DEFAULTS, ...JSON.parse(localStorage.getItem('nexus_config_ventas') || '{}') }
  } catch (_) {
    return { ...CFG_DEFAULTS }
  }
}

// ─── Listas de precios (compartidas con la página Precios vía localStorage) ──
const PRICELIST_DEFAULTS = [
  { id: 1, name: 'Tarifa General',   currency: 'MXN', type: 'Porcentaje', active: true,  discount: 0 },
  { id: 2, name: 'Distribuidores',   currency: 'MXN', type: 'Porcentaje', active: true,  discount: 10 },
  { id: 3, name: 'Exportación USD',  currency: 'USD', type: 'Fijo',       active: false, discount: 0 },
]

function getPricelists() {
  try {
    const ls = JSON.parse(localStorage.getItem('nexus_pricelists') || 'null')
    if (Array.isArray(ls) && ls.length) return ls
  } catch (_) {}
  return PRICELIST_DEFAULTS
}

// ─── Extras por orden (firma, pagos, lista de precios, UdM/empaque por línea) ─
// El backend no persiste estos campos, así que viven en localStorage por orden.
function getExtras(orderId) {
  try { return JSON.parse(localStorage.getItem(`nexus_venta_extras_${orderId}`) || '{}') } catch (_) { return {} }
}

function setExtras(orderId, patch) {
  const next = { ...getExtras(orderId), ...patch }
  localStorage.setItem(`nexus_venta_extras_${orderId}`, JSON.stringify(next))
  return next
}

// ─── Plantillas de presupuesto (config: plantillas_presupuesto) ──────────────
const QUOTE_TEMPLATES = {
  std: { label: 'Cotización estándar', lineas: [
    { display_type: 'line_section', name: 'Productos', product_uom_qty: 0, price_unit: 0, discount: 0 },
  ]},
  serv: { label: 'Servicios profesionales', lineas: [
    { display_type: 'line_section', name: 'Servicios profesionales', product_uom_qty: 0, price_unit: 0, discount: 0 },
    { name: 'Implementación y configuración', product_uom_qty: 1, price_unit: 15000, discount: 0 },
    { name: 'Capacitación (por sesión)',       product_uom_qty: 2, price_unit: 2500,  discount: 0 },
    { name: 'Soporte mensual',                 product_uom_qty: 1, price_unit: 3500,  discount: 0 },
  ]},
}

// ─── Métodos de envío (config: costos_envio) ─────────────────────────────────
const SHIPPING_METHODS = [
  { key: 'estandar', label: 'Entrega estándar (3-5 días)', costo: 99 },
  { key: 'express',  label: 'Entrega express (24 hrs)',    costo: 199 },
  { key: 'recoger',  label: 'Recoger en tienda',           costo: 0 },
]

// Barra de botones bajo las líneas de pedido (única fuente — incluye envío si está activo)
function _btnBarLineas(ordId) {
  const cfg = getCfg()
  return `
  <div id="btn-bar-lineas" style="display:flex;gap:8px;padding:10px 20px;border-top:1px solid var(--border);background:var(--bg-app)">
    <button class="o-btn-secondary o-btn-sm" onclick="window._agregarProductoInline(${ordId})" id="btn-add-product" style="gap:6px">＋ Agregar producto</button>
    <button class="o-btn-secondary o-btn-sm" onclick="window._agregarSeccion(${ordId})" style="gap:6px">＋ Agregar sección</button>
    ${cfg.costos_envio ? `<button class="o-btn-secondary o-btn-sm" onclick="window._agregarEnvio(${ordId})" style="gap:6px">🚚 Agregar envío</button>` : ''}
  </div>`
}

// ─── Mapas de estado ────────────────────────────────────────────────────────
const STATE_LABEL = {
  draft:     'Cotización',
  sent:      'Enviado',
  sale:      'Pedido de Venta',
  done:      'Realizado',
  cancel:    'Cancelado',
}

const STATE_BADGE = {
  draft:   'o-badge-gray',
  sent:    'o-badge-info',
  sale:    'o-badge-success',
  done:    'o-badge-violet',
  cancel:  'o-badge-danger',
}

const INV_STATUS_LABEL = {
  no:             '—',
  to_invoice:     'Por Facturar',
  invoiced:       'Facturado',
}
const INV_STATUS_BADGE = {
  no:          '',
  to_invoice:  'o-badge-warn',
  invoiced:    'o-badge-success',
}

// ─── Entry point ────────────────────────────────────────────────────────────
export async function renderVentas(params) {
  return mount(params)
}

export async function mount(params) {
  ensureLayout()
  setBreadcrumb([{ label: 'Ventas' }])

  const id = params?.id
  if (id) {
    await _abrirVenta(parseInt(id))
    return
  }

  // Registrar helpers globales
  window._setPage     = p => { _page = p; _load() }
  window._setFilter   = f => { _filter = f; _page = 1; _load() }
  window._setView     = v => { _view = v; _renderContent() }
  window._abrirVenta  = _abrirVenta
  window._sortBy      = () => {}
  window._toggleSel   = id => {
    if (_selIDs.has(id)) _selIDs.delete(id); else _selIDs.add(id)
    document.getElementById('nx-sel-count')?.textContent && (_renderSelBar())
  }
  window._toggleAll = chk => {
    if (chk.checked) _records.forEach(r => _selIDs.add(r.id))
    else _selIDs.clear()
    document.querySelectorAll('.o-chk-row').forEach(c => c.checked = chk.checked)
    _renderSelBar()
  }

  setPage(_html())
  await _load()
}

function _html() {
  return `
  <div class="nx-module-page" style="min-height:100vh;background:var(--bg-app)">
    <!-- Control Panel -->
    <div class="o-cp" style="gap:10px;flex-wrap:wrap">
      <div class="o-cp-left">
        <button class="o-btn-primary" onclick="window._nuevaVenta()" id="btn-nueva-venta">
          ＋ Nueva
        </button>
        <div class="o-dropdown" style="position:relative">
          <button class="o-btn-filter" onclick="this.nextElementSibling.classList.toggle('open')" id="btn-filtros">
            ☰ Filtros ▾
          </button>
          <div class="o-dropdown-menu" id="dd-filtros">
            <div class="o-dd-item ${!_filter?'o-dd-item-active':''}" onclick="window._setFilter('');document.getElementById('dd-filtros').classList.remove('open')">Todos</div>
            <div class="o-dd-item ${_filter==='draft'?'o-dd-item-active':''}" onclick="window._setFilter('draft');document.getElementById('dd-filtros').classList.remove('open')">Cotizaciones</div>
            <div class="o-dd-item ${_filter==='sent'?'o-dd-item-active':''}" onclick="window._setFilter('sent');document.getElementById('dd-filtros').classList.remove('open')">Enviados</div>
            <div class="o-dd-item ${_filter==='sale'?'o-dd-item-active':''}" onclick="window._setFilter('sale');document.getElementById('dd-filtros').classList.remove('open')">Pedidos</div>
            <div class="o-dd-item ${_filter==='to_invoice'?'o-dd-item-active':''}" onclick="window._setFilter('to_invoice');document.getElementById('dd-filtros').classList.remove('open')">Por Facturar</div>
            <div class="o-dd-item ${_filter==='done'?'o-dd-item-active':''}" onclick="window._setFilter('done');document.getElementById('dd-filtros').classList.remove('open')">Realizados</div>
          </div>
        </div>
      </div>
      <div class="o-cp-center">
        <div class="o-search-bar">
          <span class="o-search-icon">🔍</span>
          <input class="o-search-input" id="venta-search" placeholder="Buscar por número, cliente, referencia..."
            value="${_search}"
            onkeyup="if(event.key==='Enter'){window._doSearch(this.value)}"
            oninput="if(!this.value){window._doSearch('')}">
          ${_search ? `<button style="background:none;border:none;cursor:pointer;color:var(--text-400);font-size:16px" onclick="document.getElementById('venta-search').value='';window._doSearch('')">×</button>` : ''}
        </div>
      </div>
      <div class="o-cp-right">
        <span id="nx-count" style="font-size:12px;color:var(--text-400)">${_total} registros</span>
        <div class="o-view-switcher">
          <button class="o-view-btn ${_view==='list'?'o-active':''}" title="Lista" onclick="window._setView('list')">☰</button>
          <button class="o-view-btn ${_view==='kanban'?'o-active':''}" title="Kanban" onclick="window._setView('kanban')">⊞</button>
        </div>
      </div>
    </div>
    <!-- Barra de selección múltiple (oculta inicialmente) -->
    <div id="nx-sel-bar" style="display:none;align-items:center;gap:12px;padding:8px 16px;background:#EEF2FF;border-bottom:1px solid var(--primary)">
      <span id="nx-sel-count" style="font-size:13px;font-weight:700;color:var(--primary)">0 seleccionados</span>
      <button class="o-btn-secondary o-btn-sm" onclick="window._cancelarSeleccionados()">❌ Cancelar</button>
      <button class="o-btn-secondary o-btn-sm" onclick="window._exportarCSV()">⬇ Exportar CSV</button>
      <button class="o-btn-secondary o-btn-sm" onclick="window._limpiarSel()">× Desmarcar todo</button>
    </div>
    <!-- Contenido principal -->
    <div id="nx-content" style="flex:1">${skeletonTable(6, 7)}</div>
  </div>`
}

// Exponer toast para los handlers onclick inline (botones de pago/firma/vista previa)
window.toast = toast

window._doSearch = q => { _search = q; _page = 1; _load() }
window._nuevaVenta = _nuevaVenta
window._cancelarSeleccionados = _cancelarSeleccionados
window._exportarCSV = _exportarCSV
window._limpiarSel = () => { _selIDs.clear(); _renderContent() }

// ─── Carga de datos ─────────────────────────────────────────────────────────
async function _load() {
  const c = document.getElementById('nx-content')
  if (!c) return
  c.innerHTML = skeletonTable(6, 7)
  try {
    const params = new URLSearchParams({ pagina: _page, limite: 80 })
    if (_filter && _filter !== 'to_invoice') params.set('estado', _filter)
    if (_filter === 'to_invoice') params.set('invoice_status', 'to_invoice')
    if (_search) params.set('buscar', _search)
    const res = await api.get(`/ventas?${params}`)
    _records = res?.data || []
    _total   = res?.total ?? _records.length
    const cnt = document.getElementById('nx-count')
    if (cnt) cnt.textContent = `${_total} registros`
    _renderContent()
  } catch (e) {
    c.innerHTML = `<div style="padding:60px;text-align:center;color:var(--text-400)">⚠️ ${e.message}</div>`
  }
}

function _renderContent() {
  const c = document.getElementById('nx-content')
  if (!c) return
  c.innerHTML = _view === 'kanban' ? _renderKanban(_records) : _renderLista(_records)
  if (_view === 'list') _initSeleccion()
  _renderSelBar()
}

function _renderSelBar() {
  const bar = document.getElementById('nx-sel-bar')
  const cnt = document.getElementById('nx-sel-count')
  if (!bar || !cnt) return
  if (_selIDs.size > 0) {
    bar.style.display = 'flex'
    cnt.textContent = `${_selIDs.size} seleccionado${_selIDs.size > 1 ? 's' : ''}`
  } else {
    bar.style.display = 'none'
  }
}

// ─── Vista Lista ─────────────────────────────────────────────────────────────
function _renderLista(rows) {
  if (!rows.length) return `<div style="padding:60px;text-align:center;color:var(--text-400)">
    <div style="font-size:48px;margin-bottom:12px">📋</div>
    <div style="font-size:16px;font-weight:600;margin-bottom:8px">No hay registros</div>
    <div style="font-size:13px">Crea una nueva cotización con el botón <strong>+ Nueva</strong></div>
  </div>`

  return `<div class="o-list-view">
    <table class="o-list-table">
      <thead>
        <tr>
          <th class="o-list-chk"><input type="checkbox" class="o-chk" onchange="window._toggleAll(this)"></th>
          <th class="o-col-sortable" onclick="window._sortBy('name')">NÚMERO ↕</th>
          <th>FECHA</th>
          <th>CLIENTE</th>
          <th>REFERENCIA CLIENTE</th>
          <th>VENDEDOR</th>
          <th class="o-col-right">TOTAL</th>
          <th>ESTADO</th>
          <th>FACTURACIÓN</th>
        </tr>
      </thead>
      <tbody>
        ${rows.map(r => `
        <tr class="o-list-row${_selIDs.has(r.id)?' selected':''}" onclick="window._abrirVenta(${r.id})">
          <td class="o-list-chk" onclick="event.stopPropagation()">
            <input type="checkbox" class="o-chk o-chk-row" ${_selIDs.has(r.id)?'checked':''} onchange="window._toggleSel(${r.id})">
          </td>
          <td class="o-td-primary" style="font-family:monospace">${r.name || '#'+r.id}</td>
          <td class="o-td-muted">${fmtDate(r.date_order)}</td>
          <td>
            <div class="o-partner-cell">
              <div class="o-avatar o-avatar-sm" style="background:${_avatarColor(r.partner_name)}">${(r.partner_name||'?')[0].toUpperCase()}</div>
              <span style="font-weight:500">${r.partner_name || '—'}</span>
            </div>
          </td>
          <td class="o-td-muted">${r.client_order_ref || '—'}</td>
          <td class="o-td-muted">
            ${r.user_name ? `<div class="o-partner-cell">
              <div class="o-avatar o-avatar-sm" style="background:#6366F1">${(r.user_name||'A')[0]}</div>
              <span>${r.user_name}</span>
            </div>` : '<span style="color:var(--text-400)">Administrador</span>'}
          </td>
          <td class="o-td-amount">${_fmtK(r.amount_total)}</td>
          <td><span class="o-badge ${STATE_BADGE[r.state]||'o-badge-gray'}">${STATE_LABEL[r.state]||r.state}</span></td>
          <td>${r.invoice_status && r.invoice_status !== 'no'
            ? `<span class="o-badge ${INV_STATUS_BADGE[r.invoice_status]||''}">${INV_STATUS_LABEL[r.invoice_status]||r.invoice_status}</span>`
            : '<span style="color:var(--text-300)">—</span>'}</td>
        </tr>`).join('')}
      </tbody>
    </table>
    <div class="o-list-footer">
      <span class="o-list-count">${_total} registros</span>
      ${paginationHtml(_page, _total > _page * 80, window._setPage)}
    </div>
  </div>`
}

// ─── Vista Kanban ────────────────────────────────────────────────────────────
function _renderKanban(rows) {
  const COLS = [
    { key: 'draft',  label: 'Cotización',       color: '#6B7280' },
    { key: 'sent',   label: 'Enviado',           color: '#3B82F6' },
    { key: 'sale',   label: 'Pedido de Venta',   color: '#10B981' },
    { key: 'done',   label: 'Realizado',         color: '#8B5CF6' },
  ]
  const byState = {}
  COLS.forEach(c => byState[c.key] = rows.filter(r => r.state === c.key))

  return `<div style="display:flex;gap:16px;padding:20px;overflow-x:auto;min-height:calc(100vh - 180px);align-items:flex-start;background:var(--bg-app)">
    ${COLS.map(col => `
    <div style="background:var(--bg-card);border:1px solid var(--border);border-radius:12px;min-width:280px;max-width:300px;flex-shrink:0;display:flex;flex-direction:column;box-shadow:var(--shadow-sm)">
      <div style="display:flex;align-items:center;justify-content:space-between;padding:12px 16px;font-size:12px;font-weight:700;text-transform:uppercase;letter-spacing:.05em;color:#fff;background:${col.color};border-radius:12px 12px 0 0">
        <span>${col.label}</span>
        <span style="background:rgba(255,255,255,.25);padding:2px 8px;border-radius:12px">${byState[col.key].length}</span>
      </div>
      <div style="padding:10px;display:flex;flex-direction:column;gap:8px;flex:1;overflow-y:auto;max-height:65vh">
        ${byState[col.key].length === 0 ? `<div style="text-align:center;padding:24px;color:var(--text-300);font-size:12px">Sin registros</div>` : ''}
        ${byState[col.key].map(r => `
        <div onclick="window._abrirVenta(${r.id})"
          style="background:var(--bg-card);border:1px solid var(--border);border-radius:10px;padding:14px;cursor:pointer;transition:all .15s;box-shadow:var(--shadow-sm)"
          onmouseover="this.style.borderColor='${col.color}';this.style.transform='translateY(-2px)';this.style.boxShadow='0 6px 20px rgba(0,0,0,.1)'"
          onmouseout="this.style.borderColor='';this.style.transform='';this.style.boxShadow=''">
          <div style="display:flex;justify-content:space-between;align-items:flex-start;margin-bottom:6px">
            <strong style="font-family:monospace;font-size:13px;color:var(--primary)">${r.name||'#'+r.id}</strong>
            <span style="font-size:11px;color:var(--text-400)">${fmtDate(r.date_order)}</span>
          </div>
          <div style="font-weight:600;margin-bottom:4px;font-size:13px;color:var(--text-900)">${r.partner_name||'—'}</div>
          ${r.client_order_ref ? `<div style="font-size:11px;color:var(--text-400);margin-bottom:6px">Ref: ${r.client_order_ref}</div>` : ''}
          <div style="display:flex;justify-content:space-between;align-items:center;margin-top:8px;padding-top:8px;border-top:1px solid var(--border)">
            <span class="o-badge ${INV_STATUS_BADGE[r.invoice_status]||'o-badge-gray'}" style="font-size:10px">${INV_STATUS_LABEL[r.invoice_status]||'—'}</span>
            <strong style="font-size:14px;color:${col.color};font-variant-numeric:tabular-nums">${fmtMxn(r.amount_total)}</strong>
          </div>
        </div>`).join('')}
      </div>
      <div style="padding:10px 14px;border-top:1px solid var(--border);font-size:12px;font-weight:700;color:var(--text-500)">
        Total: ${fmtMxn(byState[col.key].reduce((s,r)=>s+parseFloat(r.amount_total||0),0))}
      </div>
    </div>`).join('')}
  </div>`
}

// ─── Formulario de detalle (Odoo-style) ──────────────────────────────────────
async function _abrirVenta(id) {
  // Actualizar URL sin disparar el router — igual que Odoo: F5 permanece en el mismo registro
  history.replaceState(null, '', `#ventas?id=${id}`)

  setPage(`<div style="padding:40px">${skeletonTable(3, 5)}</div>`)
  try {
    const [ordRes, linRes] = await Promise.all([
      api.get(`/ventas/${id}`),
      api.get(`/ventas/${id}/lineas`),
    ])
    // Facturas y entrega vinculadas
    let facturas = []
    let entrega  = null
    try { const fr = await api.get(`/ventas/${id}/facturas`); facturas = fr?.data || [] } catch(_) {}
    try { const er = await api.get(`/ventas/${id}/entrega`);  entrega  = er?.data || null  } catch(_) {}

    _currentOrder  = ordRes?.data || ordRes
    _currentLineas = linRes?.data || []
    _renderFormulario(_currentOrder, _currentLineas, facturas, entrega)
  } catch (e) {
    setPage(`<div style="padding:40px;text-align:center;color:red">⚠️ ${e.message}</div>`)
  }
}

function _renderFormulario(v, lineas, facturas = [], entrega = null) {
  const cfg = getCfg()
  const extras = getExtras(v.id)
  const isEditable = (v.state === 'draft' || v.state === 'sent') && !v.locked
  const isSale = v.state === 'sale' || v.state === 'done'
  const statusSteps = [
    { key: 'draft', label: 'Cotización' },
    { key: 'sent',  label: 'Enviado' },
    { key: 'sale',  label: 'Pedido de Venta' },
    { key: 'done',  label: 'Realizado' },
  ]
  const currentIdx = statusSteps.findIndex(s => s.key === v.state)

  // Calcular totales
  const subtotal = lineas.reduce((s, l) => s + parseFloat(l.price_subtotal || 0), 0)
  const impuesto = subtotal * 0.16
  const total    = subtotal + impuesto

  // Envío real: líneas de pedido cuyo nombre comienza con "Envío"
  const envioLineas = lineas.filter(l => l.display_type !== 'line_section' && /^env[ií]o/i.test(l.name || ''))
  const costoEnvio  = envioLineas.reduce((s, l) => s + parseFloat(l.price_subtotal || 0), 0)
  const metodoEnvio = envioLineas.length ? (envioLineas[0].name.split('—')[1]?.trim() || envioLineas[0].name) : null

  // Pagos en línea registrados (config: pago_online)
  const pagos       = extras.pagos || []
  const totalPagado = pagos.reduce((s, p) => s + parseFloat(p.monto || 0), 0)

  // Firma digital (config: firma_online)
  const firmaNombre = v.signature_name || extras.firma?.name || null
  const firmaFecha  = extras.firma?.fecha ? fmtDate(extras.firma.fecha) : ''

  // Lista de precios aplicada (config: listas_precios)
  const pricelists  = getPricelists().filter(p => p.active)
  const plAplicada  = getPricelists().find(p => p.id === extras.pricelist_id)

  const html = `
  <div id="venta-form" style="min-height:100vh;background:var(--bg-app)">

    <!-- TOPBAR -->
    <div style="display:flex;align-items:center;justify-content:space-between;padding:10px 20px;background:var(--bg-card);border-bottom:1px solid var(--border);position:sticky;top:50px;z-index:20;flex-wrap:wrap;gap:8px">
      <div style="display:flex;align-items:center;gap:8px">
        <button class="o-btn-secondary o-btn-sm" onclick="window._go('ventas')" style="gap:6px">
          ← Ventas
        </button>
        <button class="o-btn-secondary o-btn-sm" onclick="window._prevRecord()">‹</button>
        <button class="o-btn-secondary o-btn-sm" onclick="window._nextRecord()">›</button>
      </div>
      <div style="display:flex;gap:8px;flex-wrap:wrap">
        ${_renderActionButtons(v)}
      </div>
    </div>

    <!-- STATUS BAR -->
    <div style="display:flex;align-items:center;padding:8px 24px;background:var(--bg-card);border-bottom:1px solid var(--border);gap:0">
      ${statusSteps.map((s, i) => {
        const isDone   = i < currentIdx
        const isActive = i === currentIdx
        const isFuture = i > currentIdx
        if (v.state === 'cancel') return ''
        return `
          ${i > 0 ? `<span style="color:var(--text-300);font-size:14px;margin:0 2px">›</span>` : ''}
          <button onclick="return false"
            style="padding:6px 16px;border-radius:20px;border:none;font-size:12px;font-weight:700;cursor:default;
              ${isActive ? 'background:var(--primary);color:#fff;' : ''}
              ${isDone   ? 'color:var(--primary);opacity:.6;background:transparent;' : ''}
              ${isFuture ? 'color:var(--text-400);background:transparent;' : ''}"
          >${isDone ? '✓ ' : ''}${s.label}</button>`
      }).join('')}
      ${v.state === 'cancel' ? `<span class="o-badge o-badge-danger" style="font-size:13px">Cancelado</span>` : ''}
    </div>

    <!-- SMART BUTTONS — Flujo 3 módulos: Ventas → Almacén → Facturación -->
    <div style="display:flex;gap:10px;padding:10px 24px;background:var(--bg-card);border-bottom:1px solid var(--border);flex-wrap:wrap">
      ${(v.state === 'sale' || v.state === 'done') ? `
      <!-- Smart Button: Entrega → navega al módulo Almacén (igual que Odoo) -->
      <button onclick="window._verEntrega(${v.id})"
        style="display:flex;flex-direction:column;align-items:center;gap:2px;padding:8px 18px;border:1px solid ${entrega?.state==='entregado'?'#10B981':entrega?.state==='parcial'?'#F59E0B':'#6366F1'};border-radius:10px;background:var(--bg-card);cursor:pointer;transition:all .15s;min-width:90px"
        onmouseover="this.style.background='#EEF2FF'" onmouseout="this.style.background=''">
        <span style="font-size:22px">${entrega?.state==='entregado'?'✅':entrega?.state==='parcial'?'📦':'🚚'}</span>
        <span style="font-size:12px;font-weight:700;color:${entrega?.state==='entregado'?'#10B981':entrega?.state==='parcial'?'#F59E0B':'#6366F1'}">
          1 Entrega
        </span>
        <span style="font-size:10px;color:var(--text-400)">Almacén</span>
      </button>` : ''}

      <!-- Smart Button: Facturas -->
      <button onclick="window._verFacturas(${v.id})"
        style="display:flex;flex-direction:column;align-items:center;gap:2px;padding:8px 18px;border:1px solid ${facturas.length?'#10B981':'var(--border)'};border-radius:10px;background:var(--bg-card);cursor:pointer;transition:all .15s;min-width:80px"
        onmouseover="this.style.background='#EEF2FF'" onmouseout="this.style.background=''">
        <span style="font-size:20px;font-weight:800;color:${facturas.length?'#10B981':'var(--text-400)'}">${facturas.length}</span>
        <span style="font-size:11px;color:var(--text-500)">Facturas</span>
      </button>

      <!-- Smart Button: Líneas -->
      <button onclick="window._verLineas(${v.id})"
        style="display:flex;flex-direction:column;align-items:center;gap:2px;padding:8px 18px;border:1px solid var(--border);border-radius:10px;background:var(--bg-card);cursor:pointer;transition:all .15s;min-width:80px"
        onmouseover="this.style.borderColor='var(--primary)';this.style.background='#EEF2FF'" onmouseout="this.style.borderColor='';this.style.background=''">
        <span style="font-size:20px;font-weight:800;color:var(--primary)">${lineas.length}</span>
        <span style="font-size:11px;color:var(--text-500)">Líneas</span>
      </button>
    </div>

    <!-- FORM SHEET -->
    <div style="background:var(--bg-card);border-radius:12px;margin:16px 20px 0;border:1px solid var(--border);overflow:hidden">

      <!-- Encabezado del documento -->
      <div style="padding:20px 24px 16px;border-bottom:1px solid var(--border)">
        <div style="display:flex;align-items:flex-start;gap:16px">
          <div style="flex:1">
            <h1 style="font-family:'Plus Jakarta Sans',sans-serif;font-size:22px;font-weight:800;color:var(--text-900);margin:0 0 6px">${v.name || 'Nueva Cotización'}</h1>
            <span class="o-badge ${STATE_BADGE[v.state]||'o-badge-gray'}">${STATE_LABEL[v.state]||v.state}</span>
          </div>
        </div>
      </div>

      <!-- CAMPOS DEL FORMULARIO -->
      <div style="display:grid;grid-template-columns:1fr 1fr;gap:0 32px;padding:16px 24px">

        <!-- Columna izquierda -->
        <div>
          ${_campo('Cliente', _m2oField('f-partner', v.partner_name || '', 'buscar-clientes', 'partner_id', v.partner_id, isEditable, v.id), true)}
          ${_campo('Dirección de Facturación', _m2oField('f-invoice', v.partner_invoice_name || v.partner_name || '', 'buscar-clientes', 'partner_invoice_id', v.partner_invoice_id, isEditable, v.id))}
          ${_campo('Dirección de Envío', _m2oField('f-shipping', v.partner_shipping_name || v.partner_name || '', 'buscar-clientes', 'partner_shipping_id', v.partner_shipping_id, isEditable, v.id))}
          ${_campo('Referencia del Cliente',
            isEditable
              ? `<input class="o-field-input" id="f-client-ref" value="${v.client_order_ref||''}" onblur="window._guardarCampo(${v.id},'client_order_ref',this.value)">`
              : `<span>${v.client_order_ref||'—'}</span>`
          )}
          ${cfg.plantillas_presupuesto ? _campo('Plantilla de Presupuesto',
            isEditable
              ? `<select class="o-field-input" onchange="if(this.value)window._aplicarPlantilla(${v.id},this.value)">
                  <option value="">— Sin plantilla —</option>
                  ${Object.entries(QUOTE_TEMPLATES).map(([k, t]) =>
                    `<option value="${k}" ${extras.plantilla === k ? 'selected' : ''}>${t.label}</option>`).join('')}
                </select>`
              : `<span>${QUOTE_TEMPLATES[extras.plantilla]?.label || '—'}</span>`
          ) : ''}
        </div>

        <!-- Columna derecha -->
        <div>
          ${_campo('Fecha Pedido', `<span style="font-weight:600">${fmtDate(v.date_order)}</span>`)}
          ${cfg.fecha_entrega ? _campo('Fecha Compromiso',
            isEditable
              ? `<input class="o-field-input" type="date" value="${v.commitment_date?v.commitment_date.split('T')[0]:''}" onblur="window._guardarCampo(${v.id},'commitment_date',this.value)">`
              : `<span>${v.commitment_date ? fmtDate(v.commitment_date) : '—'}</span>`
          ) : ''}
          ${_campo('Fecha Validez',
            isEditable
              ? `<input class="o-field-input" type="date" value="${v.validity_date?v.validity_date.split('T')[0]:''}" onblur="window._guardarCampo(${v.id},'validity_date',this.value)">`
              : `<span>${v.validity_date ? fmtDate(v.validity_date) : '—'}</span>`
          )}
          ${_campo('Origen',
            isEditable
              ? `<input class="o-field-input" placeholder="Referencia de origen..." value="${v.origin||''}" onblur="window._guardarCampo(${v.id},'origin',this.value)">`
              : `<span>${v.origin||'—'}</span>`
          )}
          ${_campo('Plazo de Pago', `<span>${v.payment_term_name||'—'}</span>`)}
          ${_campo('Moneda', `<span>${v.currency_name||'MXN'}</span>`)}
        </div>
      </div>

      <!-- NOTEBOOK TABS -->
      <div style="border-top:1px solid var(--border)">
        <div style="display:flex;border-bottom:1px solid var(--border);background:var(--bg-app);padding:0 20px;overflow-x:auto" id="venta-tabs">
          <button class="o-tab active" data-tab="lineas" onclick="window._switchTab('lineas',this)">Líneas de Pedido</button>
          <button class="o-tab" data-tab="info" onclick="window._switchTab('info',this)">Otra Información</button>
          <button class="o-tab" data-tab="notas" onclick="window._switchTab('notas',this)">Notas y Términos</button>
        </div>

        <!-- TAB: Líneas de Pedido -->
        <div id="tab-lineas" style="padding:0">
          ${_renderTabLineas(lineas, v.state, v.locked, v.id)}
          ${isEditable ? _btnBarLineas(v.id) : ''}
        </div>

        <!-- TOTALES (separado de tab-lineas para que el picker no quede debajo) -->
        <div id="totales-area" style="display:flex;justify-content:flex-end;padding:16px 24px;border-top:1px solid var(--border)">
          <table style="width:280px">
            <tr>
              <td style="padding:4px 8px;font-size:13px;color:var(--text-600)">Subtotal</td>
              <td id="tot-subtotal" style="padding:4px 8px;font-size:13px;text-align:right;font-weight:600;font-variant-numeric:tabular-nums">${fmtMxn(subtotal)}</td>
            </tr>
            <tr>
              <td style="padding:4px 8px;font-size:13px;color:var(--text-600)">IVA (16%)</td>
              <td id="tot-iva" style="padding:4px 8px;font-size:13px;text-align:right;font-weight:600;font-variant-numeric:tabular-nums">${fmtMxn(impuesto)}</td>
            </tr>
            <tr style="border-top:2px solid var(--border)">
              <td style="padding:8px 8px 4px;font-size:16px;font-weight:800;color:var(--text-900)">TOTAL</td>
              <td id="tot-total" style="padding:8px 8px 4px;font-size:16px;font-weight:800;color:var(--primary);text-align:right;font-variant-numeric:tabular-nums">${fmtMxn(total)}</td>
            </tr>
            ${cfg.margenes ? `
            <tr>
              <td style="padding:4px 8px;font-size:12px;color:var(--text-400)">Margen</td>
              <td id="tot-margen" style="padding:4px 8px;font-size:12px;text-align:right;font-weight:600;color:#10B981;font-variant-numeric:tabular-nums">${fmtMxn(_calcMargen(lineas))}</td>
            </tr>` : ''}
          </table>
        </div>

        <!-- TAB: Otra Información -->
        <div id="tab-info" style="padding:16px 24px;display:none">
          <div style="display:grid;grid-template-columns:1fr 1fr;gap:0 32px">
            <div>
              <h4 style="font-size:11px;text-transform:uppercase;color:var(--text-400);letter-spacing:.05em;margin:0 0 12px;font-weight:700">Ventas</h4>
              ${_campo('Vendedor', `<span>${v.user_name||'Administrador'}</span>`)}
              ${_campo('Equipo de Ventas', `<span>${v.team_name||'—'}</span>`)}
              ${_campo('Empresa', `<span>${v.company_id ? 'NexusTech' : '—'}</span>`)}
              ${cfg.listas_precios ? _campo('Lista de Precios',
                isEditable
                  ? `<select class="o-field-input" onchange="window._aplicarListaPrecios(${v.id},this.value)">
                      <option value="">Tarifa pública (sin descuento)</option>
                      ${pricelists.map(p => `<option value="${p.id}" ${extras.pricelist_id === p.id ? 'selected' : ''}>${p.name} (${p.currency}${p.type === 'Porcentaje' && p.discount ? ` · −${p.discount}%` : ''})</option>`).join('')}
                    </select>`
                  : `<span>${plAplicada?.name || v.pricelist_name || 'Tarifa pública'}</span>`
              ) : ''}
            </div>
            <div>
              <h4 style="font-size:11px;text-transform:uppercase;color:var(--text-400);letter-spacing:.05em;margin:0 0 12px;font-weight:700">Contabilidad</h4>
              ${_campo('Estado Facturación', `<span class="o-badge ${INV_STATUS_BADGE[v.invoice_status]||'o-badge-gray'}">${INV_STATUS_LABEL[v.invoice_status]||'—'}</span>`)}
              ${_campo('Política Facturación', `<span>${cfg.politica_facturacion==='cantidad_pedida'?'Cantidades pedidas':'Cantidades entregadas'}</span>`)}
              ${cfg.listas_precios && cfg.descuento_precio && plAplicada?.type === 'Porcentaje' && plAplicada?.discount
                ? _campo('Desc. de Lista', `<span style="color:#10B981;font-weight:600">−${plAplicada.discount}% (${plAplicada.name})</span>`)
                : ''}
              ${_campo('Bloqueado', `<span>${v.locked ? 'Sí' : 'No'}</span>`)}
            </div>
          </div>
          ${cfg.costos_envio ? `
          <div style="border-top:1px solid var(--border);margin-top:16px;padding-top:16px">
            <h4 style="font-size:11px;text-transform:uppercase;color:var(--text-400);letter-spacing:.05em;margin:0 0 12px;font-weight:700">Envío</h4>
            <div style="display:grid;grid-template-columns:1fr 1fr;gap:0 32px">
              <div>
                ${_campo('Método de Entrega', `<span>${metodoEnvio || '—'}</span>`)}
              </div>
              <div>
                ${_campo('Costo de Envío', `<span style="font-weight:600">${envioLineas.length ? fmtMxn(costoEnvio) : '—'}</span>`)}
              </div>
            </div>
            ${isEditable ? `
            <button class="o-btn-secondary o-btn-sm" onclick="window._agregarEnvio(${v.id})" style="margin-top:4px">🚚 ${envioLineas.length ? 'Cambiar' : 'Agregar'} costo de envío</button>` : ''}
          </div>` : ''}
        </div>

        <!-- TAB: Notas y Términos -->
        <div id="tab-notas" style="padding:16px 24px;display:none">
          ${cfg.notas_cierre && isSale ? `
          <div style="margin-bottom:16px;padding:12px 16px;background:#EEF2FF;border-radius:8px;border-left:3px solid var(--primary)">
            <div style="font-size:12px;font-weight:700;color:var(--primary);margin-bottom:6px">ℹ️ NOTA DE CIERRE</div>
            <div style="font-size:13px;color:var(--text-700)">${cfg.terminos||'Gracias por su preferencia. Cualquier reclamación debe hacerse en los 5 días hábiles siguientes a la entrega.'}</div>
          </div>` : ''}
          <div style="margin-bottom:12px">
            <label style="font-size:12px;font-weight:700;color:var(--text-400);margin-bottom:6px;display:block">TÉRMINOS Y CONDICIONES</label>
            ${isEditable
              ? `<textarea class="o-field-input" rows="4" style="resize:vertical;width:100%;box-sizing:border-box" onblur="window._guardarCampo(${v.id},'note',this.value)">${v.note||cfg.terminos||''}</textarea>`
              : `<div style="font-size:13px;color:var(--text-600);min-height:60px;white-space:pre-wrap">${v.note||cfg.terminos||'Sin notas.'}</div>`
            }
          </div>
          ${cfg.firma_online ? `
          <div style="border-top:1px solid var(--border);padding-top:16px;margin-top:8px">
            <h4 style="font-size:12px;font-weight:700;color:var(--text-500);margin:0 0 12px;text-transform:uppercase;letter-spacing:.05em">✍️ Firma Digital</h4>
            ${firmaNombre ? `
            <div style="display:flex;align-items:center;gap:12px;padding:12px 16px;background:#F0FDF4;border-radius:8px;border:1px solid #10B981">
              <span style="font-size:18px">✅</span>
              <div>
                <div style="font-weight:600;color:#065F46">${firmaNombre}</div>
                <div style="font-size:11px;color:#059669">Firmado digitalmente${firmaFecha ? ` el ${firmaFecha}` : ''}</div>
              </div>
            </div>` : (v.state === 'draft' || v.state === 'sent') ? `
            <div style="display:flex;gap:8px;align-items:center">
              <input id="firma-nombre" class="o-field-input" placeholder="Nombre completo del firmante..." style="flex:1;max-width:340px">
              <button class="o-btn-primary o-btn-sm" onclick="window._firmarCotizacion(${v.id})">✍️ Firmar</button>
            </div>
            <div style="font-size:11px;color:var(--text-400);margin-top:6px">Al firmar, la cotización queda aceptada por el cliente y puede confirmarse como pedido de venta.</div>` : `
            <div style="border:2px dashed var(--border);border-radius:8px;padding:16px;text-align:center;color:var(--text-400);font-size:13px">Sin firma registrada</div>`}
          </div>` : ''}
          ${cfg.pago_online ? `
          <div style="border-top:1px solid var(--border);padding-top:16px;margin-top:8px">
            <h4 style="font-size:12px;font-weight:700;color:var(--text-500);margin:0 0 12px;text-transform:uppercase;letter-spacing:.05em">💳 Pago en Línea</h4>
            ${pagos.length ? `
            <div style="margin-bottom:10px">
              ${pagos.map(p => `
              <div style="display:flex;justify-content:space-between;align-items:center;padding:8px 14px;background:#F0FDF4;border:1px solid #10B981;border-radius:8px;margin-bottom:6px;font-size:13px">
                <span>✅ ${p.metodo} — ${fmtDate(p.fecha)}</span>
                <strong style="color:#065F46">${fmtMxn(p.monto)}</strong>
              </div>`).join('')}
              <div style="font-size:12px;color:var(--text-500);text-align:right">Pagado: <strong>${fmtMxn(totalPagado)}</strong> de ${fmtMxn(total)}</div>
            </div>` : ''}
            ${totalPagado >= total - 0.01 && total > 0 ? `
            <div style="font-size:13px;font-weight:700;color:#10B981">✅ Pedido pagado en su totalidad</div>` :
            (v.state === 'draft' || v.state === 'sent') ? `
            <div style="display:flex;gap:8px;flex-wrap:wrap">
              <button class="o-btn-secondary o-btn-sm" onclick="window._registrarPagoOnline(${v.id},'Tarjeta')">💳 Pagar con Tarjeta</button>
              <button class="o-btn-secondary o-btn-sm" onclick="window._registrarPagoOnline(${v.id},'Transferencia')">🏦 Transferencia Bancaria</button>
            </div>` : `<div style="font-size:13px;color:var(--text-400)">Sin pagos en línea registrados</div>`}
          </div>` : ''}
        </div>
      </div>
    </div>

    <!-- CHATTER -->
    <div style="background:var(--bg-card);border-radius:12px;margin:16px 20px 24px;border:1px solid var(--border);overflow:hidden">
      <div style="padding:12px 20px;border-bottom:1px solid var(--border);background:var(--bg-app)">
        <div style="display:flex;gap:8px">
          <button class="o-chatter-btn" onclick="window._enviarMensaje(${v.id})">💬 Enviar mensaje</button>
          <button class="o-chatter-btn" onclick="window._agregarNota(${v.id})">📝 Agregar nota interna</button>
        </div>
      </div>
      <div id="chatter-${v.id}" style="padding:16px 20px;min-height:60px;font-size:13px;color:var(--text-400)">
        Sin actividad registrada.
      </div>
    </div>

  </div>

  <!-- Modal crear factura -->
  <div id="modal-factura" style="display:none;position:fixed;inset:0;z-index:950;background:rgba(0,0,0,.45);backdrop-filter:blur(3px);align-items:center;justify-content:center;padding:16px">
    <div style="background:var(--bg-card);border-radius:14px;box-shadow:0 24px 64px rgba(0,0,0,.22);border:1px solid var(--border);width:100%;max-width:520px;animation:slideUp .18s cubic-bezier(.34,1.56,.64,1)">
      <div style="display:flex;align-items:center;justify-content:space-between;padding:16px 20px;border-bottom:1px solid var(--border);background:var(--bg-app);border-radius:14px 14px 0 0">
        <h3 style="font-size:15px;font-weight:700;color:var(--text-900);margin:0">📄 Crear Factura</h3>
        <button onclick="document.getElementById('modal-factura').style.display='none'" style="background:none;border:none;cursor:pointer;font-size:18px;color:var(--text-400)">×</button>
      </div>
      <div style="padding:20px">
        <!-- Resumen del pedido -->
        <div style="background:#EEF2FF;border-radius:8px;padding:12px 16px;margin-bottom:16px">
          <div style="font-size:12px;font-weight:700;color:var(--primary);margin-bottom:6px">${v.name} — ${v.partner_name||'Sin cliente'}</div>
          <div style="display:flex;justify-content:space-between;font-size:13px">
            <span style="color:var(--text-600)">Subtotal</span><span>${fmtMxn(subtotal)}</span>
          </div>
          <div style="display:flex;justify-content:space-between;font-size:13px;margin-top:4px">
            <span style="color:var(--text-600)">IVA (16%)</span><span>${fmtMxn(impuesto)}</span>
          </div>
          <div style="display:flex;justify-content:space-between;font-size:14px;font-weight:800;margin-top:8px;padding-top:8px;border-top:1px solid var(--border)">
            <span>Total</span><span style="color:var(--primary)">${fmtMxn(total)}</span>
          </div>
        </div>

        <!-- Selector de tipo -->
        <div style="margin-bottom:14px">
          <label style="font-size:12px;font-weight:700;color:var(--text-600);margin-bottom:6px;display:block">TIPO DE FACTURA</label>
          <select class="o-field-input" id="tipo-factura" onchange="window._onTipoFacturaChange(this.value,${total})">
            <option value="delivered">Factura Regular (cantidades entregadas)</option>
            <option value="percentage">Anticipo — Porcentaje (%)</option>
            <option value="fixed">Anticipo — Monto Fijo</option>
          </select>
        </div>

        <!-- Campo dinámico: porcentaje (visible solo si tipo=percentage) -->
        <div id="factura-pct-row" style="display:none;margin-bottom:14px">
          <label style="font-size:12px;font-weight:700;color:var(--text-600);margin-bottom:6px;display:block">PORCENTAJE DEL ANTICIPO</label>
          <div style="display:flex;align-items:center;gap:8px">
            <input type="number" class="o-field-input" id="factura-pct" value="30" min="1" max="100" step="1"
              style="width:100px" oninput="window._calcAnticipo('pct',this.value,${total})">
            <span style="font-size:13px;color:var(--text-500)">%</span>
            <span style="font-size:13px;font-weight:700;color:var(--primary)" id="factura-pct-monto">${fmtMxn(total * 0.3)}</span>
          </div>
        </div>

        <!-- Campo dinámico: monto fijo (visible solo si tipo=fixed) -->
        <div id="factura-fixed-row" style="display:none;margin-bottom:14px">
          <label style="font-size:12px;font-weight:700;color:var(--text-600);margin-bottom:6px;display:block">MONTO DEL ANTICIPO</label>
          <div style="display:flex;align-items:center;gap:8px">
            <span style="font-size:13px;color:var(--text-500)">$</span>
            <input type="number" class="o-field-input" id="factura-fixed" value="${(total * 0.3).toFixed(2)}" min="0.01" step="0.01"
              style="width:160px" oninput="window._calcAnticipo('fixed',this.value,${total})">
            <span style="font-size:13px;font-weight:600;color:#059669" id="factura-fixed-pct">(30%)</span>
          </div>
        </div>

        <div style="font-size:12px;color:var(--text-400)">
          Política de facturación: <strong>${cfg.politica_facturacion === 'cantidad_entregada' ? 'Cantidades entregadas' : 'Cantidades pedidas'}</strong>
        </div>
      </div>
      <div style="display:flex;gap:8px;justify-content:flex-end;padding:12px 20px;border-top:1px solid var(--border);background:var(--bg-app);border-radius:0 0 14px 14px">
        <button class="o-btn-secondary" onclick="document.getElementById('modal-factura').style.display='none'">Cancelar</button>
        <button class="o-btn-primary" onclick="window._ejecutarCrearFactura(${v.id})">✓ Crear Factura</button>
      </div>
    </div>
  </div>

  <!-- Producto picker inline dropdown -->
  <div id="product-picker-dropdown" style="display:none;position:fixed;z-index:500;background:var(--bg-card);border:1.5px solid var(--primary);border-radius:8px;box-shadow:0 8px 24px rgba(0,0,0,.12);min-width:320px;max-height:280px;overflow-y:auto"></div>

  `

  setPage(html)
  _initTabSwitcher()
  _initFormGlobals(v, lineas)
}

function _renderActionButtons(v) {
  const cfg = getCfg()
  const btnPreview = cfg.compra_online
    ? `<button class="o-btn-secondary" onclick="window._vistaPreviaCliente(${v.id})">👁 Vista Previa</button>`
    : ''

  // Menú de acciones ⋮ (siempre disponible, igual que Odoo action menu)
  const menuAcciones = `
    <div class="o-dropdown" style="position:relative">
      <button class="o-btn-secondary" id="btn-acciones-${v.id}" onclick="document.getElementById('dd-acciones-${v.id}').classList.toggle('open')" style="padding:6px 10px;min-width:0">⋮</button>
      <div class="o-dropdown-menu" id="dd-acciones-${v.id}" style="right:0;left:auto;min-width:180px">
        <div class="o-dd-item" onclick="window._duplicarVenta(${v.id})">📋 Duplicar</div>
        <div class="o-dd-item" onclick="window._imprimirCotizacion(${v.id})">🖨 Imprimir / PDF</div>
        ${v.state !== 'cancel' ? `<div class="o-dd-item" style="color:#DC2626" onclick="window._accionVenta(${v.id},'cancelar')">✕ Cancelar</div>` : ''}
      </div>
    </div>`

  switch(v.state) {
    case 'draft':
      return `
        <button class="o-btn-primary" onclick="window._accionVenta(${v.id},'confirmar')" style="background:#10B981">✓ Confirmar</button>
        <button class="o-btn-secondary" onclick="window._accionVenta(${v.id},'enviar')">📧 Enviar</button>
        ${btnPreview}
        ${menuAcciones}
      `
    case 'sent':
      return `
        <button class="o-btn-primary" onclick="window._accionVenta(${v.id},'confirmar')" style="background:#10B981">✓ Confirmar</button>
        ${btnPreview}
        ${menuAcciones}
      `
    case 'sale':
      return `
        <button class="o-btn-primary" onclick="window._abrirModalFactura(${v.id})">📄 Crear Factura</button>
        ${!v.locked
          ? `<button class="o-btn-secondary" onclick="window._accionVenta(${v.id},'bloquear',{locked:true})">🔒 Bloquear</button>`
          : `<button class="o-btn-secondary" onclick="window._accionVenta(${v.id},'bloquear',{locked:false})">🔓 Desbloquear</button>`
        }
        ${menuAcciones}
      `
    case 'done':
      return `
        <span class="o-badge o-badge-violet" style="font-size:13px;padding:6px 14px">✓ Realizado</span>
        ${menuAcciones}
      `
    case 'cancel':
      return `
        <button class="o-btn-secondary" onclick="window._restaurarBorrador(${v.id})">↩ Restaurar a Borrador</button>
        ${menuAcciones}
      `
    default:
      return ''
  }
}

function _campo(label, content, required = false) {
  return `
  <div style="display:grid;grid-template-columns:150px 1fr;align-items:start;padding:5px 0;min-height:32px">
    <label style="font-size:12px;font-weight:600;color:var(--text-400);padding-top:7px">${label}${required?'<span style="color:#DC2626;margin-left:2px">*</span>':''}</label>
    <div style="font-size:13px;color:var(--text-900)">${content}</div>
  </div>`
}

function _m2oField(id, value, endpoint, fieldName, fieldId, editable, orderId) {
  if (!editable) return `<span style="font-weight:500;color:var(--primary)">${value || '—'}</span>`
  return `
  <div class="o-m2o-field" style="position:relative">
    <input class="o-field-input" id="${id}"
      value="${value}"
      autocomplete="off"
      placeholder="Buscar..."
      data-field="${fieldName}"
      data-order="${orderId}"
      oninput="window._m2oInput(this,'${endpoint}')"
      onblur="setTimeout(()=>window._hideM2o('${id}'),200)">
    <div id="${id}-dd" style="display:none;position:absolute;top:calc(100% + 2px);left:0;right:0;background:var(--bg-card);border:1.5px solid var(--primary);border-radius:8px;box-shadow:0 8px 24px rgba(0,0,0,.12);z-index:200;max-height:220px;overflow-y:auto"></div>
  </div>`
}

// ─── Tab Líneas ──────────────────────────────────────────────────────────────
function _renderTabLineas(lineas, state, locked, orderId) {
  const cfg = getCfg()
  const extras = getExtras(orderId)
  const isEditable  = (state === 'draft' || state === 'sent') && !locked
  const isSale      = state === 'sale' || state === 'done'
  // El descuento se muestra si están activos los descuentos manuales o los descuentos de lista de precios
  const showDisc    = cfg.descuentos !== false || !!(cfg.listas_precios && cfg.descuento_precio)
  const showMargen  = !!cfg.margenes
  const showUom     = !!cfg.unidades_medida
  const showPack    = !!cfg.empaquetado
  // Mostrar qty_delivered / qty_invoiced cuando la orden está confirmada (como Odoo)
  const showDelivery = isSale
  const UOMS  = ['Unidades', 'Piezas', 'Docena', 'Caja', 'kg', 'g', 'Litros', 'm']
  const PACKS = [{ label: '—', qty: 0 }, { label: 'Paquete x6', qty: 6 }, { label: 'Caja x12', qty: 12 }, { label: 'Caja x24', qty: 24 }]
  const nCols = 5 + (showDisc ? 1 : 0) + (showMargen ? 1 : 0) + (showUom ? 1 : 0) + (showPack ? 1 : 0) + (isEditable ? 1 : 0) + (showDelivery ? 2 : 0)

  if (!lineas.length) return `<div style="padding:32px;text-align:center;color:var(--text-400);font-size:13px">
    Sin líneas de pedido. Haz clic en <strong>＋ Agregar producto</strong> para comenzar.
  </div>`

  return `
  <div style="overflow-x:auto">
    <table class="o-list-table" style="margin-top:0;min-width:600px">
      <thead>
        <tr style="background:var(--bg-table-head)">
          <th style="width:200px">PRODUCTO</th>
          <th style="width:240px">DESCRIPCIÓN</th>
          <th style="width:90px;text-align:center">CANTIDAD</th>
          ${showUom ? `<th style="width:90px">UDM</th>` : ''}
          ${showPack ? `<th style="width:110px">EMPAQUE</th>` : ''}
          <th style="width:110px;text-align:right">PRECIO UNIT.</th>
          ${showDisc ? `<th style="width:80px;text-align:right">DESCUENTO</th>` : ''}
          ${showMargen ? `<th style="width:100px;text-align:right">MARGEN</th>` : ''}
          ${showDelivery ? `
          <th style="width:90px;text-align:center" title="Cantidad Entregada">ENTREGADO</th>
          <th style="width:90px;text-align:center" title="Cantidad Facturada">FACTURADO</th>` : ''}
          <th style="width:110px;text-align:right">SUBTOTAL</th>
          ${isEditable ? `<th style="width:36px"></th>` : ''}
        </tr>
      </thead>
      <tbody>
        ${lineas.map(l => {
          if (l.display_type === 'line_section') {
            return `<tr><td colspan="${nCols}" style="padding:8px 14px;font-weight:700;font-size:13px;background:var(--bg-app);color:var(--text-600);border-top:2px solid var(--border)">${l.name}</td></tr>`
          }
          const subtotal  = parseFloat(l.price_subtotal || 0)
          const costo     = parseFloat(l.cost || 0)
          const qty       = parseFloat(l.product_uom_qty) || 0
          const margen    = subtotal - qty * costo
          const qtyDel    = parseFloat(l.qty_delivered || 0)
          const qtyInv    = parseFloat(l.qty_invoiced || 0)
          const qtyToDo   = parseFloat(l.qty_to_invoice || 0)
          const uomLinea  = extras.uoms?.[l.id] || l.uom_name || l.product_uom_name || 'Unidades'
          const packLinea = extras.packs?.[l.id] || '—'
          // Color del qty_delivered: verde si entregado completo, naranja si parcial
          const delColor  = qtyDel >= qty && qty > 0 ? '#10B981' : qtyDel > 0 ? '#F59E0B' : 'var(--text-400)'
          // Color del qty_invoiced: verde si facturado completo, naranja si parcial
          const invColor  = qtyInv >= qty && qty > 0 ? '#10B981' : qtyInv > 0 ? '#F59E0B' : 'var(--text-400)'
          return `
          <tr style="border-bottom:1px solid var(--border)">
            <td style="padding:8px 14px;font-weight:600;font-size:13px">
              ${l.product_name || l.name?.split('—')[0]?.trim() || '—'}
              ${cfg.variantes && l.product_id ? `<div style="font-size:10px;color:var(--text-400);font-weight:400">Variante · ref. interna #${l.product_id}</div>` : ''}
            </td>
            <td style="padding:8px 14px;font-size:12px;color:var(--text-500)">${l.name || ''}</td>
            <td style="padding:8px 14px;text-align:center">
              ${isEditable
                ? `<input class="o-qty-input" type="number" value="${qty}" min="0" step="0.001" style="width:70px;text-align:center" onchange="window._inlineEdit(${l.order_id||orderId},${l.id},'product_uom_qty',this.value)">`
                : `<span>${qty}</span>`
              }
            </td>
            ${showUom ? `
            <td style="padding:8px 14px;font-size:12px;color:var(--text-500)">
              ${isEditable
                ? `<select class="o-field-input" style="font-size:12px;padding:4px 6px" onchange="window._setLineaUom(${l.order_id||orderId},${l.id},this.value)">
                    ${UOMS.map(u => `<option ${u === uomLinea ? 'selected' : ''}>${u}</option>`).join('')}
                  </select>`
                : `<span>${uomLinea}</span>`
              }
            </td>` : ''}
            ${showPack ? `
            <td style="padding:8px 14px;font-size:12px;color:var(--text-500)">
              ${isEditable
                ? `<select class="o-field-input" style="font-size:12px;padding:4px 6px" onchange="window._setLineaEmpaque(${l.order_id||orderId},${l.id},this.value)">
                    ${PACKS.map(p => `<option value="${p.label}|${p.qty}" ${p.label === packLinea ? 'selected' : ''}>${p.label}</option>`).join('')}
                  </select>`
                : `<span>${packLinea}</span>`
              }
            </td>` : ''}
            <td style="padding:8px 14px;text-align:right">
              ${isEditable
                ? `<input class="o-price-input" type="number" value="${parseFloat(l.price_unit)||0}" min="0" step="0.01" style="width:100px;text-align:right" onchange="window._inlineEdit(${l.order_id||orderId},${l.id},'price_unit',this.value)">`
                : `<span>${fmtMxn(l.price_unit)}</span>`
              }
            </td>
            ${showDisc ? `
            <td style="padding:8px 14px;text-align:right">
              ${isEditable
                ? `<div style="display:flex;align-items:center;justify-content:flex-end;gap:2px"><input class="o-disc-input" type="number" value="${parseFloat(l.discount)||0}" min="0" max="100" step="0.1" style="width:55px;text-align:right" onchange="window._inlineEdit(${l.order_id||orderId},${l.id},'discount',this.value)"><span style="color:var(--text-400);font-size:12px">%</span></div>`
                : `<span>${parseFloat(l.discount)||0}%</span>`
              }
            </td>` : ''}
            ${showMargen ? `
            <td style="padding:8px 14px;text-align:right;font-variant-numeric:tabular-nums;color:${margen >= 0 ? '#10B981' : '#DC2626'};font-weight:600">
              ${fmtMxn(margen)}${subtotal > 0 ? ` <span style="font-size:11px;color:var(--text-400)">(${(margen/subtotal*100).toFixed(1)}%)</span>` : ''}
            </td>` : ''}
            ${showDelivery ? `
            <td style="padding:8px 14px;text-align:center;font-size:12px;font-weight:600;color:${delColor}">
              ${qtyDel}${qty > 0 ? `<span style="font-size:10px;color:var(--text-300)">/${qty}</span>` : ''}
            </td>
            <td style="padding:8px 14px;text-align:center;font-size:12px;font-weight:600;color:${invColor}">
              ${qtyInv}${qty > 0 ? `<span style="font-size:10px;color:var(--text-300)">/${qty}</span>` : ''}
              ${qtyToDo > 0 ? `<div style="font-size:10px;color:#F59E0B">+${qtyToDo} pend.</div>` : ''}
            </td>` : ''}
            <td style="padding:8px 14px;text-align:right;font-weight:700;font-variant-numeric:tabular-nums">${fmtMxn(subtotal)}</td>
            ${isEditable ? `
            <td style="padding:4px 8px;text-align:center">
              <button onclick="window._eliminarLinea(${l.order_id||orderId},${l.id})"
                style="background:none;border:none;cursor:pointer;color:var(--text-300);font-size:18px;line-height:1;padding:2px 6px;border-radius:4px"
                onmouseover="this.style.color='#DC2626';this.style.background='#FEE2E2'"
                onmouseout="this.style.color='';this.style.background=''">×</button>
            </td>` : ''}
          </tr>`
        }).join('')}
      </tbody>
    </table>
  </div>`
}

// ─── Init form globals ───────────────────────────────────────────────────────
function _initFormGlobals(v, lineas) {
  // Cerrar el menú de acciones ⋮ al hacer clic fuera de él
  document.addEventListener('click', (e) => {
    if (!e.target.closest('.o-dropdown')) {
      document.querySelectorAll('.o-dropdown-menu.open').forEach(m => m.classList.remove('open'))
    }
  }, { capture: true, once: false, passive: true })

  window._switchTab = (tab, btn) => {
    document.querySelectorAll('#venta-tabs .o-tab').forEach(b => b.classList.remove('active'))
    btn.classList.add('active')
    document.querySelectorAll('[id^="tab-"]').forEach(p => p.style.display = 'none')
    const pane = document.getElementById(`tab-${tab}`)
    if (pane) pane.style.display = ''
  }

  window._prevRecord = () => {
    const idx = _records.findIndex(r => r.id === v.id)
    if (idx > 0) _abrirVenta(_records[idx - 1].id)
  }
  window._nextRecord = () => {
    const idx = _records.findIndex(r => r.id === v.id)
    if (idx >= 0 && idx < _records.length - 1) _abrirVenta(_records[idx + 1].id)
  }

  window._guardarCampo = async (id, campo, valor) => {
    try {
      await api.put(`/ventas/${id}`, { [campo]: valor })
    } catch (e) { toast('Error', e.message, 'error') }
  }

  window._m2oInput = async (el, endpoint) => {
    const q = el.value
    const ddId = el.id + '-dd'
    const dd = document.getElementById(ddId)
    if (!dd) return
    if (!q || q.length < 1) { dd.style.display = 'none'; return }
    clearTimeout(_inlinePickerTimer)
    _inlinePickerTimer = setTimeout(async () => {
      try {
        const res = await api.get(`/ventas/${endpoint}?q=${encodeURIComponent(q)}`)
        const items = res?.data || []
        if (!items.length) { dd.style.display = 'none'; return }
        dd.style.display = 'block'
        dd.innerHTML = items.map(it => `
          <div style="padding:8px 12px;cursor:pointer;font-size:13px;border-bottom:1px solid var(--border)"
            onmouseover="this.style.background='#EEF2FF'"
            onmouseout="this.style.background=''"
            onmousedown="window._selM2o('${el.id}',${it.id},'${(it.name||'').replace(/'/g,"\\'")}','${el.dataset.field}',${el.dataset.order})">
            <div style="font-weight:600">${it.name||''}</div>
            ${it.email ? `<div style="font-size:11px;color:var(--text-400)">${it.email}</div>` : ''}
          </div>`).join('')
      } catch(_) {}
    }, 250)
  }

  window._hideM2o = id => {
    const dd = document.getElementById(id + '-dd')
    if (dd) dd.style.display = 'none'
  }

  window._selM2o = async (inputId, id, name, field, orderId) => {
    const inp = document.getElementById(inputId)
    if (inp) inp.value = name
    const dd = document.getElementById(inputId + '-dd')
    if (dd) dd.style.display = 'none'
    try {
      await api.put(`/ventas/${orderId}`, { [field]: id })
      if (typeof _abrirVenta === 'function') {
        await _abrirVenta(orderId)
      }
    } catch(e) { toast('Error', e.message, 'error') }
  }

  window._inlineEdit = async (ordId, lineId, campo, valor) => {
    try {
      await api.put(`/ventas/${ordId}/lineas/${lineId}`, { [campo]: parseFloat(valor) })
      // Refrescar totales
      const linRes = await api.get(`/ventas/${ordId}/lineas`)
      _currentLineas = linRes?.data || []
      _actualizarTotales(_currentLineas)
    } catch (e) { toast('Error', e.message, 'error') }
  }

  window._eliminarLinea = async (ordId, lineId) => {
    try {
      await api.del(`/ventas/${ordId}/lineas/${lineId}`)
      const linRes = await api.get(`/ventas/${ordId}/lineas`)
      _currentLineas = linRes?.data || []
      const pane = document.getElementById('tab-lineas')
      if (pane) {
        pane.innerHTML = _renderTabLineas(_currentLineas, v.state, v.locked, ordId) +
          ((v.state === 'draft' || v.state === 'sent') && !v.locked ? _btnBarLineas(ordId) : '')
        _actualizarTotales(_currentLineas)
      }
      toast('Línea eliminada', '', 'success')
    } catch (e) { toast('Error', e.message, 'error') }
  }

  window._agregarProductoInline = (ordId) => _mostrarPickerInline(ordId)
  window._agregarSeccion = async (ordId) => {
    const nombre = prompt('Nombre de la sección:')
    if (!nombre) return
    try {
      await api.post(`/ventas/${ordId}/lineas`, { display_type: 'line_section', name: nombre, product_uom_qty: 0, price_unit: 0, discount: 0 })
      _recargarLineas(ordId, v.state, v.locked)
    } catch (e) { toast('Error', e.message, 'error') }
  }

  window._accionVenta = async (id, accion, body = {}) => {
    const cfg = getCfg()
    if (accion === 'confirmar' && cfg.advertencias) {
      if (!confirm('¿Confirmar este pedido de venta?\nSe generará la orden de entrega en Almacén.')) return
    }
    if (accion === 'confirmar' && cfg.aviso_stock) {
      // Advertencia de stock real: consulta la disponibilidad de cada producto en Almacén
      const productLines = _currentLineas.filter(l => l.display_type !== 'line_section' && l.product_id)
      const faltantes = []
      for (const l of productLines) {
        try {
          const r = await api.get(`/stock/producto/${l.product_id}`)
          const disponible = (r?.data || []).reduce((s, x) => s + parseFloat(x.cantidad_disponible || 0), 0)
          if (parseFloat(l.product_uom_qty) > disponible) {
            faltantes.push(`• ${l.product_name || l.name}: pedido ${parseFloat(l.product_uom_qty)}, disponible ${disponible}`)
          }
        } catch (_) {}
      }
      if (faltantes.length && !confirm(`⚠️ Stock insuficiente:\n${faltantes.join('\n')}\n\n¿Confirmar de todos modos?`)) return
    }
    if (accion === 'cancelar' && !confirm('¿Cancelar esta orden?')) return
    try {
      const res = await api.put(`/ventas/${id}/${accion}`, body)
      if (res?.data?.ok || res?.success) {
        // Config: bloquear pedido al confirmarlo (como Odoo "Lock Confirmed Sales")
        if (accion === 'confirmar' && cfg.bloquear_confirmado) {
          try { await api.put(`/ventas/${id}/bloquear`, { locked: true }) } catch (_) {}
        }
        // Log automático en el chatter (igual que el historial de Odoo)
        const logMsg = {
          confirmar: '✅ Pedido confirmado. Se generó la orden de entrega en Almacén.',
          enviar:    '📧 Cotización marcada como enviada al cliente.',
          cancelar:  '✕ Orden cancelada.',
          bloquear:  body.locked ? '🔒 Pedido bloqueado contra edición.' : '🔓 Pedido desbloqueado para edición.',
        }[accion]
        if (logMsg) {
          const key = `nexus_chatter_${id}`
          const msgs = JSON.parse(localStorage.getItem(key) || '[]')
          msgs.unshift({ texto: logMsg, fecha: new Date().toISOString(), usuario: 'Sistema' })
          localStorage.setItem(key, JSON.stringify(msgs.slice(0, 50)))
        }
        toast('Éxito', _accionMsg(accion), 'success')
        await _abrirVenta(id)
      }
    } catch (e) { toast('Error', e.message, 'error') }
  }

  window._restaurarBorrador = async (id) => {
    try {
      await api.put(`/ventas/${id}/borrador`, {})
      toast('Restaurado', 'Orden restaurada a borrador', 'success')
      await _abrirVenta(id)
    } catch (e) { toast('Error', e.message, 'error') }
  }

  window._abrirModalFactura = (id) => {
    const m = document.getElementById('modal-factura')
    if (m) m.style.display = 'flex'
  }

  window._onTipoFacturaChange = (tipo, totalOrder) => {
    document.getElementById('factura-pct-row').style.display   = tipo === 'percentage' ? 'block' : 'none'
    document.getElementById('factura-fixed-row').style.display = tipo === 'fixed'      ? 'block' : 'none'
  }

  window._calcAnticipo = (mode, val, totalOrder) => {
    const t = parseFloat(totalOrder) || 0
    if (mode === 'pct') {
      const pct = Math.min(100, Math.max(0, parseFloat(val) || 0))
      const monto = (t * pct / 100)
      const el = document.getElementById('factura-pct-monto')
      if (el) el.textContent = fmtMxn(monto)
    } else {
      const monto = Math.min(t, Math.max(0, parseFloat(val) || 0))
      const pct = t > 0 ? (monto / t * 100).toFixed(1) : '0'
      const el = document.getElementById('factura-fixed-pct')
      if (el) el.textContent = `(${pct}%)`
    }
  }

  window._ejecutarCrearFactura = async (id) => {
    const tipo = document.getElementById('tipo-factura')?.value || 'delivered'
    const payload = { advance_payment_method: tipo }
    if (tipo === 'percentage') {
      const pct = parseFloat(document.getElementById('factura-pct')?.value || '30')
      if (isNaN(pct) || pct <= 0 || pct > 100) {
        toast('Error', 'El porcentaje debe estar entre 1 y 100', 'error'); return
      }
      payload.amount = pct
    } else if (tipo === 'fixed') {
      const monto = parseFloat(document.getElementById('factura-fixed')?.value || '0')
      if (isNaN(monto) || monto <= 0) {
        toast('Error', 'El monto del anticipo debe ser mayor a 0', 'error'); return
      }
      payload.fixed_amount = monto
    }
    try {
      const res = await api.post(`/ventas/${id}/crear-factura`, payload)
      const factId = res?.data?.factura_id
      document.getElementById('modal-factura').style.display = 'none'
      toast('Factura creada', res?.data?.factura_name || 'Factura generada exitosamente', 'success')
      if (factId) {
        setTimeout(() => { window._go(`facturas?id=${factId}`) }, 800)
      } else {
        await _abrirVenta(id)
      }
    } catch (e) { toast('Error creando factura', e.message, 'error') }
  }

  // ── Navegación entre módulos ──────────────────────────────────────────────
  window._verFacturas = async (id) => {
    try {
      const res = await api.get(`/ventas/${id}/facturas`)
      const facts = res?.data || []
      if (facts.length === 1) {
        // Navegar directo a esa factura
        window._go(`facturas?id=${facts[0].id}`)
      } else if (facts.length > 1) {
        // Mostrar selector de facturas
        const modal = document.createElement('div')
        modal.style.cssText = 'position:fixed;inset:0;z-index:960;background:rgba(0,0,0,.45);display:flex;align-items:center;justify-content:center;padding:16px'
        modal.innerHTML = `
          <div style="background:var(--bg-card);border-radius:14px;box-shadow:0 24px 64px rgba(0,0,0,.22);border:1px solid var(--border);width:100%;max-width:480px">
            <div style="padding:16px 20px;border-bottom:1px solid var(--border);display:flex;justify-content:space-between;align-items:center">
              <h3 style="margin:0;font-size:15px;font-weight:700">Facturas vinculadas</h3>
              <button onclick="this.closest('[style*=fixed]').remove()" style="background:none;border:none;cursor:pointer;font-size:18px">×</button>
            </div>
            <div style="padding:12px">
              ${facts.map(f => `
              <div onclick="window._go('facturas?id='+f.id)" style="padding:10px 14px;border:1px solid var(--border);border-radius:8px;cursor:pointer;margin-bottom:8px;display:flex;justify-content:space-between;align-items:center"
                onmouseover="this.style.background='#EEF2FF'" onmouseout="this.style.background=''">
                <div>
                  <div style="font-weight:700;font-family:monospace">${f.name || '#'+f.id}</div>
                  <div style="font-size:11px;color:var(--text-400)">${f.invoice_date || ''}</div>
                </div>
                <div style="text-align:right">
                  <div style="font-weight:700;color:var(--primary)">${f.amount_total ? '$'+parseFloat(f.amount_total).toFixed(2) : '—'}</div>
                  <span class="o-badge ${f.payment_state==='paid'?'o-badge-success':f.state==='posted'?'o-badge-info':'o-badge-gray'}" style="font-size:10px">${f.state==='posted'?'Publicada':f.state==='draft'?'Borrador':'—'}</span>
                </div>
              </div>`).join('')}
            </div>
          </div>`
        document.body.appendChild(modal)
        modal.onclick = e => { if (e.target === modal) modal.remove() }
      } else {
        // Sin facturas: ir al módulo de facturas con referencia al pedido
        window._go(`facturas?orden=${id}`)
      }
    } catch(e) { window._go(`facturas?orden=${id}`) }
  }

  // ── Entrega → navega al módulo Almacén (igual que Odoo) ──────────────────
  window._verEntrega = async (id) => {
    try {
      const res = await api.get(`/ventas/${id}/picking`)
      const picking = res?.data
      if (picking && picking.picking_id) {
        // Navegar al módulo Almacén con el picking específico
        window._go(`stock?picking=${picking.picking_id}&origen=${id}`)
      } else {
        // No hay picking — puede ser un pedido de servicio puro
        window._go(`stock?orden=${id}`)
      }
    } catch(e) {
      window._go('stock')
    }
  }

  window._verLineas   = () => window._switchTab('lineas', document.querySelector('[data-tab="lineas"]'))

  // ── Imprimir PDF real (usando CSS de impresión) ───────────────────────────
  window._imprimirCotizacion = window._imprimirPedido = (ordId) => {
    const v = _currentOrder
    if (!v) return
    const lineas = _currentLineas.filter(l => l.display_type !== 'line_section')
    const subtotal = lineas.reduce((s, l) => s + parseFloat(l.price_subtotal || 0), 0)
    const iva = subtotal * 0.16
    const total = subtotal + iva
    const printWin = window.open('', '_blank', 'width=800,height=900')
    if (!printWin) { toast('Error', 'Habilita ventanas emergentes para imprimir', 'error'); return }
    printWin.document.write(`
<!DOCTYPE html><html><head><meta charset="UTF-8">
<title>${v.name || 'Cotización'} - NexusTech ERP</title>
<style>
  body { font-family: Arial, sans-serif; font-size: 12px; color: #111; margin: 30px; }
  h1 { font-size: 22px; margin: 0; } h3 { font-size: 13px; margin: 0; color: #555; }
  .header { display: flex; justify-content: space-between; margin-bottom: 24px; border-bottom: 2px solid #6366F1; padding-bottom: 16px; }
  .badge { display: inline-block; padding: 3px 10px; border-radius: 12px; font-size: 11px; font-weight: bold;
    background: ${ v.state === 'sale' ? '#D1FAE5' : v.state === 'done' ? '#EDE9FE' : '#F3F4F6' };
    color: ${ v.state === 'sale' ? '#065F46' : v.state === 'done' ? '#5B21B6' : '#374151' }; }
  .grid { display: grid; grid-template-columns: 1fr 1fr; gap: 0 40px; margin-bottom: 24px; }
  .field { display: grid; grid-template-columns: 130px 1fr; padding: 4px 0; border-bottom: 1px solid #F3F4F6; }
  .field label { color: #6B7280; font-size: 11px; }
  table { width: 100%; border-collapse: collapse; margin-bottom: 20px; font-size: 12px; }
  th { background: #F9FAFB; padding: 8px; text-align: left; border-bottom: 2px solid #E5E7EB; font-size: 11px; color: #6B7280; text-transform: uppercase; }
  td { padding: 8px; border-bottom: 1px solid #F3F4F6; }
  td.right, th.right { text-align: right; }
  .totals { display: flex; justify-content: flex-end; }
  .totals table { width: 260px; }
  .total-row td { font-size: 14px; font-weight: bold; border-top: 2px solid #E5E7EB; color: #6366F1; }
  .footer { margin-top: 30px; font-size: 11px; color: #6B7280; border-top: 1px solid #E5E7EB; padding-top: 12px; }
  @media print { body { margin: 15px; } }
</style></head><body>
<div class="header">
  <div>
    <h1>NexusTech ERP</h1>
    <h3>${ v.name || 'Cotización' } &nbsp; <span class="badge">${ STATE_LABEL[v.state] || v.state }</span></h3>
  </div>
  <div style="text-align:right">
    <div style="font-size:11px;color:#6B7280">Fecha: <strong>${ fmtDate(v.date_order) }</strong></div>
    ${ v.validity_date ? `<div style="font-size:11px;color:#6B7280">Válido hasta: <strong>${ fmtDate(v.validity_date) }</strong></div>` : '' }
  </div>
</div>
<div class="grid">
  <div>
    <div class="field"><label>Cliente</label><span>${ v.partner_name || '—' }</span></div>
    ${ v.partner_invoice_name ? `<div class="field"><label>Dir. Facturación</label><span>${ v.partner_invoice_name }</span></div>` : '' }
    ${ v.client_order_ref ? `<div class="field"><label>Ref. Cliente</label><span>${ v.client_order_ref }</span></div>` : '' }
  </div>
  <div>
    <div class="field"><label>Vendedor</label><span>${ v.user_name || '—' }</span></div>
    <div class="field"><label>Plazo de Pago</label><span>${ v.payment_term_name || '—' }</span></div>
    <div class="field"><label>Moneda</label><span>${ v.currency_name || 'MXN' }</span></div>
  </div>
</div>
<table>
  <thead><tr>
    <th>Producto</th><th>Descripción</th>
    <th class="right">Cantidad</th><th class="right">Precio Unit.</th>
    <th class="right">Desc.</th><th class="right">Subtotal</th>
  </tr></thead>
  <tbody>
    ${ _currentLineas.map(l => {
      if (l.display_type === 'line_section') return `<tr><td colspan="6" style="font-weight:bold;background:#F9FAFB;padding:6px 8px;">${l.name}</td></tr>`
      return `<tr>
        <td>${l.product_name || l.name || '—'}</td>
        <td style="color:#6B7280">${l.name || ''}</td>
        <td class="right">${parseFloat(l.product_uom_qty)||0}</td>
        <td class="right">$${parseFloat(l.price_unit||0).toFixed(2)}</td>
        <td class="right">${parseFloat(l.discount||0)}%</td>
        <td class="right" style="font-weight:600">$${parseFloat(l.price_subtotal||0).toFixed(2)}</td>
      </tr>`
    }).join('') }
  </tbody>
</table>
<div class="totals">
  <table>
    <tr><td>Subtotal</td><td class="right">$${ subtotal.toFixed(2) }</td></tr>
    <tr><td>IVA (16%)</td><td class="right">$${ iva.toFixed(2) }</td></tr>
    <tr class="total-row"><td>TOTAL MXN</td><td class="right">$${ total.toFixed(2) }</td></tr>
  </table>
</div>
${ v.note ? `<div class="footer"><strong>Términos y condiciones:</strong><br>${v.note}</div>` : '' }
</body></html>`)
    printWin.document.close()
    setTimeout(() => { printWin.focus(); printWin.print() }, 400)
  }

  // ── Duplicar orden ──────────────────────────────────────────────────────
  window._duplicarVenta = async (id) => {
    // Cerrar el menú de acciones
    document.getElementById(`dd-acciones-${id}`)?.classList.remove('open')
    try {
      const res = await api.post(`/ventas/${id}/duplicar`, {})
      const nuevaId = res?.data?.id
      const nuevaNombre = res?.data?.name || 'Nueva cotización'
      toast('Duplicado', `${nuevaNombre} creada como borrador`, 'success')
      if (nuevaId) setTimeout(() => _abrirVenta(nuevaId), 600)
    } catch (e) { toast('Error', e.message, 'error') }
  }

  window._enviarMensaje = window._agregarNota = async (id) => {
    const texto = prompt('Escribe tu nota o mensaje:')
    if (!texto?.trim()) return
    // Guardar en chatter local (localStorage por orden hasta que el backend tenga tabla de mensajes)
    const key = `nexus_chatter_${id}`
    const msgs = JSON.parse(localStorage.getItem(key) || '[]')
    msgs.unshift({ texto, fecha: new Date().toISOString(), usuario: 'Administrador' })
    localStorage.setItem(key, JSON.stringify(msgs.slice(0, 50)))
    _renderChatter(id)
    toast('Nota agregada', '', 'success')
  }

  // ── Renderizar chatter con mensajes locales ───────────────────────────────
  function _renderChatter(id) {
    const el = document.getElementById(`chatter-${id}`)
    if (!el) return
    const key = `nexus_chatter_${id}`
    const msgs = JSON.parse(localStorage.getItem(key) || '[]')
    if (!msgs.length) { el.innerHTML = '<div style="font-size:13px;color:var(--text-400)">Sin actividad registrada.</div>'; return }
    el.innerHTML = msgs.map(m => `
      <div style="display:flex;gap:10px;margin-bottom:12px;padding-bottom:12px;border-bottom:1px solid var(--border)">
        <div class="o-avatar o-avatar-sm" style="background:#6366F1;flex-shrink:0">${(m.usuario||'A')[0]}</div>
        <div style="flex:1">
          <div style="font-size:12px;font-weight:700;color:var(--text-700)">${m.usuario || 'Sistema'} <span style="font-weight:400;color:var(--text-400)">${fmtDate(m.fecha)}</span></div>
          <div style="font-size:13px;color:var(--text-600);margin-top:3px">${m.texto}</div>
        </div>
      </div>
    `).join('')
  }
  // Inicializar el chatter al abrir el formulario
  _renderChatter(v.id)
}

function _accionMsg(accion) {
  return { confirmar: 'Pedido confirmado', enviar: 'Cotización enviada', cancelar: 'Orden cancelada', bloquear: 'Estado actualizado' }[accion] || 'Acción completada'
}

// ─── Inline product picker ───────────────────────────────────────────────────
function _mostrarPickerInline(ordId) {
  // Si ya hay una fila de búsqueda activa, solo enfocarla
  const existing = document.getElementById('inline-picker-row')
  if (existing) { existing.querySelector('input')?.focus(); return }

  // Columnas según configuración del módulo (el picker solo aparece en modo editable,
  // así que la columna de acciones siempre está presente)
  const cfg = getCfg()
  const showDisc   = cfg.descuentos !== false || !!(cfg.listas_precios && cfg.descuento_precio)
  const showMargen = !!cfg.margenes
  const showUom    = !!cfg.unidades_medida
  const showPack   = !!cfg.empaquetado
  const nCols = 5 + (showDisc ? 1 : 0) + (showMargen ? 1 : 0) + (showUom ? 1 : 0) + (showPack ? 1 : 0) + 1

  let tbody = document.querySelector('#tab-lineas table tbody')

  if (!tbody) {
    // No hay tabla aún (sin líneas): crear la tabla con thead vacío para alojar el picker
    const pane = document.getElementById('tab-lineas')
    if (!pane) return

    // Reemplazar el mensaje "Sin líneas" con una tabla mínima
    // Sin overflow-x:auto para no recortar el dropdown
    const wrapper = document.createElement('div')
    wrapper.innerHTML = `
      <table class="o-list-table" style="margin-top:0;min-width:600px;width:100%">
        <thead>
          <tr style="background:var(--bg-table-head)">
            <th style="width:200px">PRODUCTO</th>
            <th style="width:240px">DESCRIPCIÓN</th>
            <th style="width:90px;text-align:center">CANTIDAD</th>
            ${showUom ? `<th style="width:90px">UDM</th>` : ''}
            ${showPack ? `<th style="width:110px">EMPAQUE</th>` : ''}
            <th style="width:110px;text-align:right">PRECIO UNIT.</th>
            ${showDisc ? `<th style="width:80px;text-align:right">DESCUENTO</th>` : ''}
            ${showMargen ? `<th style="width:100px;text-align:right">MARGEN</th>` : ''}
            <th style="width:110px;text-align:right">SUBTOTAL</th>
            <th style="width:36px"></th>
          </tr>
        </thead>
        <tbody id="lines-tbody-dynamic"></tbody>
      </table>`
    pane.innerHTML = ''
    pane.appendChild(wrapper)
    tbody = document.getElementById('lines-tbody-dynamic')

    // Volver a añadir botones debajo
    const btnBarWrap = document.createElement('div')
    btnBarWrap.innerHTML = _btnBarLineas(ordId)
    pane.appendChild(btnBarWrap.firstElementChild)
  }

  const tr = document.createElement('tr')
  tr.id = 'inline-picker-row'
  tr.style.cssText = 'background:#EEF2FF;border-bottom:1px solid var(--primary)'
  tr.innerHTML = `
    <td colspan="${nCols - 1}" style="padding:8px 14px">
      <input id="inline-product-input" class="o-field-input" placeholder="🔍 Buscar producto por nombre o código..."
        style="width:100%;font-size:13px" autocomplete="off"
        oninput="window._buscarProductoInline(this.value,${ordId})"
        onkeydown="if(event.key==='Escape'){window._cancelarPickerInline()}">
    </td>
    <td style="padding:8px;text-align:center">
      <button onclick="window._cancelarPickerInline()" style="background:none;border:none;cursor:pointer;color:var(--text-400);font-size:18px">×</button>
    </td>`
  tbody.appendChild(tr)

  // Dropdown anclado al body con position:fixed para nunca ser recortado por overflow
  let dd = document.getElementById('inline-product-dd')
  if (!dd) {
    dd = document.createElement('div')
    dd.id = 'inline-product-dd'
    dd.style.cssText = 'display:none;position:fixed;background:var(--bg-card);border:1.5px solid var(--primary);border-radius:8px;box-shadow:0 8px 24px rgba(0,0,0,.18);z-index:9999;max-height:280px;overflow-y:auto;min-width:400px'
    document.body.appendChild(dd)
  }

  const inp = document.getElementById('inline-product-input')
  if (inp) {
    inp.focus()
    // Posicionar el dropdown usando getBoundingClientRect (inmune a overflow)
    const _positionDD = () => {
      const rect = inp.getBoundingClientRect()
      dd.style.top  = (rect.bottom + 4) + 'px'
      dd.style.left = rect.left + 'px'
      dd.style.width = rect.width + 'px'
    }
    inp.addEventListener('focus', _positionDD)
    inp.addEventListener('input', _positionDD)
    _positionDD()
  }
}

window._cancelarPickerInline = () => {
  document.getElementById('inline-picker-row')?.remove()
  // Limpiar el dropdown del body
  const dd = document.getElementById('inline-product-dd')
  if (dd) { dd.style.display = 'none'; dd.innerHTML = '' }
}

window._buscarProductoInline = async (q, ordId) => {
  const dd = document.getElementById('inline-product-dd')
  if (!dd) return
  if (!q || q.length < 1) { dd.style.display = 'none'; return }
  clearTimeout(_inlinePickerTimer)
  _inlinePickerTimer = setTimeout(async () => {
    try {
      const res = await api.get(`/ventas/buscar-productos?q=${encodeURIComponent(q)}`)
      const items = res?.data || []
      dd.style.display = 'block'
      if (!items.length) {
        dd.innerHTML = `<div style="padding:12px;font-size:13px;color:var(--text-400);text-align:center">Sin resultados para "${q}"</div>`
        return
      }
      dd.innerHTML = items.map(p => `
        <div style="padding:10px 14px;cursor:pointer;border-bottom:1px solid var(--border);display:flex;justify-content:space-between;align-items:center"
          onmouseover="this.style.background='#EEF2FF'"
          onmouseout="this.style.background=''"
          onclick="window._selProductoInline(${ordId},${p.id},'${(p.name||'').replace(/'/g,"\\'")}',${parseFloat(p.precio||0)})">
          <div>
            <div style="font-weight:600;font-size:13px">${p.name||''}</div>
            ${p.codigo ? `<div style="font-size:11px;color:var(--text-400)">${p.codigo}</div>` : ''}
          </div>
          <span style="font-weight:700;color:var(--primary);font-size:13px">${fmtMxn(p.precio||0)}</span>
        </div>`).join('')
    } catch(_) {}
  }, 200)
}

window._selProductoInline = async (ordId, productId, productName, precio) => {
  // Limpiar picker row Y dropdown del body ANTES de cualquier otra cosa
  document.getElementById('inline-picker-row')?.remove()
  const dd = document.getElementById('inline-product-dd')
  if (dd) { dd.style.display = 'none'; dd.innerHTML = ''; dd.remove() }

  // Lista de precios aplicada a la orden: el descuento de la lista entra en la línea nueva
  const cfg = getCfg()
  let descuentoLista = 0
  if (cfg.listas_precios) {
    const pl = getPricelists().find(p => p.id === getExtras(ordId).pricelist_id)
    if (pl && pl.type === 'Porcentaje') descuentoLista = parseFloat(pl.discount) || 0
  }

  try {
    await api.post(`/ventas/${ordId}/lineas`, {
      product_id: productId,
      product_uom_qty: 1,
      price_unit: precio,
      discount: descuentoLista,
    })
    await _recargarLineas(ordId, _currentOrder?.state, _currentOrder?.locked)
    toast('Producto agregado', descuentoLista ? `${productName} (−${descuentoLista}% por lista de precios)` : productName, 'success')
  } catch (e) { toast('Error', e.message, 'error') }
}

// ─── Handlers de configuración: lista de precios, plantillas, envío, firma, pago ──

// Aplica una lista de precios a la orden: persiste la selección y fija el descuento
// de la lista en todas las líneas de producto (el backend recalcula los subtotales)
window._aplicarListaPrecios = async (ordId, plIdRaw) => {
  const plId = parseInt(plIdRaw) || null
  const pl = getPricelists().find(p => p.id === plId)
  setExtras(ordId, { pricelist_id: pl ? plId : null })

  const descuento = pl && pl.type === 'Porcentaje' ? (parseFloat(pl.discount) || 0) : 0
  const productLines = _currentLineas.filter(l => l.display_type !== 'line_section' && !/^env[ií]o/i.test(l.name || ''))
  try {
    for (const l of productLines) {
      await api.put(`/ventas/${ordId}/lineas/${l.id}`, { discount: descuento })
    }
    await _recargarLineas(ordId, _currentOrder?.state, _currentOrder?.locked)
    toast('Lista de precios',
      pl ? `"${pl.name}" aplicada${descuento ? ` — descuento del ${descuento}% en ${productLines.length} línea(s)` : ''}`
         : 'Tarifa pública aplicada (sin descuento)', 'success')
  } catch (e) { toast('Error', e.message, 'error') }
}

// Inserta las líneas predefinidas de una plantilla de presupuesto
window._aplicarPlantilla = async (ordId, key) => {
  const t = QUOTE_TEMPLATES[key]
  if (!t) return
  try {
    for (const ln of t.lineas) {
      await api.post(`/ventas/${ordId}/lineas`, ln)
    }
    setExtras(ordId, { plantilla: key })
    await _recargarLineas(ordId, _currentOrder?.state, _currentOrder?.locked)
    toast('Plantilla aplicada', `${t.label} — ${t.lineas.length} línea(s) agregada(s)`, 'success')
  } catch (e) { toast('Error', e.message, 'error') }
}

// Agrega (o reemplaza) la línea de costo de envío de la orden
window._agregarEnvio = (ordId) => {
  const m = document.createElement('div')
  m.style.cssText = 'position:fixed;inset:0;z-index:960;background:rgba(0,0,0,.45);display:flex;align-items:center;justify-content:center;padding:16px'
  m.innerHTML = `
    <div style="background:var(--bg-card);border-radius:14px;border:1px solid var(--border);width:100%;max-width:420px;box-shadow:0 24px 64px rgba(0,0,0,.22)">
      <div style="padding:16px 20px;border-bottom:1px solid var(--border);display:flex;justify-content:space-between;align-items:center">
        <h3 style="margin:0;font-size:15px;font-weight:700">🚚 Costo de Envío</h3>
        <button onclick="this.closest('[style*=fixed]').remove()" style="background:none;border:none;cursor:pointer;font-size:18px;color:var(--text-400)">×</button>
      </div>
      <div style="padding:20px;display:flex;flex-direction:column;gap:14px">
        <div>
          <label style="font-size:12px;font-weight:700;color:var(--text-600);display:block;margin-bottom:6px">MÉTODO DE ENTREGA</label>
          <select id="envio-metodo" class="o-field-input" onchange="document.getElementById('envio-costo').value=this.selectedOptions[0].dataset.costo">
            ${SHIPPING_METHODS.map((s, i) => `<option value="${s.label}" data-costo="${s.costo}" ${i === 0 ? 'selected' : ''}>${s.label} — ${fmtMxn(s.costo)}</option>`).join('')}
          </select>
        </div>
        <div>
          <label style="font-size:12px;font-weight:700;color:var(--text-600);display:block;margin-bottom:6px">COSTO (MXN)</label>
          <input id="envio-costo" type="number" min="0" step="0.01" class="o-field-input" value="${SHIPPING_METHODS[0].costo}">
        </div>
      </div>
      <div style="display:flex;gap:8px;justify-content:flex-end;padding:12px 20px;border-top:1px solid var(--border);background:var(--bg-app);border-radius:0 0 14px 14px">
        <button class="o-btn-secondary" onclick="this.closest('[style*=fixed]').remove()">Cancelar</button>
        <button class="o-btn-primary" onclick="window._confirmarEnvio(${ordId})">✓ Agregar a la orden</button>
      </div>
    </div>`
  document.body.appendChild(m)
  m.onclick = e => { if (e.target === m) m.remove() }

  window._confirmarEnvio = async (oid) => {
    const metodo = document.getElementById('envio-metodo')?.value || 'Entrega estándar'
    const costo  = parseFloat(document.getElementById('envio-costo')?.value) || 0
    m.remove()
    try {
      // Reemplazar línea de envío previa para no duplicar cargos
      const previas = _currentLineas.filter(l => l.display_type !== 'line_section' && /^env[ií]o/i.test(l.name || ''))
      for (const l of previas) { await api.del(`/ventas/${oid}/lineas/${l.id}`) }
      await api.post(`/ventas/${oid}/lineas`, {
        name: `Envío — ${metodo}`,
        product_uom_qty: 1,
        price_unit: costo,
        discount: 0,
      })
      await _abrirVenta(oid)
      toast('Envío agregado', `${metodo}: ${fmtMxn(costo)}`, 'success')
    } catch (e) { toast('Error', e.message, 'error') }
  }
}

// Firma digital de la cotización (config: firma_online) — como en el portal de Odoo,
// la firma acepta la cotización y ofrece confirmarla como pedido
window._firmarCotizacion = async (ordId) => {
  const nombre = document.getElementById('firma-nombre')?.value?.trim()
  if (!nombre) { toast('Firma', 'Escribe el nombre completo del firmante', 'error'); return }
  setExtras(ordId, { firma: { name: nombre, fecha: new Date().toISOString() } })
  try { await api.put(`/ventas/${ordId}`, { signature_name: nombre }) } catch (_) {}
  toast('Cotización firmada', `Firmada por ${nombre}`, 'success')
  if (confirm(`✍️ ${nombre} firmó la cotización.\n¿Confirmar el pedido de venta ahora?`)) {
    window._accionVenta(ordId, 'confirmar')
  } else {
    await _abrirVenta(ordId)
  }
}

// Registro de pago en línea (config: pago_online) — al cubrir el total ofrece confirmar
window._registrarPagoOnline = async (ordId, metodo) => {
  const extras = getExtras(ordId)
  const pagado = (extras.pagos || []).reduce((s, p) => s + parseFloat(p.monto || 0), 0)
  const subtotal = _currentLineas.reduce((s, l) => s + parseFloat(l.price_subtotal || 0), 0)
  const total = subtotal * 1.16
  const pendiente = Math.max(0, total - pagado)
  const montoRaw = prompt(`💳 Pago con ${metodo}\nPendiente: ${fmtMxn(pendiente)}\n\nMonto a pagar:`, pendiente.toFixed(2))
  if (montoRaw === null) return
  const monto = parseFloat(montoRaw)
  if (!(monto > 0)) { toast('Pago', 'Monto inválido', 'error'); return }
  const pagos = [...(extras.pagos || []), { metodo, monto, fecha: new Date().toISOString() }]
  setExtras(ordId, { pagos })
  toast('Pago registrado', `${metodo}: ${fmtMxn(monto)}`, 'success')
  const estado = _currentOrder?.state
  if (pagado + monto >= total - 0.01 && (estado === 'draft' || estado === 'sent')) {
    if (confirm('✅ El pago cubre el total de la cotización.\n¿Confirmar el pedido de venta ahora?')) {
      window._accionVenta(ordId, 'confirmar')
      return
    }
  }
  await _abrirVenta(ordId)
}

// Vista previa del cliente (config: compra_online) — réplica del portal de Odoo
window._vistaPreviaCliente = (ordId) => {
  const v = _currentOrder
  const cfg = getCfg()
  if (!v || v.id !== ordId) return
  const lineas = _currentLineas.filter(l => l.display_type !== 'line_section')
  const subtotal = lineas.reduce((s, l) => s + parseFloat(l.price_subtotal || 0), 0)
  const total = subtotal * 1.16
  const m = document.createElement('div')
  m.style.cssText = 'position:fixed;inset:0;z-index:970;background:rgba(0,0,0,.55);display:flex;align-items:center;justify-content:center;padding:16px;overflow-y:auto'
  m.innerHTML = `
    <div style="background:#fff;border-radius:14px;width:100%;max-width:680px;max-height:90vh;overflow-y:auto;box-shadow:0 24px 64px rgba(0,0,0,.3)">
      <div style="padding:14px 24px;background:#1F2937;color:#fff;border-radius:14px 14px 0 0;display:flex;justify-content:space-between;align-items:center">
        <span style="font-size:13px;font-weight:700">👁 Vista previa del portal del cliente</span>
        <button onclick="this.closest('[style*=fixed]').remove()" style="background:none;border:none;cursor:pointer;font-size:20px;color:#fff">×</button>
      </div>
      <div style="padding:28px 32px">
        <div style="display:flex;justify-content:space-between;align-items:flex-start;margin-bottom:20px">
          <div>
            <h2 style="margin:0 0 4px;font-size:20px;font-weight:800;color:#111">Cotización ${v.name || ''}</h2>
            <div style="font-size:13px;color:#6B7280">${v.partner_name || ''} · ${fmtDate(v.date_order)}</div>
          </div>
          <span class="o-badge ${STATE_BADGE[v.state] || 'o-badge-gray'}">${STATE_LABEL[v.state] || v.state}</span>
        </div>
        <table style="width:100%;border-collapse:collapse;font-size:13px;margin-bottom:16px">
          <thead>
            <tr style="border-bottom:2px solid #E5E7EB;text-align:left">
              <th style="padding:8px 4px">Concepto</th>
              <th style="padding:8px 4px;text-align:center">Cant.</th>
              <th style="padding:8px 4px;text-align:right">Precio</th>
              <th style="padding:8px 4px;text-align:right">Subtotal</th>
            </tr>
          </thead>
          <tbody>
            ${lineas.map(l => `
            <tr style="border-bottom:1px solid #F3F4F6">
              <td style="padding:8px 4px">${l.product_name || l.name || ''}</td>
              <td style="padding:8px 4px;text-align:center">${parseFloat(l.product_uom_qty) || 0}</td>
              <td style="padding:8px 4px;text-align:right">${fmtMxn(l.price_unit)}${parseFloat(l.discount) ? ` <span style="color:#10B981;font-size:11px">(−${parseFloat(l.discount)}%)</span>` : ''}</td>
              <td style="padding:8px 4px;text-align:right;font-weight:600">${fmtMxn(l.price_subtotal)}</td>
            </tr>`).join('')}
          </tbody>
        </table>
        <div style="display:flex;justify-content:flex-end;margin-bottom:20px">
          <table style="font-size:13px;min-width:220px">
            <tr><td style="padding:3px 16px 3px 0;color:#6B7280">Subtotal</td><td style="text-align:right;font-weight:600">${fmtMxn(subtotal)}</td></tr>
            <tr><td style="padding:3px 16px 3px 0;color:#6B7280">IVA (16%)</td><td style="text-align:right;font-weight:600">${fmtMxn(subtotal * 0.16)}</td></tr>
            <tr style="border-top:2px solid #E5E7EB"><td style="padding:6px 16px 0 0;font-weight:800">TOTAL</td><td style="text-align:right;font-weight:800;color:#6366F1">${fmtMxn(total)}</td></tr>
          </table>
        </div>
        ${v.note || cfg.terminos ? `<div style="font-size:12px;color:#6B7280;border-top:1px solid #E5E7EB;padding-top:12px;white-space:pre-wrap;margin-bottom:16px">${v.note || cfg.terminos}</div>` : ''}
        ${(v.state === 'draft' || v.state === 'sent') ? `
        <div style="display:flex;gap:8px;justify-content:center;border-top:1px solid #E5E7EB;padding-top:16px">
          ${cfg.firma_online ? `<button class="o-btn-primary" onclick="this.closest('[style*=fixed]').remove();window._switchTab('notas',document.querySelector('[data-tab=notas]'));document.getElementById('firma-nombre')?.focus()">✍️ Firmar cotización</button>` : ''}
          ${cfg.pago_online ? `<button class="o-btn-secondary" onclick="this.closest('[style*=fixed]').remove();window._registrarPagoOnline(${ordId},'Tarjeta')">💳 Pagar en línea</button>` : ''}
        </div>` : ''}
      </div>
    </div>`
  document.body.appendChild(m)
  m.onclick = e => { if (e.target === m) m.remove() }
}

// UdM por línea (config: unidades_medida) — se persiste por línea
window._setLineaUom = (ordId, lineId, uom) => {
  const extras = getExtras(ordId)
  setExtras(ordId, { uoms: { ...(extras.uoms || {}), [lineId]: uom } })
  toast('Unidad de medida', uom, 'success')
}

// Empaque por línea (config: empaquetado) — fija la cantidad al múltiplo del paquete
window._setLineaEmpaque = async (ordId, lineId, valor) => {
  const [label, qtyRaw] = (valor || '').split('|')
  const qty = parseFloat(qtyRaw) || 0
  const extras = getExtras(ordId)
  setExtras(ordId, { packs: { ...(extras.packs || {}), [lineId]: label } })
  if (qty > 0) {
    try {
      await api.put(`/ventas/${ordId}/lineas/${lineId}`, { product_uom_qty: qty })
      await _recargarLineas(ordId, _currentOrder?.state, _currentOrder?.locked)
      toast('Empaque aplicado', `${label} → cantidad ${qty}`, 'success')
    } catch (e) { toast('Error', e.message, 'error') }
  }
}


async function _mostrarPickerModal(ordId) {
  // Este fallback ya no debería ejecutarse — _mostrarPickerInline siempre crea la tabla
  // Se mantiene por compatibilidad
  toast('Agrega un producto usando el botón ＋ Agregar producto', '', 'info')
}

async function _recargarLineas(ordId, state, locked) {
  const linRes = await api.get(`/ventas/${ordId}/lineas`)
  _currentLineas = linRes?.data || []
  const pane = document.getElementById('tab-lineas')
  if (!pane) return
  const isEditable = (state === 'draft' || state === 'sent') && !locked
  pane.innerHTML = _renderTabLineas(_currentLineas, state, locked, ordId) +
    (isEditable ? _btnBarLineas(ordId) : '')
  _actualizarTotales(_currentLineas)
}

function _renderTotalesHtml(lineas) {
  const subtotal = lineas.reduce((s, l) => s + parseFloat(l.price_subtotal || 0), 0)
  const iva = subtotal * 0.16
  return `<div style="display:flex;justify-content:flex-end;padding:16px 24px;border-top:1px solid var(--border)">
    <table style="width:280px">
      <tr><td style="padding:4px 8px;font-size:13px;color:var(--text-600)">Subtotal</td><td style="padding:4px 8px;text-align:right;font-weight:600">${fmtMxn(subtotal)}</td></tr>
      <tr><td style="padding:4px 8px;font-size:13px;color:var(--text-600)">IVA (16%)</td><td style="padding:4px 8px;text-align:right;font-weight:600">${fmtMxn(iva)}</td></tr>
      <tr style="border-top:2px solid var(--border)">
        <td style="padding:8px 8px 4px;font-size:16px;font-weight:800">TOTAL</td>
        <td style="padding:8px 8px 4px;font-size:16px;font-weight:800;color:var(--primary);text-align:right">${fmtMxn(subtotal + iva)}</td>
      </tr>
    </table>
  </div>`
}

function _calcMargen(lineas) {
  return lineas.reduce((s, l) => {
    if (l.display_type === 'line_section') return s
    const qty = parseFloat(l.product_uom_qty) || 0
    return s + (parseFloat(l.price_subtotal || 0) - qty * parseFloat(l.cost || 0))
  }, 0)
}

function _actualizarTotales(lineas) {
  const subtotal = lineas.reduce((s, l) => s + parseFloat(l.price_subtotal || 0), 0)
  const iva = subtotal * 0.16
  const total = subtotal + iva
  const s = document.getElementById('tot-subtotal')
  const i = document.getElementById('tot-iva')
  const t = document.getElementById('tot-total')
  const m = document.getElementById('tot-margen')
  if (s) s.textContent = fmtMxn(subtotal)
  if (i) i.textContent = fmtMxn(iva)
  if (t) t.textContent = fmtMxn(total)
  if (m) m.textContent = fmtMxn(_calcMargen(lineas))
}

// ─── Nueva venta ─────────────────────────────────────────────────────────────
async function _nuevaVenta() {
  // Odoo: click Nueva → abre el formulario completo directamente (sin modal)
  // Mostramos el formulario en blanco. Al guardar se crea el registro y se queda en el form.
  const cfg = getCfg()
  const hoy = new Date().toISOString().slice(0, 10)
  // Config: validez del presupuesto (0 = sin expiración)
  const validezDias = parseInt(cfg.validez_cotizacion) || 0
  const vencimiento = validezDias > 0
    ? new Date(Date.now() + validezDias*24*60*60*1000).toISOString().slice(0, 10)
    : ''

  setBreadcrumb([
    { label: 'Ventas', href: '#ventas' },
    { label: 'Nueva Cotización' }
  ])

  setPage(`<div id="venta-form" style="min-height:100vh;background:var(--bg-app)">

    <!-- TOPBAR -->
    <div style="display:flex;align-items:center;justify-content:space-between;padding:10px 20px;background:var(--bg-card);border-bottom:1px solid var(--border);position:sticky;top:50px;z-index:20;flex-wrap:wrap;gap:8px">
      <div style="display:flex;align-items:center;gap:8px">
        <button class="o-btn-secondary o-btn-sm" onclick="window._go('ventas')">← Ventas</button>
      </div>
      <div style="display:flex;gap:8px;flex-wrap:wrap">
        <button class="o-btn-primary" onclick="window._guardarNueva()" id="btn-guardar-nueva">💾 Guardar</button>
        <button class="o-btn-secondary o-btn-sm" onclick="window._go('ventas')">Descartar</button>
      </div>
    </div>

    <!-- STATUS BAR -->
    <div style="display:flex;align-items:center;padding:8px 24px;background:var(--bg-card);border-bottom:1px solid var(--border);gap:0">
      <button style="padding:6px 16px;border-radius:20px;border:none;font-size:12px;font-weight:700;background:var(--primary);color:#fff;cursor:default">Cotización</button>
      <span style="color:var(--text-300);font-size:14px;margin:0 2px">›</span>
      <button style="padding:6px 16px;border-radius:20px;border:none;font-size:12px;font-weight:700;color:var(--text-400);background:transparent;cursor:default">Enviado</button>
      <span style="color:var(--text-300);font-size:14px;margin:0 2px">›</span>
      <button style="padding:6px 16px;border-radius:20px;border:none;font-size:12px;font-weight:700;color:var(--text-400);background:transparent;cursor:default">Pedido de Venta</button>
      <span style="color:var(--text-300);font-size:14px;margin:0 2px">›</span>
      <button style="padding:6px 16px;border-radius:20px;border:none;font-size:12px;font-weight:700;color:var(--text-400);background:transparent;cursor:default">Realizado</button>
    </div>

    <!-- FORM SHEET -->
    <div style="background:var(--bg-card);border-radius:12px;margin:16px 20px 0;border:1px solid var(--border);overflow:hidden">

      <!-- Encabezado -->
      <div style="padding:20px 24px 16px;border-bottom:1px solid var(--border)">
        <h1 style="font-family:'Plus Jakarta Sans',sans-serif;font-size:22px;font-weight:800;color:var(--text-900);margin:0 0 6px">Nueva Cotización</h1>
        <span class="o-badge o-badge-gray">Borrador</span>
      </div>

      <!-- Campos principales -->
      <div style="display:grid;grid-template-columns:1fr 1fr;gap:0 32px;padding:16px 24px">

        <!-- Columna izquierda -->
        <div>
          <div class="o-form-field" style="display:grid;grid-template-columns:140px 1fr;align-items:center;padding:7px 0;border-bottom:1px solid var(--border)">
            <span style="font-size:12px;color:var(--text-500);font-weight:600">Cliente <span style="color:#DC2626">*</span></span>
            <div style="position:relative">
              <input id="n-partner-name" class="o-field-input" placeholder="Buscar cliente..." autocomplete="off"
                oninput="window._buscarClienteNueva(this.value)" style="width:100%">
              <input type="hidden" id="n-partner-id">
              <div id="n-partner-dd" style="display:none;position:absolute;top:calc(100%+2px);left:0;right:0;background:var(--bg-card);border:1.5px solid var(--primary);border-radius:8px;box-shadow:0 8px 24px rgba(0,0,0,.12);z-index:300;max-height:220px;overflow-y:auto"></div>
            </div>
          </div>
          <div class="o-form-field" style="display:grid;grid-template-columns:140px 1fr;align-items:center;padding:7px 0;border-bottom:1px solid var(--border)">
            <span style="font-size:12px;color:var(--text-500);font-weight:600">Dirección de Facturación</span>
            <div style="position:relative">
              <input id="n-partner-invoice-name" class="o-field-input" placeholder="Buscar dirección de facturación..." autocomplete="off"
                oninput="window._buscarInvoiceNueva(this.value)" style="width:100%">
              <input type="hidden" id="n-partner-invoice-id">
              <div id="n-partner-invoice-dd" style="display:none;position:absolute;top:calc(100%+2px);left:0;right:0;background:var(--bg-card);border:1.5px solid var(--primary);border-radius:8px;box-shadow:0 8px 24px rgba(0,0,0,.12);z-index:300;max-height:220px;overflow-y:auto"></div>
            </div>
          </div>
          <div class="o-form-field" style="display:grid;grid-template-columns:140px 1fr;align-items:center;padding:7px 0;border-bottom:1px solid var(--border)">
            <span style="font-size:12px;color:var(--text-500);font-weight:600">Dirección de Envío</span>
            <div style="position:relative">
              <input id="n-partner-shipping-name" class="o-field-input" placeholder="Buscar dirección de envío..." autocomplete="off"
                oninput="window._buscarShippingNueva(this.value)" style="width:100%">
              <input type="hidden" id="n-partner-shipping-id">
              <div id="n-partner-shipping-dd" style="display:none;position:absolute;top:calc(100%+2px);left:0;right:0;background:var(--bg-card);border:1.5px solid var(--primary);border-radius:8px;box-shadow:0 8px 24px rgba(0,0,0,.12);z-index:300;max-height:220px;overflow-y:auto"></div>
            </div>
          </div>
          <div class="o-form-field" style="display:grid;grid-template-columns:140px 1fr;align-items:center;padding:7px 0;border-bottom:1px solid var(--border)">
            <span style="font-size:12px;color:var(--text-500);font-weight:600">Referencia Cliente</span>
            <input id="n-ref" class="o-field-input" placeholder="Número de referencia del cliente...">
          </div>
          <div class="o-form-field" style="display:grid;grid-template-columns:140px 1fr;align-items:center;padding:7px 0;border-bottom:1px solid var(--border)">
            <span style="font-size:12px;color:var(--text-500);font-weight:600">Vendedor</span>
            <input id="n-vendedor" class="o-field-input" placeholder="Nombre del vendedor..." value="">
          </div>
        </div>

        <!-- Columna derecha -->
        <div>
          <div class="o-form-field" style="display:grid;grid-template-columns:140px 1fr;align-items:center;padding:7px 0;border-bottom:1px solid var(--border)">
            <span style="font-size:12px;color:var(--text-500);font-weight:600">Fecha de Orden</span>
            <input id="n-fecha" type="date" class="o-field-input" value="${hoy}">
          </div>
          <div class="o-form-field" style="display:grid;grid-template-columns:140px 1fr;align-items:center;padding:7px 0;border-bottom:1px solid var(--border)">
            <span style="font-size:12px;color:var(--text-500);font-weight:600">Vencimiento</span>
            <input id="n-vence" type="date" class="o-field-input" value="${vencimiento}">
          </div>
          ${cfg.fecha_entrega ? `
          <div class="o-form-field" style="display:grid;grid-template-columns:140px 1fr;align-items:center;padding:7px 0;border-bottom:1px solid var(--border)">
            <span style="font-size:12px;color:var(--text-500);font-weight:600">Fecha Compromiso</span>
            <input id="n-entrega" type="date" class="o-field-input">
          </div>` : ''}
          <div class="o-form-field" style="display:grid;grid-template-columns:140px 1fr;align-items:center;padding:7px 0;border-bottom:1px solid var(--border)">
            <span style="font-size:12px;color:var(--text-500);font-weight:600">Términos de Pago</span>
            <select id="n-payment" class="o-field-input">
              <option value="">— Seleccionar —</option>
              <option value="30">Neto 30 días</option>
              <option value="15">Neto 15 días</option>
              <option value="0">Pago inmediato</option>
            </select>
          </div>
        </div>
      </div>

      <!-- Tabs: Líneas de Pedido / Otra Información / Notas -->
      <div style="border-top:1px solid var(--border)">
        <div class="o-tabs" style="display:flex;border-bottom:1px solid var(--border);background:var(--bg-app);padding:0 16px">
          <button class="o-tab active" data-ntab="lineas" onclick="window._ntab('lineas',this)" style="padding:10px 16px;border:none;background:none;font-size:13px;font-weight:600;cursor:pointer;border-bottom:2px solid var(--primary);color:var(--primary)">Líneas de Pedido</button>
          <button class="o-tab" data-ntab="info" onclick="window._ntab('info',this)" style="padding:10px 16px;border:none;background:none;font-size:13px;font-weight:600;cursor:pointer;border-bottom:2px solid transparent;color:var(--text-500)">Otra Información</button>
          <button class="o-tab" data-ntab="notas" onclick="window._ntab('notas',this)" style="padding:10px 16px;border:none;background:none;font-size:13px;font-weight:600;cursor:pointer;border-bottom:2px solid transparent;color:var(--text-500)">Notas y Términos</button>
        </div>

        <!-- Panel Líneas -->
        <div id="ntab-lineas" style="padding:0">
          <table style="width:100%;border-collapse:collapse;font-size:13px">
            <thead style="background:var(--bg-app)">
              <tr>
                <th style="padding:8px 12px;text-align:left;font-size:11px;font-weight:700;color:var(--text-500);border-bottom:1px solid var(--border)">PRODUCTO</th>
                <th style="padding:8px 12px;text-align:left;font-size:11px;font-weight:700;color:var(--text-500);border-bottom:1px solid var(--border)">DESCRIPCIÓN</th>
                <th style="padding:8px 12px;text-align:right;font-size:11px;font-weight:700;color:var(--text-500);border-bottom:1px solid var(--border)">CANT.</th>
                <th style="padding:8px 12px;text-align:right;font-size:11px;font-weight:700;color:var(--text-500);border-bottom:1px solid var(--border)">PRECIO</th>
                <th style="padding:8px 12px;text-align:right;font-size:11px;font-weight:700;color:var(--text-500);border-bottom:1px solid var(--border)">IMPUESTO</th>
                <th style="padding:8px 12px;text-align:right;font-size:11px;font-weight:700;color:var(--text-500);border-bottom:1px solid var(--border)">SUBTOTAL</th>
              </tr>
            </thead>
            <tbody id="n-lineas-tbody">
              <tr id="n-empty-row">
                <td colspan="6" style="padding:32px;text-align:center;color:var(--text-400);font-size:13px">
                  Guarda la cotización para agregar productos
                </td>
              </tr>
            </tbody>
          </table>
          <div style="padding:10px 12px">
            <button class="o-btn-secondary o-btn-sm" onclick="window._guardarNueva(true)" style="font-size:12px">+ Agregar producto</button>
          </div>
          <div style="padding:12px 24px;border-top:1px solid var(--border);display:flex;justify-content:flex-end">
            <table style="font-size:13px;min-width:260px">
              <tr><td style="padding:3px 16px 3px 0;color:var(--text-500)">Subtotal:</td><td style="text-align:right;font-weight:600">$0.00</td></tr>
              <tr><td style="padding:3px 16px 3px 0;color:var(--text-500)">IVA (16%):</td><td style="text-align:right;font-weight:600">$0.00</td></tr>
              <tr style="border-top:2px solid var(--border)"><td style="padding:6px 16px 3px 0;font-weight:700;font-size:14px">TOTAL:</td><td style="text-align:right;font-weight:800;font-size:15px;color:var(--primary)">$0.00 MXN</td></tr>
            </table>
          </div>
        </div>

        <!-- Panel Info -->
        <div id="ntab-info" style="padding:16px 24px;display:none">
          <div style="display:grid;grid-template-columns:1fr 1fr;gap:0 32px">
            <div>
              <div style="display:grid;grid-template-columns:160px 1fr;align-items:center;padding:7px 0;border-bottom:1px solid var(--border)">
                <span style="font-size:12px;color:var(--text-500);font-weight:600">Equipo de Ventas</span>
                <input id="n-team" class="o-field-input" placeholder="Equipo de ventas...">
              </div>
              <div style="display:grid;grid-template-columns:160px 1fr;align-items:center;padding:7px 0;border-bottom:1px solid var(--border)">
                <span style="font-size:12px;color:var(--text-500);font-weight:600">Etiquetas</span>
                <input id="n-tags" class="o-field-input" placeholder="Etiquetas...">
              </div>
            </div>
            <div>
              <div style="display:grid;grid-template-columns:160px 1fr;align-items:center;padding:7px 0;border-bottom:1px solid var(--border)">
                <span style="font-size:12px;color:var(--text-500);font-weight:600">Empresa</span>
                <input id="n-empresa" class="o-field-input" value="NEXUSTECH" readonly style="background:var(--bg-app)">
              </div>
            </div>
          </div>
        </div>

        <!-- Panel Notas -->
        <div id="ntab-notas" style="padding:16px 24px;display:none">
          <div style="margin-bottom:12px">
            <label style="font-size:12px;font-weight:600;color:var(--text-500);display:block;margin-bottom:4px">TÉRMINOS Y CONDICIONES</label>
            <textarea id="n-nota" class="o-field-input" rows="4" placeholder="Escribe los términos y condiciones de esta cotización..." style="width:100%;resize:vertical;box-sizing:border-box">${cfg.terminos || ''}</textarea>
          </div>
        </div>
      </div>
    </div>

    <div style="height:60px"></div>
  </div>`)

  // Tab switcher
  window._ntab = (tab, btn) => {
    document.querySelectorAll('[data-ntab]').forEach(b => {
      b.style.borderBottomColor = 'transparent'
      b.style.color = 'var(--text-500)'
    })
    document.querySelectorAll('[id^="ntab-"]').forEach(p => p.style.display = 'none')
    btn.style.borderBottomColor = 'var(--primary)'
    btn.style.color = 'var(--primary)'
    const panel = document.getElementById(`ntab-${tab}`)
    if (panel) panel.style.display = ''
  }

  // Buscador de clientes
  window._buscarClienteNueva = async q => {
    const dd = document.getElementById('n-partner-dd')
    if (!q || !dd) { if (dd) dd.style.display = 'none'; return }
    try {
      const res = await api.get(`/ventas/buscar-clientes?q=${encodeURIComponent(q)}`)
      const items = res?.data || []
      dd.style.display = items.length ? 'block' : 'none'
      dd.innerHTML = items.map(it => `
        <div style="padding:8px 12px;cursor:pointer;border-bottom:1px solid var(--border)"
          onmouseover="this.style.background='#EEF2FF'" onmouseout="this.style.background=''"
          onclick="
            document.getElementById('n-partner-name').value='${(it.name||'').replace(/'/g,"\\'")}';
            document.getElementById('n-partner-id').value='${it.id}';
            document.getElementById('n-partner-dd').style.display='none';
            if (document.getElementById('n-partner-invoice-name') && !document.getElementById('n-partner-invoice-name').value) {
              document.getElementById('n-partner-invoice-name').value='${(it.name||'').replace(/'/g,"\\'")}';
              document.getElementById('n-partner-invoice-id').value='${it.id}';
            }
            if (document.getElementById('n-partner-shipping-name') && !document.getElementById('n-partner-shipping-name').value) {
              document.getElementById('n-partner-shipping-name').value='${(it.name||'').replace(/'/g,"\\'")}';
              document.getElementById('n-partner-shipping-id').value='${it.id}';
            }">
          <div style="font-weight:600;font-size:13px">${it.name||''}</div>
          ${it.email ? `<div style="font-size:11px;color:var(--text-400)">${it.email}</div>` : ''}
        </div>`).join('')
    } catch(_) {}
  }

  window._buscarInvoiceNueva = async q => {
    const dd = document.getElementById('n-partner-invoice-dd')
    if (!q || !dd) { if (dd) dd.style.display = 'none'; return }
    try {
      const res = await api.get(`/ventas/buscar-clientes?q=${encodeURIComponent(q)}`)
      const items = res?.data || []
      dd.style.display = items.length ? 'block' : 'none'
      dd.innerHTML = items.map(it => `
        <div style="padding:8px 12px;cursor:pointer;border-bottom:1px solid var(--border)"
          onmouseover="this.style.background='#EEF2FF'" onmouseout="this.style.background=''"
          onclick="
            document.getElementById('n-partner-invoice-name').value='${(it.name||'').replace(/'/g,"\\'")}';
            document.getElementById('n-partner-invoice-id').value='${it.id}';
            document.getElementById('n-partner-invoice-dd').style.display='none'">
          <div style="font-weight:600;font-size:13px">${it.name||''}</div>
          ${it.email ? `<div style="font-size:11px;color:var(--text-400)">${it.email}</div>` : ''}
        </div>`).join('')
    } catch(_) {}
  }

  window._buscarShippingNueva = async q => {
    const dd = document.getElementById('n-partner-shipping-dd')
    if (!q || !dd) { if (dd) dd.style.display = 'none'; return }
    try {
      const res = await api.get(`/ventas/buscar-clientes?q=${encodeURIComponent(q)}`)
      const items = res?.data || []
      dd.style.display = items.length ? 'block' : 'none'
      dd.innerHTML = items.map(it => `
        <div style="padding:8px 12px;cursor:pointer;border-bottom:1px solid var(--border)"
          onmouseover="this.style.background='#EEF2FF'" onmouseout="this.style.background=''"
          onclick="
            document.getElementById('n-partner-shipping-name').value='${(it.name||'').replace(/'/g,"\\'")}';
            document.getElementById('n-partner-shipping-id').value='${it.id}';
            document.getElementById('n-partner-shipping-dd').style.display='none'">
          <div style="font-weight:600;font-size:13px">${it.name||''}</div>
          ${it.email ? `<div style="font-size:11px;color:var(--text-400)">${it.email}</div>` : ''}
        </div>`).join('')
    } catch(_) {}
  }

  // Guardar nueva cotización
  window._guardarNueva = async (irAProductos = false) => {
    const pid = parseInt(document.getElementById('n-partner-id')?.value || '0')
    if (!pid) { toast('Error', 'Selecciona un cliente antes de guardar', 'error'); return }
    const btn = document.getElementById('btn-guardar-nueva')
    if (btn) { btn.disabled = true; btn.textContent = '⏳ Guardando...' }
    try {
      // Calcular días de validez a partir de la fecha de vencimiento elegida
      const vence = document.getElementById('n-vence')?.value
      let validity_days = null
      if (vence) {
        validity_days = Math.max(1, Math.round((new Date(vence + 'T00:00:00') - Date.now()) / 86400000))
      } else if (validezDias === 0) {
        validity_days = 0  // sin expiración
      }
      const payload = {
        partner_id: pid,
        partner_invoice_id: parseInt(document.getElementById('n-partner-invoice-id')?.value || '0') || null,
        partner_shipping_id: parseInt(document.getElementById('n-partner-shipping-id')?.value || '0') || null,
        client_order_ref: document.getElementById('n-ref')?.value || '',
        note: document.getElementById('n-nota')?.value || '',
        validity_days,
      }
      const res = await api.post('/ventas', payload)
      const id = res?.data?.id
      if (id) {
        // Fecha compromiso (config: fecha_entrega) se guarda tras crear la orden
        const fechaEntrega = document.getElementById('n-entrega')?.value
        if (fechaEntrega) {
          try { await api.put(`/ventas/${id}`, { commitment_date: fechaEntrega }) } catch (_) {}
        }
        toast('Cotización creada', res.data.name || `S${String(id).padStart(5,'0')}`, 'success')
        await _abrirVenta(id)
      }
    } catch (e) {
      toast('Error', e.message, 'error')
      if (btn) { btn.disabled = false; btn.textContent = '💾 Guardar' }
    }
  }
}

// ─── Acciones masivas ────────────────────────────────────────────────────────
async function _cancelarSeleccionados() {
  if (!_selIDs.size) return
  if (!confirm(`¿Cancelar ${_selIDs.size} orden(es)?`)) return
  const ids = [..._selIDs]
  let ok = 0
  for (const id of ids) {
    try { await api.put(`/ventas/${id}/cancelar`, {}); ok++ } catch(_) {}
  }
  _selIDs.clear()
  toast(`${ok} canceladas`, '', 'success')
  _load()
}

async function _exportarCSV() {
  const rows = _selIDs.size > 0
    ? _records.filter(r => _selIDs.has(r.id))
    : _records
  const cols = ['name','date_order','partner_name','client_order_ref','amount_total','state','invoice_status']
  const header = ['Número','Fecha','Cliente','Ref. Cliente','Total','Estado','Facturación']
  const csv = [header.join(','), ...rows.map(r => cols.map(c => `"${r[c]||''}"`).join(','))].join('\n')
  const a = document.createElement('a')
  a.href = 'data:text/csv;charset=utf-8,' + encodeURIComponent(csv)
  a.download = `ventas-${new Date().toISOString().slice(0,10)}.csv`
  a.click()
}

// ─── Init selección múltiple ─────────────────────────────────────────────────
function _initSeleccion() {
  document.querySelectorAll('.o-chk-row').forEach(chk => {
    const id = parseInt(chk.closest('tr')?.querySelector('[onclick*="_abrirVenta"]')?.onclick?.toString().match(/\d+/)?.[0] || '0')
    if (id && _selIDs.has(id)) chk.checked = true
  })
}

function _initTabSwitcher() {
  // Tabs ya se manejan con onclick inline
}

// ─── Helpers ─────────────────────────────────────────────────────────────────
function _fmtK(n) {
  const v = parseFloat(n || 0)
  if (v >= 1000000) return `$${(v/1000000).toFixed(1)}M`
  if (v >= 1000) return `$${(v/1000).toFixed(1)}k`
  return fmtMxn(v)
}

function _avatarColor(name) {
  const colors = ['#6366F1','#8B5CF6','#EC4899','#F59E0B','#10B981','#3B82F6','#EF4444','#14B8A6']
  if (!name) return colors[0]
  let h = 0
  for (let i = 0; i < name.length; i++) h = name.charCodeAt(i) + ((h << 5) - h)
  return colors[Math.abs(h) % colors.length]
}
