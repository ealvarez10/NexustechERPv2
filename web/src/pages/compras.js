import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { fmtMxn, fmtDate, fmtNum, paginationHtml, skeletonTable, toast, stateBadge } from '../ui.js'
import { api } from '../api.js'
import { editarCompra } from './forms/edit_forms.js'

/* ─── Estado ─── */
let _currentView = 'list'   // 'list' | 'kanban'
let _currentPage = 1
let _records     = []
let _searchQuery = ''

const ESTADO = {
  draft:    { lbl: 'Borrador',        cls: 'o-badge-gray',    kanban: 'Borrador'             },
  sent:     { lbl: 'Enviada',         cls: 'o-badge-info',    kanban: 'Enviada al Proveedor' },
  purchase: { lbl: 'Orden de Compra', cls: 'o-badge-success', kanban: 'Órdenes de Compra'    },
  done:     { lbl: 'Realizada',       cls: 'o-badge-warn',    kanban: 'Realizada'            },
  cancel:   { lbl: 'Cancelada',       cls: 'o-badge-danger',  kanban: 'Cancelada'            },
}

const STATUS_BAR = ['draft', 'sent', 'purchase', 'done']

/* ═══════════════════════════════════════════════
   ENTRY POINT
   ═══════════════════════════════════════════════ */
export async function renderCompras() {
  ensureLayout()
  _currentView = 'list'
  _currentPage = 1
  _searchQuery = ''
  setBreadcrumb([{ label: 'Compras' }])
  _renderControlPanel()
  await _loadAndRender()
}

/* ═══════════════════════════════════════════════
   CONTROL PANEL
   ═══════════════════════════════════════════════ */
function _renderControlPanel() {
  setPage(`
  <div class="o-cp" id="compras-cp">
    <div class="o-cp-left">
      <button class="o-btn-primary" onclick="window._compraNueva()">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
        Nuevo
      </button>
    </div>
    <div class="o-cp-center">
      <div class="o-search-bar">
        <svg class="o-search-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/></svg>
        <input id="o-search-compras" class="o-search-input" type="text" placeholder="Buscar folio o proveedor…" value="${_searchQuery}">
        <div class="o-search-filters">
          <button class="o-filter-btn" onclick="window._compraFiltroEstado('draft')">Borrador</button>
          <button class="o-filter-btn" onclick="window._compraFiltroEstado('purchase')">Confirmadas</button>
          <button class="o-filter-btn" onclick="window._compraFiltroEstado('done')">Realizadas</button>
        </div>
      </div>
    </div>
    <div class="o-cp-right">
      <div class="o-view-switcher">
        <button class="o-view-btn o-active" onclick="window._compraSetView('list')" title="Lista">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="8" y1="6" x2="21" y2="6"/><line x1="8" y1="12" x2="21" y2="12"/><line x1="8" y1="18" x2="21" y2="18"/><line x1="3" y1="6" x2="3.01" y2="6"/><line x1="3" y1="12" x2="3.01" y2="12"/><line x1="3" y1="18" x2="3.01" y2="18"/></svg>
        </button>
        <button class="o-view-btn" onclick="window._compraSetView('kanban')" title="Kanban">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="1" y="4" width="6" height="16" rx="1"/><rect x="9" y="4" width="6" height="10" rx="1"/><rect x="17" y="4" width="6" height="13" rx="1"/></svg>
        </button>
      </div>
    </div>
  </div>
  <div id="compras-content" class="o-view-content">
    ${skeletonTable(8, 6)}
  </div>`)

  setTimeout(() => {
    document.getElementById('o-search-compras')?.addEventListener('input', (e) => {
      _searchQuery = e.target.value.toLowerCase()
      _filterTableLocal()
    })
  }, 100)
}

function _filterTableLocal() {
  document.querySelectorAll('#compras-content .o-list-row, #compras-content .o-kanban-card').forEach(r => {
    r.style.display = r.textContent.toLowerCase().includes(_searchQuery) ? '' : 'none'
  })
}

/* ═══════════════════════════════════════════════
   LOAD & RENDER
   ═══════════════════════════════════════════════ */
async function _loadAndRender() {
  try {
    const res  = await api.compras(_currentPage)
    _records   = res?.data || []
    const hasMore = _records.length >= 20

    const el = document.getElementById('compras-content')
    if (!el) return
    if (_currentView === 'kanban') el.innerHTML = _renderKanban(_records)
    else el.innerHTML = _renderList(_records, hasMore)
  } catch (err) {
    console.error(err)
    toast('Error', err.message, 'error')
  }
}

/* ═══════════════════════════════════════════════
   VISTA LISTA
   ═══════════════════════════════════════════════ */
function _renderList(records, hasMore) {
  if (!records.length) return `<div class="o-empty-state"><p>Sin órdenes de compra</p></div>`

  return `
  <div class="o-list-view">
    <table class="o-list-table">
      <thead>
        <tr>
          <th class="o-list-chk"><input type="checkbox" class="o-chk" onclick="window._chkAllCompras(this)"></th>
          <th class="o-col-sortable">Número</th>
          <th class="o-col-sortable">Proveedor</th>
          <th>Fecha</th>
          <th>Fecha Esperada</th>
          <th>Estado</th>
          <th class="o-col-right">Total</th>
        </tr>
      </thead>
      <tbody>
        ${records.map(c => {
          const e = ESTADO[c.state] || { lbl: c.state || '—', cls: 'o-badge-gray' }
          return `
          <tr class="o-list-row" onclick="window._verCompra(${c.id})">
            <td class="o-list-chk"><input type="checkbox" class="o-chk" onclick="event.stopPropagation()"></td>
            <td class="o-td-mono o-td-primary">${c.name || `#${c.id}`}</td>
            <td class="o-td-primary">${c.partner_name || '—'}</td>
            <td class="o-td-muted">${fmtDate(c.date_order)}</td>
            <td class="o-td-muted">${fmtDate(c.date_planned)}</td>
            <td><span class="o-badge ${e.cls}">${e.lbl}</span></td>
            <td class="o-td-amount" style="font-weight:700">${fmtMxn(parseFloat(c.amount_total || 0))}</td>
          </tr>`
        }).join('')}
      </tbody>
    </table>
    <div class="o-list-footer">
      <span class="o-list-count">${records.length} orden${records.length !== 1 ? 'es' : ''}</span>
      ${paginationHtml(_currentPage, hasMore, (p) => { _currentPage = p; _loadAndRender() })}
    </div>
  </div>`
}

/* ═══════════════════════════════════════════════
   VISTA KANBAN  (columnas por estado)
   ═══════════════════════════════════════════════ */
function _renderKanban(records) {
  const cols = ['draft', 'sent', 'purchase', 'done']
  return `
  <div class="o-kanban-columns">
    ${cols.map(estado => {
      const info  = ESTADO[estado]
      const group = records.filter(c => c.state === estado)
      const total = group.reduce((a, c) => a + parseFloat(c.amount_total || 0), 0)
      return `
      <div class="o-kanban-col">
        <div class="o-kanban-col-header">
          <span class="o-badge ${info.cls}">${info.kanban}</span>
          <span class="o-kanban-col-count">${group.length}</span>
        </div>
        <div class="o-kanban-col-sum">${fmtMxn(total)}</div>
        <div class="o-kanban-col-cards">
          ${group.map(c => `
          <div class="o-kanban-card" onclick="window._verCompra(${c.id})">
            <div class="o-kanban-title">${c.name || '#' + c.id}</div>
            <div class="o-kanban-sub">${c.partner_name || '—'}</div>
            <div style="display:flex;justify-content:space-between;margin-top:8px">
              <span class="o-td-muted" style="font-size:12px">${fmtDate(c.date_order)}</span>
              <strong>${fmtMxn(parseFloat(c.amount_total || 0))}</strong>
            </div>
          </div>`).join('')}
          ${group.length === 0 ? '<div class="o-kanban-empty-col">Sin órdenes</div>' : ''}
        </div>
      </div>`
    }).join('')}
  </div>`
}

/* ═══════════════════════════════════════════════
   VISTA FORMULARIO
   ═══════════════════════════════════════════════ */
window._verCompra = async (id) => {
  setBreadcrumb([
    { label: 'Compras', onclick: () => renderCompras() },
    { label: 'Cargando…', id: 'bc-compra-name' }
  ])
  setPage(`<div class="o-form-loading">${skeletonTable(4, 3)}</div>`)

  try {
    // Try fetching single record; fallback to cached
    let c = _records.find(x => x.id === id)
    try {
      const fresh = await api.compra(id)
      if (fresh && (fresh.id || fresh.name)) c = fresh
    } catch (_) {}
    if (!c) { toast('Error', 'Orden no encontrada', 'error'); return }

    const bcEl = document.getElementById('bc-compra-name')
    if (bcEl) bcEl.textContent = c.name || `Compra #${id}`

    const e = ESTADO[c.state] || { lbl: c.state || '—', cls: 'o-badge-gray' }
    const statusIdx = STATUS_BAR.indexOf(c.state)

    // Build lineas if available
    const lineas = c.order_line || c.lineas || []

    setPage(`
    <div class="o-form-topbar">
      <button class="o-back-btn" onclick="window._comprasBack()">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
        Compras
      </button>
      <div class="o-form-actions">
        ${c.state === 'draft' ? `<button class="o-btn-primary" onclick="alert('Confirmar OC — próximamente')">Confirmar OC</button>` : ''}
        ${c.state === 'purchase' ? `<button class="o-btn-secondary" onclick="alert('Recibir mercancía — próximamente')">Recibir</button>` : ''}
        <button class="o-btn-secondary" onclick="alert('Crear factura — próximamente')">Crear Factura</button>
        <button class="o-btn-secondary" onclick="window._editarCompraForm(${c.id})">Editar</button>
      </div>
    </div>

    <!-- STATUS BAR -->
    <div class="o-status-bar">
      ${STATUS_BAR.map((s, i) => {
        const info = ESTADO[s]
        const active  = i === statusIdx
        const done    = i < statusIdx
        return `<div class="o-status-step ${active ? 'active' : done ? 'done' : ''}">${info.lbl}</div>`
      }).join('<div class="o-status-arrow">›</div>')}
    </div>

    <!-- SMART BUTTONS -->
    <div class="o-smart-buttons">
      <button class="o-smart-btn" onclick="alert('Facturas de esta OC')">
        <span class="o-smart-count">0</span>
        <span class="o-smart-label">Facturas</span>
      </button>
      <button class="o-smart-btn" onclick="alert('Recepciones de esta OC')">
        <span class="o-smart-count">0</span>
        <span class="o-smart-label">Recepciones</span>
      </button>
    </div>

    <!-- FORM SHEET -->
    <div class="o-form-sheet">
      <div class="o-sheet-header">
        <div class="o-sheet-title-block">
          <h1 class="o-form-title">${c.name || 'Nueva Orden de Compra'}</h1>
          <span class="o-badge ${e.cls}">${e.lbl}</span>
        </div>
      </div>

      <div class="o-form-grid">
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">Proveedor</label><div class="o-field-value o-td-primary">${c.partner_name || '—'}</div></div>
          <div class="o-field-group"><label class="o-field-label">Fecha de Orden</label><div class="o-field-value">${fmtDate(c.date_order)}</div></div>
          <div class="o-field-group"><label class="o-field-label">Referencia Proveedor</label><div class="o-field-value o-td-mono">${c.partner_ref || '—'}</div></div>
        </div>
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">Responsable</label><div class="o-field-value">${c.user_name || c.user || '—'}</div></div>
          <div class="o-field-group"><label class="o-field-label">Empresa</label><div class="o-field-value">${c.company_name || c.company || 'NexusTech'}</div></div>
          <div class="o-field-group"><label class="o-field-label">Términos de Pago</label><div class="o-field-value">${c.payment_term_name || c.payment_term || '—'}</div></div>
        </div>
      </div>

      <!-- NOTEBOOK -->
      <div class="o-notebook">
        <div class="o-tabs" id="compra-tabs">
          <button class="o-tab active" onclick="window._compraTab('productos', this)">Productos</button>
          <button class="o-tab" onclick="window._compraTab('adicional', this)">Información Adicional</button>
        </div>

        <div class="o-tab-pane" id="tab-productos">
          ${lineas.length ? `
          <table class="o-list-table">
            <thead><tr><th>Producto</th><th>Descripción</th><th class="o-col-right">Cantidad</th><th class="o-col-right">Precio</th><th class="o-col-right">Subtotal</th></tr></thead>
            <tbody>
              ${lineas.map(l => `
              <tr>
                <td class="o-td-primary">${l.product_name || l.name || '—'}</td>
                <td class="o-td-muted">${l.name || l.description || '—'}</td>
                <td class="o-td-amount">${fmtNum(parseFloat(l.product_qty || l.qty || 0))}</td>
                <td class="o-td-amount">${fmtMxn(parseFloat(l.price_unit || 0))}</td>
                <td class="o-td-amount" style="font-weight:700">${fmtMxn(parseFloat(l.price_subtotal || 0))}</td>
              </tr>`).join('')}
            </tbody>
          </table>` : `<div class="o-empty-state" style="padding:32px 0"><p>Sin líneas de productos</p></div>`}
          <div class="o-form-totals">
            <div class="o-total-row"><span>Subtotal</span><span>${fmtMxn(parseFloat(c.amount_untaxed || 0))}</span></div>
            <div class="o-total-row"><span>IVA</span><span>${fmtMxn(parseFloat(c.amount_tax || 0))}</span></div>
            <div class="o-total-row o-total-final"><span>Total</span><span>${fmtMxn(parseFloat(c.amount_total || 0))}</span></div>
          </div>
        </div>
        <div class="o-tab-pane" id="tab-adicional" style="display:none">
          <div class="o-field-group"><label class="o-field-label">Nota / Términos</label>
            <textarea class="o-textarea" rows="4">${c.notes || c.note || ''}</textarea></div>
        </div>
      </div>
    </div>

    <!-- CHATTER -->
    <div class="o-chatter">
      <div class="o-chatter-header">Registro de actividad</div>
      <div class="o-chatter-composer">
        <div class="o-avatar o-avatar-sm" style="background:var(--o-primary)">U</div>
        <input class="o-chatter-input" type="text" placeholder="Escribe un mensaje o nota interna…">
        <button class="o-btn-primary o-btn-sm">Enviar</button>
      </div>
      <div class="o-chatter-messages">
        <div class="o-msg">
          <div class="o-avatar o-avatar-sm" style="background:var(--o-primary)">S</div>
          <div class="o-msg-body">
            <div class="o-msg-meta"><strong>Sistema</strong> <span>${fmtDate(c.date_order || new Date().toISOString())}</span></div>
            <div class="o-msg-text">Orden de compra creada.</div>
          </div>
        </div>
      </div>
    </div>`)

    window._editarCompraForm = (cid) => editarCompra({ id: cid, ...c }, () => window._verCompra(cid))
    window._compraTab = (tab, btn) => {
      document.querySelectorAll('#compra-tabs .o-tab').forEach(b => b.classList.remove('active'))
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
window._comprasBack = () => renderCompras()
window._compraNueva = () => { import('./forms/create_forms.js').then(m => m.nuevaCompra(() => renderCompras())) }

window._compraSetView = (view) => {
  _currentView = view
  document.querySelectorAll('#compras-cp .o-view-btn').forEach(b => b.classList.remove('o-active'))
  const idx = view === 'list' ? 0 : 1
  document.querySelectorAll('#compras-cp .o-view-btn')[idx]?.classList.add('o-active')
  const el = document.getElementById('compras-content')
  if (el) {
    if (view === 'kanban') el.innerHTML = _renderKanban(_records)
    else el.innerHTML = _renderList(_records, false)
  }
}

window._compraFiltroEstado = (estado) => {
  const filtered = _records.filter(c => c.state === estado)
  const el = document.getElementById('compras-content')
  if (el) el.innerHTML = _renderList(filtered, false)
}

window._chkAllCompras = (master) => document.querySelectorAll('#compras-content .o-chk').forEach(c => c.checked = master.checked)
