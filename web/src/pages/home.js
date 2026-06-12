import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { api } from '../api.js'


export async function renderHome() {
  ensureLayout()
  setBreadcrumb([{ label: 'Inicio' }])

  // Obtener todas las apps desde la BD
  let serverApps = []
  try {
    const res = await api.get('/apps')
    serverApps = Array.isArray(res?.data) ? res.data : (Array.isArray(res) ? res : [])
  } catch(e) {
    console.error('Error cargando apps', e)
  }

  // Filtrar las apps instaladas
  // El App Store se llama "apps" y siempre debe estar visible (o estar preinstalado en la DB)
  let installedApps = serverApps.filter(a => a.estado === 'installed' || a.id === 'apps')

  // Si por alguna razón la BD está vacía, mostrar solo App Store
  if (installedApps.length === 0) {
    installedApps = [{ id: 'apps', nombre: 'Aplicaciones', descripcion: 'Catálogo de Módulos', icono: '🛍️', gradiente: '#1E293B,#0F172A', estado: 'installed' }]
  }

  setPage(`
    <div class="nx-home">
      <div class="nx-home-header">
        <h1 class="nx-home-title">Aplicaciones</h1>
        <div class="nx-home-search">
          <input type="search" placeholder="Buscar módulo…" id="home-search" oninput="window._filterApps(this.value)">
        </div>
      </div>
      <div class="nx-app-grid" id="home-app-grid">
        ${installedApps.map((a, i) => `
          <div class="nx-app-card" data-id="${a.id}" onclick="window._go('${a.id}')" style="animation-delay:${i * 50}ms">
            <div class="nx-app-icon" style="background:linear-gradient(135deg,${a.gradiente || '#475569,#1E293B'})">${a.icono || '📦'}</div>
            ${a.kpi_url ? `<div class="nx-app-badge" id="app-badge-${a.id}">…</div>` : ''}
            <div class="nx-app-name">${a.nombre}</div>
            <div class="nx-app-desc">${a.descripcion || ''}</div>
          </div>
        `).join('')}
      </div>
    </div>
  `)

  // Cargar KPIs en paralelo
  await Promise.allSettled(
    installedApps.filter(a => a.kpi_url).map(async (app) => {
      try {
        const res = await api.get(app.kpi_url)
        const d = res?.data ?? res
        const val = app.kpi_field && d ? (d[app.kpi_field] ?? '—') : (Array.isArray(d) ? d.length : '—')
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
