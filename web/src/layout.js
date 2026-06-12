import { auth } from './auth.js'
import { go } from './router.js'
import { toast } from './ui.js'

// Submenús específicos por app
const SUB_NAV = {
  ventas: [
    { id: 'ventas', label: 'Órdenes' },
    { id: 'precios', label: 'Precios Especiales' },
    { id: 'productos', label: 'Productos' },
    { id: 'reportes_ventas', label: 'Reportes' },
    { id: 'config_ventas', label: 'Configuración' }
  ],
  compras: [
    { id: 'compras', label: 'Órdenes' },
    { id: 'productos_compra', label: 'Productos' },
    { id: 'reportes_compras', label: 'Reportes' },
    { id: 'config_compras', label: 'Configuración' }
  ],
  nomina: [
    { id: 'nomina', label: 'Empleados' },
    { id: 'config_nomina', label: 'Configuración' }
  ],
  contabilidad: [
    { id: 'contabilidad', label: 'Asientos' },
    { id: 'config_contabilidad', label: 'Configuración' }
  ],
  crm: [
    { id: 'crm', label: 'Mi flujo' },
    { id: 'ventas', label: 'Ventas' },
    { id: 'reportes_crm', label: 'Reportes' },
    { id: 'config_crm', label: 'Configuración' }
  ],
  facturacion: [
    { id: 'facturas', label: 'Facturas' },
    { id: 'pagos', label: 'Pagos' },
    { id: 'reportes_facturacion', label: 'Reportes' },
    { id: 'config_facturacion', label: 'Configuración' }
  ],
  inventario: [
    { id: 'stock', label: 'Tablero' },
    { id: 'operaciones', label: 'Operaciones' },
    { id: 'productos', label: 'Productos' },
    { id: 'reportes_inventario', label: 'Reportes' },
    { id: 'config_inventario', label: 'Configuración' }
  ],
  contactos: [
    { id: 'partners', label: 'Contactos' },
    { id: 'etiquetas', label: 'Etiquetas' },
    { id: 'config_contactos', label: 'Configuración' }
  ],
  mercadily: [
    { id: 'mercadily', label: 'Configuración Tienda' }
  ]
}

export function ensureLayout() {
  if (document.getElementById('__shell')) return
  const user = auth.getUser()
  const initials = (user.nombre || user.name || 'AD').substring(0,2).toUpperCase()

  document.getElementById('app').innerHTML = `
  <div class="app-shell odoo-layout" id="__shell">
    <!-- ODOO TOPBAR -->
    <header class="odoo-topbar">
      <div class="odoo-topbar-left">
        <button class="app-drawer-btn" title="Aplicaciones" onclick="window._go('home')">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="white">
            <path d="M4 4h4v4H4V4zm6 0h4v4h-4V4zm6 0h4v4h-4V4zM4 10h4v4H4v-4zm6 0h4v4h-4v-4zm6 0h4v4h-4v-4zM4 16h4v4H4v-4zm6 0h4v4h-4v-4zm6 0h4v4h-4v-4z"/>
          </svg>
        </button>
        <div class="app-title" id="odoo-app-title">NexusTech ERP</div>
        <nav class="app-nav" id="odoo-app-nav"></nav>
      </div>
      
      <div class="odoo-topbar-right">
        <div class="topbar-search">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/>
          </svg>
          <input type="text" placeholder="Search..." id="global-search">
        </div>
        <button class="topbar-action" title="Notificaciones">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"></path><path d="M13.73 21a2 2 0 0 1-3.46 0"></path></svg>
          <span class="notif-dot"></span>
        </button>
        <div class="company-name" style="cursor:pointer" onclick="window._logout()" title="Cerrar sesión">
          NEXUSTECH
        </div>
        <div class="avatar-sm" style="cursor:pointer" onclick="window._logout()">${initials}</div>
      </div>
    </header>

    <!-- CONTENT -->
    <main class="page full-width" id="__page"></main>
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

  // Update active nav on hash change
  window.addEventListener('hashchange', updateNav)
  updateNav()
}

export function setPage(html) {
  const pg = document.getElementById('__page')
  if (pg) { pg.innerHTML = html; pg.scrollTop = 0 }
}

export function setBreadcrumb(items) {
  // Ya no usamos breadcrumb, la navegación se maneja en el submenú superior
}

function updateNav() {
  const hash = window.location.hash.replace('#','') || 'home'
  
  // Determinar App actual basado en el hash
  let currentApp = 'NexusTech ERP'
  let subMenu = []

  if (hash.startsWith('ventas') || hash === 'precios' || hash.startsWith('reportes_ventas') || hash.startsWith('config_ventas')) {
    currentApp = 'VENTAS'
    subMenu = SUB_NAV.ventas
  } else if (hash.startsWith('compras') || hash.startsWith('config_compras') || hash.startsWith('productos_compra') || hash.startsWith('reportes_compras')) {
    currentApp = 'COMPRAS'
    subMenu = SUB_NAV.compras
  } else if (hash.startsWith('crm') || hash.startsWith('config_crm') || hash.startsWith('reportes_crm')) {
    currentApp = 'CRM'
    subMenu = SUB_NAV.crm
  } else if (hash.startsWith('facturas') || hash.startsWith('pagos') || hash.startsWith('config_facturacion') || hash.startsWith('reportes_facturacion')) {
    currentApp = 'FACTURACIÓN'
    subMenu = SUB_NAV.facturacion
  } else if (hash.startsWith('stock') || hash.startsWith('productos') || hash.startsWith('operaciones') || hash.startsWith('reportes_inventario') || hash.startsWith('config_inventario')) {
    currentApp = 'INVENTARIO'
    subMenu = SUB_NAV.inventario
  } else if (hash.startsWith('partners') || hash.startsWith('etiquetas') || hash.startsWith('config_contactos')) {
    currentApp = 'CONTACTOS'
    subMenu = SUB_NAV.contactos
  } else if (hash.startsWith('account') || hash.startsWith('contabilidad') || hash.startsWith('config_contabilidad')) {
    currentApp = 'CONTABILIDAD'
    subMenu = [
      { id: 'account', label: 'Asientos (Odoo2Rs)' },
      { id: 'contabilidad', label: 'Asientos (Mock)' },
      { id: 'config_contabilidad', label: 'Configuración' }
    ]
  } else if (hash.startsWith('nomina') || hash.startsWith('config_nomina')) {
    currentApp = 'NÓMINA'
    subMenu = SUB_NAV.nomina
  } else if (hash.startsWith('mercadily')) {
    currentApp = 'MERCADILY'
    subMenu = SUB_NAV.mercadily
  } else if (hash.startsWith('mail')) {
    currentApp = 'MENSAJERÍA'
    subMenu = [{ id: 'mail', label: 'Bandeja de Entrada' }]
  } else if (hash.startsWith('apps')) {
    currentApp = 'APLICACIONES'
    subMenu = []
  }

  // Actualizar Título
  const titleEl = document.getElementById('odoo-app-title')
  if (titleEl) titleEl.textContent = currentApp

  // Actualizar Navegación
  const navEl = document.getElementById('odoo-app-nav')
  if (navEl) {
    if (subMenu.length > 0) {
      navEl.innerHTML = subMenu.map(n => `
        <a class="app-nav-link ${n.id === hash ? 'active' : ''}" href="#${n.id}" onclick="event.preventDefault();window._go('${n.id}')">
          ${n.label}
        </a>
      `).join('')
    } else {
      navEl.innerHTML = ''
    }
  }
}
