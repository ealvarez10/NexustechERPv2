import './style.css'
import { on, start } from './router.js'
import { renderLogin }     from './pages/login.js'
import { renderDashboard } from './pages/dashboard.js'
import { renderVentas }    from './pages/ventas.js'
import { renderFacturas }  from './pages/facturas.js'
import { renderProductos } from './pages/productos.js'
import { renderPartners }  from './pages/partners.js'
import { renderStock }     from './pages/stock.js'
import { renderGeneric }   from './pages/generic.js'

// Routes
on('login',     renderLogin)
on('dashboard', renderDashboard)
on('ventas',    renderVentas)
on('facturas',  renderFacturas)
on('productos', renderProductos)
on('partners',  renderPartners)
on('stock',     renderStock)

// Generic placeholders for unimplemented modules
on('cfdi',      () => renderGeneric('cfdi',      'CFDI 4.0',      'Timbrado, cancelación y representación impresa', '🔏'))
on('nomina',    () => renderGeneric('nomina',    'Nómina IMSS',   'Cálculo de nómina, IMSS e Infonavit',            '👔'))
on('search',    () => renderGeneric('search',    'NexusSearch',   'Búsqueda global de alta velocidad en Rust',      '🔍'))
on('reportes',  () => renderGeneric('reportes',  'Reportes',      'Reportes ejecutivos y BI avanzado',              '📈'))
on('404',       () => renderGeneric('404',       'Página no encontrada', 'La ruta solicitada no existe',           '🔍'))

// Start router
start()
