import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { fmtMxn, fmtDate, fmtNum, paginationHtml, skeletonTable, toast } from '../ui.js'
import { api } from '../api.js'
import { ajustarStock } from './forms/edit_forms.js'

/* ─── Estado ─── */
let _currentPage = 1
let _records     = []
let _searchQuery = ''

/* ═══════════════════════════════════════════════
   ENTRY POINT
   ═══════════════════════════════════════════════ */
export async function renderStock() {
  ensureLayout()
  _currentPage = 1
  _searchQuery = ''
  setBreadcrumb([{ label: 'Inventario' }])
  _renderControlPanel()
  await _loadAndRender()
}

/* ═══════════════════════════════════════════════
   CONTROL PANEL
   ═══════════════════════════════════════════════ */
function _renderControlPanel() {
  setPage(`
  <div class="o-cp" id="stock-cp">
    <div class="o-cp-left">
      <button class="o-btn-primary" onclick="window._stockAjustarGlobal()">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
        Ajustar Cantidad
      </button>
    </div>
    <div class="o-cp-center">
      <div class="o-search-bar">
        <svg class="o-search-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/></svg>
        <input id="o-search-stock" class="o-search-input" type="text" placeholder="Buscar producto o ubicación…" value="${_searchQuery}">
        <div class="o-search-filters">
          <button class="o-filter-btn" onclick="window._stockFiltro('bajo')" id="sf-bajo">Stock Bajo</button>
          <button class="o-filter-btn" onclick="window._stockFiltro('cero')" id="sf-cero">En Cero</button>
        </div>
      </div>
    </div>
    <div class="o-cp-right">
      <div class="o-view-switcher">
        <button class="o-view-btn o-active" title="Vista Lista">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="8" y1="6" x2="21" y2="6"/><line x1="8" y1="12" x2="21" y2="12"/><line x1="8" y1="18" x2="21" y2="18"/><line x1="3" y1="6" x2="3.01" y2="6"/><line x1="3" y1="12" x2="3.01" y2="12"/><line x1="3" y1="18" x2="3.01" y2="18"/></svg>
        </button>
      </div>
    </div>
  </div>
  <div id="stock-content" class="o-view-content">
    ${skeletonTable(10, 5)}
  </div>`)

  setTimeout(() => {
    document.getElementById('o-search-stock')?.addEventListener('input', (e) => {
      _searchQuery = e.target.value.toLowerCase()
      _filterTableLocal()
    })
  }, 100)
}

function _filterTableLocal() {
  document.querySelectorAll('#stock-content tbody tr').forEach(r => {
    r.style.display = r.textContent.toLowerCase().includes(_searchQuery) ? '' : 'none'
  })
}

/* ═══════════════════════════════════════════════
   LOAD & RENDER
   ═══════════════════════════════════════════════ */
async function _loadAndRender() {
  try {
    const res  = await api.stock(_currentPage)
    _records   = res?.data || []
    const hasMore = _records.length >= 20

    const el = document.getElementById('stock-content')
    if (!el) return
    el.innerHTML = _renderList(_records, hasMore)
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
      <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" opacity=".3"><path d="M5 8h14M5 8a2 2 0 1 0 0-4h14a2 2 0 1 0 0 4M5 8v10a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V8m-9 4h4"/></svg>
      <p>Sin registros de inventario</p>
    </div>`

  return `
  <div class="o-list-view">
    <table class="o-list-table">
      <thead>
        <tr>
          <th class="o-list-chk"><input type="checkbox" class="o-chk" onclick="window._chkAllStock(this)"></th>
          <th class="o-col-sortable">Producto</th>
          <th>Ubicación</th>
          <th class="o-col-right o-col-sortable">Disponible</th>
          <th class="o-col-right">Reservado</th>
          <th>Unidad</th>
        </tr>
      </thead>
      <tbody>
        ${records.map(s => {
          const qty     = parseFloat(s.cantidad_disponible ?? s.qty_available ?? 0)
          const res     = parseFloat(s.cantidad_reservada  ?? s.reserved_qty  ?? 0)
          const qtyColor= qty <= 0 ? '#ef4444' : qty < 10 ? '#f59e0b' : '#10b981'
          const nombre  = s.product_name || s.nombre || `Producto #${s.product_id || s.id}`
          const loc     = s.ubicacion || s.location || 'WH/Stock'
          const uom     = s.uom_name  || s.unidad  || 'Unidades'
          return `
          <tr class="o-list-row" onclick="window._verStockItem(${s.product_id || s.id})">
            <td class="o-list-chk"><input type="checkbox" class="o-chk" onclick="event.stopPropagation()"></td>
            <td class="o-td-primary">${nombre}</td>
            <td class="o-td-muted">${loc}</td>
            <td class="o-td-amount" style="color:${qtyColor};font-weight:700">${fmtNum(qty)}</td>
            <td class="o-td-amount o-td-muted">${fmtNum(res)}</td>
            <td class="o-td-muted">${uom}</td>
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
   VISTA FORMULARIO  (detalle de stock)
   ═══════════════════════════════════════════════ */
window._verStockItem = async (productId) => {
  setBreadcrumb([
    { label: 'Inventario', onclick: () => renderStock() },
    { label: 'Detalle de stock', id: 'bc-stock-name' }
  ])
  setPage(`<div class="o-form-loading">${skeletonTable(3, 3)}</div>`)

  try {
    const res   = await api.stockProducto(productId)
    const arr   = Array.isArray(res?.data) ? res.data : (res?.data ? [res.data] : [])
    const main  = arr[0] || {}
    const qty   = parseFloat(main.cantidad_disponible ?? 0)
    const res2  = parseFloat(main.cantidad_reservada  ?? 0)
    const total_val = qty * parseFloat(main.valor_unitario || 0)
    const nombre = main.product_name || `Producto #${productId}`

    const bcEl = document.getElementById('bc-stock-name')
    if (bcEl) bcEl.textContent = nombre

    const qtyColor = qty <= 0 ? '#ef4444' : qty < 10 ? '#f59e0b' : '#10b981'

    setPage(`
    <div class="o-form-topbar">
      <button class="o-back-btn" onclick="window._stockBack()">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
        Inventario
      </button>
      <div class="o-form-actions">
        <button class="o-btn-primary" onclick="window._ajustarStockForm(${productId})">Ajustar Cantidad</button>
      </div>
    </div>

    <div class="o-smart-buttons">
      <button class="o-smart-btn">
        <span class="o-smart-count" style="color:${qtyColor}">${fmtNum(qty)}</span>
        <span class="o-smart-label">Disponible</span>
      </button>
      <button class="o-smart-btn">
        <span class="o-smart-count">${fmtNum(res2)}</span>
        <span class="o-smart-label">Reservado</span>
      </button>
    </div>

    <div class="o-form-sheet">
      <div class="o-sheet-header">
        <div class="o-sheet-title-block">
          <h1 class="o-form-title">${nombre}</h1>
        </div>
      </div>

      <div class="o-form-grid">
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">Producto</label><div class="o-field-value">${nombre}</div></div>
          <div class="o-field-group"><label class="o-field-label">Ubicación</label><div class="o-field-value">${main.ubicacion || 'WH/Stock'}</div></div>
          <div class="o-field-group"><label class="o-field-label">Cantidad Disponible</label><div class="o-field-value" style="color:${qtyColor};font-weight:700;font-size:20px">${fmtNum(qty)}</div></div>
          <div class="o-field-group"><label class="o-field-label">Cantidad Reservada</label><div class="o-field-value">${fmtNum(res2)}</div></div>
        </div>
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">Unidad de Medida</label><div class="o-field-value">${main.uom_name || main.unidad || 'Unidades'}</div></div>
          <div class="o-field-group"><label class="o-field-label">Valor Unitario</label><div class="o-field-value">${fmtMxn(parseFloat(main.valor_unitario || 0))}</div></div>
          <div class="o-field-group"><label class="o-field-label">Valor Total</label><div class="o-field-value o-field-price">${fmtMxn(total_val)}</div></div>
        </div>
      </div>

      <div class="o-notebook">
        <div class="o-tabs">
          <button class="o-tab active">Movimientos</button>
        </div>
        <div class="o-tab-pane">
          <div class="o-empty-state" style="padding:32px 0">
            <p style="color:var(--o-text-secondary)">Historial de movimientos de inventario — próximamente</p>
          </div>
        </div>
      </div>
    </div>`)

    window._ajustarStockForm = (pid) => {
      const s = _records.find(x => (x.product_id || x.id) === pid)
      if (s) ajustarStock(s, () => window._verStockItem(pid))
      else ajustarStock({ product_id: pid, product_name: nombre }, () => window._verStockItem(pid))
    }

  } catch (err) {
    console.error(err)
    toast('Error', err.message, 'error')
  }
}

/* ═══════════════════════════════════════════════
   GLOBAL HANDLERS
   ═══════════════════════════════════════════════ */
window._stockBack          = () => renderStock()
window._stockAjustarGlobal = () => alert('Selecciona un producto para ajustar')
window._stockFiltro        = (f) => {
  // local filter
  document.querySelectorAll('#stock-content tbody tr').forEach(r => {
    const qty = parseFloat(r.querySelector('td:nth-child(4)')?.textContent || '0')
    if      (f === 'bajo') r.style.display = qty < 10 ? '' : 'none'
    else if (f === 'cero') r.style.display = qty <= 0 ? '' : 'none'
    else                   r.style.display = ''
  })
}
window._chkAllStock = (master) => document.querySelectorAll('#stock-content .o-chk').forEach(c => c.checked = master.checked)
