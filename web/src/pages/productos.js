import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { fmtMxn, fmtDate, fmtNum, paginationHtml, skeletonTable, toast } from '../ui.js'
import { api } from '../api.js'
import { editarProducto } from './forms/edit_forms.js'

/* ─── Estado del módulo ─── */
let _currentView = 'list'   // 'list' | 'kanban'
let _currentPage = 1
let _records     = []
let _searchQuery = ''
let _tipoFiltro  = ''

/* ═══════════════════════════════════════════════
   ENTRY POINT
   ═══════════════════════════════════════════════ */
export async function renderProductos() {
  ensureLayout()
  _currentView = 'list'
  _currentPage = 1
  _searchQuery = ''
  _tipoFiltro  = ''
  setBreadcrumb([{ label: 'Productos' }])
  _renderControlPanel()
  await _loadAndRender()
}

/* ═══════════════════════════════════════════════
   CONTROL PANEL
   ═══════════════════════════════════════════════ */
function _renderControlPanel() {
  setPage(`
  <div class="o-cp" id="productos-cp">
    <div class="o-cp-left">
      <button class="o-btn-primary" onclick="window._productoNuevo()">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
        Nuevo
      </button>
    </div>
    <div class="o-cp-center">
      <div class="o-search-bar">
        <svg class="o-search-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/></svg>
        <input id="o-search-productos" class="o-search-input" type="text" placeholder="Buscar producto o código…" value="${_searchQuery}">
        <div class="o-search-filters">
          <button class="o-filter-btn" onclick="window._productoFiltroTipo('')" data-active id="ptf-todos">Todos</button>
          <button class="o-filter-btn" onclick="window._productoFiltroTipo('consu')" id="ptf-consu">Consumibles</button>
          <button class="o-filter-btn" onclick="window._productoFiltroTipo('service')" id="ptf-serv">Servicios</button>
          <button class="o-filter-btn" onclick="window._productoFiltroTipo('product')" id="ptf-prod">Almacenables</button>
        </div>
      </div>
    </div>
    <div class="o-cp-right">
      <div class="o-view-switcher">
        <button class="o-view-btn ${_currentView==='list'?'o-active':''}" onclick="window._productoSetView('list')" title="Vista Lista">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="8" y1="6" x2="21" y2="6"/><line x1="8" y1="12" x2="21" y2="12"/><line x1="8" y1="18" x2="21" y2="18"/><line x1="3" y1="6" x2="3.01" y2="6"/><line x1="3" y1="12" x2="3.01" y2="12"/><line x1="3" y1="18" x2="3.01" y2="18"/></svg>
        </button>
        <button class="o-view-btn ${_currentView==='kanban'?'o-active':''}" onclick="window._productoSetView('kanban')" title="Vista Kanban">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="1" y="4" width="6" height="16" rx="1"/><rect x="9" y="4" width="6" height="10" rx="1"/><rect x="17" y="4" width="6" height="13" rx="1"/></svg>
        </button>
      </div>
    </div>
  </div>
  <div id="productos-content" class="o-view-content">
    ${skeletonTable(10, 6)}
  </div>`)

  let timer
  setTimeout(() => {
    document.getElementById('o-search-productos')?.addEventListener('input', (e) => {
      clearTimeout(timer)
      timer = setTimeout(() => {
        _searchQuery = e.target.value.trim()
        _currentPage = 1
        _loadAndRender()
      }, 380)
    })
  }, 100)
}

/* ═══════════════════════════════════════════════
   LOAD & RENDER
   ═══════════════════════════════════════════════ */
async function _loadAndRender() {
  try {
    const res  = await api.productos(_currentPage, _searchQuery)
    _records   = (res?.data || []).filter(p => !_tipoFiltro || (p.type_ || p.type) === _tipoFiltro)
    const hasMore = (res?.data || []).length >= 20

    const el = document.getElementById('productos-content')
    if (!el) return

    if (_currentView === 'kanban') {
      el.innerHTML = _renderKanban(_records)
    } else {
      el.innerHTML = _renderList(_records, hasMore)
    }
  } catch (err) {
    console.error(err)
    toast('Error', err.message, 'error')
  }
}

/* ═══════════════════════════════════════════════
   VISTA LISTA
   ═══════════════════════════════════════════════ */
function _renderList(records, hasMore) {
  if (!records.length) return `
    <div class="o-empty-state">
      <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" opacity=".3"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/></svg>
      <p>${_searchQuery ? `Sin resultados para "${_searchQuery}"` : 'Sin productos en catálogo'}</p>
    </div>`

  return `
  <div class="o-list-view">
    <table class="o-list-table">
      <thead>
        <tr>
          <th class="o-list-chk"><input type="checkbox" class="o-chk" onclick="window._chkAllProductos(this)"></th>
          <th style="width:56px">Imagen</th>
          <th class="o-col-sortable">Nombre</th>
          <th>SKU</th>
          <th class="o-col-right">Precio Venta</th>
          <th class="o-col-right">Costo</th>
          <th>Tipo</th>
          <th class="o-col-right">Stock</th>
        </tr>
      </thead>
      <tbody>
        ${records.map((p, i) => {
          const nombre  = typeof p.name === 'object'
            ? (p.name?.es_MX || p.name?.en_US || Object.values(p.name)[0] || `Producto #${p.id}`)
            : (p.name || p.nombre || `Producto #${p.id}`)
          const tp      = p.type_ || p.type || ''
          const tipo    = tp === 'consu' ? 'Consumible' : tp === 'service' ? 'Servicio' : tp === 'product' ? 'Almacenable' : 'Consumible'
          const tipoCls = tp === 'service' ? 'o-badge-info' : tp === 'consu' ? 'o-badge-warn' : 'o-badge-success'
          const precio  = fmtMxn(parseFloat(p.list_price || p.precio || 0))
          const costo   = fmtMxn(parseFloat(p.standard_price || p.costo || 0))
          const hue     = (p.id * 67) % 360
          const letter  = nombre[0]?.toUpperCase() || 'P'
          return `
          <tr class="o-list-row" onclick="window._verProducto(${p.id})">
            <td class="o-list-chk"><input type="checkbox" class="o-chk" onclick="event.stopPropagation()"></td>
            <td>
              <div class="o-prod-thumb" style="background:linear-gradient(135deg,hsl(${hue},50%,60%),hsl(${(hue+60)%360},60%,50%))">${letter}</div>
            </td>
            <td class="o-td-primary">${nombre}</td>
            <td class="o-td-mono">${p.default_code || '—'}</td>
            <td class="o-td-amount">${precio}</td>
            <td class="o-td-amount o-td-muted">${costo}</td>
            <td><span class="o-badge ${tipoCls}">${tipo}</span></td>
            <td class="o-td-amount">${p.qty_available != null ? fmtNum(parseFloat(p.qty_available)) : '—'}</td>
          </tr>`
        }).join('')}
      </tbody>
    </table>
    <div class="o-list-footer">
      <span class="o-list-count">${records.length} producto${records.length !== 1 ? 's' : ''}</span>
      ${paginationHtml(_currentPage, hasMore, (p) => { _currentPage = p; _loadAndRender() })}
    </div>
  </div>`
}

/* ═══════════════════════════════════════════════
   VISTA KANBAN
   ═══════════════════════════════════════════════ */
function _renderKanban(records) {
  if (!records.length) return `
    <div class="o-empty-state">
      <p>Sin productos${_searchQuery ? ` para "${_searchQuery}"` : ''}</p>
    </div>`

  return `
  <div class="o-kanban-grid">
    ${records.map(p => {
      const nombre  = typeof p.name === 'object'
        ? (p.name?.es_MX || p.name?.en_US || `Producto #${p.id}`)
        : (p.name || `Producto #${p.id}`)
      const tp      = p.type_ || p.type || ''
      const tipo    = tp === 'consu' ? 'Consumible' : tp === 'service' ? 'Servicio' : 'Almacenable'
      const tipoCls = tp === 'service' ? 'o-badge-info' : tp === 'consu' ? 'o-badge-warn' : 'o-badge-success'
      const precio  = fmtMxn(parseFloat(p.list_price || 0))
      const hue     = (p.id * 67) % 360
      const letter  = nombre[0]?.toUpperCase() || 'P'
      return `
      <div class="o-kanban-card" onclick="window._verProducto(${p.id})">
        <div class="o-kanban-img" style="background:linear-gradient(135deg,hsl(${hue},50%,65%),hsl(${(hue+60)%360},60%,55%))">
          <span style="font-size:40px;font-weight:800;color:rgba(255,255,255,.7)">${letter}</span>
        </div>
        <div class="o-kanban-body">
          <div class="o-kanban-title">${nombre}</div>
          <div class="o-kanban-sub">${p.default_code || '(sin SKU)'}</div>
          <div style="display:flex;justify-content:space-between;align-items:center;margin-top:8px">
            <span class="o-badge ${tipoCls}">${tipo}</span>
            <strong class="o-kanban-price">${precio}</strong>
          </div>
        </div>
      </div>`
    }).join('')}
  </div>`
}

/* ═══════════════════════════════════════════════
   VISTA FORMULARIO
   ═══════════════════════════════════════════════ */
window._verProducto = async (id) => {
  setBreadcrumb([
    { label: 'Productos', onclick: () => renderProductos() },
    { label: 'Cargando…', id: 'bc-prod-name' }
  ])
  setPage(`<div class="o-form-loading">${skeletonTable(4, 3)}</div>`)

  try {
    const p = await api.producto(id)
    if (!p) { toast('Error', 'Producto no encontrado', 'error'); return }

    const bcEl = document.getElementById('bc-prod-name')
    if (bcEl) bcEl.textContent = typeof p.name === 'object' ? (p.name?.es_MX || p.name?.en_US || 'Producto') : (p.name || 'Producto')

    const nombre  = typeof p.name === 'object'
      ? (p.name?.es_MX || p.name?.en_US || `Producto #${p.id}`)
      : (p.name || `Producto #${p.id}`)
    const tp      = p.type_ || p.type || ''
    const tipo    = tp === 'consu' ? 'Consumible' : tp === 'service' ? 'Servicio' : tp === 'product' ? 'Almacenable' : 'Consumible'
    const tipoCls = tp === 'service' ? 'o-badge-info' : tp === 'consu' ? 'o-badge-warn' : 'o-badge-success'
    const precio  = fmtMxn(parseFloat(p.list_price || 0))
    const costo   = fmtMxn(parseFloat(p.standard_price || 0))
    const hue     = (p.id * 67) % 360
    const letter  = nombre[0]?.toUpperCase() || 'P'
    const categ   = (() => {
      const c = p.categ_name || p.categoria || ''
      return c === 'Goods' ? 'Mercancía' : c === 'Services' ? 'Servicios' : c || '—'
    })()

    setPage(`
    <div class="o-form-topbar">
      <button class="o-back-btn" onclick="window._productosBack()">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
        Productos
      </button>
      <div class="o-form-actions">
        <button class="o-btn-secondary" onclick="window._editarProductoForm(${p.id})">Editar</button>
        <button class="o-btn-primary"   onclick="alert('Crear pedido — próximamente')">Crear Pedido</button>
      </div>
    </div>

    <div class="o-smart-buttons">
      <button class="o-smart-btn" onclick="alert('Stock disponible')">
        <span class="o-smart-count">${p.qty_available != null ? fmtNum(parseFloat(p.qty_available)) : 0}</span>
        <span class="o-smart-label">En Stock</span>
      </button>
      <button class="o-smart-btn" onclick="alert('Ventas del producto')">
        <span class="o-smart-count">0</span>
        <span class="o-smart-label">Ventas</span>
      </button>
      <button class="o-smart-btn" onclick="alert('Compras del producto')">
        <span class="o-smart-count">0</span>
        <span class="o-smart-label">Compras</span>
      </button>
    </div>

    <div class="o-form-sheet">
      <div class="o-sheet-header">
        <div class="o-prod-thumb o-prod-thumb-lg" style="background:linear-gradient(135deg,hsl(${hue},50%,65%),hsl(${(hue+60)%360},60%,55%))">${letter}</div>
        <div class="o-sheet-title-block">
          <h1 class="o-form-title">${nombre}</h1>
          <div style="display:flex;gap:6px;margin-top:4px">
            <span class="o-badge ${tipoCls}">${tipo}</span>
            ${p.active !== false ? '<span class="o-badge o-badge-success">Activo</span>' : '<span class="o-badge o-badge-gray">Inactivo</span>'}
          </div>
        </div>
      </div>

      <div class="o-form-grid">
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">SKU / Código interno</label><div class="o-field-value o-field-mono">${p.default_code || '—'}</div></div>
          <div class="o-field-group"><label class="o-field-label">Tipo de Producto</label><div class="o-field-value"><span class="o-badge ${tipoCls}">${tipo}</span></div></div>
          <div class="o-field-group"><label class="o-field-label">Unidad de Medida</label><div class="o-field-value">${p.uom_name || p.uom || 'Unidad'}</div></div>
          <div class="o-field-group"><label class="o-field-label">Peso</label><div class="o-field-value">${p.weight != null ? p.weight + ' kg' : '—'}</div></div>
        </div>
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">Precio de Venta</label><div class="o-field-value o-field-price">${precio}</div></div>
          <div class="o-field-group"><label class="o-field-label">Costo</label><div class="o-field-value o-td-muted">${costo}</div></div>
          <div class="o-field-group"><label class="o-field-label">Impuestos</label><div class="o-field-value">${p.taxes_name || 'IVA 16%'}</div></div>
          <div class="o-field-group"><label class="o-field-label">Categoría</label><div class="o-field-value">${categ}</div></div>
        </div>
      </div>

      <div class="o-notebook">
        <div class="o-tabs" id="prod-tabs">
          <button class="o-tab active" onclick="window._prodTab('info', this)">Información General</button>
          <button class="o-tab" onclick="window._prodTab('ventas', this)">Ventas</button>
          <button class="o-tab" onclick="window._prodTab('compras', this)">Compras</button>
          <button class="o-tab" onclick="window._prodTab('inventario', this)">Inventario</button>
        </div>

        <div class="o-tab-pane" id="tab-info">
          <div class="o-field-group"><label class="o-field-label">Descripción</label>
            <div class="o-field-value">${p.description || p.descripcion || '—'}</div></div>
          <div class="o-form-grid" style="margin-top:12px">
            <div class="o-form-col">
              <div class="o-field-group"><label class="o-field-label">Peso (kg)</label><div class="o-field-value">${p.weight ?? '—'}</div></div>
            </div>
            <div class="o-form-col">
              <div class="o-field-group"><label class="o-field-label">Volumen (m³)</label><div class="o-field-value">${p.volume ?? '—'}</div></div>
            </div>
          </div>
        </div>
        <div class="o-tab-pane" id="tab-ventas" style="display:none">
          <div class="o-field-group"><label class="o-field-label">Política de facturación</label><div class="o-field-value">Cantidades ordenadas</div></div>
          <div class="o-field-group"><label class="o-field-label">Descripción en pedido de venta</label><div class="o-field-value">—</div></div>
        </div>
        <div class="o-tab-pane" id="tab-compras" style="display:none">
          <div class="o-field-group"><label class="o-field-label">Precio de compra</label><div class="o-field-value">${costo}</div></div>
          <div class="o-field-group"><label class="o-field-label">Proveedor preferido</label><div class="o-field-value">—</div></div>
        </div>
        <div class="o-tab-pane" id="tab-inventario" style="display:none">
          <div class="o-form-grid">
            <div class="o-form-col">
              <div class="o-field-group"><label class="o-field-label">Método de costeo</label><div class="o-field-value">Precio estándar</div></div>
            </div>
            <div class="o-form-col">
              <div class="o-field-group"><label class="o-field-label">Rutas</label><div class="o-field-value">Comprar</div></div>
            </div>
          </div>
        </div>
      </div>
    </div>`)

    window._editarProductoForm = (pid) => editarProducto({ id: pid, ...p }, () => window._verProducto(pid))
    window._prodTab = (tab, btn) => {
      document.querySelectorAll('#prod-tabs .o-tab').forEach(b => b.classList.remove('active'))
      btn.classList.add('active')
      document.querySelectorAll('.o-tab-pane').forEach(el => el.style.display = 'none')
      const pane = document.getElementById(`tab-${tab}`)
      if (pane) pane.style.display = ''
    }

  } catch (err) {
    console.error(err)
    toast('Error', err.message, 'error')
  }
}

/* ═══════════════════════════════════════════════
   GLOBAL HANDLERS
   ═══════════════════════════════════════════════ */
window._productosBack = () => renderProductos()

window._productoSetView = (view) => {
  _currentView = view
  document.querySelectorAll('#productos-cp .o-view-btn').forEach(b => b.classList.remove('o-active'))
  const idx = view === 'list' ? 0 : 1
  document.querySelectorAll('#productos-cp .o-view-btn')[idx]?.classList.add('o-active')
  const el = document.getElementById('productos-content')
  if (el) {
    if (view === 'kanban') el.innerHTML = _renderKanban(_records)
    else el.innerHTML = _renderList(_records, false)
  }
}

window._productoFiltroTipo = (tipo) => {
  _tipoFiltro = tipo
  _currentPage = 1
  document.querySelectorAll('#productos-cp .o-filter-btn').forEach(b => b.removeAttribute('data-active'))
  const map = { '': 'ptf-todos', 'consu': 'ptf-consu', 'service': 'ptf-serv', 'product': 'ptf-prod' }
  document.getElementById(map[tipo])?.setAttribute('data-active', '')
  _loadAndRender()
}

window._productoNuevo = () => { import('./forms/create_forms.js').then(m => m.nuevoProducto(() => _loadAndRender())) }
window._chkAllProductos = (master) => document.querySelectorAll('#productos-content .o-chk').forEach(c => c.checked = master.checked)
