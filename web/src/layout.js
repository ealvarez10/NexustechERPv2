import { auth } from './auth.js'
import { go } from './router.js'
import { toast } from './ui.js'

const NAV = [
  { id:'home',      icon:'⊞',  label:'Inicio',          section:'Principal' },
  { id:'dashboard', icon:'📊', label:'Dashboard',      section:'Principal' },
  { id:'ventas',    icon:'💰', label:'Ventas',          section:'Principal' },
  { id:'cotizaciones', icon:'📝', label:'Cotizaciones', section:'Principal' },
  { id:'facturas',  icon:'🧾', label:'Facturación',     section:'Principal' },
  { id:'productos', icon:'📦', label:'Productos',       section:'Principal' },
  { id:'partners',  icon:'👥', label:'Clientes',        section:'Principal' },
  { id:'stock',     icon:'🏭', label:'Inventario',      section:'Principal' },
  { id:'cfdi',      icon:'🔏', label:'CFDI 4.0',        section:'Fiscal', badge:'NUEVO' },
  { id:'nomina',    icon:'👔', label:'Nómina IMSS',     section:'Fiscal' },
  { id:'compras',   icon:'🛒', label:'Compras',         section:'Operaciones' },
  { id:'search',    icon:'🔍', label:'NexusSearch',     section:'Sistema' },
  { id:'reportes',  icon:'📈', label:'Reportes',        section:'Sistema' },
]

export function ensureLayout() {
  if (document.getElementById('__shell')) return
  const user = auth.getUser()
  const initials = (user.nombre || user.name || 'AD').substring(0,2).toUpperCase()
  const sections = [...new Set(NAV.map(n => n.section))]

  document.getElementById('app').innerHTML = `
  <div class="app-shell" id="__shell">
    <!-- SIDEBAR -->
    <nav class="sidebar" id="__sidebar">
      <div class="sidebar-brand" style="cursor:pointer" onclick="window._go('home')" title="Ir al inicio">
        <div class="brand-logo">N</div>
        <div class="sidebar-brand-text">
          <div class="brand-name">NexusTech</div>
          <div class="brand-version">ERP v2.0</div>
        </div>
      </div>

      <div class="sidebar-nav">
        ${sections.map(sec => `
        <div class="nav-section">
          <div class="nav-section-title">${sec}</div>
          ${NAV.filter(n => n.section === sec).map(n => `
          <a class="nav-link" id="nl-${n.id}" href="#${n.id}" onclick="event.preventDefault();window._go('${n.id}')">
            <span style="font-size:16px">${n.icon}</span>
            <span>${n.label}</span>
            ${n.badge ? `<span class="nav-badge">${n.badge}</span>` : ''}
          </a>`).join('')}
        </div>`).join('')}
      </div>

      <div class="sidebar-user">
        <div class="user-pill">
          <div class="avatar">${initials}</div>
          <div class="user-info">
            <div class="user-name">${user.nombre || user.name || 'Administrador'}</div>
            <div class="user-role">${user.email || 'admin@nexustech.mx'}</div>
          </div>
        </div>
        <button class="btn btn-secondary btn-sm" style="width:100%;margin-top:8px;justify-content:center" onclick="window._logout()">
          🚪 Cerrar sesión
        </button>
      </div>
      <!-- Sidebar collapse toggle -->
      <button class="sidebar-toggle-btn" id="sidebar-toggle" title="Colapsar panel" onclick="window._toggleSidebar()">
        ◀
      </button>
    </nav>

    <!-- MAIN -->
    <div class="main-area">
      <!-- TOPBAR -->
      <header class="topbar">
        <nav class="breadcrumb" id="__breadcrumb">
          <span class="breadcrumb-item">Inicio</span>
        </nav>
        <div class="topbar-spacer"></div>
        <div class="topbar-search">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/>
          </svg>
          <input type="text" placeholder="Búsqueda global..." id="global-search">
          <span class="topbar-kbd">⌘K</span>
        </div>
        <button class="topbar-action" title="Notificaciones">
          🔔
          <span class="notif-dot"></span>
        </button>
        <button class="topbar-action" title="Configuración">⚙️</button>
        <div class="avatar-sm">${initials}</div>
      </header>

      <!-- CONTENT -->
      <main class="page" id="__page"></main>
    </div>
  </div>`

  // Global helpers
  window._go = (hash) => { go(hash) }
  window._logout = () => {
    auth.clear()
    const shell = document.getElementById('__shell')
    if (shell) shell.remove()
    go('login')
    toast('Sesión cerrada', 'Hasta pronto', 'info')
  }

  // Sidebar collapse
  window._toggleSidebar = () => {
    const sb = document.getElementById('__sidebar')
    const btn = document.getElementById('sidebar-toggle')
    if (!sb) return
    const collapsed = sb.classList.toggle('collapsed')
    localStorage.setItem('nx_sidebar_collapsed', collapsed ? '1' : '0')
    if (btn) btn.textContent = collapsed ? '▶' : '◀'
  }

  // Restore sidebar state
  if (localStorage.getItem('nx_sidebar_collapsed') === '1') {
    const sb = document.getElementById('__sidebar')
    const btn = document.getElementById('sidebar-toggle')
    if (sb) sb.classList.add('collapsed')
    if (btn) btn.textContent = '▶'
  }

  // Update active nav on hash change
  window.addEventListener('hashchange', updateNav)
  updateNav()
}

export function setPage(html) {
  const pg = document.getElementById('__page')
  if (pg) { pg.innerHTML = html; pg.scrollTop = 0 }
}

export function setBreadcrumb(items) {
  const bc = document.getElementById('__breadcrumb')
  if (!bc) return
  bc.innerHTML = items.map((it, i) => `
    <span class="breadcrumb-item"${i < items.length-1 && it.href ? ` onclick="window._go('${it.href}')"` : ''}>
      ${it.label}
      ${i < items.length-1 ? '<span class="breadcrumb-sep">/</span>' : ''}
    </span>`).join('')
}

function updateNav() {
  const hash = window.location.hash.replace('#','') || 'home'
  document.querySelectorAll('.nav-link').forEach(el => {
    el.classList.toggle('active', el.id === `nl-${hash}`)
  })
}
