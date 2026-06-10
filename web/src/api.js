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
  // Auth
  login: (login, password) => req('POST', '/auth/login', { login, password }),

  // Dashboard
  dashboard: ()      => req('GET', '/dashboard'),
  ventaKpis: ()      => req('GET', '/ventas/kpis'),
  factKpis:  ()      => req('GET', '/facturas/kpis'),
  stockKpis: ()      => req('GET', '/stock/kpis'),

  // Ventas
  ventas:  (p=1)     => req('GET', `/ventas?pagina=${p}`),
  venta:   (id)      => req('GET', `/ventas/${id}`),

  // Facturas
  facturas: (p=1)    => req('GET', `/facturas?pagina=${p}`),
  factura:  (id)     => req('GET', `/facturas/${id}`),
  porCobrar: ()      => req('GET', '/facturas/por-cobrar'),

  // Productos
  productos: (p=1, q='') => req('GET', `/productos?pagina=${p}&q=${encodeURIComponent(q)}`),
  producto:  (id)    => req('GET', `/productos/${id}`),

  // Partners
  partners:   (p=1)  => req('GET', `/partners?pagina=${p}`),
  clientes:   (p=1)  => req('GET', `/clientes?pagina=${p}`),
  proveedores:(p=1)  => req('GET', `/proveedores?pagina=${p}`),

  // Stock
  stock:     (p=1)   => req('GET', `/stock?pagina=${p}`),
  stockBajo: ()      => req('GET', '/stock/bajo'),
  stockProducto: (id) => req('GET', `/stock/producto/${id}`),

  // CFDI
  timbrar: (body)    => req('POST', '/cfdi/timbrar', body),
  cancelar: (body)   => req('POST', '/cfdi/cancelar', body),

  // Search
  searchSync: ()     => req('POST', '/search/sync', {}),
  searchStatus: ()   => req('GET', '/search/status'),

  // Health
  health: ()         => req('GET', '/health'),
}
