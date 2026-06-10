/**
 * Home — Grilla de aplicaciones del sistema
 * Réplica del App Switcher Odoo Enterprise
 */
import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { api } from '../api.js'

const APPS = [
  {
    id: 'ventas',
    icon: '📊',
    gradient: 'linear-gradient(135deg, #4F46E5, #7C3AED)',
    nombre: 'Ventas',
    desc: 'Órdenes y Cotizaciones',
    kpiEndpoint: '/ventas/kpis',
    kpiField: 'total_ordenes',
  },
  {
    id: 'facturas',
    icon: '🧾',
    gradient: 'linear-gradient(135deg, #059669, #0EA5E9)',
    nombre: 'Facturación',
    desc: 'Facturas y Cobros',
    kpiEndpoint: '/facturas/kpis',
    kpiField: 'total_facturas',
  },
  {
    id: 'partners',
    icon: '👥',
    gradient: 'linear-gradient(135deg, #7C3AED, #EC4899)',
    nombre: 'Clientes',
    desc: 'Directorio y Contactos',
    kpiEndpoint: null,
    kpiField: null,
  },
  {
    id: 'stock',
    icon: '📦',
    gradient: 'linear-gradient(135deg, #D97706, #EA580C)',
    nombre: 'Inventario',
    desc: 'Almacén y Movimientos',
    kpiEndpoint: '/stock/kpis',
    kpiField: 'total_productos',
  },
  {
    id: 'compras',
    icon: '🛒',
    gradient: 'linear-gradient(135deg, #2563EB, #4F46E5)',
    nombre: 'Compras',
    desc: 'Órdenes de Compra',
    kpiEndpoint: '/compras/kpis',
    kpiField: 'total_ordenes',
  },
  {
    id: 'productos',
    icon: '🏷️',
    gradient: 'linear-gradient(135deg, #0D9488, #059669)',
    nombre: 'Productos',
    desc: 'Catálogo y Precios',
    kpiEndpoint: null,
    kpiField: null,
  },
  {
    id: 'cfdi',
    icon: '🔏',
    gradient: 'linear-gradient(135deg, #E11D48, #DC2626)',
    nombre: 'CFDI 4.0',
    desc: 'Timbrado Electrónico',
    kpiEndpoint: '/cfdi/kpis',
    kpiField: 'timbrados_hoy',
  },
  {
    id: 'nomina',
    icon: '👔',
    gradient: 'linear-gradient(135deg, #0EA5E9, #2563EB)',
    nombre: 'Nómina IMSS',
    desc: 'Empleados y Recibos',
    kpiEndpoint: '/nomina/kpis',
    kpiField: 'total_empleados',
  },
  {
    id: 'reportes',
    icon: '📈',
    gradient: 'linear-gradient(135deg, #475569, #1E293B)',
    nombre: 'Reportes',
    desc: 'Análisis y Estadísticas',
    kpiEndpoint: null,
    kpiField: null,
  },
]

/** Skeleton grid de 9 cards */
function skeletonGrid() {
  return `
  <div class="nx-home">
    <div class="nx-home-header">
      <h1 class="nx-home-title">Aplicaciones</h1>
      <div class="nx-home-search">
        <input type="search" placeholder="Buscar módulo..." id="home-search" autocomplete="off">
      </div>
    </div>
    <div class="nx-app-grid" id="home-app-grid">
      ${APPS.map(() => `
        <div class="nx-app-card" style="pointer-events:none">
          <div class="nx-app-icon skeleton" style="background:none"></div>
          <div class="nx-app-name skeleton" style="height:14px;width:70%;margin:0 auto 6px"></div>
          <div class="nx-app-desc skeleton" style="height:11px;width:55%;margin:0 auto"></div>
        </div>
      `).join('')}
    </div>
  </div>`
}

export async function renderHome() {
  ensureLayout()
  setBreadcrumb([{ label: 'Inicio' }])
  setPage(skeletonGrid())

  // Cargar KPIs de cada módulo EN PARALELO
  const kpiPromises = APPS.map(app =>
    app.kpiEndpoint
      ? api.get(app.kpiEndpoint).catch(() => null)
      : Promise.resolve(null)
  )
  const kpiResults = await Promise.allSettled(kpiPromises)

  // Renderizar grid real con stagger
  const grid = document.getElementById('home-app-grid')
  if (!grid) return

  grid.innerHTML = APPS.map((app, i) => {
    const res = kpiResults[i]
    let count = null
    if (res.status === 'fulfilled' && res.value && app.kpiField) {
      const data = res.value?.data ?? res.value
      count = data?.[app.kpiField] ?? null
    }

    return `
      <div class="nx-app-card"
           data-app-id="${app.id}"
           data-name="${app.nombre.toLowerCase()}"
           style="animation-delay:${i * 50}ms"
           onclick="window._go('${app.id}')">
        <div class="nx-app-icon" style="background:${app.gradient}">
          <span class="nx-app-emoji">${app.icon}</span>
        </div>
        ${count !== null ? `<div class="nx-app-badge">${Number(count).toLocaleString('es-MX')}</div>` : ''}
        <div class="nx-app-name">${app.nombre}</div>
        <div class="nx-app-desc">${app.desc}</div>
      </div>
    `
  }).join('')

  // Búsqueda
  const searchInput = document.getElementById('home-search')
  if (searchInput) {
    searchInput.addEventListener('input', (e) => {
      const q = e.target.value.toLowerCase().trim()
      document.querySelectorAll('#home-app-grid .nx-app-card').forEach(card => {
        const name = card.dataset.name || ''
        const text = card.textContent.toLowerCase()
        card.classList.toggle('hidden', q !== '' && !name.includes(q) && !text.includes(q))
      })
    })
    // Auto-focus en la búsqueda al cargar
    searchInput.focus()
  }
}
