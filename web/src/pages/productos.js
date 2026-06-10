import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { fmtMxn, fmtNum, paginationHtml, skeletonTable, toast,
         openDetailModal, detailRow, detailSection } from '../ui.js'
import { api } from '../api.js'

let _page = 1
let _query = ''

export async function renderProductos() {
  ensureLayout()
  setBreadcrumb([{ label: 'Dashboard', href: 'dashboard' }, { label: 'Productos' }])
  _page = 1
  _query = ''
  await loadProductos()
}

async function loadProductos() {
  setPage(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">Productos</h1>
      <p class="page-subtitle" id="prod-sub">Cargando catálogo…</p>
    </div>
    <div class="page-actions">
      <input type="text" id="buscar-prod" class="search-input" placeholder="🔍 Buscar producto o código…" style="width:240px" value="${_query}">
      <button class="btn btn-primary">+ Nuevo Producto</button>
    </div>
  </div>

  <div class="data-card anim-2">
    <div class="data-card-header">
      <div class="data-card-title">Catálogo de Productos</div>
      <select id="filtro-tipo" class="search-input" style="width:150px;font-size:12px">
        <option value="">Todos</option>
        <option value="consu">Consumibles</option>
        <option value="service">Servicios</option>
        <option value="product">Almacenables</option>
      </select>
    </div>
    <div id="prod-tabla">${skeletonTable(10, 6)}</div>
  </div>`)

  try {
    const res = await api.productos(_page, _query)
    const productos = res?.data || []
    const hasMore = productos.length >= 20

    const sub = document.getElementById('prod-sub')
    if (sub) sub.textContent = `${productos.length} productos${_query ? ` para "${_query}"` : ''} · Página ${_page}`

    const tablaEl = document.getElementById('prod-tabla')
    if (tablaEl) {
      if (productos.length === 0) {
        tablaEl.innerHTML = `<p style="text-align:center;padding:40px;color:var(--text-400)">
          ${_query ? `Sin resultados para "${_query}"` : 'Sin productos en catálogo'}
        </p>`
      } else {
        tablaEl.innerHTML = `
        <table class="data-table">
          <thead><tr>
            <th>Código</th><th>Nombre</th><th>Tipo</th>
            <th>Precio Venta</th><th>Categoría</th><th>Estado</th>
          </tr></thead>
          <tbody>
            ${productos.map(p => {
              const nombre = (p.name && typeof p.name === 'object')
                ? (p.name.es_MX || p.name.en_US || Object.values(p.name)[0] || `Producto #${p.id}`)
                : (p.name || p.nombre || `Producto #${p.id}`)
              const tp = p.type_ || p.type || ''
              const tipo = tp === 'consu' ? 'Consumible' : tp === 'service' ? 'Servicio' : tp === 'product' ? 'Almacenable' : 'Consumible'
              const tipoColor = tp === 'service' ? 'violet' : tp === 'consu' ? 'sky' : 'indigo'
              const precio = fmtMxn(parseFloat(p.list_price || p.precio || 0))
              const activo = p.active !== false
              const categRaw = p.categ_name || p.categoria || ''
              const categ = categRaw === 'Goods' ? 'Mercancía' : categRaw === 'Services' ? 'Servicios' : categRaw || '—'
              return `
              <tr data-tipo="${tp}" data-id="${p.id}" style="cursor:pointer" onclick="window._verProducto(${p.id})" title="Ver detalle">
                <td class="td-mono">${p.default_code || '—'}</td>
                <td class="td-primary">${nombre}</td>
                <td><span class="badge badge-${tipoColor}">${tipo}</span></td>
                <td class="td-amount" style="font-weight:700">${precio}</td>
                <td style="color:var(--text-400);font-size:12px">${categ}</td>
                <td><span class="badge badge-${activo ? 'emerald' : 'gray'}">${activo ? 'Activo' : 'Inactivo'}</span></td>
              </tr>`
            }).join('')}
          </tbody>
        </table>
        ${paginationHtml(_page, hasMore, (p) => { _page = p; loadProductos() })}`
      }
    }

    // Búsqueda con debounce
    let timer
    document.getElementById('buscar-prod')?.addEventListener('input', (e) => {
      clearTimeout(timer)
      timer = setTimeout(() => {
        _query = e.target.value.trim()
        _page = 1
        loadProductos()
      }, 400)
    })

    // Filtro local por tipo
    document.getElementById('filtro-tipo')?.addEventListener('change', (e) => {
      const val = e.target.value
      document.querySelectorAll('#prod-tabla tbody tr').forEach(r => {
        r.style.display = !val || r.dataset.tipo === val ? '' : 'none'
      })
    })

    // Ver detalle del producto
    window._verProducto = (id) => {
      const p = productos.find(x => x.id === id)
      if (!p) return
      const nombre = (p.name && typeof p.name === 'object')
        ? (p.name.es_MX || p.name.en_US || '')
        : (p.name || '')
      const tp = p.type_ || p.type || ''
      const tipo = tp === 'consu' ? 'Consumible' : tp === 'service' ? 'Servicio' : 'Almacenable'
      const categRaw = p.categ_name || ''
      const categ = categRaw === 'Goods' ? 'Mercancía' : categRaw === 'Services' ? 'Servicios' : categRaw || '—'

      openDetailModal(
        'Detalle de Producto',
        async () => p,
        () => `
        ${detailSection('Identificación', [
          detailRow('Nombre', nombre),
          detailRow('Código interno', p.default_code || '—'),
          detailRow('Código de barras', p.barcode || '—'),
          detailRow('Tipo', tipo),
          detailRow('Categoría', categ),
          detailRow('Estado', `<span class="badge badge-${p.active!==false?'emerald':'gray'}">${p.active!==false?'Activo':'Inactivo'}</span>`),
        ].join(''))}
        ${detailSection('Precios', [
          detailRow('Precio de venta', fmtMxn(parseFloat(p.list_price || 0))),
          detailRow('Costo estándar', fmtMxn(parseFloat(p.standard_price || 0))),
        ].join(''))}
        <div style="display:flex;gap:10px;margin-top:16px">
          <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
          <button class="btn btn-primary btn-sm" onclick="alert('Editar producto — próximamente')">✏️ Editar</button>
        </div>`
      )
    }

  } catch (err) {
    console.error(err)
    toast('Error al cargar productos', err.message, 'error')
  }
}
