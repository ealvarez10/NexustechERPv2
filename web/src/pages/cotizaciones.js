/**
 * cotizaciones.js — Módulo Cotizaciones — UI estilo Odoo Enterprise
 * Vista Lista + Kanban + Formulario completo con chatter
 */
import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { api } from '../api.js'
import { toast, fmtMxn, stateBadge, skeletonTable } from '../ui.js'

let _view = 'list', _page = 1, _search = '', _filter = null, _records = []

export async function renderCotizaciones() {
  ensureLayout()
  setBreadcrumb([{ label: 'Cotizaciones' }])
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
        <button class="o-btn-new" onclick="window._newCot()">+ Nueva Cotización</button>
        <span class="o-cp-sep"></span>
        <div class="o-dropdown" id="dd-cf">
          <button class="o-btn-filter" onclick="window._tog('dd-cf')">📂 Filtros ▾</button>
          <div class="o-dropdown-menu" id="dd-cf-menu">
            <div class="o-dropdown-item" onclick="window._cf('draft')">Borradores</div>
            <div class="o-dropdown-item" onclick="window._cf('sent')">Enviadas</div>
            <div class="o-dropdown-item" onclick="window._cf('sale')">Confirmadas</div>
            <div class="o-dropdown-divider"></div>
            <div class="o-dropdown-item" onclick="window._cf(null)">❌ Sin filtro</div>
          </div>
        </div>
        <div class="o-search-box">
          <span style="color:var(--text-400)">🔍</span>
          <input type="search" placeholder="Buscar cotización…" id="cs" oninput="window._sc(this.value)">
        </div>
        <span class="o-record-count" id="ccount"></span>
      </div>
      <div class="o-cp-right">
        <div class="o-view-switcher">
          <button class="o-view-btn ${_view === 'list' ? 'active' : ''}" onclick="window._cvv('list')" title="Lista">☰</button>
          <button class="o-view-btn ${_view === 'kanban' ? 'active' : ''}" onclick="window._cvv('kanban')" title="Kanban">⬜</button>
        </div>
      </div>
    </div>`
  _initDD()
  window._cvv = (v) => { _view = v; _renderCP(); _load() }
  window._sc = _deb((q) => { _search = q; _page = 1; _load() }, 300)
  window._cf = (s) => { _filter = s; _page = 1; _load(); window._cdd() }
  window._newCot = () => _abrirFormNueva()
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
    const res = await api.cotizaciones(_page)
    _records = res?.data || []
    let rows = _filter ? _records.filter(r => r.state === _filter) : _records
    if (_search) {
      const q = _search.toLowerCase()
      rows = rows.filter(r => (r.name || '').toLowerCase().includes(q) || (r.partner_name || '').toLowerCase().includes(q))
    }
    const cc = document.getElementById('ccount'); if (cc) cc.textContent = rows.length + ' registros'
    c.innerHTML = _view === 'kanban' ? _kanban(rows) : _list(rows)
    if (_view === 'list') _initCB()
  } catch (e) {
    c.innerHTML = `<div style="padding:40px;text-align:center;color:var(--text-400)">⚠️ ${e.message}</div>`
  }
}

const LABEL_MAP = { draft: 'Borrador', sent: 'Enviada', sale: 'Confirmada', cancel: 'Cancelada' }

function _list(rows) {
  if (!rows.length) return `<div style="padding:60px;text-align:center"><div style="font-size:48px;margin-bottom:12px">📝</div><p style="color:var(--text-400)">Sin cotizaciones. Crea la primera.</p></div>`
  return `
    <div class="o-list-actions-bar" id="clab"><span class="o-actions-count" id="csel-cnt">0 seleccionados</span>
      <button class="o-action-btn-sm" onclick="alert('Exportar')">Exportar</button>
    </div>
    <div class="o-list-view"><table>
      <thead><tr>
        <th class="th-check"><input type="checkbox" class="o-list-checkbox" id="cca" onchange="window._cca(this.checked)"></th>
        <th>Número</th><th>Cliente</th><th>Fecha</th><th>Validez</th><th>Estado</th><th style="text-align:right">Total</th>
      </tr></thead>
      <tbody>
        ${rows.map(r => `
          <tr onclick="window._vCot(${r.id})" data-id="${r.id}">
            <td class="td-check" onclick="event.stopPropagation()"><input type="checkbox" class="o-list-checkbox crc" data-id="${r.id}" onchange="window._crc()"></td>
            <td><strong>${r.name || '-'}</strong></td>
            <td>${r.partner_name || r.partner_id || '-'}</td>
            <td>${r.date_order?.slice(0, 10) || '-'}</td>
            <td>${r.validity_date?.slice(0, 10) || '<span style="color:var(--text-300)">—</span>'}</td>
            <td>${stateBadge(r.state, LABEL_MAP[r.state] || r.state)}</td>
            <td style="text-align:right;font-weight:700;color:var(--primary)">${fmtMxn(r.amount_total)}</td>
          </tr>`).join('')}
      </tbody></table></div>
    <div style="padding:12px 20px;display:flex;justify-content:space-between;align-items:center;background:var(--bg-card);border-top:1px solid var(--border)">
      <span style="font-size:13px;color:var(--text-400)">${rows.length} registros</span>
      <div style="display:flex;gap:8px">
        <button class="o-action-btn-sm" ${_page <= 1 ? 'disabled' : ''} onclick="window._cp(${_page - 1})">‹ Anterior</button>
        <span style="padding:5px 10px;font-size:13px">${_page}</span>
        <button class="o-action-btn-sm" onclick="window._cp(${_page + 1})">Siguiente ›</button>
      </div></div>`
}

const KANBAN_COLS = [
  { key: 'draft', label: 'Borrador', color: '#9CA3AF' },
  { key: 'sent',  label: 'Enviada',  color: '#2563EB' },
  { key: 'sale',  label: 'Confirmada', color: '#059669' },
]

function _kanban(rows) {
  const g = {}; KANBAN_COLS.forEach(c => g[c.key] = [])
  rows.forEach(r => { if (g[r.state]) g[r.state].push(r); else if (g['draft']) g['draft'].push(r) })
  return `<div class="o-kanban-view">${KANBAN_COLS.map(col => `
    <div class="o-kanban-col">
      <div class="o-kanban-col-header" style="border-top:3px solid ${col.color}">
        <span>${col.label}</span><span class="o-kanban-col-count">${g[col.key].length}</span>
      </div>
      <div class="o-kanban-cards">
        ${g[col.key].map(r => `
          <div class="o-kanban-card" onclick="window._vCot(${r.id})">
            <div class="o-kanban-card-title">${r.name || '#' + r.id}</div>
            <div style="font-size:12px;color:var(--text-400);margin-bottom:8px">${r.partner_name || ''}</div>
            <div class="o-kanban-card-meta">
              <span style="font-size:11px">${r.validity_date?.slice(0, 10) ? '⏰ ' + r.validity_date.slice(0, 10) : r.date_order?.slice(0, 10) || ''}</span>
              <span class="o-kanban-card-amount">${fmtMxn(r.amount_total)}</span>
            </div>
          </div>`).join('') || '<div style="padding:16px;text-align:center;color:var(--text-300);font-size:12px">Vacío</div>'}
      </div>
    </div>`).join('')}</div>`
}

function _initCB() {
  window._cca = (c) => { document.querySelectorAll('.crc').forEach(cb => cb.checked = c); window._crc() }
  window._crc = () => {
    const n = document.querySelectorAll('.crc:checked').length
    const b = document.getElementById('clab'), s = document.getElementById('csel-cnt')
    if (b) b.classList.toggle('visible', n > 0)
    if (s) s.textContent = n + ' seleccionado' + (n !== 1 ? 's' : '')
    document.querySelectorAll('[data-id]').forEach(tr => {
      const cb = tr.querySelector('.crc'); if (cb) tr.classList.toggle('selected', cb.checked)
    })
  }
}

window._cp = (p) => { _page = p; _load() }

// ===== FORMULARIO NUEVA COTIZACIÓN (inline) =====
function _abrirFormNueva() {
  setBreadcrumb([{ label: 'Cotizaciones', href: '#cotizaciones' }, { label: 'Nueva cotización' }])
  setPage(`
    <div class="o-form-view">
      <div class="o-statusbar">
        <div class="o-statusbar-status">
          <div class="o-status-step active">Borrador</div>
          <span class="o-status-arrow">›</span>
          <div class="o-status-step">Enviada</div>
        </div>
        <div class="o-statusbar-buttons">
          <button class="btn btn-secondary btn-sm" onclick="window._go('cotizaciones')">← Volver</button>
        </div>
      </div>
      <div class="o-form-sheet">
        <div class="o-form-title-row">
          <h1 class="o-form-record-title">Nueva Cotización</h1>
        </div>
        <div class="o-form-group-wrapper">
          <div class="o-form-group">
            <div class="o-form-col">
              <div class="o-field-row">
                <div class="o-field-label">Cliente *</div>
                <div class="o-field-value"><input id="nc-partner" class="form-control" placeholder="Nombre del cliente" autocomplete="off"></div>
              </div>
              <div class="o-field-row">
                <div class="o-field-label">Referencia</div>
                <div class="o-field-value"><input id="nc-ref" class="form-control" placeholder="Ref. del cliente"></div>
              </div>
            </div>
            <div class="o-form-col">
              <div class="o-field-row">
                <div class="o-field-label">Validez</div>
                <div class="o-field-value"><input id="nc-validez" class="form-control" type="date"></div>
              </div>
              <div class="o-field-row">
                <div class="o-field-label">Notas</div>
                <div class="o-field-value"><input id="nc-nota" class="form-control" placeholder="Observaciones opcionales"></div>
              </div>
            </div>
          </div>
        </div>
        <div style="display:flex;gap:12px;padding:16px 0">
          <button class="btn btn-primary" onclick="window._guardarNuevaCot()">💾 Guardar cotización</button>
          <button class="btn btn-secondary" onclick="window._go('cotizaciones')">Cancelar</button>
        </div>
      </div>
    </div>`)

  window._guardarNuevaCot = async () => {
    const partnerNombre = document.getElementById('nc-partner')?.value?.trim()
    const ref           = document.getElementById('nc-ref')?.value?.trim() || null
    const validez       = document.getElementById('nc-validez')?.value || null
    const nota          = document.getElementById('nc-nota')?.value?.trim() || null

    if (!partnerNombre) return toast('Campo requerido', 'Ingresa el nombre del cliente', 'warning')

    let partnerId = 1
    try {
      const res = await api.get(`/partners?pagina=1&q=${encodeURIComponent(partnerNombre)}&por_pagina=5`)
      const lista = res?.data ?? []
      const match = lista.find(p => p.name?.toLowerCase() === partnerNombre.toLowerCase())
      if (match) { partnerId = match.id }
      else if (lista.length > 0) { partnerId = lista[0].id }
      else return toast('Cliente no encontrado', `No se encontró "${partnerNombre}"`, 'warning')
    } catch (e) {
      return toast('Error', 'No se pudo buscar el cliente: ' + e.message, 'error')
    }

    try {
      const res = await api.crearCotizacion({
        partner_id: partnerId, partner_invoice_id: partnerId, partner_shipping_id: partnerId,
        note: nota, client_order_ref: ref, validity_date: validez || null,
      })
      const newId = res?.data?.id ?? res?.id
      toast('Cotización creada', `ID ${newId}`, 'success')
      if (newId) setTimeout(() => window._vCot(newId), 400)
      else window._go('cotizaciones')
    } catch (e) { toast('Error al crear cotización', e.message, 'error') }
  }
}

// ===== FORMULARIO COTIZACIÓN =====
window._vCot = async (id) => {
  setBreadcrumb([{ label: 'Cotizaciones', href: '#cotizaciones' }, { label: 'Cargando…' }])
  setPage(`<div style="padding:40px">${skeletonTable(3, 5)}</div>`)
  try {
    const res = await api.cotizacion(id)
    const c = res?.data || res; if (!c) throw new Error('No encontrada')
    setBreadcrumb([{ label: 'Cotizaciones', href: '#cotizaciones' }, { label: c.name || '#' + id }])

    const STEPS = ['draft', 'sent']
    const si = STEPS.indexOf(c.state)
    const STEP_LABELS = { draft: 'Borrador', sent: 'Enviada' }

    setPage(`
      <div class="o-form-view" id="cfv">
        <div class="o-statusbar">
          <div class="o-statusbar-status">
            ${STEPS.map((s, i) => `
              <div class="o-status-step ${s === c.state ? 'active' : ''} ${i < si ? 'done' : ''}">
                ${i < si ? '✔ ' : ''}${STEP_LABELS[s] || s}
              </div>${i < STEPS.length - 1 ? '<span class="o-status-arrow">›</span>' : ''}`).join('')}
            ${c.state === 'sale' ? '<span class="o-status-arrow">›</span><div class="o-status-step done">✔ Confirmada</div>' : ''}
            ${c.state === 'cancel' ? '<span class="o-status-arrow">›</span><div class="o-status-step active" style="color:#DC2626">Cancelada</div>' : ''}
          </div>
          <div class="o-statusbar-buttons">
            ${(c.state === 'draft' || c.state === 'sent') ? `
              <button class="btn btn-secondary btn-sm" onclick="window._emailCot(${id})">✉️ Enviar por Email</button>
              <button class="btn btn-primary btn-sm" onclick="window._confirmarCot(${id})">✅ Confirmar Pedido</button>
            ` : ''}
            ${c.state === 'sale' ? `<button class="btn btn-secondary btn-sm" onclick="window._vVenta(${id})">📋 Ver Orden</button>` : ''}
            ${c.state !== 'cancel' && c.state !== 'sale' ? `<button class="btn btn-sm" style="background:#FEE2E2;color:#DC2626;border:none;padding:6px 14px;border-radius:8px;font-weight:600;cursor:pointer" onclick="window._cancelarCot(${id})">❌ Cancelar</button>` : ''}
            <button class="btn btn-secondary btn-sm" onclick="window._go('cotizaciones')">← Volver</button>
          </div>
        </div>
        <div class="o-smart-buttons">
          <button class="o-smart-btn" ${c.state === 'sale' ? `onclick="window._vVenta(${id})"` : ''}>
            <span class="o-count">${c.state === 'sale' ? '1' : '0'}</span>
            <span class="o-label">📋 Órdenes</span>
          </button>
          <button class="o-smart-btn"><span class="o-count">0</span><span class="o-label">✉️ Emails</span></button>
        </div>
        <div class="o-form-sheet">
          <div class="o-form-title-row">
            <h1 class="o-form-record-title">${c.name || 'Nueva Cotización'}</h1>
            <span class="o-form-subtitle">${c.partner_name || ''}</span>
          </div>
          <div class="o-form-group-wrapper">
            <div class="o-form-group">
              <div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">Cliente</div><div class="o-field-value"><strong>${c.partner_name || c.partner_id || '<span class="o-field-empty">—</span>'}</strong></div></div>
                <div class="o-field-row"><div class="o-field-label">Fecha</div><div class="o-field-value">${c.date_order?.slice(0, 10) || '<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Validez</div><div class="o-field-value">${c.validity_date?.slice(0, 10) || '<span class="o-field-empty">—</span>'}</div></div>
              </div>
              <div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">Estado</div><div class="o-field-value">${stateBadge(c.state, LABEL_MAP[c.state] || c.state)}</div></div>
                <div class="o-field-row"><div class="o-field-label">Referencia</div><div class="o-field-value">${c.client_order_ref || '<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Vendedor</div><div class="o-field-value">${c.user_id || c.user_name || '<span class="o-field-empty">—</span>'}</div></div>
              </div>
            </div>
          </div>
          <div class="o-notebook">
            <div class="o-tabs">
              <button class="o-tab active" onclick="window._ct('cl')">Líneas</button>
              <button class="o-tab" onclick="window._ct('cc')">Condiciones</button>
            </div>
            <div class="o-tab-panel active" id="tab-panel-cl">
              <table class="o-editable-table"><thead><tr>
                <th>Producto</th><th>Descripción</th>
                <th style="text-align:right">Qty</th>
                <th style="text-align:right">Precio</th>
                <th style="text-align:right">Desc.</th>
                <th style="text-align:right">Subtotal</th>
              </tr></thead>
              <tbody id="clineas"><tr><td colspan="6" style="text-align:center;padding:20px;color:var(--text-400)">⏳ Cargando…</td></tr></tbody></table>
              <div class="o-lines-totals"><table>
                <tr><td>Subtotal:</td><td style="text-align:right;font-weight:600">${fmtMxn(c.amount_untaxed)}</td></tr>
                <tr><td>IVA (16%):</td><td style="text-align:right;font-weight:600">${fmtMxn(c.amount_tax)}</td></tr>
                <tr class="total-row"><td>TOTAL:</td><td style="text-align:right">${fmtMxn(c.amount_total)}</td></tr>
              </table></div>
            </div>
            <div class="o-tab-panel" id="tab-panel-cc">
              <div class="o-form-group"><div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">Notas</div><div class="o-field-value">${c.note || '<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Plazo de pago</div><div class="o-field-value">${c.payment_term_name || c.payment_term || '<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Política entrega</div><div class="o-field-value">${c.picking_policy || '<span class="o-field-empty">—</span>'}</div></div>
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
              <div class="o-msg-avatar" style="background:#D97706">C</div>
              <div class="o-msg-content">
                <div class="o-msg-header">
                  <span class="o-msg-author">Sistema</span>
                  <span class="o-msg-date">${new Date().toLocaleDateString('es-MX')}</span>
                </div>
                <div class="o-msg-text">Cotización ${c.name || ''} — Estado: ${LABEL_MAP[c.state] || c.state}</div>
              </div>
            </div>
          </div>
        </div>
      </div>`)

    // Tabs
    window._ct = (tabId) => {
      document.querySelectorAll('.o-tab').forEach(t => t.classList.remove('active'))
      document.querySelectorAll('.o-tab-panel').forEach(p => p.classList.remove('active'))
      const btn = document.querySelector(`.o-tab[onclick*="'${tabId}'"]`)
      if (btn) btn.classList.add('active')
      const panel = document.getElementById('tab-panel-' + tabId)
      if (panel) panel.classList.add('active')
    }

    // Cargar líneas
    try {
      const lr = await api.get(`/cotizaciones/${id}/lineas`)
      const ls = lr?.data || []
      const lb = document.getElementById('clineas')
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
          : '<tr><td colspan="6" style="text-align:center;padding:16px;color:var(--text-400)">Sin líneas de cotización</td></tr>'
      }
    } catch (_) { /* líneas opcionales */ }

    // Acciones del formulario
    window._emailCot = async (cid) => {
      try {
        await api.put(`/cotizaciones/${cid}/enviar`, {})
        toast('OK', 'Cotización enviada por email', 'success')
        window._vCot(cid)
      } catch (e) { toast('Error', e.message, 'error') }
    }
    window._confirmarCot = async (cid) => {
      if (!confirm('¿Confirmar cotización como pedido de venta?')) return
      try {
        await api.confirmarCotizacion(cid)
        toast('OK', 'Cotización confirmada como venta', 'success')
        setTimeout(() => window._go('ventas'), 600)
      } catch (e) { toast('Error', e.message, 'error') }
    }
    window._cancelarCot = async (cid) => {
      if (!confirm('¿Cancelar cotización?')) return
      try {
        await api.cancelarCotizacion(cid)
        toast('Cancelado', '', 'info')
        window._go('cotizaciones')
      } catch (e) { toast('Error', e.message, 'error') }
    }

  } catch (e) {
    setPage(`<div style="padding:40px;text-align:center"><p style="color:#DC2626">⚠️ ${e.message}</p><button class="o-btn-new" onclick="window._go('cotizaciones')">Volver</button></div>`)
  }
}

function _deb(fn, ms) { let t; return (...a) => { clearTimeout(t); t = setTimeout(() => fn(...a), ms) } }
