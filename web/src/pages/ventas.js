/**
 * ventas.js — Módulo Ventas — UI estilo Odoo Enterprise
 * Vista Lista + Kanban + Formulario completo con chatter
 */
import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { api } from '../api.js'
import { toast, fmtMxn, stateBadge, skeletonTable } from '../ui.js'

let _view = 'list', _page = 1, _search = '', _filter = null, _records = []

export async function renderVentas() {
  ensureLayout()
  setBreadcrumb([{ label: 'Ventas' }])
  setPage(`<div class="nx-module-page"><div id="mcp"></div><div id="mcontent">${skeletonTable(5, 6)}</div></div>`)
  _renderCP()
  await _load()
}

function _renderCP() {
  const el = document.getElementById('mcp')
  if (!el) return
  el.innerHTML = `
    <div class="o-control-panel">
      <div class="o-cp-left">
        <button class="o-btn-new" onclick="window._newVenta()">+ Nueva Venta</button>
        <span class="o-cp-sep"></span>
        <div class="o-dropdown" id="dd-vf">
          <button class="o-btn-filter" onclick="window._tog('dd-vf')">📂 Filtros ▾</button>
          <div class="o-dropdown-menu" id="dd-vf-menu">
            <div class="o-dropdown-item" onclick="window._fv('sale')">Confirmadas</div>
            <div class="o-dropdown-item" onclick="window._fv('draft')">Borradores</div>
            <div class="o-dropdown-item" onclick="window._fv('done')">Realizadas</div>
            <div class="o-dropdown-item" onclick="window._fv('cancel')">Canceladas</div>
            <div class="o-dropdown-divider"></div>
            <div class="o-dropdown-item" onclick="window._fv(null)">❌ Sin filtro</div>
          </div>
        </div>
        <div class="o-search-box">
          <span style="color:var(--text-400)">🔍</span>
          <input type="search" placeholder="Buscar venta…" id="vs" oninput="window._sv(this.value)">
        </div>
        <span class="o-record-count" id="vcount"></span>
      </div>
      <div class="o-cp-right">
        <div class="o-view-switcher">
          <button class="o-view-btn ${_view === 'list' ? 'active' : ''}" onclick="window._vv('list')" title="Lista">☰</button>
          <button class="o-view-btn ${_view === 'kanban' ? 'active' : ''}" onclick="window._vv('kanban')" title="Kanban">⬜</button>
        </div>
      </div>
    </div>`
  _initDD()
  window._vv = (v) => { _view = v; _renderCP(); _load() }
  window._sv = _deb((q) => { _search = q; _page = 1; _load() }, 300)
  window._fv = (s) => { _filter = s; _page = 1; _load(); window._cdd() }
  window._newVenta = () => toast('Info', 'Usa el backend para crear órdenes', 'info')
}

function _initDD() {
  window._tog = (id) => {
    const m = document.getElementById(id + '-menu'); if (!m) return
    const o = m.classList.contains('open'); window._cdd(); if (!o) m.classList.add('open')
  }
  window._cdd = () => document.querySelectorAll('.o-dropdown-menu.open').forEach(m => m.classList.remove('open'))
  if (!window._ddInit) {
    document.addEventListener('click', e => { if (!e.target.closest('.o-dropdown')) window._cdd() })
    window._ddInit = true
  }
}

async function _load() {
  const c = document.getElementById('mcontent'); if (!c) return
  c.innerHTML = skeletonTable(5, 6)
  try {
    const res = await api.ventas(_page)
    _records = res?.data || []
    let rows = _filter ? _records.filter(r => r.state === _filter) : _records
    if (_search) {
      const q = _search.toLowerCase()
      rows = rows.filter(r => (r.name || '').toLowerCase().includes(q) || (r.partner_name || '').toLowerCase().includes(q))
    }
    const vc = document.getElementById('vcount'); if (vc) vc.textContent = rows.length + ' registros'
    c.innerHTML = _view === 'kanban' ? _kanban(rows) : _list(rows)
    if (_view === 'list') _initCB()
  } catch (e) {
    c.innerHTML = `<div style="padding:40px;text-align:center;color:var(--text-400)">⚠️ ${e.message}</div>`
  }
}

const LABEL_MAP = { sale: 'Confirmada', done: 'Realizada', draft: 'Borrador', cancel: 'Cancelada', sent: 'Enviada' }
const INV_MAP   = { invoiced: 'Facturada', to_invoice: 'Por Facturar', no: '—' }

function _list(rows) {
  if (!rows.length) return `<div style="padding:60px;text-align:center"><div style="font-size:48px;margin-bottom:12px">📋</div><p style="color:var(--text-400)">Sin ventas. Crea la primera.</p></div>`
  return `
    <div class="o-list-actions-bar" id="lab"><span class="o-actions-count" id="sel-cnt">0 seleccionados</span>
      <button class="o-action-btn-sm" onclick="alert('Exportar')">Exportar</button>
    </div>
    <div class="o-list-view"><table>
      <thead><tr>
        <th class="th-check"><input type="checkbox" class="o-list-checkbox" id="ca" onchange="window._ca(this.checked)"></th>
        <th>Número</th><th>Cliente</th><th>Fecha</th><th>Estado</th><th style="text-align:right">Total</th><th>Facturación</th>
      </tr></thead>
      <tbody>
        ${rows.map(r => `
          <tr onclick="window._vVenta(${r.id})" data-id="${r.id}">
            <td class="td-check" onclick="event.stopPropagation()"><input type="checkbox" class="o-list-checkbox rc" data-id="${r.id}" onchange="window._rc()"></td>
            <td><strong>${r.name || '-'}</strong></td>
            <td>${r.partner_name || r.partner_id || '-'}</td>
            <td>${r.date_order?.slice(0, 10) || '-'}</td>
            <td>${stateBadge(r.state, LABEL_MAP[r.state] || r.state)}</td>
            <td style="text-align:right;font-weight:700;color:var(--primary)">${fmtMxn(r.amount_total)}</td>
            <td>${r.invoice_status ? stateBadge(r.invoice_status, INV_MAP[r.invoice_status] || r.invoice_status) : '-'}</td>
          </tr>`).join('')}
      </tbody></table></div>
    <div style="padding:12px 20px;display:flex;justify-content:space-between;align-items:center;background:var(--bg-card);border-top:1px solid var(--border)">
      <span style="font-size:13px;color:var(--text-400)">${rows.length} registros</span>
      <div style="display:flex;gap:8px">
        <button class="o-action-btn-sm" ${_page <= 1 ? 'disabled' : ''} onclick="window._vp(${_page - 1})">‹ Anterior</button>
        <span style="padding:5px 10px;font-size:13px">${_page}</span>
        <button class="o-action-btn-sm" onclick="window._vp(${_page + 1})">Siguiente ›</button>
      </div></div>`
}

const KANBAN_COLS = [
  { key: 'draft',  label: 'Borrador',   color: '#D97706' },
  { key: 'sent',   label: 'Enviado',    color: '#2563EB' },
  { key: 'sale',   label: 'Confirmado', color: '#059669' },
  { key: 'done',   label: 'Realizado',  color: '#166534' },
  { key: 'cancel', label: 'Cancelado',  color: '#DC2626' },
]

function _kanban(rows) {
  const g = {}; KANBAN_COLS.forEach(c => g[c.key] = [])
  rows.forEach(r => { if (g[r.state]) g[r.state].push(r); else g['draft']?.push(r) })
  return `<div class="o-kanban-view">${KANBAN_COLS.map(col => `
    <div class="o-kanban-col">
      <div class="o-kanban-col-header" style="border-top:3px solid ${col.color}">
        <span>${col.label}</span><span class="o-kanban-col-count">${g[col.key].length}</span>
      </div>
      <div class="o-kanban-cards">
        ${g[col.key].map(r => `
          <div class="o-kanban-card" onclick="window._vVenta(${r.id})">
            <div class="o-kanban-card-title">${r.name || '#' + r.id}</div>
            <div style="font-size:12px;color:var(--text-400);margin-bottom:8px">${r.partner_name || r.partner_id || ''}</div>
            <div class="o-kanban-card-meta">
              <span style="font-size:11px">${r.date_order?.slice(0, 10) || ''}</span>
              <span class="o-kanban-card-amount">${fmtMxn(r.amount_total)}</span>
            </div>
          </div>`).join('') || '<div style="padding:16px;text-align:center;color:var(--text-300);font-size:12px">Vacío</div>'}
      </div>
    </div>`).join('')}</div>`
}

function _initCB() {
  window._ca = (c) => { document.querySelectorAll('.rc').forEach(cb => cb.checked = c); window._rc() }
  window._rc = () => {
    const n = document.querySelectorAll('.rc:checked').length
    const b = document.getElementById('lab'), s = document.getElementById('sel-cnt')
    if (b) b.classList.toggle('visible', n > 0)
    if (s) s.textContent = n + ' seleccionado' + (n !== 1 ? 's' : '')
    document.querySelectorAll('[data-id]').forEach(tr => {
      const cb = tr.querySelector('.rc'); if (cb) tr.classList.toggle('selected', cb.checked)
    })
  }
}

window._vp = (p) => { _page = p; _load() }

// ===== FORMULARIO VENTA =====
window._vVenta = async (id) => {
  setBreadcrumb([{ label: 'Ventas', href: '#ventas' }, { label: 'Cargando…' }])
  setPage(`<div style="padding:40px">${skeletonTable(3, 5)}</div>`)
  try {
    const res = await api.venta(id)
    const v = res?.data || res; if (!v) throw new Error('No encontrado')
    setBreadcrumb([{ label: 'Ventas', href: '#ventas' }, { label: v.name || '#' + id }])
    const STEPS = ['draft', 'sent', 'sale', 'done']
    if (v.state === 'cancel') STEPS.push('cancel')
    const si = STEPS.indexOf(v.state)

    setPage(`
      <div class="o-form-view" id="fv">
        <div class="o-statusbar">
          <div class="o-statusbar-status">
            ${STEPS.map((s, i) => `
              <div class="o-status-step ${s === v.state ? 'active' : ''} ${i < si ? 'done' : ''}">
                ${i < si ? '✔ ' : ''}${{ draft: 'Borrador', sent: 'Enviado', sale: 'Confirmado', done: 'Realizado', cancel: 'Cancelado' }[s] || s}
              </div>${i < STEPS.length - 1 ? '<span class="o-status-arrow">›</span>' : ''}`).join('')}
          </div>
          <div class="o-statusbar-buttons">
            ${(v.state === 'draft' || v.state === 'sent') ? `<button class="btn btn-primary btn-sm" onclick="window._confV(${id})">📊 Confirmar</button>` : ''}
            ${v.state === 'sale' ? `<button class="btn btn-secondary btn-sm" onclick="toast('Info','Próximamente','info')">🧾 Crear Factura</button>` : ''}
            ${v.state !== 'cancel' && v.state !== 'done' ? `<button class="btn btn-sm" style="background:#FEE2E2;color:#DC2626;border:none;padding:6px 14px;border-radius:8px;font-weight:600;cursor:pointer" onclick="window._cancV(${id})">❌ Cancelar</button>` : ''}
            <button class="btn btn-secondary btn-sm" onclick="window._go('ventas')">← Volver</button>
          </div>
        </div>
        <div class="o-smart-buttons">
          <button class="o-smart-btn"><span class="o-count">0</span><span class="o-label">🧾 Facturas</span></button>
          <button class="o-smart-btn"><span class="o-count">0</span><span class="o-label">🚚 Entregas</span></button>
          <button class="o-smart-btn"><span class="o-count">1</span><span class="o-label">📋 Líneas</span></button>
        </div>
        <div class="o-form-sheet">
          <div class="o-form-title-row">
            <h1 class="o-form-record-title">${v.name || 'Nueva Venta'}</h1>
            <span class="o-form-subtitle">${v.partner_name || ''}</span>
          </div>
          <div class="o-form-group-wrapper">
            <div class="o-form-group">
              <div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">Cliente</div><div class="o-field-value">${v.partner_name || v.partner_id || '<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Fecha</div><div class="o-field-value">${v.date_order?.slice(0, 16)?.replace('T', ' ') || '<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Ref. Cliente</div><div class="o-field-value">${v.client_order_ref || '<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Validez</div><div class="o-field-value">${v.validity_date || '<span class="o-field-empty">—</span>'}</div></div>
              </div>
              <div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">Estado</div><div class="o-field-value">${stateBadge(v.state, LABEL_MAP[v.state] || v.state)}</div></div>
                <div class="o-field-row"><div class="o-field-label">Empresa</div><div class="o-field-value">${v.company_id || v.company_name || '<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Facturación</div><div class="o-field-value">${v.invoice_status ? stateBadge(v.invoice_status, INV_MAP[v.invoice_status] || v.invoice_status) : '<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Notas</div><div class="o-field-value">${v.note || '<span class="o-field-empty">—</span>'}</div></div>
              </div>
            </div>
          </div>
          <div class="o-notebook">
            <div class="o-tabs">
              <button class="o-tab active" onclick="window._st('vl')">Líneas de Pedido</button>
              <button class="o-tab" onclick="window._st('vi')">Otra Información</button>
              <button class="o-tab" onclick="window._st('vx')">Fiscal</button>
            </div>
            <div class="o-tab-panel active" id="tab-panel-vl">
              <table class="o-editable-table"><thead><tr>
                <th>Producto</th><th>Descripción</th>
                <th style="text-align:right">Qty</th>
                <th style="text-align:right">Precio</th>
                <th style="text-align:right">Desc.</th>
                <th style="text-align:right">Subtotal</th>
              </tr></thead>
              <tbody id="vlineas"><tr><td colspan="6" style="text-align:center;padding:20px;color:var(--text-400)">⏳ Cargando…</td></tr></tbody></table>
              <div class="o-lines-totals"><table>
                <tr><td>Subtotal:</td><td style="text-align:right;font-weight:600">${fmtMxn(v.amount_untaxed)}</td></tr>
                <tr><td>IVA (16%):</td><td style="text-align:right;font-weight:600">${fmtMxn(v.amount_tax)}</td></tr>
                <tr class="total-row"><td>TOTAL:</td><td style="text-align:right">${fmtMxn(v.amount_total)}</td></tr>
              </table></div>
            </div>
            <div class="o-tab-panel" id="tab-panel-vi">
              <div class="o-form-group"><div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">Vendedor</div><div class="o-field-value">${v.user_id || v.user_name || '<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Equipo</div><div class="o-field-value">${v.team_id || v.team_name || '<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Política entrega</div><div class="o-field-value">${v.picking_policy || '<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Plazo pago</div><div class="o-field-value">${v.payment_term_name || v.payment_term || '<span class="o-field-empty">—</span>'}</div></div>
              </div></div>
            </div>
            <div class="o-tab-panel" id="tab-panel-vx">
              <div class="o-form-group"><div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">UUID CFDI</div><div class="o-field-value"><span class="o-field-empty">Pendiente</span></div></div>
                <div class="o-field-row"><div class="o-field-label">Folio fiscal</div><div class="o-field-value"><span class="o-field-empty">—</span></div></div>
              </div></div>
            </div>
          </div>
        </div>
        <div class="o-chatter">
          <div class="o-chatter-topbar">
            <button class="o-chatter-btn">✉️ Enviar mensaje</button>
            <button class="o-chatter-btn">📋 Nota interna</button>
            <button class="o-chatter-btn">📎 Adjuntar</button>
          </div>
          <div class="o-chatter-thread">
            <div class="o-message">
              <div class="o-msg-avatar" style="background:#4F46E5">S</div>
              <div class="o-msg-content">
                <div class="o-msg-header">
                  <span class="o-msg-author">Sistema</span>
                  <span class="o-msg-date">${new Date().toLocaleDateString('es-MX')}</span>
                </div>
                <div class="o-msg-text">Orden ${v.name || ''} registrada. Estado: ${LABEL_MAP[v.state] || v.state}</div>
              </div>
            </div>
          </div>
        </div>
      </div>`)

    // Tabs
    window._st = (tabId) => {
      document.querySelectorAll('.o-tab').forEach(t => t.classList.remove('active'))
      document.querySelectorAll('.o-tab-panel').forEach(p => p.classList.remove('active'))
      const btn = document.querySelector(`.o-tab[onclick*="'${tabId}'"]`)
      if (btn) btn.classList.add('active')
      const panel = document.getElementById('tab-panel-' + tabId)
      if (panel) panel.classList.add('active')
    }

    // Cargar líneas
    try {
      const lr = await api.get(`/ventas/${id}/lineas`)
      const ls = lr?.data || []
      const lb = document.getElementById('vlineas')
      if (lb) {
        lb.innerHTML = ls.length
          ? ls.map(l => `<tr>
              <td>${l.product_id ? '#' + l.product_id : '<span class="o-field-empty">—</span>'}</td>
              <td>${l.name || '-'}</td>
              <td style="text-align:right">${l.product_uom_qty ?? 0}</td>
              <td style="text-align:right">${fmtMxn(l.price_unit)}</td>
              <td style="text-align:right">${l.discount ? l.discount + '%' : '0%'}</td>
              <td style="text-align:right;font-weight:700">${fmtMxn(l.price_subtotal)}</td>
            </tr>`).join('')
          : '<tr><td colspan="6" style="text-align:center;padding:16px;color:var(--text-400)">Sin líneas de pedido</td></tr>'
      }
    } catch (_) { /* líneas opcionales */ }

    // Acciones del formulario
    window._confV = async (vid) => {
      if (!confirm('¿Confirmar orden?')) return
      try {
        await api.put(`/ventas/${vid}/confirmar`, {})
        toast('OK', 'Venta confirmada', 'success')
        window._vVenta(vid)
      } catch (e) { toast('Error', e.message, 'error') }
    }
    window._cancV = async (vid) => {
      if (!confirm('¿Cancelar orden?')) return
      try {
        await api.put(`/ventas/${vid}/cancelar`, {})
        toast('Cancelado', '', 'info')
        window._go('ventas')
      } catch (e) { toast('Error', e.message, 'error') }
    }

  } catch (e) {
    setPage(`<div style="padding:40px;text-align:center"><p style="color:#DC2626">⚠️ ${e.message}</p><button class="o-btn-new" onclick="window._go('ventas')">Volver</button></div>`)
  }
}

function _deb(fn, ms) { let t; return (...a) => { clearTimeout(t); t = setTimeout(() => fn(...a), ms) } }
