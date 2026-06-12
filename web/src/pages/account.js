/**
 * account.js — Módulo Contabilidad — UI estilo Odoo Enterprise
 * Lista interactiva + Kanban + Formulario completo + Creación
 * Datos reales desde /api/v1/account-moves
 */
import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { api } from '../api.js'
import { toast, fmtMxn, stateBadge, skeletonTable } from '../ui.js'

let _view = 'list', _page = 1, _search = '', _filter = null, _type = null, _records = []

export async function renderAccount(params = {}) {
  ensureLayout()
  setBreadcrumb([{ label: 'Contabilidad' }, { label: 'Asientos' }])

  if (params.id) {
    setPage(`<div class="nx-module-page"><div style="padding:40px">${skeletonTable(3, 5)}</div></div>`)
    await _viewForm(parseInt(params.id))
    return
  }

  setPage(`<div class="nx-module-page"><div id="mcp"></div><div id="mcontent">${skeletonTable(5, 7)}</div></div>`)
  _renderCP()
  await _load()
}

// ─── Panel de control ─────────────────────────────────────────────────────────
function _renderCP() {
  const el = document.getElementById('mcp')
  if (!el) return
  el.innerHTML = `
    <div class="o-control-panel">
      <div class="o-cp-left">
        <button class="o-btn-new" onclick="window._acNew()">+ Nuevo Asiento</button>
        <span class="o-cp-sep"></span>
        <div class="o-dropdown" id="dd-acf">
          <button class="o-btn-filter" onclick="window._tog('dd-acf')">📂 Tipo ▾</button>
          <div class="o-dropdown-menu" id="dd-acf-menu">
            <div class="o-dropdown-item" onclick="window._acft('out_invoice')">🧾 Facturas cliente</div>
            <div class="o-dropdown-item" onclick="window._acft('in_invoice')">📥 Facturas proveedor</div>
            <div class="o-dropdown-item" onclick="window._acft('out_refund')">↩️ N/C cliente</div>
            <div class="o-dropdown-item" onclick="window._acft('in_refund')">↩️ N/C proveedor</div>
            <div class="o-dropdown-item" onclick="window._acft('entry')">📒 Entradas de diario</div>
            <div class="o-dropdown-divider"></div>
            <div class="o-dropdown-item" onclick="window._acft(null)">❌ Todos los tipos</div>
          </div>
        </div>
        <div class="o-dropdown" id="dd-acs">
          <button class="o-btn-filter" onclick="window._tog('dd-acs')">🔖 Estado ▾</button>
          <div class="o-dropdown-menu" id="dd-acs-menu">
            <div class="o-dropdown-item" onclick="window._acff('draft')">Borrador</div>
            <div class="o-dropdown-item" onclick="window._acff('posted')">Publicado</div>
            <div class="o-dropdown-item" onclick="window._acff('cancel')">Cancelado</div>
            <div class="o-dropdown-divider"></div>
            <div class="o-dropdown-item" onclick="window._acff(null)">❌ Sin filtro</div>
          </div>
        </div>
        <div class="o-search-box">
          <span style="color:var(--text-400)">🔍</span>
          <input type="search" placeholder="Buscar asiento…" id="acs-q" oninput="window._acfs(this.value)">
        </div>
        <span class="o-record-count" id="ac-count"></span>
      </div>
      <div class="o-cp-right">
        <button class="o-btn-secondary" style="margin-right:8px;font-size:16px" onclick="window._go('config_contabilidad')" title="Ajustes">⚙️</button>
        <div class="o-view-switcher">
          <button class="o-view-btn ${_view === 'list'   ? 'active' : ''}" onclick="window._acvv('list')"   title="Lista">☰</button>
          <button class="o-view-btn ${_view === 'kanban' ? 'active' : ''}" onclick="window._acvv('kanban')" title="Kanban">⬜</button>
        </div>
      </div>
    </div>`
  _initDD()
  window._acvv = (v)  => { _view = v; _renderCP(); _load() }
  window._acfs = _deb((q) => { _search = q; _page = 1; _load() }, 300)
  window._acff = (s)  => { _filter = s; _page = 1; _load(); window._cdd() }
  window._acft = (t)  => { _type = t;   _page = 1; _load(); window._cdd() }
  window._acNew = ()  => _openCreateModal()
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

// ─── Carga real de datos ──────────────────────────────────────────────────────
async function _load() {
  const c = document.getElementById('mcontent'); if (!c) return
  c.innerHTML = skeletonTable(5, 7)
  try {
    const params = { pagina: _page }
    if (_type)   params.move_type = _type
    if (_filter) params.state     = _filter
    if (_search) params.q         = _search

    const res = await api.accountMoves(params)
    // La API devuelve { success, data: { data: [...], total, pagina } }
    _records = Array.isArray(res?.data?.data) ? res.data.data
             : Array.isArray(res?.data)        ? res.data
             : []

    const cnt = document.getElementById('ac-count')
    if (cnt) cnt.textContent = _records.length + ' registros'
    c.innerHTML = _view === 'kanban' ? _kanban(_records) : _list(_records)
    if (_view === 'list') _initCB()
  } catch (e) {
    c.innerHTML = `<div style="padding:40px;text-align:center;color:var(--text-400)">⚠️ ${e.message}</div>`
  }
}

// ─── Etiquetas ────────────────────────────────────────────────────────────────
const STATE_LABELS = { draft: 'Borrador', posted: 'Publicado', cancel: 'Cancelado' }
const TYPE_LABELS  = {
  out_invoice: 'Factura cliente', in_invoice: 'Factura prov.',
  out_refund: 'N/C cliente',      in_refund: 'N/C proveedor',
  entry: 'Entrada diario',        out_receipt: 'Recibo salida', in_receipt: 'Recibo entrada'
}

// ─── Vista Lista ──────────────────────────────────────────────────────────────
function _list(rows) {
  if (!rows.length) return `
    <div style="padding:80px;text-align:center">
      <div style="font-size:56px;margin-bottom:16px">📒</div>
      <p style="color:var(--text-400);font-size:16px">Sin asientos contables. Crea el primero.</p>
      <button class="o-btn-new" style="margin-top:16px" onclick="window._acNew()">+ Nuevo Asiento</button>
    </div>`
  return `
    <div class="o-list-actions-bar" id="ac-lab">
      <span class="o-actions-count" id="ac-sel-cnt">0 seleccionados</span>
      <button class="o-action-btn-sm" onclick="toast('Info','Exportar próximamente','info')">Exportar</button>
    </div>
    <div class="o-list-view"><table>
      <thead><tr>
        <th class="th-check"><input type="checkbox" class="o-list-checkbox" id="ac-ca" onchange="window._acca(this.checked)"></th>
        <th>Número</th><th>Tipo</th><th>Empresa / Proveedor</th>
        <th>Fecha</th><th>Fecha venc.</th><th>Referencia</th>
        <th>Estado</th>
        <th style="text-align:right">Subtotal</th>
        <th style="text-align:right">Total</th>
        <th style="text-align:right">Saldo pend.</th>
      </tr></thead>
      <tbody>
        ${rows.map(r => {
          const saldo = parseFloat(r.amount_residual) || 0
          const total = parseFloat(r.amount_total) || 0
          const sub   = parseFloat(r.amount_untaxed) || 0
          const vencido = saldo > 0 && r.invoice_date_due && new Date(r.invoice_date_due) < new Date()
          return `
          <tr onclick="window._acView(${r.id})" data-id="${r.id}" style="cursor:pointer">
            <td class="td-check" onclick="event.stopPropagation()">
              <input type="checkbox" class="o-list-checkbox acrc" data-id="${r.id}" onchange="window._acrc()">
            </td>
            <td><strong style="color:var(--primary)">${r.name || '/'}</strong></td>
            <td><span style="font-size:11px;color:var(--text-400);white-space:nowrap">${TYPE_LABELS[r.move_type] || r.move_type || '—'}</span></td>
            <td>${r.partner_name
              ? `<a href="#" onclick="event.stopPropagation()" style="color:var(--primary)">${r.partner_name}</a>`
              : '<span style="color:var(--text-300)">—</span>'}</td>
            <td style="white-space:nowrap">${r.invoice_date?.slice(0,10) || r.date?.slice(0,10) || '—'}</td>
            <td style="white-space:nowrap;color:${vencido?'#DC2626':'inherit'}">${r.invoice_date_due?.slice(0,10) || '—'}</td>
            <td style="font-size:12px;color:var(--text-400)">${r.ref_ || r.invoice_origin || '—'}</td>
            <td>${stateBadge(r.state, STATE_LABELS[r.state] || r.state)}</td>
            <td style="text-align:right;font-size:13px">${fmtMxn(sub)}</td>
            <td style="text-align:right;font-weight:700;color:var(--primary)">${fmtMxn(total)}</td>
            <td style="text-align:right;font-weight:600;color:${saldo>0?'#DC2626':'var(--text-400)'}">${fmtMxn(saldo)}</td>
          </tr>`
        }).join('')}
      </tbody>
    </table></div>
    <div style="padding:12px 20px;display:flex;justify-content:space-between;align-items:center;background:var(--bg-card);border-top:1px solid var(--border)">
      <span style="font-size:13px;color:var(--text-400)">${rows.length} registros</span>
      <div style="display:flex;gap:8px">
        <button class="o-action-btn-sm" ${_page<=1?'disabled':''} onclick="window._acpg(${_page-1})">‹ Anterior</button>
        <span style="padding:5px 10px;font-size:13px">${_page}</span>
        <button class="o-action-btn-sm" onclick="window._acpg(${_page+1})">Siguiente ›</button>
      </div>
    </div>`
}

// ─── Vista Kanban ─────────────────────────────────────────────────────────────
const KANBAN_COLS = [
  { key: 'draft',  label: 'Borrador',  color: '#9CA3AF' },
  { key: 'posted', label: 'Publicado', color: '#059669' },
  { key: 'cancel', label: 'Cancelado', color: '#DC2626' },
]
function _kanban(rows) {
  const g = {}; KANBAN_COLS.forEach(c => g[c.key] = [])
  rows.forEach(r => { if (g[r.state]) g[r.state].push(r); else g['draft'].push(r) })
  return `<div class="o-kanban-view">${KANBAN_COLS.map(col => `
    <div class="o-kanban-col">
      <div class="o-kanban-col-header" style="border-top:3px solid ${col.color}">
        <span>${col.label}</span><span class="o-kanban-col-count">${g[col.key].length}</span>
      </div>
      <div class="o-kanban-cards">
        ${g[col.key].map(r => `
          <div class="o-kanban-card" onclick="window._acView(${r.id})">
            <div class="o-kanban-card-title">${r.name || '/'}</div>
            <div style="font-size:12px;color:var(--text-400);margin:4px 0 8px">${r.partner_name || TYPE_LABELS[r.move_type] || r.move_type || ''}</div>
            <div style="font-size:11px;color:var(--text-400);margin-bottom:6px">${r.date?.slice(0,10) || ''}</div>
            <div class="o-kanban-card-meta">
              <span style="font-size:11px">${TYPE_LABELS[r.move_type] || ''}</span>
              <span class="o-kanban-card-amount">${fmtMxn(parseFloat(r.amount_total)||0)}</span>
            </div>
          </div>`).join('') || '<div style="padding:16px;text-align:center;color:var(--text-300);font-size:12px">Vacío</div>'}
      </div>
    </div>`).join('')}</div>`
}

// ─── Checkboxes ───────────────────────────────────────────────────────────────
function _initCB() {
  window._acca = (c) => { document.querySelectorAll('.acrc').forEach(cb => cb.checked = c); window._acrc() }
  window._acrc = () => {
    const n = document.querySelectorAll('.acrc:checked').length
    const b = document.getElementById('ac-lab'), s = document.getElementById('ac-sel-cnt')
    if (b) b.classList.toggle('visible', n > 0)
    if (s) s.textContent = n + ' seleccionado' + (n !== 1 ? 's' : '')
    document.querySelectorAll('[data-id]').forEach(tr => {
      const cb = tr.querySelector('.acrc'); if (cb) tr.classList.toggle('selected', cb.checked)
    })
  }
}

window._acpg = (p) => { _page = p; _load() }

// ─── Formulario de detalle ────────────────────────────────────────────────────
window._acView = async (id) => {
  setBreadcrumb([{ label: 'Contabilidad', href: '#account' }, { label: 'Cargando…' }])
  setPage(`<div style="padding:40px">${skeletonTable(3,5)}</div>`)
  await _viewForm(id)
}

async function _viewForm(id) {
  try {
    const res = await api.accountMove(id)
    // La API devuelve { success, data: { ...campos } }
    const f = res?.data
    if (!f || typeof f !== 'object') throw new Error('Asiento no encontrado')

    const typeLabel  = TYPE_LABELS[f.move_type] || f.move_type || 'Asiento'
    const stateLabel = STATE_LABELS[f.state] || f.state
    const saldo      = parseFloat(f.amount_residual) || 0
    const total      = parseFloat(f.amount_total) || 0
    const sub        = parseFloat(f.amount_untaxed) || 0

    setBreadcrumb([{ label: 'Contabilidad', href: '#account' }, { label: f.name || '/' }])

    const STEPS = ['draft', 'posted']
    if (f.state === 'cancel') STEPS.push('cancel')
    const si = STEPS.indexOf(f.state)

    setPage(`
      <div class="o-form-view" id="ac-fv">
        <div class="o-statusbar">
          <div class="o-statusbar-status">
            ${STEPS.map((s, i) => `
              <div class="o-status-step ${s === f.state ? 'active' : ''} ${i < si ? 'done' : ''}">
                ${i < si ? '✔ ' : ''}${STATE_LABELS[s] || s}
              </div>${i < STEPS.length - 1 ? '<span class="o-status-arrow">›</span>' : ''}`).join('')}
          </div>
          <div class="o-statusbar-buttons">
            ${f.state === 'draft'  ? `<button class="btn btn-primary btn-sm" onclick="window._acConfirm(${id})">✅ Confirmar</button>` : ''}
            ${f.state === 'posted' ? `<button class="btn btn-secondary btn-sm" onclick="window._acReset(${id})">↩️ A borrador</button>` : ''}
            <button class="btn btn-secondary btn-sm" onclick="toast('PDF','Próximamente','info')">📄 PDF</button>
            ${f.state !== 'cancel' ? `<button class="btn btn-sm" style="background:#FEE2E2;color:#DC2626;border:none;padding:6px 14px;border-radius:8px;font-weight:600;cursor:pointer" onclick="window._acCancel(${id})">❌ Cancelar</button>` : ''}
            <button class="btn btn-secondary btn-sm" onclick="window._go('account')">← Volver</button>
          </div>
        </div>

        <div class="o-smart-buttons">
          <button class="o-smart-btn" id="smb-lineas"><span class="o-count">…</span><span class="o-label">📋 Líneas</span></button>
          <button class="o-smart-btn"><span class="o-count">0</span><span class="o-label">💳 Pagos</span></button>
          <button class="o-smart-btn"><span class="o-count">0</span><span class="o-label">📎 Adjuntos</span></button>
        </div>

        <div class="o-form-sheet">
          <div class="o-form-title-row">
            <h1 class="o-form-record-title">${f.name || 'Nuevo Asiento'}</h1>
            <span class="o-form-subtitle">${typeLabel}${f.partner_name ? ' · ' + f.partner_name : ''}</span>
          </div>
          <div class="o-form-group-wrapper">
            <div class="o-form-group">
              <div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">Número</div><div class="o-field-value"><strong>${f.name || '<span class="o-field-empty">Borrador</span>'}</strong></div></div>
                <div class="o-field-row"><div class="o-field-label">Tipo</div><div class="o-field-value">${typeLabel}</div></div>
                <div class="o-field-row"><div class="o-field-label">${f.move_type === 'in_invoice' || f.move_type === 'in_refund' ? 'Proveedor' : 'Cliente'}</div>
                  <div class="o-field-value"><strong>${f.partner_name || '<span class="o-field-empty">—</span>'}</strong></div></div>
                <div class="o-field-row"><div class="o-field-label">Fecha factura</div><div class="o-field-value">${f.invoice_date?.slice(0,10) || f.date?.slice(0,10) || '<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Fecha contable</div><div class="o-field-value">${f.date?.slice(0,10) || '<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Fecha vencimiento</div>
                  <div class="o-field-value" style="color:${saldo>0&&f.invoice_date_due?'#DC2626':'inherit'}">${f.invoice_date_due?.slice(0,10) || '<span class="o-field-empty">—</span>'}</div></div>
              </div>
              <div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">Estado</div><div class="o-field-value">${stateBadge(f.state, stateLabel)}</div></div>
                <div class="o-field-row"><div class="o-field-label">Diario</div><div class="o-field-value">${f.journal_name || f.journal_id || '<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Moneda</div><div class="o-field-value">${f.currency_name || 'MXN'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Referencia</div><div class="o-field-value">${f.ref_ || f.invoice_origin || '<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Subtotal</div><div class="o-field-value">${fmtMxn(sub)}</div></div>
                <div class="o-field-row"><div class="o-field-label">Total</div>
                  <div class="o-field-value" style="font-weight:700;font-size:18px;color:var(--primary)">${fmtMxn(total)}</div></div>
                <div class="o-field-row"><div class="o-field-label">Saldo pendiente</div>
                  <div class="o-field-value" style="font-weight:600;color:${saldo>0?'#DC2626':'#059669'}">${fmtMxn(saldo)}</div></div>
              </div>
            </div>
          </div>

          <div class="o-notebook">
            <div class="o-tabs">
              <button class="o-tab active" onclick="window._acTab('lineas')">Líneas de asiento</button>
              <button class="o-tab" onclick="window._acTab('info')">Otra información</button>
            </div>
            <div class="o-tab-panel active" id="tab-panel-lineas">
              <table class="o-editable-table"><thead><tr>
                <th>Cuenta</th><th>Descripción</th><th>Empresa / Proveedor</th>
                <th style="text-align:right">Débito</th>
                <th style="text-align:right">Crédito</th>
                <th style="text-align:right">Saldo</th>
              </tr></thead><tbody id="ac-lineas">
                <tr><td colspan="6" style="text-align:center;padding:30px;color:var(--text-400)">⏳ Cargando líneas…</td></tr>
              </tbody></table>
              <div class="o-lines-totals"><table id="ac-totals"></table></div>
            </div>
            <div class="o-tab-panel" id="tab-panel-info">
              <div class="o-form-group"><div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">Origen documento</div><div class="o-field-value">${f.invoice_origin || '<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Ref. pago</div><div class="o-field-value">${f.payment_reference || '<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Ref. interna</div><div class="o-field-value">${f.ref_ || '<span class="o-field-empty">—</span>'}</div></div>
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
              <div class="o-msg-avatar" style="background:var(--primary)">S</div>
              <div class="o-msg-content">
                <div class="o-msg-header">
                  <span class="o-msg-author">Sistema</span>
                  <span class="o-msg-date">${new Date().toLocaleDateString('es-MX')}</span>
                </div>
                <div class="o-msg-text">Asiento <strong>${f.name || '/'}</strong> — ${typeLabel} — Estado: <strong>${stateLabel}</strong></div>
              </div>
            </div>
          </div>
        </div>
      </div>`)

    // Tabs
    window._acTab = (tabId) => {
      document.querySelectorAll('.o-tab').forEach(t => t.classList.remove('active'))
      document.querySelectorAll('.o-tab-panel').forEach(p => p.classList.remove('active'))
      const btn = document.querySelector(`.o-tab[onclick*="'${tabId}'"]`)
      if (btn) btn.classList.add('active')
      const panel = document.getElementById('tab-panel-' + tabId)
      if (panel) panel.classList.add('active')
    }

    // Cargar líneas reales
    try {
      const lr = await api.accountMoveLineas(id)
      // api.ok(rows) → { success, data: [...] }
      const ls = Array.isArray(lr?.data) ? lr.data : []
      const smb = document.getElementById('smb-lineas')
      if (smb) smb.querySelector('.o-count').textContent = ls.length

      const lb = document.getElementById('ac-lineas')
      const tb = document.getElementById('ac-totals')
      if (lb) {
        lb.innerHTML = ls.length
          ? ls.map(l => {
              const deb = parseFloat(l.debit) || 0
              const cre = parseFloat(l.credit) || 0
              const bal = deb - cre
              return `<tr>
                <td><code style="font-size:12px;background:var(--bg-alt);padding:2px 6px;border-radius:4px">${l.account_code || '—'} ${l.account_name ? '· ' + l.account_name : ''}</code></td>
                <td style="color:var(--text-500)">${l.name || '—'}</td>
                <td>${l.partner_name || '—'}</td>
                <td style="text-align:right;color:#059669;font-weight:600">${deb ? fmtMxn(deb) : '—'}</td>
                <td style="text-align:right;color:#DC2626;font-weight:600">${cre ? fmtMxn(cre) : '—'}</td>
                <td style="text-align:right">${fmtMxn(bal)}</td>
              </tr>`
            }).join('')
          : `<tr><td colspan="6" style="text-align:center;padding:24px;color:var(--text-400)">Sin líneas de asiento</td></tr>`
        if (tb && ls.length) {
          const totalDeb = ls.reduce((s, l) => s + (parseFloat(l.debit)  || 0), 0)
          const totalCre = ls.reduce((s, l) => s + (parseFloat(l.credit) || 0), 0)
          tb.innerHTML = `
            <tr><td>Total débito:</td><td style="text-align:right;font-weight:600;color:#059669">${fmtMxn(totalDeb)}</td></tr>
            <tr><td>Total crédito:</td><td style="text-align:right;font-weight:600;color:#DC2626">${fmtMxn(totalCre)}</td></tr>
            <tr class="total-row"><td>Diferencia:</td><td style="text-align:right">${fmtMxn(totalDeb-totalCre)}</td></tr>`
        }
      }
    } catch (_) { /* líneas opcionales */ }

    // Acciones
    window._acConfirm = async (fid) => {
      if (!confirm('¿Confirmar y publicar este asiento?')) return
      try {
        await api.accountConfirmar(fid)
        toast('OK', 'Asiento confirmado y publicado', 'success')
        await _viewForm(fid)
      } catch(e) { toast('Error', e.message, 'error') }
    }
    window._acReset = async (fid) => {
      if (!confirm('¿Restablecer a borrador?')) return
      try {
        await api.accountBorrador(fid)
        toast('Info', 'Asiento restablecido a borrador', 'info')
        await _viewForm(fid)
      } catch(e) { toast('Error', e.message, 'error') }
    }
    window._acCancel = async (fid) => {
      if (!confirm('¿Cancelar este asiento? Esta acción no se puede deshacer fácilmente.')) return
      try {
        await api.accountCancelar(fid)
        toast('Cancelado', 'Asiento cancelado', 'info')
        window._go('account')
      } catch(e) { toast('Error', e.message, 'error') }
    }

  } catch (e) {
    setPage(`<div style="padding:40px;text-align:center">
      <p style="color:#DC2626">⚠️ ${e.message}</p>
      <button class="o-btn-new" onclick="window._go('account')">Volver a Contabilidad</button>
    </div>`)
  }
}

// ─── Modal de creación ────────────────────────────────────────────────────────
function _openCreateModal() {
  // Eliminar modal previo si existe
  document.getElementById('ac-modal')?.remove()
  const modal = document.createElement('div')
  modal.id = 'ac-modal'
  modal.innerHTML = `
    <div class="o-modal-backdrop" onclick="document.getElementById('ac-modal').remove()"></div>
    <div class="o-modal" style="max-width:500px">
      <div class="o-modal-header">
        <h3 class="o-modal-title">📒 Nuevo Asiento</h3>
        <button class="o-modal-close" onclick="document.getElementById('ac-modal').remove()">✕</button>
      </div>
      <div class="o-modal-body">
        <div class="o-field-row" style="margin-bottom:12px">
          <div class="o-field-label">Tipo de asiento</div>
          <select id="ac-new-type" class="o-input" style="width:100%">
            <option value="out_invoice">🧾 Factura de cliente</option>
            <option value="in_invoice">📥 Factura de proveedor</option>
            <option value="out_refund">↩️ Nota de crédito cliente</option>
            <option value="in_refund">↩️ Nota de crédito proveedor</option>
            <option value="entry">📒 Entrada de diario</option>
          </select>
        </div>
        <div class="o-field-row" style="margin-bottom:12px">
          <div class="o-field-label">Referencia interna</div>
          <input id="ac-new-ref" type="text" class="o-input" style="width:100%" placeholder="Ej. PO-2026-001, Ajuste contable…">
        </div>
        <div style="font-size:12px;color:var(--text-400);margin-top:8px">
          El número de secuencia se asignará automáticamente al confirmar el asiento.
        </div>
      </div>
      <div class="o-modal-footer">
        <button class="btn btn-secondary btn-sm" onclick="document.getElementById('ac-modal').remove()">Cancelar</button>
        <button class="btn btn-primary btn-sm" id="ac-new-save">Crear Asiento</button>
      </div>
    </div>`
  document.body.appendChild(modal)

  document.getElementById('ac-new-save').addEventListener('click', async () => {
    const btn = document.getElementById('ac-new-save')
    btn.disabled = true; btn.textContent = 'Creando…'
    try {
      const body = {
        move_type: document.getElementById('ac-new-type').value,
        ref_: document.getElementById('ac-new-ref').value || null,
      }
      const res = await api.accountCrear(body)
      const newId = res?.data?.id || res?.id
      document.getElementById('ac-modal').remove()
      toast('Creado', 'Asiento creado en borrador', 'success')
      if (newId) {
        await _viewForm(newId)
      } else {
        await _load()
      }
    } catch(e) {
      toast('Error', e.message, 'error')
      btn.disabled = false; btn.textContent = 'Crear Asiento'
    }
  })
}

function _deb(fn, ms) { let t; return (...a) => { clearTimeout(t); t = setTimeout(() => fn(...a), ms) } }
