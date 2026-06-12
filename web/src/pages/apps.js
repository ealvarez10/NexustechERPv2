import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { api } from '../api.js'
import { toast } from '../ui.js'

export async function renderApps() {
  ensureLayout()
  setBreadcrumb([{ label: 'Catálogo de Aplicaciones' }])

  let apps = []
  try {
    const res = await api.get('/apps')
    apps = Array.isArray(res?.data) ? res.data : (Array.isArray(res) ? res : [])
  } catch (e) {
    console.error('Error cargando apps', e)
    toast('Error', 'No se pudo cargar el catálogo', 'error')
    return
  }

  // Filtrar "apps" (la app store misma no se instala/desinstala desde aquí)
  apps = apps.filter(a => a.id !== 'apps')

  const renderGrid = () => {
    return apps.map((a, i) => {
      const installed = a.estado === 'installed'
      return `
        <div class="nx-app-card" data-id="${a.id}" style="animation-delay:${i * 50}ms; cursor: default; height: auto; padding-bottom: 20px;">
          <div class="nx-app-icon" style="background:linear-gradient(135deg,${a.gradiente || '#475569,#1E293B'})">${a.icono || '📦'}</div>
          <div class="nx-app-name">${a.nombre}</div>
          <div class="nx-app-desc" style="margin-bottom: 16px;">${a.descripcion || ''}</div>
          
          <div style="margin-top: auto; width: 100%; display: flex; justify-content: center;">
            ${installed 
              ? `<button class="btn btn-secondary btn-sm" onclick="window._uninstallApp('${a.id}')">Desinstalar</button>` 
              : `<button class="btn btn-primary btn-sm" onclick="window._installApp('${a.id}')">Instalar</button>`
            }
          </div>
        </div>
      `
    }).join('')
  }

  const updateUI = () => {
    const grid = document.getElementById('apps-grid')
    if (grid) grid.innerHTML = renderGrid()
  }

  setPage(`
    <div class="nx-home">
      <div class="nx-home-header">
        <h1 class="nx-home-title">Catálogo de Aplicaciones</h1>
        <div class="nx-home-search">
          <input type="search" placeholder="Buscar módulo…" id="apps-search" oninput="window._filterAppsStore(this.value)">
        </div>
      </div>
      <div class="nx-app-grid" id="apps-grid">
        ${renderGrid()}
      </div>
    </div>
  `)

  window._filterAppsStore = (q) => {
    const query = q.toLowerCase().trim()
    document.querySelectorAll('#apps-grid .nx-app-card').forEach(card => {
      const name = card.querySelector('.nx-app-name')?.textContent.toLowerCase() || ''
      const desc = card.querySelector('.nx-app-desc')?.textContent.toLowerCase() || ''
      card.classList.toggle('hidden', !!query && !name.includes(query) && !desc.includes(query))
    })
  }

  window._installApp = async (id) => {
    try {
      await api.post(`/apps/${id}/install`)
      toast('Instalado', 'La aplicación se ha instalado correctamente', 'success')
      const app = apps.find(a => a.id === id)
      if (app) app.estado = 'installed'
      updateUI()
    } catch (e) {
      toast('Error', 'Fallo al instalar la aplicación', 'error')
    }
  }

  window._uninstallApp = async (id) => {
    if (!confirm('¿Estás seguro de desinstalar esta aplicación? Sus vistas y funciones se ocultarán.')) return
    try {
      await api.post(`/apps/${id}/uninstall`)
      toast('Desinstalado', 'La aplicación ha sido removida del dashboard', 'info')
      const app = apps.find(a => a.id === id)
      if (app) app.estado = 'uninstalled'
      updateUI()
    } catch (e) {
      toast('Error', 'Fallo al desinstalar la aplicación', 'error')
    }
  }
}
