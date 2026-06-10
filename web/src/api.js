const BASE = '/api/v1'

function token() { return localStorage.getItem('nx_token') }

export class ApiError extends Error {
  constructor(status, msg) { super(msg); this.status = status; }
}

async function req(method, path, body) {
  const tk = token()
  const res = await fetch(BASE + path, {
    method,
    headers: {
      'Content-Type': 'application/json',
      ...(tk ? { 'Authorization': `Bearer ${tk}` } : {})
    },
    ...(body !== undefined ? { body: JSON.stringify(body) } : {})
  })
  if (res.status === 401) {
    localStorage.removeItem('nx_token')
    localStorage.removeItem('nx_user')
    window.location.hash = 'login'
    return null
  }
  if (!res.ok) throw new ApiError(res.status, await res.text())
  const ct = res.headers.get('content-type') || ''
  return ct.includes('application/json') ? res.json() : res.text()
}

export const api = {
  // Generic
  get:  (path)         => req('GET',  path),
  post: (path, body)   => req('POST', path, body),
  put:  (path, body)   => req('PUT',  path, body),
  del:  (path)         => req('DELETE', path),

  // Auth
  login: (login, password) => req('POST', '/auth/login', { login, password }),
  logout: ()               => req('POST', '/auth/logout', {}),

  // Dashboard
  dashboard: ()      => req('GET', '/dashboard'),
  ventaKpis: ()      => req('GET', '/ventas/kpis'),
  factKpis:  ()      => req('GET', '/facturas/kpis'),
  stockKpis: ()      => req('GET', '/stock/kpis'),

  // Ventas
  ventas:  (p=1)     => req('GET', `/ventas?pagina=${p}`),
  venta:   (id)      => req('GET', `/ventas/${id}`),

  // Facturas
  facturas:  (p=1)   => req('GET', `/facturas?pagina=${p}`),
  factura:   (id)    => req('GET', `/facturas/${id}`),
  porCobrar: ()      => req('GET', '/facturas/por-cobrar'),

  // Productos
  productos: (p=1, q='') => req('GET', `/productos?pagina=${p}&q=${encodeURIComponent(q)}`),
  producto:  (id)        => req('GET', `/productos/${id}`),

  // Partners
  partners:    (p=1) => req('GET', `/partners?pagina=${p}`),
  partner:     (id)  => req('GET', `/partners/${id}`),
  clientes:    (p=1) => req('GET', `/clientes?pagina=${p}`),
  proveedores: (p=1) => req('GET', `/proveedores?pagina=${p}`),

  // Stock / Inventario
  stock:        (p=1) => req('GET', `/stock?pagina=${p}`),
  stockKpis:    ()    => req('GET', '/stock/kpis'),
  stockBajo:    ()    => req('GET', '/stock/bajo'),
  stockProducto:(id)  => req('GET', `/stock/producto/${id}`),

  // CFDI 4.0
  cfdiTimbrados: (p=1) => req('GET', `/cfdi/timbrados?pagina=${p}`),
  cfdiTimbrado:  (uuid)=> req('GET', `/cfdi/timbrados/${uuid}`),
  cfdiKpis:      ()    => req('GET', '/cfdi/kpis'),
  timbrar:  (body)     => req('POST', '/cfdi/timbrar', body),
  cancelarCfdi:(body)  => req('POST', '/cfdi/cancelar', body),

  // Nómina
  nomina:      (p=1)  => req('GET', `/nomina?pagina=${p}`),
  empleado:    (id)   => req('GET', `/nomina/${id}`),
  nominaKpis:  ()     => req('GET', '/nomina/kpis'),

  // Compras
  compras:      (p=1) => req('GET', `/compras?pagina=${p}`),
  compra:       (id)  => req('GET', `/compras/${id}`),
  comprasKpis:  ()    => req('GET', '/compras/kpis'),

  // Cotizaciones
  cotizaciones:        (p=1)           => req('GET', `/cotizaciones?pagina=${p}`),
  cotizacionKpis:      ()              => req('GET', '/cotizaciones/kpis'),
  cotizacion:          (id)            => req('GET', `/cotizaciones/${id}`),
  crearCotizacion:     (data)          => req('POST', '/cotizaciones', data),
  confirmarCotizacion: (id)            => req('PUT', `/cotizaciones/${id}/confirmar`),
  cancelarCotizacion:  (id)            => req('PUT', `/cotizaciones/${id}/cancelar`),
  actualizarCotizacion:(id, data)      => req('PUT', `/cotizaciones/${id}`, data),
  agregarLinea:        (orderId, data) => req('POST', `/cotizaciones/${orderId}/lineas`, data),
  eliminarLinea:       (orderId, lineaId) => req('DELETE', `/cotizaciones/${orderId}/lineas/${lineaId}`),

  // Búsqueda
  searchSync:   ()    => req('POST', '/search/sync', {}),
  searchStatus: ()    => req('GET', '/search/status'),

  // Health
  health: ()          => req('GET', '/health'),

  // PUT endpoints para módulos de edición
  putVenta:    (id, data) => req('PUT', `/ventas/${id}`,    data),
  putPartner:  (id, data) => req('PUT', `/partners/${id}`,  data),
  putProducto: (id, data) => req('PUT', `/productos/${id}`, data),
  putCompra:   (id, data) => req('PUT', `/compras/${id}`,   data),
  putEmpleado: (id, data) => req('PUT', `/nomina/${id}`,    data),
  ajusteStock: (productId, data) => req('PUT', `/stock/${productId}/ajuste`, data),
}
