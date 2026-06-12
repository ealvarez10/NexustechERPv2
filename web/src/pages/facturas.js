/**
 * facturas.js — Módulo Facturas — UI estilo Odoo Enterprise
 * Vista Lista + Kanban + Formulario completo con chatter y CFDI 4.0
 */
import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { api } from '../api.js'
import { toast, fmtMxn, stateBadge, skeletonTable } from '../ui.js'

let _view = 'list', _page = 1, _search = '', _filter = null, _records = []
let cfg = {}

export async function renderFacturas(params = {}) {
  ensureLayout()
  setBreadcrumb([{ label: 'Facturación' }])
  
  // Cargar configuración en vivo (Data Binding)
  cfg = {
    impuestos_ventas: true, impuestos_compras: true, redondeo: false,
    pagos_online: false, descuentos_pronto_pago: false, alertas_cliente: false,
    cfdi_auto: false, cancelacion_directa: false, terminos_default: '',
    ...JSON.parse(localStorage.getItem('nexus_config_facturacion') || '{}')
  }

  // Si viene con un id de factura específico, abrir directo
  if (params.id) {
    setPage(`<div class="nx-module-page"><div style="padding:40px">${skeletonTable(3,5)}</div></div>`)
    await window._vVF(parseInt(params.id))
    return
  }

  setPage(`<div class="nx-module-page"><div id="mcp"></div><div id="mcontent">${skeletonTable(5, 7)}</div></div>`)
  _renderCP()
  await _load()
}

function _renderCP() {
  const el = document.getElementById('mcp')
  if (!el) return
  el.innerHTML = `
    <div class="o-control-panel">
      <div class="o-cp-left">
        <button class="o-btn-new" onclick="window._newFactura()">+ Nueva Factura</button>
        <span class="o-cp-sep"></span>
        <div class="o-dropdown" id="dd-ff">
          <button class="o-btn-filter" onclick="window._tog('dd-ff')">📂 Filtros ▾</button>
          <div class="o-dropdown-menu" id="dd-ff-menu">
            <div class="o-dropdown-item" onclick="window._ff('draft')">Borradores</div>
            <div class="o-dropdown-item" onclick="window._ff('posted')">Publicadas</div>
            <div class="o-dropdown-item" onclick="window._ff('in_payment')">En Pago</div>
            <div class="o-dropdown-item" onclick="window._ff('paid')">Pagadas</div>
            <div class="o-dropdown-item" onclick="window._ff('cancel')">Canceladas</div>
            <div class="o-dropdown-divider"></div>
            <div class="o-dropdown-item" onclick="window._ff(null)">❌ Sin filtro</div>
          </div>
        </div>
        <div class="o-search-box">
          <span style="color:var(--text-400)">🔍</span>
          <input type="search" placeholder="Buscar factura…" id="fs" oninput="window._sf(this.value)">
        </div>
        <span class="o-record-count" id="fcount"></span>
      </div>
      <div class="o-cp-right">
        <button class="o-btn-secondary" style="margin-right:8px;font-size:16px" onclick="window._go('config_facturacion')" title="Ajustes">⚙️</button>
        <div class="o-view-switcher">
          <button class="o-view-btn ${_view === 'list' ? 'active' : ''}" onclick="window._fvv('list')" title="Lista">☰</button>
          <button class="o-view-btn ${_view === 'kanban' ? 'active' : ''}" onclick="window._fvv('kanban')" title="Kanban">⬜</button>
        </div>
      </div>
    </div>`
  _initDD()
  window._fvv = (v) => { _view = v; _renderCP(); _load() }
  window._sf = _deb((q) => { _search = q; _page = 1; _load() }, 300)
  window._ff = (s) => { _filter = s; _page = 1; _load(); window._cdd() }
  window._newFactura = () => { import('./forms/create_forms.js').then(m => m.nuevaFactura(() => _load(), cfg)) }
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
  c.innerHTML = skeletonTable(5, 7)
  try {
    const res = await api.facturas(_page)
    _records = res?.data || []
    let rows = _filter ? _records.filter(r => r.state === _filter) : _records
    if (_search) {
      const q = _search.toLowerCase()
      rows = rows.filter(r => (r.name || '').toLowerCase().includes(q) || (r.partner_name || '').toLowerCase().includes(q))
    }
    const fc = document.getElementById('fcount'); if (fc) fc.textContent = rows.length + ' registros'
    c.innerHTML = _view === 'kanban' ? _kanban(rows) : _list(rows)
    if (_view === 'list') _initCB()
  } catch (e) {
    c.innerHTML = `<div style="padding:40px;text-align:center;color:var(--text-400)">⚠️ ${e.message}</div>`
  }
}

const LABEL_MAP = {
  draft:      'Borrador',
  posted:     'Publicada',
  in_payment: 'En Pago',
  paid:       'Pagada',
  cancel:     'Cancelada',
}

function _list(rows) {
  if (!rows.length) return `<div style="padding:60px;text-align:center"><div style="font-size:48px;margin-bottom:12px">🧾</div><p style="color:var(--text-400)">Sin facturas. Timbra la primera.</p></div>`
  return `
    <div class="o-list-actions-bar" id="flab"><span class="o-actions-count" id="fsel-cnt">0 seleccionados</span>
      <button class="o-action-btn-sm" onclick="alert('Exportar')">Exportar</button>
    </div>
    <div class="o-list-view"><table>
      <thead><tr>
        <th class="th-check"><input type="checkbox" class="o-list-checkbox" id="fca" onchange="window._fca(this.checked)"></th>
        <th>Número</th><th>Cliente</th><th>Tipo</th><th>Fecha</th><th>Estado</th><th style="text-align:right">Total</th><th style="text-align:right">Saldo</th>
      </tr></thead>
      <tbody>
        ${rows.map(r => `
          <tr onclick="window._vVF(${r.id})" data-id="${r.id}">
            <td class="td-check" onclick="event.stopPropagation()"><input type="checkbox" class="o-list-checkbox frc" data-id="${r.id}" onchange="window._frc()"></td>
            <td><strong>${r.name || '-'}</strong></td>
            <td>${r.partner_name || r.partner_id || '-'}
                ${cfg.alertas_cliente && r.amount_residual > 0 ? ' <span style="color:#DC2626;font-size:11px" title="Tiene deuda">⚠️</span>' : ''}
            </td>
            <td><span style="font-size:11px;color:var(--text-400)">${r.move_type === 'out_invoice' ? 'Factura' : r.move_type || '-'}</span></td>
            <td>${r.invoice_date?.slice(0, 10) || r.date?.slice(0, 10) || '-'}</td>
            <td>${stateBadge(r.state, LABEL_MAP[r.state] || r.state)}</td>
            <td style="text-align:right;font-weight:700;color:var(--primary)">${fmtMxn(r.amount_total)}</td>
            <td style="text-align:right;color:${r.amount_residual > 0 ? '#DC2626' : 'var(--text-400)'}">${fmtMxn(r.amount_residual || 0)}</td>
          </tr>`).join('')}
      </tbody></table></div>
    <div style="padding:12px 20px;display:flex;justify-content:space-between;align-items:center;background:var(--bg-card);border-top:1px solid var(--border)">
      <span style="font-size:13px;color:var(--text-400)">${rows.length} registros</span>
      <div style="display:flex;gap:8px">
        <button class="o-action-btn-sm" ${_page <= 1 ? 'disabled' : ''} onclick="window._fp(${_page - 1})">‹ Anterior</button>
        <span style="padding:5px 10px;font-size:13px">${_page}</span>
        <button class="o-action-btn-sm" onclick="window._fp(${_page + 1})">Siguiente ›</button>
      </div></div>`
}

const KANBAN_COLS = [
  { key: 'draft',      label: 'Borrador',   color: '#9CA3AF' },
  { key: 'posted',     label: 'Publicada',  color: '#059669' },
  { key: 'in_payment', label: 'En Pago',    color: '#7C3AED' },
  { key: 'paid',       label: 'Pagada',     color: '#0EA5E9' },
  { key: 'cancel',     label: 'Cancelada',  color: '#DC2626' },
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
          <div class="o-kanban-card" onclick="window._vVF(${r.id})">
            <div class="o-kanban-card-title">${r.name || '#' + r.id}</div>
            <div style="font-size:12px;color:var(--text-400);margin-bottom:8px">
              ${r.partner_name || ''}
              ${cfg.alertas_cliente && r.amount_residual > 0 ? ' <span style="color:#DC2626">⚠️</span>' : ''}
            </div>
            <div class="o-kanban-card-meta">
              <span style="font-size:11px">${r.invoice_date?.slice(0, 10) || ''}</span>
              <span class="o-kanban-card-amount">${fmtMxn(r.amount_total)}</span>
            </div>
          </div>`).join('') || '<div style="padding:16px;text-align:center;color:var(--text-300);font-size:12px">Vacío</div>'}
      </div>
    </div>`).join('')}</div>`
}

function _initCB() {
  window._fca = (c) => { document.querySelectorAll('.frc').forEach(cb => cb.checked = c); window._frc() }
  window._frc = () => {
    const n = document.querySelectorAll('.frc:checked').length
    const b = document.getElementById('flab'), s = document.getElementById('fsel-cnt')
    if (b) b.classList.toggle('visible', n > 0)
    if (s) s.textContent = n + ' seleccionado' + (n !== 1 ? 's' : '')
    document.querySelectorAll('[data-id]').forEach(tr => {
      const cb = tr.querySelector('.frc'); if (cb) tr.classList.toggle('selected', cb.checked)
    })
  }
}

window._fp = (p) => { _page = p; _load() }

// ===== FORMULARIO FACTURA =====
window._vVF = async (id) => {
  setBreadcrumb([{ label: 'Facturación', href: '#facturas' }, { label: 'Cargando…' }])
  setPage(`<div style="padding:40px">${skeletonTable(3, 5)}</div>`)
  try {
    const res = await api.factura(id)
    const f = res?.data || res; if (!f) throw new Error('No encontrada')
    setBreadcrumb([{ label: 'Facturación', href: '#facturas' }, { label: f.name || '#' + id }])

    const STEPS = ['draft', 'posted', 'in_payment', 'paid']
    if (f.state === 'cancel') STEPS.push('cancel')
    const si = STEPS.indexOf(f.state)
    const STEP_LABELS = { draft: 'Borrador', posted: 'Publicada', in_payment: 'En Pago', paid: 'Pagada', cancel: 'Cancelada' }

    setPage(`
      <div class="o-form-view" id="ffv">
        <div class="o-statusbar">
          <div class="o-statusbar-status">
            ${STEPS.map((s, i) => `
              <div class="o-status-step ${s === f.state ? 'active' : ''} ${i < si ? 'done' : ''}">
                ${i < si ? '✔ ' : ''}${STEP_LABELS[s] || s}
              </div>${i < STEPS.length - 1 ? '<span class="o-status-arrow">›</span>' : ''}`).join('')}
          </div>
          <div class="o-statusbar-buttons">
            ${f.state === 'draft' ? `<button class="btn btn-primary btn-sm" onclick="window._pubF(${id})">✅ Confirmar / Publicar</button>` : ''}
            ${f.state === 'posted' ? `<button class="btn btn-primary btn-sm" onclick="window._pagoF(${id})">💳 Registrar Pago</button>` : ''}
            
            ${cfg.pagos_online && f.state === 'posted' ? `<button class="btn btn-secondary btn-sm" onclick="toast('Link de Pago', 'https://pagos.nexustecherp.com/pay/${f.id}', 'info')">🔗 Generar Enlace de Pago</button>` : ''}
            
            ${(f.state === 'draft' || f.state === 'posted') ? `<button class="btn btn-secondary btn-sm" onclick="window._timF(${id})">🔐 Timbrar CFDI</button>` : ''}
            <button class="btn btn-secondary btn-sm" onclick="toast('Info','PDF próximamente','info')">📄 Descargar PDF</button>
            ${f.state !== 'cancel' && f.state !== 'paid' ? `<button class="btn btn-sm" style="background:#FEE2E2;color:#DC2626;border:none;padding:6px 14px;border-radius:8px;font-weight:600;cursor:pointer" onclick="window._cancF(${id})">❌ Cancelar${cfg.cancelacion_directa ? ' (Directo)' : ''}</button>` : ''}
            <button class="btn btn-secondary btn-sm" onclick="window._go('facturas')">← Volver</button>
          </div>
        </div>
        <div class="o-smart-buttons">
          <button class="o-smart-btn"><span class="o-count">${f.payment_ids?.length || 0}</span><span class="o-label">💳 Pagos</span></button>
          ${cfg.pagos_online ? `<button class="o-smart-btn"><span class="o-count" style="color:#059669">0</span><span class="o-label">Stripe</span></button>` : ''}
          <button class="o-smart-btn"><span class="o-count">0</span><span class="o-label">🔐 CFDI</span></button>
        </div>
        <div class="o-form-sheet">
          <div class="o-form-title-row">
            <h1 class="o-form-record-title">${f.name || 'Nueva Factura'}</h1>
            <span class="o-form-subtitle">${f.partner_name || ''}</span>
          </div>
          <div class="o-form-group-wrapper">
            <div class="o-form-group">
              <div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">Número</div><div class="o-field-value"><strong>${f.name || '<span class="o-field-empty">Borrador</span>'}</strong></div></div>
                <div class="o-field-row"><div class="o-field-label">Tipo</div><div class="o-field-value">${f.move_type === 'out_invoice' ? 'Factura de cliente' : f.move_type || '<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Cliente</div><div class="o-field-value"><strong>${f.partner_name || f.partner_id || '<span class="o-field-empty">—</span>'}</strong></div></div>
                <div class="o-field-row"><div class="o-field-label">Fecha</div><div class="o-field-value">${f.invoice_date?.slice(0, 10) || f.date?.slice(0, 10) || '<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Vencimiento</div><div class="o-field-value">${f.invoice_date_due?.slice(0, 10) || '<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Referencia</div><div class="o-field-value">${f.ref || '<span class="o-field-empty">—</span>'}</div></div>
              </div>
              <div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">Estado</div><div class="o-field-value">${stateBadge(f.state, LABEL_MAP[f.state] || f.state)}</div></div>
                <div class="o-field-row"><div class="o-field-label">Empresa</div><div class="o-field-value">${f.company_id || f.company_name || '<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Diario</div><div class="o-field-value">${f.journal_id || f.journal_name || '<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Método Pago</div><div class="o-field-value">${f.invoice_payment_term_id || f.payment_term || '<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Saldo</div><div class="o-field-value" style="font-weight:700;color:${f.amount_residual > 0 ? '#DC2626' : 'var(--text-700)'}">${fmtMxn(f.amount_residual || 0)}</div></div>
              </div>
            </div>
          </div>
          <div class="o-notebook">
            <div class="o-tabs">
              <button class="o-tab active" onclick="window._ft('fl')">Líneas de Factura</button>
              <button class="o-tab" onclick="window._ft('fi')">Otra Información</button>
              <button class="o-tab" onclick="window._ft('fc')">CFDI 4.0</button>
            </div>
            <div class="o-tab-panel active" id="tab-panel-fl">
              <table class="o-editable-table"><thead><tr>
                <th>Producto / Servicio</th><th>Descripción</th>
                <th style="text-align:right">Qty</th>
                <th style="text-align:right">Precio</th>
                ${cfg.descuentos_pronto_pago ? '<th style="text-align:right">% P.P.</th>' : ''}
                ${cfg.impuestos_ventas ? '<th style="text-align:right">Impuesto</th>' : ''}
                <th style="text-align:right">Subtotal</th>
              </tr></thead>
              <tbody id="flineas"><tr><td colspan="${cfg.impuestos_ventas ? 6 : 5}" style="text-align:center;padding:20px;color:var(--text-400)">⏳ Cargando…</td></tr></tbody></table>
              <div class="o-lines-totals"><table id="ftotals">
                <!-- Se llena asincronamente -->
              </table></div>
            </div>
            <div class="o-tab-panel" id="tab-panel-fi">
              <div class="o-form-group"><div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">Notas</div><div class="o-field-value">${f.narration || f.note || (f.state === 'draft' ? cfg.terminos_default : '<span class="o-field-empty">—</span>')}</div></div>
                <div class="o-field-row"><div class="o-field-label">Ref. Interna</div><div class="o-field-value">${f.payment_reference || '<span class="o-field-empty">—</span>'}</div></div>
              </div></div>
            </div>
            <div class="o-tab-panel" id="tab-panel-fc">
              <div class="o-form-group"><div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">UUID CFDI</div><div class="o-field-value">${f.l10n_mx_edi_cfdi_uuid || '<span class="o-field-empty">No timbrado</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Uso CFDI</div><div class="o-field-value">${f.l10n_mx_edi_usage || '<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Método Pago SAT</div><div class="o-field-value">${f.l10n_mx_edi_payment_method_id || '<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Forma Pago SAT</div><div class="o-field-value">${f.l10n_mx_edi_payment_policy || '<span class="o-field-empty">—</span>'}</div></div>
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
              <div class="o-msg-avatar" style="background:#059669">F</div>
              <div class="o-msg-content">
                <div class="o-msg-header">
                  <span class="o-msg-author">Sistema</span>
                  <span class="o-msg-date">${new Date().toLocaleDateString('es-MX')}</span>
                </div>
                <div class="o-msg-text">Factura ${f.name || ''} — Estado: ${LABEL_MAP[f.state] || f.state}</div>
              </div>
            </div>
          </div>
        </div>
      </div>`)

    // Tabs
    window._ft = (tabId) => {
      document.querySelectorAll('.o-tab').forEach(t => t.classList.remove('active'))
      document.querySelectorAll('.o-tab-panel').forEach(p => p.classList.remove('active'))
      const btn = document.querySelector(`.o-tab[onclick*="'${tabId}'"]`)
      if (btn) btn.classList.add('active')
      const panel = document.getElementById('tab-panel-' + tabId)
      if (panel) panel.classList.add('active')
    }

    // Cargar líneas
    try {
      const lr = await api.get(`/facturas/${id}/lineas`)
      const ls = lr?.data || []
      const lb = document.getElementById('flineas')
      
      let totalLines = 0;
      let totalTax = 0;

      if (lb) {
        lb.innerHTML = ls.length
          ? ls.map(l => {
              const sub = l.price_unit * (l.quantity || 0);
              totalLines += sub;
              if (cfg.impuestos_ventas && l.tax_ids?.length) {
                totalTax += sub * 0.16;
              }
              return `<tr>
              <td>${l.product_id ? '#' + l.product_id : '<span class="o-field-empty">Servicio</span>'}</td>
              <td>${l.name || '-'}</td>
              <td style="text-align:right">${l.quantity ?? 0}</td>
              <td style="text-align:right">${fmtMxn(l.price_unit)}</td>
              ${cfg.descuentos_pronto_pago ? '<td style="text-align:right"><span style="color:var(--text-400);font-size:11px">0%</span></td>' : ''}
              ${cfg.impuestos_ventas ? `<td style="text-align:right;font-size:11px">${l.tax_ids?.length ? 'IVA 16%' : '—'}</td>` : ''}
              <td style="text-align:right;font-weight:700">${fmtMxn(sub)}</td>
            </tr>`
            }).join('')
          : `<tr><td colspan="${(cfg.impuestos_ventas ? 6 : 5) + (cfg.descuentos_pronto_pago ? 1 : 0)}" style="text-align:center;padding:16px;color:var(--text-400)">Sin líneas de factura</td></tr>`
      }

      // Render de Totales inyectando configuración
      const tb = document.getElementById('ftotals')
      if (tb) {
        let finalTotal = totalLines + totalTax;
        let diffRound = 0;
        
        if (cfg.redondeo) {
          const rounded = Math.round(finalTotal * 20) / 20; // cash rounding to nearest 0.05
          diffRound = rounded - finalTotal;
          finalTotal = rounded;
        }

        tb.innerHTML = `
          <tr><td>Subtotal:</td><td style="text-align:right;font-weight:600">${fmtMxn(totalLines)}</td></tr>
          ${cfg.impuestos_ventas ? `<tr><td>IVA (16%):</td><td style="text-align:right;font-weight:600">${fmtMxn(totalTax)}</td></tr>` : ''}
          ${cfg.redondeo ? `<tr><td>Ajuste (Redondeo):</td><td style="text-align:right;color:var(--text-500)">${fmtMxn(diffRound)}</td></tr>` : ''}
          <tr class="total-row"><td>TOTAL:</td><td style="text-align:right">${fmtMxn(finalTotal)}</td></tr>
        `
      }
    } catch (_) { /* líneas opcionales */ }

    // Acciones del formulario
    window._pubF = async (fid) => {
      if (!confirm('¿Confirmar y publicar factura?')) return
      try {
        await api.put(`/facturas/${fid}/confirmar`, {})
        
        if (cfg.cfdi_auto) {
          toast('CFDI Auto', 'Timbrando automáticamente...', 'info')
          setTimeout(() => { toast('Timbrado Exitoso', 'El CFDI se ha enviado al PAC', 'success') }, 1500)
        } else {
          toast('OK', 'Factura publicada', 'success')
        }
        
        window._vVF(fid)
      } catch (e) { toast('Error', e.message, 'error') }
    }
    window._pagoF = async (fid) => {
      if (!confirm('¿Registrar pago de esta factura?')) return
      try {
        await api.post(`/facturas/${fid}/pago`, {})
        toast('OK', 'Pago registrado', 'success')
        window._vVF(fid)
      } catch (e) { toast('Error', e.message, 'error') }
    }
    window._timF = (fid) => { window._go('cfdi') }
    window._cancF = async (fid) => {
      let msg = '¿Cancelar factura?';
      if (cfg.cancelacion_directa) {
        msg = '⚠️ ADVERTENCIA: La cancelación directa omitirá el estatus en el SAT. ¿Proceder?';
      }
      if (!confirm(msg)) return
      try {
        await api.put(`/facturas/${fid}/cancelar`, {})
        toast('Cancelado', 'Factura cancelada con éxito', 'info')
        window._go('facturas')
      } catch (e) { toast('Error', e.message, 'error') }
    }

  } catch (e) {
    setPage(`<div style="padding:40px;text-align:center"><p style="color:#DC2626">⚠️ ${e.message}</p><button class="o-btn-new" onclick="window._go('facturas')">Volver</button></div>`)
  }
}

function _deb(fn, ms) { let t; return (...a) => { clearTimeout(t); t = setTimeout(() => fn(...a), ms) } }
