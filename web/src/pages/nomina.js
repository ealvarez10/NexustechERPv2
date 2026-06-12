import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { fmtMxn, fmtDate, fmtNum, paginationHtml, skeletonTable, toast } from '../ui.js'
import { api } from '../api.js'
import { editarEmpleado } from './forms/edit_forms.js'

/* ─── Estado ─── */
let _currentPage = 1
let _records     = []
let _searchQuery = ''

/* ═══════════════════════════════════════════════
   ENTRY POINT
   ═══════════════════════════════════════════════ */
export async function renderNomina() {
  ensureLayout()
  _currentPage = 1
  _searchQuery = ''
  setBreadcrumb([{ label: 'Nómina' }])
  _renderControlPanel()
  await _loadAndRender()
}

/* ═══════════════════════════════════════════════
   CONTROL PANEL
   ═══════════════════════════════════════════════ */
function _renderControlPanel() {
  setPage(`
  <div class="o-cp" id="nomina-cp">
    <div class="o-cp-left">
      <button class="o-btn-primary" onclick="window._nominaNuevoEmpleado()">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
        Nuevo
      </button>
    </div>
    <div class="o-cp-center">
      <div class="o-search-bar">
        <svg class="o-search-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/></svg>
        <input id="o-search-nomina" class="o-search-input" type="text" placeholder="Buscar empleado o departamento…" value="${_searchQuery}">
        <div class="o-search-filters">
          <button class="o-filter-btn" onclick="window._nominaFiltro('activos')">Activos</button>
          <button class="o-filter-btn" onclick="window._nominaFiltro('baja')">De baja</button>
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
  <div id="nomina-content" class="o-view-content">
    ${skeletonTable(10, 6)}
  </div>`)

  setTimeout(() => {
    document.getElementById('o-search-nomina')?.addEventListener('input', (e) => {
      _searchQuery = e.target.value.toLowerCase()
      _filterTableLocal()
    })
  }, 100)
}

function _filterTableLocal() {
  document.querySelectorAll('#nomina-content tbody tr').forEach(r => {
    r.style.display = r.textContent.toLowerCase().includes(_searchQuery) ? '' : 'none'
  })
}

/* ═══════════════════════════════════════════════
   LOAD & RENDER
   ═══════════════════════════════════════════════ */
async function _loadAndRender() {
  try {
    const res  = await api.nomina(_currentPage)
    _records   = res?.data || []
    const hasMore = _records.length >= 20

    const el = document.getElementById('nomina-content')
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
      <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" opacity=".3"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>
      <p>Sin empleados registrados</p>
    </div>`

  return `
  <div class="o-list-view">
    <table class="o-list-table">
      <thead>
        <tr>
          <th class="o-list-chk"><input type="checkbox" class="o-chk" onclick="window._chkAllNomina(this)"></th>
          <th class="o-col-sortable">Empleado</th>
          <th>Puesto</th>
          <th>Departamento</th>
          <th>N° IMSS</th>
          <th>Fecha Inicio</th>
          <th class="o-col-right">Salario Base</th>
        </tr>
      </thead>
      <tbody>
        ${records.map(e => {
          const activo   = e.active !== false
          const initials = (e.name || '?').split(' ').map(w => w[0]).slice(0, 2).join('')
          const hue      = (e.id * 47) % 360
          const puesto   = e.job_title || e.job_id_name || '—'
          const depto    = e.department_name || e.department_id_name || '—'
          const imss     = e.ssnid || e.imss || '—'
          const fechaIn  = fmtDate(e.date_start || e.fecha_inicio || null)
          const salario  = fmtMxn(parseFloat(e.wage || e.salario_base || 0))
          return `
          <tr class="o-list-row" onclick="window._verEmpleado(${e.id})">
            <td class="o-list-chk"><input type="checkbox" class="o-chk" onclick="event.stopPropagation()"></td>
            <td>
              <div class="o-partner-cell">
                <div class="o-avatar" style="background:linear-gradient(135deg,hsl(${hue},60%,55%),hsl(${(hue+50)%360},70%,45%))">${initials || '?'}</div>
                <div>
                  <div class="o-td-primary">${e.name || '—'}</div>
                  <div class="o-td-secondary"><span class="o-badge ${activo ? 'o-badge-success' : 'o-badge-gray'}">${activo ? 'Activo' : 'Baja'}</span></div>
                </div>
              </div>
            </td>
            <td class="o-td-muted">${puesto}</td>
            <td class="o-td-muted">${depto}</td>
            <td class="o-td-mono">${imss}</td>
            <td class="o-td-muted">${fechaIn}</td>
            <td class="o-td-amount" style="font-weight:700">${salario}</td>
          </tr>`
        }).join('')}
      </tbody>
    </table>
    <div class="o-list-footer">
      <span class="o-list-count">${records.length} empleado${records.length !== 1 ? 's' : ''}</span>
      ${paginationHtml(_currentPage, hasMore, (p) => { _currentPage = p; _loadAndRender() })}
    </div>
  </div>`
}

/* ═══════════════════════════════════════════════
   VISTA FORMULARIO
   ═══════════════════════════════════════════════ */
window._verEmpleado = async (id) => {
  setBreadcrumb([
    { label: 'Nómina', onclick: () => renderNomina() },
    { label: 'Cargando…', id: 'bc-emp-name' }
  ])
  setPage(`<div class="o-form-loading">${skeletonTable(4, 3)}</div>`)

  try {
    let emp = _records.find(x => x.id === id)
    try {
      const fresh = await api.empleado(id)
      if (fresh && (fresh.id || fresh.name)) emp = fresh
    } catch (_) {}
    if (!emp) { toast('Error', 'Empleado no encontrado', 'error'); return }

    const bcEl = document.getElementById('bc-emp-name')
    if (bcEl) bcEl.textContent = emp.name || 'Empleado'

    const activo   = emp.active !== false
    const initials = (emp.name || '?').split(' ').map(w => w[0]).slice(0, 2).join('')
    const hue      = (emp.id * 47) % 360

    const sbc    = fmtMxn(parseFloat(emp.sbc    || emp.salario_base_cotizacion || 0))
    const sdi    = fmtMxn(parseFloat(emp.sdi    || emp.salario_diario_integrado || 0))
    const salario = fmtMxn(parseFloat(emp.wage || emp.salario_base || 0))

    setPage(`
    <div class="o-form-topbar">
      <button class="o-back-btn" onclick="window._nominaBack()">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
        Nómina
      </button>
      <div class="o-form-actions">
        <button class="o-btn-secondary" onclick="window._editarEmpleadoForm(${emp.id})">💾 Actualizar</button>
        <button class="o-btn-primary" id="btn-calcular-nomina" data-sdi="${emp.sdi || emp.salario_diario_integrado || 0}" onclick="window._nominaCalcular(this.dataset.sdi)">Calcular Nómina</button>
      </div>
    </div>

    <!-- SMART BUTTONS -->
    <div class="o-smart-buttons">
      <button class="o-smart-btn" onclick="alert('Nóminas del empleado')">
        <span class="o-smart-count">0</span>
        <span class="o-smart-label">Nóminas</span>
      </button>
      <button class="o-smart-btn" onclick="alert('Cálculo IMSS — próximamente')">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="margin:0 auto;display:block"><path d="M9 7H6a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h9a2 2 0 0 0 2-2v-3"/><path d="M9 15h3l8.5-8.5a1.5 1.5 0 0 0-3-3L9 12v3"/><line x1="16" y1="5" x2="19" y2="8"/></svg>
        <span class="o-smart-label">Calc. IMSS</span>
      </button>
    </div>

    <!-- FORM SHEET -->
    <div class="o-form-sheet">
      <div class="o-sheet-header">
        <div class="o-avatar o-avatar-lg" style="background:linear-gradient(135deg,hsl(${hue},60%,55%),hsl(${(hue+50)%360},70%,45%))">${initials || '?'}</div>
        <div class="o-sheet-title-block">
          <h1 class="o-form-title">${emp.name || '—'}</h1>
          <div style="display:flex;gap:6px;margin-top:4px">
            <span class="o-badge ${activo ? 'o-badge-success' : 'o-badge-gray'}">${activo ? 'Activo' : 'Baja'}</span>
            ${emp.contract_type_name ? `<span class="o-badge o-badge-info">${emp.contract_type_name}</span>` : ''}
          </div>
        </div>
      </div>

      <div class="o-form-grid">
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">CURP</label><div class="o-field-value o-field-mono">${emp.curp || '—'}</div></div>
          <div class="o-field-group"><label class="o-field-label">N° Seguro Social</label><div class="o-field-value o-field-mono">${emp.ssnid || emp.imss || '—'}</div></div>
          <div class="o-field-group"><label class="o-field-label">RFC</label><div class="o-field-value o-field-mono">${emp.rfc || '—'}</div></div>
          <div class="o-field-group"><label class="o-field-label">Fecha de Inicio</label><div class="o-field-value">${fmtDate(emp.date_start || emp.fecha_inicio || null)}</div></div>
        </div>
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">Puesto</label><div class="o-field-value">${emp.job_title || emp.job_id_name || '—'}</div></div>
          <div class="o-field-group"><label class="o-field-label">Departamento</label><div class="o-field-value">${emp.department_name || emp.department_id_name || '—'}</div></div>
          <div class="o-field-group"><label class="o-field-label">Tipo de Contrato</label><div class="o-field-value">${emp.contract_type_name || emp.tipo_contrato || '—'}</div></div>
          <div class="o-field-group"><label class="o-field-label">Jornada</label><div class="o-field-value">${emp.resource_calendar_name || emp.jornada || '—'}</div></div>
        </div>
      </div>

      <!-- NOTEBOOK -->
      <div class="o-notebook">
        <div class="o-tabs" id="nom-tabs">
          <button class="o-tab active" onclick="window._nomTab('config', this)">Configuración Nómina</button>
          <button class="o-tab" onclick="window._nomTab('resumen', this)">Resumen Cálculos</button>
          <button class="o-tab" onclick="window._nomTab('historial', this)">Historial</button>
        </div>

        <div class="o-tab-pane" id="tab-config">
          <div class="o-form-grid">
            <div class="o-form-col">
              <div class="o-field-group"><label class="o-field-label">Salario Base</label><div class="o-field-value o-field-price">${salario}</div></div>
              <div class="o-field-group"><label class="o-field-label">SBC (Sal. Base Cotización)</label><div class="o-field-value">${sbc}</div></div>
              <div class="o-field-group"><label class="o-field-label">SDI (Sal. Diario Integrado)</label><div class="o-field-value">${sdi}</div></div>
              <div class="o-field-group"><label class="o-field-label">Periodicidad</label><div class="o-field-value">${emp.periodicidad || emp.payment_period || 'Mensual'}</div></div>
            </div>
            <div class="o-form-col">
              <div class="o-field-group"><label class="o-field-label">Banco</label><div class="o-field-value">${emp.bank_name || emp.banco || '—'}</div></div>
              <div class="o-field-group"><label class="o-field-label">CLABE</label><div class="o-field-value o-field-mono">${emp.acc_number || emp.clabe || '—'}</div></div>
            </div>
          </div>
        </div>

        <div class="o-tab-pane" id="tab-resumen" style="display:none">
          <div id="resumen-calc-result">
            <div style="padding:24px 0;text-align:center;color:var(--o-text-secondary)">
              Haz clic en <strong>Calcular Nómina</strong> para ver el desglose completo (ISR 2024 + IMSS 2024).
            </div>
          </div>
        </div>

        <div class="o-tab-pane" id="tab-historial" style="display:none">
          <div class="o-empty-state" style="padding:32px 0">
            <p style="color:var(--o-text-secondary)">Historial de nóminas procesadas — próximamente</p>
          </div>
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
            <div class="o-msg-meta"><strong>Sistema</strong> <span>${fmtDate(emp.date_start || new Date().toISOString())}</span></div>
            <div class="o-msg-text">Registro creado.</div>
          </div>
        </div>
      </div>
    </div>`)

    window._editarEmpleadoForm = (eid) => editarEmpleado({ id: eid, ...emp }, () => window._verEmpleado(eid))
    window._nomTab = (tab, btn) => {
      document.querySelectorAll('#nom-tabs .o-tab').forEach(b => b.classList.remove('active'))
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
window._nominaBack = () => renderNomina()
window._nominaNuevoEmpleado = () => alert('Nuevo empleado — próximamente')

window._nominaCalcular = async (sdiRaw) => {
  const sdi = parseFloat(sdiRaw) || 0
  // Si no hay SDI guardado, pedir al usuario
  const sdiInput = sdi > 0 ? sdi : parseFloat(prompt('Ingresa el SDI (Salario Diario Integrado):', '300') || '0')
  if (!sdiInput || sdiInput <= 0) return

  try {
    const token = localStorage.getItem('nexus_token')
    const res = await fetch('/api/v1/nomina/calcular', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json', Authorization: `Bearer ${token}` },
      body: JSON.stringify({ sdi: sdiInput, dias_periodo: 30, tipo: 'mensual' })
    })
    const json = await res.json()
    const d = json.data || json
    if (!d || !d.salario_bruto) { toast('Error', 'No se pudo calcular la nómina', 'error'); return }

    // Activar tab Resumen
    document.querySelectorAll('#nom-tabs .o-tab').forEach((b,i) => { b.classList.toggle('active', i===1) })
    document.querySelectorAll('.o-tab-pane').forEach((p,i) => p.style.display = i===1 ? '' : 'none')

    const el = document.getElementById('resumen-calc-result')
    if (el) el.innerHTML = `
      <table class="o-list-table">
        <thead><tr><th>Concepto</th><th class="o-col-right" style="text-align:right">Importe</th><th>Tipo</th></tr></thead>
        <tbody>
          <tr><td>Salario Bruto (${30} días)</td><td class="o-td-amount">${fmtMxn(d.salario_bruto)}</td><td><span class="o-badge o-badge-success">Percepción</span></td></tr>
          <tr style="color:var(--o-danger)"><td>ISR Retenido (SAT 2024)</td><td class="o-td-amount">–${fmtMxn(d.isr_retenido)}</td><td><span class="o-badge o-badge-danger">Deducción</span></td></tr>
          <tr style="color:var(--o-success)"><td>Subsidio al Empleo</td><td class="o-td-amount">+${fmtMxn(d.subsidio_empleo)}</td><td><span class="o-badge o-badge-success">A favor</span></td></tr>
          <tr style="color:var(--o-danger)"><td>IMSS Obrero (cuotas 2024)</td><td class="o-td-amount">–${fmtMxn(d.imss_obrero)}</td><td><span class="o-badge o-badge-danger">Deducción</span></td></tr>
          <tr style="border-top:2px solid var(--o-border);font-weight:700"><td>Total Deducciones</td><td class="o-td-amount">–${fmtMxn(d.total_deducciones)}</td><td></td></tr>
          <tr style="font-weight:800;font-size:1.05em;background:var(--o-bg-hover)"><td>💰 NETO A PAGAR</td><td class="o-td-amount" style="color:var(--o-success)">${fmtMxn(d.salario_neto)}</td><td></td></tr>
          <tr style="border-top:2px solid var(--o-border);color:var(--o-text-secondary);font-style:italic"><td colspan="3" style="padding-top:8px"><strong>Costo Total Patrón</strong></td></tr>
          <tr><td>IMSS Patronal</td><td class="o-td-amount">${fmtMxn(d.imss_patron)}</td><td><span class="o-badge o-badge-info">Costo empresa</span></td></tr>
          <tr><td>Cuota Fija IMSS</td><td class="o-td-amount">${fmtMxn(d.cuota_fija_patron)}</td><td><span class="o-badge o-badge-info">Costo empresa</span></td></tr>
          <tr style="font-weight:700"><td>Costo Total Empresa</td><td class="o-td-amount">${fmtMxn(d.costo_total_patron)}</td><td></td></tr>
        </tbody>
      </table>
      <div style="margin-top:12px;padding:8px 12px;background:var(--o-bg-hover);border-radius:6px;font-size:0.78em;color:var(--o-text-secondary)">
        Cálculo con tablas oficiales ISR 2024 (Art.152 LISR) + cuotas IMSS 2024 (DOF). SDI: ${fmtMxn(sdiInput)}/día.
      </div>
    `
    toast('Nómina calculada', 'Desglose ISR 2024 + IMSS actualizado', 'success')
  } catch(e) {
    toast('Error', e.message, 'error')
  }
}

window._nominaFiltro = (f) => {
  const filtered = f === 'activos'
    ? _records.filter(e => e.active !== false)
    : f === 'baja'
    ? _records.filter(e => e.active === false)
    : _records
  const el = document.getElementById('nomina-content')
  if (el) el.innerHTML = _renderList(filtered, false)
}

window._chkAllNomina = (master) => document.querySelectorAll('#nomina-content .o-chk').forEach(c => c.checked = master.checked)
