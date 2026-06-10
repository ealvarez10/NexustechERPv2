/**
 * Ventas — Lista / Kanban / Form — Odoo Enterprise Style
 */
import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { fmtMxn, fmtDate, paginationHtml, skeletonTable, toast, stateBadge } from '../ui.js'
import { api } from '../api.js'
import { openNuevaVenta } from './forms/venta_form.js'
import { openFormPage, fieldGroupHtml, smartButtonsHtml, chatterHtml } from '../components/form_view.js'

const ESTADO_MAP = {
  sale:   { lbl: 'Confirmada', color: 'indigo',  step: 1 },
  done:   { lbl: 'Entregada',  color: 'emerald', step: 2 },
  draft:  { lbl: 'Borrador',   color: 'gray',    step: 0 },
  cancel: { lbl: 'Cancelada',  color: 'red',     step: -1 },
  sent:   { lbl: 'Enviada',    color: 'sky',     step: 1 },
}

const KANBAN_COLS = [
  { key: 'draft',  label: 'Borrador',    color: '#9CA3AF' },
  { key: 'sent',   label: 'Enviada',     color: '#0EA5E9' },
  { key: 'sale',   label: 'Confirmada',  color: '#4F46E5' },
  { key: 'done',   label: 'Entregada',   color: '#059669' },
  { key: 'cancel', label: 'Cancelada',   color: '#DC2626' },
]

let _page = 1
let _total = 0
let _currentView = 'list'   // 'list' | 'kanban'
let _ordenes = []
let _searchQ = ''

// ─── Entry point ─────────────────────────────────────────────────────────────

export async function renderVentas() {
  ensureLayout()
  setBreadcrumb([{ label: 'Inicio', href: 'home' }, { label: 'Ventas' }])
  _page = 1
  _currentView = localStorage.getItem('ventas_view') || 'list'
  await loadVentas()
}

// ─── Control Panel ────────────────────────────────────────────────────────────

function controlPanelHtml() {
  return `
  <div class="o-control-panel" id="ventas-cp">
    <div class="o-cp-left">
      <button class="o-btn-new" onclick="window._nuevaVenta()">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3"><path d="M12 5v14M5 12h14"/></svg>
        Nuevo
      </button>
      <div class="o-search-box">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#9CA3AF" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/></svg>
        <input type="text" id="ventas-search" placeholder="Buscar..." value="${_searchQ}" autocomplete="off">
      </div>
      <button class="o-btn-filter">
        <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3"/></svg>
        Filtros
      </button>
      <button class="o-btn-group">
        <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="5" r="1"/><circle cx="12" cy="12" r="1"/><circle cx="12" cy="19" r="1"/></svg>
        Agrupar
      </button>
    </div>
    <div class="o-cp-right">
      <span class="o-record-count" id="ventas-count">${_total > 0 ? `${_total} registros` : ''}</span>
      <div class="o-view-switcher">
        <button class="o-view-btn${_currentView === 'list' ? ' active' : ''}"
                id="view-btn-list" title="Vista Lista"
                onclick="window._switchVentaView('list')">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="8" y1="6" x2="21" y2="6"/><line x1="8" y1="12" x2="21" y2="12"/><line x1="8" y1="18" x2="21" y2="18"/><line x1="3" y1="6" x2="3.01" y2="6"/><line x1="3" y1="12" x2="3.01" y2="12"/><line x1="3" y1="18" x2="3.01" y2="18"/></svg>
        </button>
        <button class="o-view-btn${_currentView === 'kanban' ? ' active' : ''}"
                id="view-btn-kanban" title="Vista Kanban"
                onclick="window._switchVentaView('kanban')">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="7" height="7"/><rect x="14" y="3" width="7" height="7"/><rect x="3" y="14" width="7" height="7"/><rect x="14" y="14" width="7" height="7"/></svg>
        </button>
      </div>
    </div>
  </div>`
}

// ─── Lista ────────────────────────────────────────────────────────────────────

function listHtml(ventas) {
  if (ventas.length === 0) {
    return `<div class="empty-state"><div class="empty-state-icon">📋</div>
      <div class="empty-state-title">Sin órdenes de venta</div>
      <div class="empty-state-desc">Crea tu primera orden de venta haciendo clic en "+ Nuevo"</div></div>`
  }

  return `
  <div class="o-list-view">
    <table>
      <thead>
        <tr>
          <th><input type="checkbox" class="o-list-checkbox" id="chk-all" onchange="window._checkAll(this)"></th>
          <th>Folio <span class="sort-icon">↕</span></th>
          <th>Cliente <span class="sort-icon">↕</span></th>
          <th>Fecha <span class="sort-icon">↕</span></th>
          <th>Subtotal <span class="sort-icon">↕</span></th>
          <th>Total <span class="sort-icon">↕</span></th>
          <th>Factura</th>
          <th>Estado</th>
        </tr>
      </thead>
      <tbody>
        ${ventas.map(v => {
          const e = ESTADO_MAP[v.state] || { lbl: v.state || '—', color: 'gray' }
          const fecha = v.date_order ? fmtDate(v.date_order) : '—'
          const inv = v.invoice_status === 'invoiced' ? 'Facturada' :
                      v.invoice_status === 'to invoice' ? 'Por facturar' : '—'
          return `
          <tr onclick="window._verVenta(${v.id})" title="Ver detalle">
            <td onclick="event.stopPropagation()">
              <input type="checkbox" class="o-list-checkbox row-chk" data-id="${v.id}"
                     onchange="window._onRowCheck()">
            </td>
            <td class="td-mono">${v.name || `#${v.id}`}</td>
            <td class="td-primary">${v.partner_name || '—'}</td>
            <td>${fecha}</td>
            <td class="td-amount">${fmtMxn(parseFloat(v.amount_untaxed || 0))}</td>
            <td class="td-amount" style="font-weight:700">${fmtMxn(parseFloat(v.amount_total || 0))}</td>
            <td><span class="badge badge-${inv === 'Facturada' ? 'emerald' : inv === 'Por facturar' ? 'amber' : 'gray'}" style="font-size:10px">${inv}</span></td>
            <td>${stateBadge(v.state, e.lbl)}</td>
          </tr>`
        }).join('')}
      </tbody>
    </table>
    ${paginationHtml(_page, ventas.length >= 20, (p) => { _page = p; loadVentas() })}
  </div>`
}

// ─── Kanban ───────────────────────────────────────────────────────────────────

function kanbanHtml(ventas) {
  return `
  <div class="o-kanban-view">
    ${KANBAN_COLS.map(col => {
      const cards = ventas.filter(v => v.state === col.key)
      const total = cards.reduce((s, v) => s + parseFloat(v.amount_total || 0), 0)
      return `
      <div class="o-kanban-col">
        <div class="o-kanban-col-header" style="border-top:3px solid ${col.color}">
          <span>${col.label}</span>
          <span class="o-kanban-col-count">${cards.length}</span>
        </div>
        <div class="o-kanban-cards">
          ${cards.length === 0
            ? `<div style="text-align:center;padding:20px;color:var(--text-300);font-size:12px">Sin registros</div>`
            : cards.map(v => `
              <div class="o-kanban-card" onclick="window._verVenta(${v.id})">
                <div class="o-kanban-card-title">${v.partner_name || '—'}</div>
                <div style="font-size:11px;color:var(--text-400);margin-bottom:8px">${v.name || `#${v.id}`}</div>
                <div class="o-kanban-card-meta">
                  <span>${v.date_order ? fmtDate(v.date_order) : '—'}</span>
                  <span class="o-kanban-card-amount">${fmtMxn(parseFloat(v.amount_total || 0))}</span>
                </div>
              </div>
            `).join('')}
        </div>
        ${cards.length > 0 ? `<div style="padding:10px 16px;font-size:12px;color:var(--text-400);border-top:1px solid var(--border);font-weight:600">Total: ${fmtMxn(total)}</div>` : ''}
      </div>`
    }).join('')}
  </div>`
}

// ─── Load ─────────────────────────────────────────────────────────────────────

async function loadVentas() {
  // Skeleton mientras carga
  setPage(`
    ${controlPanelHtml()}
    <div id="ventas-kpis" style="display:grid;grid-template-columns:repeat(4,1fr);gap:12px;padding:16px 20px">
      ${[1,2,3,4].map(() => `<div class="data-card" style="padding:16px"><div class="skeleton" style="height:40px"></div></div>`).join('')}
    </div>
    <div id="ventas-content" style="padding:0 20px 20px">
      <div class="data-card">${skeletonTable(8, 7)}</div>
    </div>
  `)

  try {
    const [kpisRes, listRes] = await Promise.allSettled([
      api.ventaKpis(),
      api.ventas(_page),
    ])

    // KPIs
    const kpis = kpisRes.status === 'fulfilled' ? (kpisRes.value?.data || kpisRes.value) : null
    const kpiEl = document.getElementById('ventas-kpis')
    if (kpiEl && kpis) {
      kpiEl.innerHTML = [
        { label: 'Total Órdenes',   val: kpis.ordenes_confirmadas ?? kpis.total_ordenes ?? 0, tipo: 'num', color: 'indigo'  },
        { label: 'Facturado Total', val: kpis.total_facturado ?? 0,                            tipo: 'mxn', color: 'emerald' },
        { label: 'Ticket Promedio', val: kpis.ticket_promedio ?? 0,                            tipo: 'mxn', color: 'violet'  },
        { label: 'Este Mes',        val: kpis.ordenes_este_mes ?? 0,                           tipo: 'num', color: 'amber'   },
      ].map(k => `
        <div class="kpi-card kpi-${k.color}" style="padding:16px">
          <div class="kpi-label">${k.label}</div>
          <div class="kpi-value" style="font-size:22px">
            ${k.tipo === 'mxn' ? fmtMxn(parseFloat(k.val)) : Number(k.val).toLocaleString('es-MX')}
          </div>
        </div>`).join('')
    }

    // Lista
    const ventas = listRes.status === 'fulfilled' ? (listRes.value?.data || listRes.value || []) : []
    _ordenes = Array.isArray(ventas) ? ventas : []
    _total = listRes.value?.total ?? _ordenes.length
    if (listRes.status === 'fulfilled' && listRes.value?.pagination?.total) {
      _total = listRes.value.pagination.total
    }

    // Update count
    const countEl = document.getElementById('ventas-count')
    if (countEl) countEl.textContent = `${_total} registros · Pág. ${_page}`

    // Render content
    renderContent()

    // Search
    document.getElementById('ventas-search')?.addEventListener('input', (e) => {
      _searchQ = e.target.value.toLowerCase()
      if (_currentView === 'list') {
        document.querySelectorAll('#ventas-content tbody tr').forEach(row => {
          row.style.display = row.textContent.toLowerCase().includes(_searchQ) ? '' : 'none'
        })
      } else {
        document.querySelectorAll('#ventas-content .o-kanban-card').forEach(card => {
          card.style.display = card.textContent.toLowerCase().includes(_searchQ) ? '' : 'none'
        })
      }
    })

  } catch (err) {
    console.error(err)
    toast('Error al cargar ventas', err.message, 'error')
    const contentEl = document.getElementById('ventas-content')
    if (contentEl) contentEl.innerHTML = `<p style="text-align:center;padding:32px;color:var(--danger)">Error de conexión: ${err.message}</p>`
  }

  // Bind globals
  window._nuevaVenta = openNuevaVenta
  window._switchVentaView = (view) => {
    _currentView = view
    localStorage.setItem('ventas_view', view)
    document.querySelectorAll('.o-view-btn').forEach(b => b.classList.remove('active'))
    document.getElementById(`view-btn-${view}`)?.classList.add('active')
    renderContent()
  }
  window._checkAll = (chk) => {
    document.querySelectorAll('.row-chk').forEach(c => { c.checked = chk.checked })
    window._onRowCheck()
  }
  window._onRowCheck = () => {
    const checked = document.querySelectorAll('.row-chk:checked')
    const bar = document.getElementById('ventas-actions-bar')
    if (checked.length > 0 && bar) {
      bar.innerHTML = `
        <div class="o-list-actions-bar">
          <span class="o-actions-count">${checked.length} seleccionado(s)</span>
          <button class="btn btn-secondary btn-sm">Exportar</button>
          <button class="btn btn-danger btn-sm">Eliminar</button>
        </div>`
    } else if (bar) bar.innerHTML = ''
  }
}

function renderContent() {
  const contentEl = document.getElementById('ventas-content')
  if (!contentEl) return

  if (_currentView === 'kanban') {
    contentEl.innerHTML = `<div id="ventas-actions-bar"></div>${kanbanHtml(_ordenes)}`
  } else {
    contentEl.innerHTML = `<div id="ventas-actions-bar"></div>${listHtml(_ordenes)}`
  }
}

// ─── Form View (detalle de venta) ─────────────────────────────────────────────

window._verVenta = async (id) => {
  // Mostrar skeleton de form completo
  setBreadcrumb([
    { label: 'Inicio', href: 'home' },
    { label: 'Ventas', href: 'ventas' },
    { label: 'Cargando...' },
  ])

  setPage(`
    <div class="o-form-view">
      <div class="o-statusbar">
        <div class="o-statusbar-status">
          ${['Borrador','Confirmada','Entregada'].map(s => `<div class="o-status-step skeleton" style="width:100px;height:28px;margin:10px 4px"></div>`).join('')}
        </div>
      </div>
      <div class="o-form-sheet" style="margin:20px 24px;padding:24px">
        ${[1,2,3,4].map(() => `<div class="skeleton" style="height:36px;margin-bottom:12px;border-radius:6px"></div>`).join('')}
      </div>
    </div>`)

  try {
    const res = await api.venta(id)
    const v = res?.data ?? res

    if (!v) { toast('Error', 'No se encontró la venta', 'error'); return }

    const e = ESTADO_MAP[v.state] || { lbl: v.state || '—', color: 'gray', step: 0 }
    const inv = v.invoice_status === 'invoiced' ? 'Facturada' :
                v.invoice_status === 'to invoice' ? 'Por facturar' : 'No facturada'

    const steps = [
      { key: 'draft',  label: 'Borrador',   done: e.step > 0 },
      { key: 'sale',   label: 'Confirmada', done: e.step > 1 },
      { key: 'done',   label: 'Entregada',  done: e.step > 2 },
    ]
    if (v.state === 'cancel') steps.push({ key: 'cancel', label: 'Cancelada', done: false })

    // Líneas de pedido
    const lineas = v.order_line || v.lineas || []
    const lineasHtml = `
      <table class="o-editable-table">
        <thead><tr>
          <th>Producto</th>
          <th>Descripción</th>
          <th style="text-align:right">Cant.</th>
          <th style="text-align:right">P. Unit.</th>
          <th style="text-align:right">Desc%</th>
          <th style="text-align:right">Subtotal</th>
        </tr></thead>
        <tbody>
          ${lineas.length > 0
            ? lineas.map(l => `
              <tr>
                <td class="td-primary">${l.product_name || l.nombre || '—'}</td>
                <td style="color:var(--text-500)">${l.name || l.descripcion || ''}</td>
                <td style="text-align:right">${l.product_uom_qty ?? l.cantidad ?? 0}</td>
                <td style="text-align:right">${fmtMxn(parseFloat(l.price_unit || l.precio_unitario || 0))}</td>
                <td style="text-align:right">${l.discount || l.descuento || 0}%</td>
                <td style="text-align:right;font-weight:700">${fmtMxn(parseFloat(l.price_subtotal || l.subtotal || 0))}</td>
              </tr>`).join('')
            : `<tr><td colspan="6" style="text-align:center;padding:20px;color:var(--text-400)">Sin líneas de pedido</td></tr>`}
        </tbody>
      </table>
      <div style="display:flex;justify-content:flex-end;padding:16px 0;gap:20px;border-top:1px solid var(--border)">
        <div style="text-align:right">
          <div style="font-size:12px;color:var(--text-400)">Subtotal</div>
          <div style="font-size:14px;font-weight:700">${fmtMxn(parseFloat(v.amount_untaxed || 0))}</div>
        </div>
        <div style="text-align:right">
          <div style="font-size:12px;color:var(--text-400)">IVA</div>
          <div style="font-size:14px;font-weight:700">${fmtMxn(parseFloat(v.amount_tax || 0))}</div>
        </div>
        <div style="text-align:right">
          <div style="font-size:12px;color:var(--text-400)">Total</div>
          <div style="font-size:18px;font-weight:800;color:var(--primary)">${fmtMxn(parseFloat(v.amount_total || 0))}</div>
        </div>
      </div>`

    const otraInfoHtml = fieldGroupHtml([
      { label: 'Política entrega', value: v.picking_policy || '—' },
      { label: 'Plazo de pago', value: v.payment_term_name || v.payment_term || '—' },
      { label: 'Notas', value: v.note || v.notes || '—' },
      { label: 'Equipo de ventas', value: v.team_name || '—' },
    ], 2)

    openFormPage({
      title: v.name || `Venta #${v.id}`,
      backLabel: 'Ventas',
      backHref: 'ventas',
      pageTitle: v.name || `#${v.id}`,
      statusSteps: steps,
      currentStatus: v.state,
      smartButtons: [
        { icon: '📄', count: v.invoice_count ?? 0, label: 'Facturas', onClick: '' },
        { icon: '🚚', count: v.delivery_count ?? 0, label: 'Entregas', onClick: '' },
      ],
      statusButtons: [
        {
          label: '✅ Confirmar',
          primary: true,
          visible: v.state === 'draft' || v.state === 'sent',
          onClick: `window._confirmarVenta(${v.id})`,
        },
        {
          label: '🔏 Timbrar CFDI',
          primary: false,
          visible: v.invoice_status === 'to invoice',
          onClick: `window._go('cfdi')`,
        },
        {
          label: '❌ Cancelar',
          primary: false,
          visible: v.state !== 'cancel' && v.state !== 'done',
          onClick: `window._cancelarVenta(${v.id})`,
        },
      ],
      groups: [
        {
          cols: 2,
          fields: [
            { label: 'Cliente', value: `<strong>${v.partner_name || v.partner_id || '—'}</strong>` },
            { label: 'Vendedor', value: v.user_name || v.salesperson || '—' },
            { label: 'Fecha Orden', value: v.date_order ? fmtDate(v.date_order) : '—' },
            { label: 'Empresa', value: v.company_name || '—' },
            { label: 'Referencia', value: v.client_order_ref || '—' },
            { label: 'Estado Factura', value: `<span class="badge badge-${inv === 'Facturada' ? 'emerald' : inv === 'Por facturar' ? 'amber' : 'gray'}">${inv}</span>` },
          ],
        },
      ],
      tabs: [
        { label: 'Líneas de Pedido', content: lineasHtml },
        { label: 'Otra Información', content: otraInfoHtml },
      ],
      messages: [
        { author: 'Sistema', initials: 'SY', date: v.date_order ? fmtDate(v.date_order) : '—', text: `Orden de venta ${v.name || ''} creada. Estado: ${e.lbl}` },
      ],
    })

    // Bind acciones del form
    window._confirmarVenta = async (vid) => {
      try {
        await api.put(`/ventas/${vid}/confirmar`, {})
        toast('Venta confirmada', 'Estado actualizado correctamente', 'success')
        window._verVenta(vid)
      } catch (err) { toast('Error', err.message, 'error') }
    }
    window._cancelarVenta = async (vid) => {
      try {
        await api.put(`/ventas/${vid}/cancelar`, {})
        toast('Venta cancelada', '', 'info')
        window._verVenta(vid)
      } catch (err) { toast('Error', err.message, 'error') }
    }

  } catch (err) {
    console.error(err)
    toast('Error al cargar venta', err.message, 'error')
  }
}
