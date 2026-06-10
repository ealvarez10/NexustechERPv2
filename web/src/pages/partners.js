import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { fmtMxn, fmtDate, fmtNum, paginationHtml, skeletonTable, toast,
         stateBadge } from '../ui.js'
import { api } from '../api.js'
import { editarPartner } from './forms/edit_forms.js'

/* ─── Estado del módulo ─── */
let _currentView = 'list'  // 'list'
let _currentPage = 1
let _records     = []
let _searchQuery = ''
let _filtro      = ''      // '' | 'clientes' | 'proveedores'

/* ═══════════════════════════════════════════════
   ENTRY POINT
   ═══════════════════════════════════════════════ */
export async function renderPartners() {
  ensureLayout()
  _currentView = 'list'
  _currentPage = 1
  _searchQuery = ''
  _filtro      = ''
  setBreadcrumb([{ label: 'Clientes / Proveedores' }])
  _renderControlPanel()
  await _loadAndRender()
}

/* ═══════════════════════════════════════════════
   CONTROL PANEL  (barra superior tipo Odoo)
   ═══════════════════════════════════════════════ */
function _renderControlPanel() {
  setPage(`
  <!-- ── ODOO CONTROL PANEL ── -->
  <div class="o-cp" id="partners-cp">
    <div class="o-cp-left">
      <button class="o-btn-primary" onclick="window._partnerNuevo()">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
        Nuevo
      </button>
    </div>
    <div class="o-cp-center">
      <div class="o-search-bar">
        <svg class="o-search-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/></svg>
        <input id="o-search-partners" class="o-search-input" type="text" placeholder="Buscar…" value="${_searchQuery}">
        <div class="o-search-filters">
          <button class="o-filter-btn" onclick="window._partnerFilter('')" id="pf-all" ${_filtro===''?'data-active':''}>Todos</button>
          <button class="o-filter-btn" onclick="window._partnerFilter('clientes')" id="pf-cli" ${_filtro==='clientes'?'data-active':''}>Clientes</button>
          <button class="o-filter-btn" onclick="window._partnerFilter('proveedores')" id="pf-prov" ${_filtro==='proveedores'?'data-active':''}>Proveedores</button>
        </div>
      </div>
    </div>
    <div class="o-cp-right">
      <div class="o-view-switcher">
        <button class="o-view-btn" data-active title="Vista Lista">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="8" y1="6" x2="21" y2="6"/><line x1="8" y1="12" x2="21" y2="12"/><line x1="8" y1="18" x2="21" y2="18"/><line x1="3" y1="6" x2="3.01" y2="6"/><line x1="3" y1="12" x2="3.01" y2="12"/><line x1="3" y1="18" x2="3.01" y2="18"/></svg>
        </button>
      </div>
    </div>
  </div>
  <!-- ── CONTENT AREA ── -->
  <div id="partners-content" class="o-view-content">
    ${skeletonTable(10, 6)}
  </div>`)

  // Search handler
  setTimeout(() => {
    document.getElementById('o-search-partners')?.addEventListener('input', (e) => {
      _searchQuery = e.target.value.toLowerCase()
      _filterTableLocal()
    })
  }, 100)
}

function _filterTableLocal() {
  document.querySelectorAll('#partners-content tbody tr').forEach(r => {
    r.style.display = r.textContent.toLowerCase().includes(_searchQuery) ? '' : 'none'
  })
}

/* ═══════════════════════════════════════════════
   LOAD & RENDER
   ═══════════════════════════════════════════════ */
async function _loadAndRender() {
  try {
    let fetchFn
    if (_filtro === 'clientes')        fetchFn = api.clientes(_currentPage)
    else if (_filtro === 'proveedores') fetchFn = api.proveedores(_currentPage)
    else                               fetchFn = api.partners(_currentPage)

    const res = await fetchFn
    _records  = res?.data || []
    const hasMore = _records.length >= 20

    const el = document.getElementById('partners-content')
    if (!el) return
    el.innerHTML = _renderList(_records, hasMore)
  } catch (err) {
    console.error(err)
    toast('Error', err.message, 'error')
    const el = document.getElementById('partners-content')
    if (el) el.innerHTML = `<div class="o-empty-state"><p>Error al cargar contactos</p></div>`
  }
}

/* ═══════════════════════════════════════════════
   VISTA LISTA
   ═══════════════════════════════════════════════ */
function _renderList(records, hasMore) {
  if (!records.length) return `
    <div class="o-empty-state">
      <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" opacity=".3"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>
      <p>Sin contactos registrados</p>
    </div>`

  return `
  <div class="o-list-view">
    <table class="o-list-table">
      <thead>
        <tr>
          <th class="o-list-chk"><input type="checkbox" class="o-chk" id="chk-all-partners" onclick="window._chkAllPartners(this)"></th>
          <th class="o-col-sortable">#</th>
          <th class="o-col-sortable">Cliente / Proveedor</th>
          <th>Email</th>
          <th>Teléfono</th>
          <th>Ciudad</th>
          <th>RFC</th>
          <th>Tipo</th>
        </tr>
      </thead>
      <tbody>
        ${records.map((p, i) => {
          const esCliente   = (p.customer_rank || 0) > 0
          const esProveedor = (p.supplier_rank || 0) > 0
          const esEmpresa   = p.is_company
          const name        = p.name || p.nombre || '—'
          const initials    = name.split(' ').map(w => w[0]).slice(0, 2).join('')
          const hue         = (p.id * 37) % 360
          return `
          <tr class="o-list-row" onclick="window._verPartner(${p.id})">
            <td class="o-list-chk"><input type="checkbox" class="o-chk" onclick="event.stopPropagation()"></td>
            <td class="o-list-num">${(_currentPage - 1) * 20 + i + 1}</td>
            <td>
              <div class="o-partner-cell">
                <div class="o-avatar" style="background:linear-gradient(135deg,hsl(${hue},60%,55%),hsl(${(hue+40)%360},70%,45%))">${initials || '?'}</div>
                <div>
                  <div class="o-td-primary">${name}</div>
                  ${esEmpresa ? '<div class="o-td-secondary">Empresa</div>' : ''}
                </div>
              </div>
            </td>
            <td class="o-td-muted">${p.email || '—'}</td>
            <td class="o-td-muted">${p.phone || '—'}</td>
            <td class="o-td-muted">${p.city || '—'}</td>
            <td class="o-td-mono">${p.vat || '—'}</td>
            <td>
              ${esCliente   ? '<span class="o-badge o-badge-success">Cliente</span>' : ''}
              ${esProveedor ? '<span class="o-badge o-badge-info" style="margin-left:2px">Proveedor</span>' : ''}
              ${!esCliente && !esProveedor ? '<span class="o-badge o-badge-gray">Contacto</span>' : ''}
            </td>
          </tr>`
        }).join('')}
      </tbody>
    </table>
    <div class="o-list-footer">
      <span class="o-list-count">${records.length} contacto${records.length !== 1 ? 's' : ''}</span>
      ${paginationHtml(_currentPage, hasMore, (p) => { _currentPage = p; _loadAndRender() })}
    </div>
  </div>`
}

/* ═══════════════════════════════════════════════
   VISTA FORMULARIO  (Odoo full-page form)
   ═══════════════════════════════════════════════ */
window._verPartner = async (id) => {
  setBreadcrumb([
    { label: 'Clientes / Proveedores', onclick: () => renderPartners() },
    { label: 'Cargando…', id: 'bc-partner-name' }
  ])

  setPage(`<div class="o-form-loading">${skeletonTable(4, 3)}</div>`)

  try {
    const p = await api.partner(id)
    if (!p) { toast('Error', 'Contacto no encontrado', 'error'); return }

    // Update breadcrumb label
    const bcEl = document.getElementById('bc-partner-name')
    if (bcEl) bcEl.textContent = p.name || 'Contacto'

    const esCliente   = (p.customer_rank || 0) > 0
    const esProveedor = (p.supplier_rank || 0) > 0
    const esEmpresa   = p.is_company
    const name        = p.name || '—'
    const initials    = name.split(' ').map(w => w[0]).slice(0, 2).join('')
    const hue         = (p.id * 37) % 360

    setPage(`
    <!-- ── FORM BREADCRUMB BAR ── -->
    <div class="o-form-topbar">
      <button class="o-back-btn" onclick="window._partnersBack()">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
        Clientes / Proveedores
      </button>
      <div class="o-form-actions">
        <button class="o-btn-secondary" onclick="window._editarPartnerForm(${p.id})">Editar</button>
        <button class="o-btn-secondary" onclick="alert('Crear factura — próximamente')">Crear Factura</button>
        <button class="o-btn-primary"   onclick="alert('Crear venta — próximamente')">Crear Venta</button>
      </div>
    </div>

    <!-- ── SMART BUTTONS ── -->
    <div class="o-smart-buttons">
      <button class="o-smart-btn" onclick="alert('Ventas del cliente')">
        <span class="o-smart-count">0</span>
        <span class="o-smart-label">Ventas</span>
      </button>
      <button class="o-smart-btn" onclick="alert('Facturas del cliente')">
        <span class="o-smart-count">0</span>
        <span class="o-smart-label">Facturas</span>
      </button>
      <button class="o-smart-btn" onclick="alert('Compras del proveedor')">
        <span class="o-smart-count">0</span>
        <span class="o-smart-label">Compras</span>
      </button>
    </div>

    <!-- ── FORM SHEET ── -->
    <div class="o-form-sheet">
      <div class="o-sheet-header">
        <div class="o-avatar o-avatar-lg" style="background:linear-gradient(135deg,hsl(${hue},60%,55%),hsl(${(hue+40)%360},70%,45%))">${initials || '?'}</div>
        <div class="o-sheet-title-block">
          <h1 class="o-form-title">${name}</h1>
          <div style="display:flex;gap:6px;margin-top:4px">
            ${esCliente   ? '<span class="o-badge o-badge-success">Cliente</span>' : ''}
            ${esProveedor ? '<span class="o-badge o-badge-info">Proveedor</span>' : ''}
            ${esEmpresa   ? '<span class="o-badge o-badge-gray">Empresa</span>'   : '<span class="o-badge o-badge-gray">Persona física</span>'}
          </div>
        </div>
      </div>

      <div class="o-form-grid">
        <!-- Col 1 -->
        <div class="o-form-col">
          <div class="o-field-group">
            <label class="o-field-label">Nombre</label>
            <div class="o-field-value">${p.name || '—'}</div>
          </div>
          <div class="o-field-group">
            <label class="o-field-label">¿Es empresa?</label>
            <div class="o-field-value">${esEmpresa ? 'Sí' : 'No'}</div>
          </div>
          <div class="o-field-group">
            <label class="o-field-label">Email</label>
            <div class="o-field-value">${p.email ? `<a href="mailto:${p.email}" class="o-link">${p.email}</a>` : '—'}</div>
          </div>
          <div class="o-field-group">
            <label class="o-field-label">Teléfono</label>
            <div class="o-field-value">${p.phone || '—'}</div>
          </div>
          <div class="o-field-group">
            <label class="o-field-label">Móvil</label>
            <div class="o-field-value">${p.mobile || '—'}</div>
          </div>
        </div>
        <!-- Col 2 -->
        <div class="o-form-col">
          <div class="o-field-group">
            <label class="o-field-label">RFC (VAT)</label>
            <div class="o-field-value o-field-mono">${p.vat || '—'}</div>
          </div>
          <div class="o-field-group">
            <label class="o-field-label">Página web</label>
            <div class="o-field-value">${p.website ? `<a href="${p.website}" class="o-link" target="_blank">${p.website}</a>` : '—'}</div>
          </div>
          <div class="o-field-group">
            <label class="o-field-label">Ciudad</label>
            <div class="o-field-value">${p.city || '—'}</div>
          </div>
          <div class="o-field-group">
            <label class="o-field-label">Estado</label>
            <div class="o-field-value">${p.state_name || p.state || '—'}</div>
          </div>
          <div class="o-field-group">
            <label class="o-field-label">País</label>
            <div class="o-field-value">${p.country_name || p.country || '—'}</div>
          </div>
        </div>
      </div>

      <!-- ── NOTEBOOK TABS ── -->
      <div class="o-notebook">
        <div class="o-tabs" id="partner-tabs">
          <button class="o-tab active" onclick="window._partnerTab('contactos', this)">Contactos y Direcciones</button>
          <button class="o-tab" onclick="window._partnerTab('ventas', this)">Ventas y Compras</button>
          <button class="o-tab" onclick="window._partnerTab('notas', this)">Notas</button>
        </div>

        <div class="o-tab-pane" id="tab-contactos">
          <p class="o-tab-empty">Sin sub-contactos registrados.</p>
        </div>
        <div class="o-tab-pane" id="tab-ventas" style="display:none">
          <div class="o-form-grid">
            <div class="o-form-col">
              <div class="o-field-group"><label class="o-field-label">Vendedor asignado</label><div class="o-field-value">—</div></div>
              <div class="o-field-group"><label class="o-field-label">Términos de pago</label><div class="o-field-value">—</div></div>
            </div>
            <div class="o-form-col">
              <div class="o-field-group"><label class="o-field-label">Lista de precios</label><div class="o-field-value">—</div></div>
            </div>
          </div>
        </div>
        <div class="o-tab-pane" id="tab-notas" style="display:none">
          <textarea class="o-textarea" rows="5" placeholder="Notas internas…"></textarea>
        </div>
      </div>
    </div>

    <!-- ── CHATTER ── -->
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
            <div class="o-msg-meta"><strong>Sistema</strong> <span>${fmtDate(new Date().toISOString())}</span></div>
            <div class="o-msg-text">Registro creado.</div>
          </div>
        </div>
      </div>
    </div>`)

    window._editarPartnerForm = (pid) => {
      const rec = { id: pid, ...p }
      editarPartner(rec, () => window._verPartner(pid))
    }

    window._partnerTab = (tab, btn) => {
      document.querySelectorAll('#partner-tabs .o-tab').forEach(b => b.classList.remove('active'))
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
window._partnersBack = () => renderPartners()

window._partnerFilter = (f) => {
  _filtro = f
  _currentPage = 1
  // highlight active filter
  document.querySelectorAll('#partners-cp .o-filter-btn').forEach(b => b.removeAttribute('data-active'))
  const idMap = { '': 'pf-all', 'clientes': 'pf-cli', 'proveedores': 'pf-prov' }
  document.getElementById(idMap[f])?.setAttribute('data-active', '')
  const el = document.getElementById('partners-content')
  if (el) el.innerHTML = skeletonTable(8, 6)
  _loadAndRender()
}

window._partnerNuevo = () => alert('Nuevo contacto — próximamente')

window._chkAllPartners = (master) => {
  document.querySelectorAll('#partners-content .o-chk').forEach(c => c.checked = master.checked)
}
