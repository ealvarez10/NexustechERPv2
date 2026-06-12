import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { fmtMxn, fmtDate, skeletonTable, toast, stateBadge } from '../ui.js'

let _searchQuery = ''
let cfg = {}

export async function renderCRM() {
  ensureLayout()
  setBreadcrumb([{ label: 'CRM / Pipeline' }])

  cfg = {
    leads: false, etapas_compartidas: false, probabilidad_ia: false, tiempo_cierre: false,
    pronostico: false, objetivos: false, actividades: true, reuniones: true, llamadas: true,
    email_alias: true, seguimiento_email: false, plantillas_email: true, mineria_leads: false, deduplicar: true,
    ...JSON.parse(localStorage.getItem('nexus_config_crm') || '{}')
  }

  _renderCP()
  _loadAndRender()
}

function _renderCP() {
  setPage(`
  <div class="o-cp" id="crm-cp">
    <div class="o-cp-left">
      <button class="o-btn-primary" onclick="alert('Nueva Oportunidad')">Nueva</button>
      ${cfg.mineria_leads ? `<button class="o-btn-secondary" onclick="alert('Minería de Leads')">Generar Leads</button>` : ''}
    </div>
    <div class="o-cp-center">
      <div class="o-search-bar">
        <svg class="o-search-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/></svg>
        <input id="o-search-crm" class="o-search-input" type="text" placeholder="Buscar oportunidad…" value="${_searchQuery}">
      </div>
    </div>
    <div class="o-cp-right">
      <button class="o-btn-secondary" style="margin-right:8px;font-size:16px;padding:4px 8px" onclick="window._go('config_crm')" title="Ajustes">⚙️</button>
      <div class="o-view-switcher">
        <button class="o-view-btn o-active" title="Kanban">⬜</button>
        <button class="o-view-btn" title="Lista">☰</button>
      </div>
    </div>
  </div>
  <div id="crm-content" class="o-view-content" style="background:#f9f9fb;padding:16px;min-height:calc(100vh - 100px);overflow-x:auto">
    ${skeletonTable(8, 4)}
  </div>`)

  setTimeout(() => {
    document.getElementById('o-search-crm')?.addEventListener('input', (e) => {
      _searchQuery = e.target.value.toLowerCase()
      _filterTableLocal()
    })
  }, 100)
}

function _filterTableLocal() {
  document.querySelectorAll('.o-kanban-card').forEach(r => {
    r.style.display = r.textContent.toLowerCase().includes(_searchQuery) ? '' : 'none'
  })
}

const MOCK_DATA = [
  { id: 1, name: 'Venta de Servidores', partner: 'Acme Corp', stage: 'new', amount: 45000, prob: 10 },
  { id: 2, name: 'Licencias ERP', partner: 'Tech Solutions', stage: 'qualified', amount: 12000, prob: 50 },
  { id: 3, name: 'Consultoría', partner: 'Global IT', stage: 'proposition', amount: 30000, prob: 80 },
]

function _loadAndRender() {
  const el = document.getElementById('crm-content')
  if (!el) return

  const cols = [
    { key: 'new', label: 'Nuevo' },
    { key: 'qualified', label: 'Calificado' },
    { key: 'proposition', label: 'Propuesta' },
    { key: 'won', label: 'Ganado' }
  ]

  if (cfg.leads) {
    cols.unshift({ key: 'lead', label: 'Leads (Sin asignar)' })
  }

  el.innerHTML = `
  <div class="o-kanban-columns" style="display:flex;gap:16px;align-items:flex-start;height:100%">
    ${cols.map(col => {
      const group = MOCK_DATA.filter(c => c.stage === col.key)
      const total = group.reduce((a, c) => a + c.amount, 0)
      return `
      <div class="o-kanban-col" style="flex:0 0 280px;background:#e5e7eb;border-radius:6px;padding:8px;display:flex;flex-direction:column;max-height:100%">
        <div style="font-weight:700;font-size:14px;color:#374151;margin-bottom:8px;display:flex;justify-content:space-between">
          <span>${col.label}</span>
          ${cfg.pronostico ? `<span style="color:#6B7280">${fmtMxn(total)}</span>` : ''}
        </div>
        <div class="o-kanban-col-cards" style="display:flex;flex-direction:column;gap:8px;overflow-y:auto">
          ${group.map(c => `
          <div class="o-kanban-card" style="background:#fff;border-radius:4px;padding:12px;box-shadow:0 1px 2px rgba(0,0,0,0.1);cursor:pointer">
            <div style="font-weight:600;font-size:14px;color:#111827">${c.name}</div>
            <div style="font-size:12px;color:#6B7280;margin:4px 0">${c.partner}</div>
            <div style="display:flex;justify-content:space-between;margin-top:8px;align-items:center">
              <strong style="color:#059669">${fmtMxn(c.amount)}</strong>
              ${cfg.probabilidad_ia ? `<span style="font-size:11px;background:#FEF3C7;color:#D97706;padding:2px 6px;border-radius:10px">IA: ${c.prob}%</span>` : ''}
            </div>
            <div style="margin-top:8px;display:flex;gap:4px">
               ${cfg.actividades ? `<span style="font-size:12px" title="Actividades">📅</span>` : ''}
               ${cfg.llamadas ? `<span style="font-size:12px" title="Llamadas">📞</span>` : ''}
               ${cfg.reuniones ? `<span style="font-size:12px" title="Reuniones">🤝</span>` : ''}
            </div>
          </div>`).join('')}
        </div>
      </div>`
    }).join('')}
  </div>`
}
