import './style.css'
import { on, start } from './router.js'
import { renderLogin }          from './pages/login.js'
import { renderHome }           from './pages/home.js'
import { renderDashboard }      from './pages/dashboard.js'
import { renderVentas }         from './pages/ventas.js'
import { renderFacturas }       from './pages/facturas.js'
import { renderProductos }      from './pages/productos.js'
import { renderPartners }       from './pages/partners.js'
import { renderStock, _renderPicking } from './pages/stock.js'
import { renderCfdi }           from './pages/cfdi.js'
import { renderNomina }         from './pages/nomina.js'
import { renderCompras }        from './pages/compras.js'
import { renderPrecios }        from './pages/precios.js'
import { renderReportesVentas } from './pages/reportes_ventas.js'
import { renderConfigVentas }   from './pages/config_ventas.js'
import { renderConfigFacturacion } from './pages/config_facturacion.js'
import { renderConfigCompras }  from './pages/config_compras.js'
import { renderConfigCRM }      from './pages/config_crm.js'
import { renderCRM }            from './pages/crm.js'
import { renderConfigInventario } from './pages/config_inventario.js'
import { renderConfigContactos }  from './pages/config_contactos.js'
import { renderSearch }         from './pages/search.js'
import { renderReportes }       from './pages/reportes.js'
import { renderGeneric }        from './pages/generic.js'
import { renderContabilidad } from './pages/contabilidad.js'
import { renderConfigContabilidad } from './pages/config_contabilidad.js'
import { renderMercadily } from './pages/mercadily.js'
import { renderApps } from './pages/apps.js'
import { renderAccount } from './pages/account.js'
import { renderMail } from './pages/mail.js'

const ROUTES = {
  account: renderAccount,
  mail: renderMail,
  apps: renderApps,
  login: renderLogin,
  home: renderHome,
  dashboard: renderDashboard,
  ventas: renderVentas,
  facturas: renderFacturas,
  stock: renderStock,
  compras: renderCompras,
  crm: renderCRM,
  partners: renderPartners,
  productos: renderProductos,
  nomina: renderNomina,
  reportes: renderReportes,
  cfdi: renderCfdi,
  precios: renderPrecios,
  reportes_ventas: renderReportesVentas,
  config_ventas: renderConfigVentas,
  config_facturacion: renderConfigFacturacion,
  config_compras: renderConfigCompras,
  config_crm: renderConfigCRM,
  config_inventario: renderConfigInventario,
  config_contactos: renderConfigContactos,
  contabilidad: renderContabilidad,
  config_contabilidad: renderConfigContabilidad,
  mercadily: renderMercadily,
  search: renderSearch
}

Object.keys(ROUTES).forEach(route => on(route, ROUTES[route]))

// Special routes
on('stock', (params) => {
  if (params.picking) _renderPicking(parseInt(params.picking), params.origen ? parseInt(params.origen) : null)
  else renderStock(params)
})
on('pagos',           renderGeneric)
on('reportes_facturacion', renderGeneric)

on('404', () => renderGeneric('404', 'Página no encontrada', 'La ruta solicitada no existe', '🔍'))

// Start router
start()
