import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { fmtMxn, fmtDate, skeletonTable, toast, stateBadge } from '../ui.js'

let _searchQuery = ''
let cfg = {}

export async function renderContabilidad() {
  ensureLayout()
  setBreadcrumb([{ label: 'Contabilidad' }])

  cfg = {
    multimoneda: false, contabilidad_analitica: false, presupuestos: false,
    activos_fijos: false, ingresos_diferidos: false, alertas_facturacion: true, auditoria: false,
    ...JSON.parse(localStorage.getItem('nexus_config_contabilidad') || '{}')
  }

  _renderCP()
  _loadAndRender()
}

function _renderCP() {
  setPage(`
  <div class="nx-module-page" style="background:var(--bg-app);min-height:100vh">
    <div class="o-cp" id="conta-cp">
      <div class="o-cp-left">
        <button class="o-btn-primary" onclick="alert('Nuevo Asiento')">Nuevo Asiento</button>
      </div>
      <div class="o-cp-center">
        <div class="o-search-bar">
          <svg class="o-search-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/></svg>
          <input id="o-search-conta" class="o-search-input" type="text" placeholder="Buscar asiento…" value="${_searchQuery}">
        </div>
      </div>
      <div class="o-cp-right">
        <button class="o-btn-secondary" style="margin-right:8px;font-size:16px;padding:4px 8px" onclick="window._go('config_contabilidad')" title="Ajustes">⚙️</button>
        <div class="o-view-switcher">
          <button class="o-view-btn o-active" title="Lista">☰</button>
        </div>
      </div>
    </div>
    <div id="conta-content" class="o-view-content" style="background:#fff;padding:16px;min-height:calc(100vh - 100px);overflow-x:auto">
      ${skeletonTable(8, 5)}
    </div>
  </div>`)

  setTimeout(() => {
    document.getElementById('o-search-conta')?.addEventListener('input', (e) => {
      _searchQuery = e.target.value.toLowerCase()
      _filterTableLocal()
    })
  }, 100)
}

function _filterTableLocal() {
  document.querySelectorAll('.o-list-row').forEach(r => {
    r.style.display = r.textContent.toLowerCase().includes(_searchQuery) ? '' : 'none'
  })
}

const MOCK_DATA = [
  { id: 1, date: '2023-10-01', ref: 'F-2023-0001', journal: 'Ventas', partner: 'Acme Corp', amount: 45000, state: 'posted' },
  { id: 2, date: '2023-10-02', ref: 'BILL-2023-001', journal: 'Compras', partner: 'Tech Solutions', amount: 12000, state: 'draft' },
]

function _loadAndRender() {
  const el = document.getElementById('conta-content')
  if (!el) return

  el.innerHTML = `
  <div class="o-list-view">
    <table>
      <thead>
        <tr>
          <th class="th-check"><input type="checkbox" class="o-list-checkbox"></th>
          <th>Fecha</th>
          <th>Referencia</th>
          <th>Diario</th>
          <th>Empresa</th>
          ${cfg.contabilidad_analitica ? '<th>Cuenta Analítica</th>' : ''}
          ${cfg.multimoneda ? '<th>Moneda</th>' : ''}
          <th style="text-align:right">Total</th>
          <th>Estado</th>
          ${cfg.auditoria ? '<th>Hash Auditoría</th>' : ''}
        </tr>
      </thead>
      <tbody>
        ${MOCK_DATA.map(a => `
        <tr class="o-list-row">
          <td class="td-check"><input type="checkbox" class="o-list-checkbox"></td>
          <td class="o-td-muted">${a.date}</td>
          <td class="o-td-primary">${a.ref}</td>
          <td>${a.journal}</td>
          <td>${a.partner}</td>
          ${cfg.contabilidad_analitica ? '<td class="o-td-muted">Admin / Ventas</td>' : ''}
          ${cfg.multimoneda ? '<td class="o-td-muted">MXN</td>' : ''}
          <td style="text-align:right;font-weight:600">${fmtMxn(a.amount)}</td>
          <td>${stateBadge(a.state === 'posted' ? 'Publicado' : 'Borrador', a.state === 'posted' ? 'success' : 'default')}</td>
          ${cfg.auditoria ? '<td class="o-td-mono" style="font-size:10px;color:#9ca3af">0xab4...</td>' : ''}
        </tr>`).join('')}
      </tbody>
    </table>
  </div>`
}
