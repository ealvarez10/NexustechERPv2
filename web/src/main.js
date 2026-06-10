import './style.css'
import { on, start } from './router.js'
import { renderLogin }     from './pages/login.js'
import { renderHome }      from './pages/home.js'
import { renderDashboard } from './pages/dashboard.js'
import { renderVentas }    from './pages/ventas.js'
import { renderFacturas }  from './pages/facturas.js'
import { renderProductos } from './pages/productos.js'
import { renderPartners }  from './pages/partners.js'
import { renderStock }     from './pages/stock.js'
import { renderCfdi }      from './pages/cfdi.js'
import { renderNomina }    from './pages/nomina.js'
import { renderCompras }   from './pages/compras.js'
import { renderCotizaciones } from './pages/cotizaciones.js'
import { renderSearch }    from './pages/search.js'
import { renderReportes }  from './pages/reportes.js'
import { renderGeneric }   from './pages/generic.js'

// Routes
on('login',     renderLogin)
on('home',      renderHome)
on('dashboard', renderDashboard)
on('ventas',    renderVentas)
on('facturas',  renderFacturas)
on('productos', renderProductos)
on('partners',  renderPartners)
on('stock',     renderStock)
on('cfdi',      renderCfdi)
on('nomina',    renderNomina)
on('compras',   renderCompras)
on('cotizaciones', renderCotizaciones)
on('search',    renderSearch)
on('reportes',  renderReportes)

on('404', () => renderGeneric('404', 'Página no encontrada', 'La ruta solicitada no existe', '🔍'))

// Start router
start()
