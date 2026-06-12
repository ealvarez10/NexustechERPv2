import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { fmtMxn, fmtDate, fmtNum, paginationHtml, skeletonTable, toast, stateBadge } from '../ui.js'
import { api } from '../api.js'
import { editarCompra } from './forms/edit_forms.js'

/* ─── Estado ─── */
let _currentView = 'list'
let _currentPage = 1
let _records     = []
let _searchQuery = ''
let cfg = {}

const ESTADO = {
  draft:    { lbl: 'Borrador',        cls: 'o-badge-gray',    kanban: 'Borrador'             },
  sent:     { lbl: 'Enviada',         cls: 'o-badge-info',    kanban: 'Enviada al Proveedor' },
  purchase: { lbl: 'Orden de Compra', cls: 'o-badge-success', kanban: 'Órdenes de Compra'    },
  done:     { lbl: 'Realizada',       cls: 'o-badge-warn',    kanban: 'Realizada'            },
  cancel:   { lbl: 'Cancelada',       cls: 'o-badge-danger',  kanban: 'Cancelada'            },
}

const STATUS_BAR = ['draft', 'sent', 'purchase', 'done']

export async function renderCompras() {
  ensureLayout()
  _currentView = 'list'
  _currentPage = 1
  _searchQuery = ''
  
  // Data Binding: Cargar Configuración de Compras
  cfg = {
    bloquear_confirmado: false, advertencias: false, precio_compra: true, descuentos: false,
    politica_facturacion: 'cantidad_pedida', bloquear_factura: false,
    variantes: false, unidades_medida: false, empaquetado: false,
    presupuesto_solicitud: false, recordatorio_recepcion: 0, costos_aterrizaje: false,
    ...JSON.parse(localStorage.getItem('nexus_config_compras') || '{}')
  }

  setBreadcrumb([{ label: 'Compras' }])
  _renderControlPanel()
  await _loadAndRender()
}

function _renderControlPanel() {
  setPage(`
  <div class="o-cp" id="compras-cp">
    <div class="o-cp-left">
      <button class="o-btn-primary" onclick="window._compraNueva()">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
        Nuevo
      </button>
      ${cfg.presupuesto_solicitud ? `<button class="o-btn-secondary" onclick="alert('Pedir Presupuestos')">Solicitar Presupuestos</button>` : ''}
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
      <button class="o-btn-secondary" style="margin-right:8px;font-size:16px;padding:4px 8px" onclick="window._go('config_compras')" title="Ajustes">⚙️</button>
      <div class="o-view-switcher">
        <button class="o-view-btn o-active" onclick="window._compraSetView('list')" title="Lista">☰</button>
        <button class="o-view-btn" onclick="window._compraSetView('kanban')" title="Kanban">⬜</button>
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

function _renderList(records, hasMore) {
  if (!records.length) return `<div class="o-empty-state"><p>Sin órdenes de compra</p></div>`

  return `
  <div class="o-list-view">
    <table class="o-list-table">
      <thead>
        <tr>
          <th class="o-list-chk"><input type="checkbox" class="o-chk" onclick="window._chkAllCompras(this)"></th>
          <th class="o-col-sortable">Referencia</th>
          <th class="o-col-sortable">Proveedor</th>
          ${cfg.recordatorio_recepcion > 0 ? '<th>Recepción Esperada</th>' : ''}
          <th>Fecha límite</th>
          <th class="o-col-right">Total</th>
          <th>Estado</th>
        </tr>
      </thead>
      <tbody>
        ${records.map(c => {
          const e = ESTADO[c.state] || { lbl: c.state || '—', cls: 'o-badge-gray' }
          return `
          <tr class="o-list-row" onclick="window._verCompra(${c.id})">
            <td class="o-list-chk"><input type="checkbox" class="o-chk" onclick="event.stopPropagation()"></td>
            <td class="o-td-mono o-td-primary" style="font-weight:700">${c.name || `#${c.id}`}</td>
            <td class="o-td-primary">${c.partner_name || '—'} ${cfg.advertencias ? '<span style="color:#DC2626;font-size:11px" title="Alerta configurada">⚠️</span>' : ''}</td>
            ${cfg.recordatorio_recepcion > 0 ? `<td><span style="color:var(--text-400)">⏳ En ${cfg.recordatorio_recepcion} días</span></td>` : ''}
            <td class="o-td-muted">${fmtDate(c.date_order)}</td>
            <td class="o-td-amount" style="font-weight:700">${fmtMxn(parseFloat(c.amount_total || 0))}</td>
            <td><span class="o-badge ${e.cls}">${e.lbl}</span></td>
          </tr>`
        }).join('')}
      </tbody>
    </table>
    <div class="o-list-footer">
      <span class="o-list-count">${records.length} registros</span>
      ${paginationHtml(_currentPage, hasMore, (p) => { _currentPage = p; _loadAndRender() })}
    </div>
  </div>`
}

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
            <div class="o-kanban-sub">${c.partner_name || '—'} ${cfg.advertencias ? '⚠️' : ''}</div>
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

window._verCompra = async (id) => {
  setBreadcrumb([
    { label: 'Compras', onclick: () => renderCompras() },
    { label: 'Cargando…', id: 'bc-compra-name' }
  ])
  setPage(`<div class="o-form-loading">${skeletonTable(4, 3)}</div>`)

  try {
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
    const lineas = c.order_line || c.lineas || []

    const readonly = cfg.bloquear_confirmado && (c.state === 'purchase' || c.state === 'done')

    setPage(`
    <div class="o-form-topbar">
      <button class="o-back-btn" onclick="window._comprasBack()">← Compras</button>
      <div class="o-form-actions">
        ${c.state === 'draft' ? `<button class="o-btn-primary" onclick="alert('Confirmar OC')">Confirmar OC</button>` : ''}
        ${c.state === 'purchase' ? `<button class="o-btn-secondary" onclick="alert('Recibir mercancía')">Recibir Productos</button>` : ''}
        ${c.state === 'purchase' || c.state === 'done' ? `<button class="o-btn-secondary" onclick="alert('Crear Factura de Proveedor')">Crear Factura</button>` : ''}
        ${!readonly ? `<button class="o-btn-secondary" onclick="window._editarCompraForm(${c.id})">Editar</button>` : ''}
        ${readonly ? `<span style="font-size:11px;color:var(--text-400);margin-left:10px">Bloqueado por configuración</span>` : ''}
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

    <div class="o-smart-buttons">
      <button class="o-smart-btn"><span class="o-smart-count">0</span><span class="o-smart-label">Recepciones</span></button>
      <button class="o-smart-btn"><span class="o-smart-count">0</span><span class="o-smart-label">Facturas</span></button>
      ${cfg.costos_aterrizaje ? `<button class="o-smart-btn"><span class="o-smart-count" style="color:var(--primary)">$0</span><span class="o-smart-label">Costos Aterr.</span></button>` : ''}
    </div>

    <div class="o-form-sheet">
      <div class="o-sheet-header">
        <h1 class="o-form-title">${c.name || 'Nueva Orden'}</h1>
        <span class="o-badge ${e.cls}">${e.lbl}</span>
      </div>

      <div class="o-form-grid">
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">Proveedor</label><div class="o-field-value o-td-primary">${c.partner_name || '—'}</div></div>
          <div class="o-field-group"><label class="o-field-label">Referencia Proveedor</label><div class="o-field-value">${c.partner_ref || '—'}</div></div>
          ${cfg.presupuesto_solicitud ? `<div class="o-field-group"><label class="o-field-label">Acuerdo / Licitación</label><div class="o-field-value">Ninguno</div></div>` : ''}
        </div>
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">Fecha Límite</label><div class="o-field-value">${fmtDate(c.date_order)}</div></div>
          <div class="o-field-group"><label class="o-field-label">Facturar por</label><div class="o-field-value">${cfg.politica_facturacion === 'cantidad_pedida' ? 'Cantidades pedidas' : 'Cantidades recibidas'}</div></div>
        </div>
      </div>

      <div class="o-notebook">
        <div class="o-tabs" id="compra-tabs">
          <button class="o-tab active" onclick="window._compraTab('productos', this)">Productos</button>
          <button class="o-tab" onclick="window._compraTab('adicional', this)">Información</button>
        </div>

        <div class="o-tab-pane" id="tab-productos">
          ${lineas.length ? `
          <table class="o-list-table">
            <thead><tr>
              <th>Producto</th>
              ${cfg.variantes ? '<th>Variante</th>' : ''}
              ${cfg.empaquetado ? '<th>Empaque</th>' : ''}
              <th class="o-col-right">Qty</th>
              ${cfg.unidades_medida ? '<th>UdM</th>' : ''}
              <th class="o-col-right">Precio</th>
              ${cfg.descuentos ? '<th class="o-col-right">Desc.%</th>' : ''}
              <th class="o-col-right">Subtotal</th>
            </tr></thead>
            <tbody>
              ${lineas.map(l => `
              <tr>
                <td class="o-td-primary">${l.product_name || l.name || '—'}</td>
                ${cfg.variantes ? `<td><span style="font-size:11px;background:#E5E7EB;padding:2px 6px;border-radius:4px">Predeterminada</span></td>` : ''}
                ${cfg.empaquetado ? `<td>Caja x1</td>` : ''}
                <td class="o-td-amount">${fmtNum(parseFloat(l.product_qty || l.qty || 0))}</td>
                ${cfg.unidades_medida ? `<td>PZ</td>` : ''}
                <td class="o-td-amount">${fmtMxn(parseFloat(l.price_unit || 0))}</td>
                ${cfg.descuentos ? `<td class="o-td-amount o-td-muted">0%</td>` : ''}
                <td class="o-td-amount" style="font-weight:700">${fmtMxn(parseFloat(l.price_subtotal || 0))}</td>
              </tr>`).join('')}
            </tbody>
          </table>` : `<div class="o-empty-state"><p>Sin líneas</p></div>`}
          
          <div class="o-form-totals">
            <div class="o-total-row"><span>Subtotal</span><span>${fmtMxn(parseFloat(c.amount_untaxed || 0))}</span></div>
            <div class="o-total-row"><span>IVA</span><span>${fmtMxn(parseFloat(c.amount_tax || 0))}</span></div>
            <div class="o-total-row o-total-final"><span>Total</span><span>${fmtMxn(parseFloat(c.amount_total || 0))}</span></div>
          </div>
        </div>
        <div class="o-tab-pane" id="tab-adicional" style="display:none">
          <div class="o-field-group"><label class="o-field-label">Notas</label><textarea class="o-textarea" rows="3" ${readonly ? 'disabled' : ''}>${c.notes || ''}</textarea></div>
        </div>
      </div>
    </div>
    `)

    window._editarCompraForm = (cid) => editarCompra({ id: cid, ...c }, () => window._verCompra(cid))
    window._compraTab = (tab, btn) => {
      document.querySelectorAll('#compra-tabs .o-tab').forEach(b => b.classList.remove('active'))
      btn.classList.add('active')
      document.querySelectorAll('.o-tab-pane').forEach(el => el.style.display = 'none')
      const pane = document.getElementById(`tab-${tab}`)
      if (pane) pane.style.display = ''
    }
  } catch (err) {
    toast('Error', err.message, 'error')
  }
}

window._comprasBack = () => renderCompras()
window._compraNueva = () => { import('./forms/create_forms.js').then(m => m.nuevaCompra(() => renderCompras())) }
window._compraSetView = (view) => { _currentView = view; _renderControlPanel(); _loadAndRender() }
window._compraFiltroEstado = (e) => { _searchQuery = e; _filterTableLocal() }
window._chkAllCompras = (m) => document.querySelectorAll('#compras-content .o-chk').forEach(c => c.checked = m.checked)
