import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { api } from '../api.js'

const APPS = [
  { id: 'ventas',       icon: '📊', grad: '#4F46E5,#7C3AED', nombre: 'Ventas',          desc: 'Órdenes y Cotizaciones',    kpi: '/ventas/kpis',        field: 'total_ordenes' },
  { id: 'facturas',     icon: '🧾', grad: '#059669,#0EA5E9', nombre: 'Facturación',     desc: 'Facturas y Pagos',          kpi: '/facturas/kpis',      field: 'total_facturas' },
  { id: 'partners',     icon: '👥', grad: '#7C3AED,#EC4899', nombre: 'Clientes',        desc: 'Contactos y Partners',      kpi: '/partners',           field: null },
  { id: 'stock',        icon: '🏭', grad: '#D97706,#EA580C', nombre: 'Inventario',      desc: 'Control de Stock',          kpi: '/stock/kpis',         field: 'total_productos_con_stock' },
  { id: 'compras',      icon: '🛒', grad: '#2563EB,#4F46E5', nombre: 'Compras',         desc: 'Órdenes de Compra',         kpi: '/compras/kpis',       field: 'total_ordenes' },
  { id: 'productos',    icon: '📦', grad: '#0D9488,#059669', nombre: 'Productos',       desc: 'Catálogo de Artículos',     kpi: '/productos',          field: null },
  { id: 'cfdi',         icon: '🔐', grad: '#E11D48,#DC2626', nombre: 'CFDI 4.0',        desc: 'Timbrado Fiscal Digital',   kpi: '/cfdi/historial',     field: null },
  { id: 'nomina',       icon: '👔', grad: '#0EA5E9,#2563EB', nombre: 'Nómina IMSS',    desc: 'Nóminas y Seguridad Social', kpi: '/nomina/kpis',        field: 'total_empleados' },
  { id: 'reportes',     icon: '📈', grad: '#475569,#1E293B', nombre: 'Reportes',        desc: 'Análisis y BI',             kpi: null,                  field: null },
  { id: 'cotizaciones', icon: '📝', grad: '#8B5CF6,#4F46E5', nombre: 'Cotizaciones',    desc: 'Borradores y Propuestas',   kpi: '/cotizaciones/kpis',  field: 'total_borradores' },
  { id: 'dashboard',    icon: '📊', grad: '#0F172A,#1E293B', nombre: 'Dashboard',       desc: 'Vista general del sistema', kpi: null,                  field: null },
]

export async function renderHome() {
  ensureLayout()
  setBreadcrumb([{ label: 'Inicio' }])
  setPage(`
    <div class="nx-home">
      <div class="nx-home-header">
        <h1 class="nx-home-title">Aplicaciones</h1>
        <div class="nx-home-search">
          <input type="search" placeholder="Buscar módulo…" id="home-search" oninput="window._filterApps(this.value)">
        </div>
      </div>
      <div class="nx-app-grid" id="home-app-grid">
        ${APPS.map((a, i) => `
          <div class="nx-app-card" data-id="${a.id}" onclick="window._go('${a.id}')" style="animation-delay:${i * 50}ms">
            <div class="nx-app-icon" style="background:linear-gradient(135deg,${a.grad})">${a.icon}</div>
            <div class="nx-app-badge" id="app-badge-${a.id}">…</div>
            <div class="nx-app-name">${a.nombre}</div>
            <div class="nx-app-desc">${a.desc}</div>
          </div>
        `).join('')}
      </div>
    </div>
  `)

  // Cargar KPIs en paralelo
  await Promise.allSettled(
    APPS.filter(a => a.kpi).map(async (app) => {
      try {
        const res = await api.get(app.kpi)
        const d = res?.data ?? res
        const val = app.field && d ? (d[app.field] ?? '—') : (Array.isArray(d) ? d.length : '—')
        const el = document.getElementById('app-badge-' + app.id)
        if (el) el.textContent = Number(val) > 999 ? (val / 1000).toFixed(1) + 'k' : val
      } catch {
        const el = document.getElementById('app-badge-' + app.id)
        if (el) el.textContent = '—'
      }
    })
  )

  // Filtro de apps
  window._filterApps = (q) => {
    const query = q.toLowerCase().trim()
    document.querySelectorAll('.nx-app-card').forEach(card => {
      const name = card.querySelector('.nx-app-name')?.textContent.toLowerCase() || ''
      const desc = card.querySelector('.nx-app-desc')?.textContent.toLowerCase() || ''
      card.classList.toggle('hidden', !!query && !name.includes(query) && !desc.includes(query))
    })
  }
}
