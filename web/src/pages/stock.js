import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { fmt, fmtMxn, paginationHtml, skeletonTable, toast,
         openDetailModal, detailRow, detailSection } from '../ui.js'
import { api } from '../api.js'

// Productos de sistema que no son del negocio — filtrar de la vista
const SYSTEM_PRODUCTS = ['deposit', 'down payment', 'downpayment', 'pago inicial']

let _page = 1

export async function renderStock() {
  ensureLayout()
  setBreadcrumb([{label:'Dashboard',href:'dashboard'},{label:'Inventario'}])
  _page = 1
  await loadStock()
}

async function loadStock() {
  setPage(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">Inventario</h1>
      <p class="page-subtitle" id="stock-sub">Cargando…</p>
    </div>
    <div class="page-actions">
      <input type="text" id="buscar-stock" class="search-input" placeholder="🔍 Buscar producto…" style="width:220px">
      <button class="btn btn-secondary" id="btn-ajuste">📋 Ajuste</button>
      <button class="btn btn-primary">+ Recepción</button>
    </div>
  </div>

  <!-- KPI row -->
  <div style="display:grid;grid-template-columns:repeat(4,1fr);gap:12px;margin-bottom:18px" id="kpi-row" class="anim-2">
    ${[1,2,3,4].map(() => `<div class="data-card" style="padding:16px"><div class="skeleton" style="height:40px"></div></div>`).join('')}
  </div>

  <div style="display:grid;grid-template-columns:1fr 340px;gap:16px" class="anim-3">
    <!-- Tabla principal -->
    <div class="data-card">
      <div class="data-card-header">
        <div class="data-card-title">Stock por Producto</div>
        <select id="filtro-stock" class="search-input" style="width:160px;font-size:12px">
          <option value="todos">Todos</option>
          <option value="bajo">⚠️ Stock bajo</option>
          <option value="ok">✅ Stock normal</option>
        </select>
      </div>
      <div id="stock-tabla">${skeletonTable(8, 5)}</div>
    </div>

    <!-- Panel stock bajo -->
    <div class="data-card" style="padding:16px">
      <div style="font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.06em;color:var(--text-400);margin-bottom:12px">⚠️ Alertas de Stock Bajo</div>
      <div id="stock-bajo-lista">${[1,2,3,4,5].map(() => `<div class="skeleton" style="height:36px;margin-bottom:8px;border-radius:8px"></div>`).join('')}</div>
    </div>
  </div>`)

  try {
    const [kpisRes, stockRes, bajoRes] = await Promise.allSettled([
      api.stockKpis(),
      api.stock(_page),
      api.stockBajo(),
    ])

    // ─── KPIs ────────────────────────────────────────────────────────────────
    const kpis = kpisRes.status === 'fulfilled' ? kpisRes.value?.data : null
    const kpiRow = document.getElementById('kpi-row')
    if (kpiRow && kpis) {
      kpiRow.innerHTML = [
        { label: 'Con stock',     val: kpis.total_productos_con_stock || 0, tipo:'num', color:'emerald', icon:'✅' },
        { label: 'Sin stock',     val: kpis.total_sin_stock            || 0, tipo:'num', color:'red',     icon:'❌' },
        { label: 'Valor Inventario', val: kpis.valor_inventario        || 0, tipo:'mxn', color:'indigo',  icon:'💰' },
        { label: 'Alertas Bajo', val: kpis.alertas_stock_bajo          || 0, tipo:'num', color:'amber',   icon:'⚠️' },
      ].map(k => `
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${k.icon} ${k.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${k.tipo === 'mxn' ? fmtMxn(parseFloat(k.val)) : Number(k.val).toLocaleString('es-MX')}
        </div>
      </div>`).join('')
    }

    // ─── Tabla principal ─────────────────────────────────────────────────────
    const allItems = stockRes.status === 'fulfilled' ? (stockRes.value?.data || []) : []
    // Filtrar productos del sistema (Deposit, Down Payment, etc.)
    const items = allItems.filter(s => {
      const name = (s.product_name || '').toLowerCase()
      return !SYSTEM_PRODUCTS.some(sp => name.includes(sp))
    })
    const hasMore = allItems.length >= 20

    const sub = document.getElementById('stock-sub')
    if (sub) sub.textContent = `${items.length} productos · Página ${_page}`

    const tablaEl = document.getElementById('stock-tabla')
    if (tablaEl) {
      if (items.length === 0) {
        tablaEl.innerHTML = '<p style="text-align:center;padding:32px;color:var(--text-400)">Sin datos de stock</p>'
      } else {
        tablaEl.innerHTML = `
        <table class="data-table">
          <thead><tr>
            <th>Producto</th>
            <th>Disponible</th>
            <th>Reservado</th>
            <th>Ubicación</th>
            <th>Estado</th>
          </tr></thead>
          <tbody>
            ${items.map(s => {
              const qty = parseFloat(s.cantidad_disponible || 0)
              const reservado = parseFloat(s.cantidad_reservada || 0)
              const color = qty <= 0 ? 'red' : qty < 10 ? 'amber' : 'emerald'
              const estado = qty <= 0 ? '❌ Sin stock' : qty < 10 ? '⚠️ Stock bajo' : '✅ Normal'
              return `
              <tr data-alerta="${qty < 10 ? 'bajo' : 'ok'}" style="cursor:pointer" onclick="window._verStock(${s.product_id})" title="Ver detalle">
                <td class="td-primary">${s.product_name || `Producto #${s.product_id}`}</td>
                <td><span class="badge badge-${color}">${fmt(qty, 0)}</span></td>
                <td style="color:var(--text-400)">${fmt(reservado, 0)}</td>
                <td class="td-mono" style="font-size:11px">${s.ubicacion || '—'}</td>
                <td><span class="badge badge-${color}">${estado}</span></td>
              </tr>`
            }).join('')}
          </tbody>
        </table>
        ${paginationHtml(_page, hasMore, (p) => { _page = p; loadStock() })}`
      }
    }

    // ─── Panel stock bajo ─────────────────────────────────────────────────────
    const bajo = bajoRes.status === 'fulfilled' ? (bajoRes.value?.data || []) : []
    const bajoEl = document.getElementById('stock-bajo-lista')
    if (bajoEl) {
      if (bajo.length === 0) {
        bajoEl.innerHTML = '<p style="color:var(--emerald);font-size:13px;text-align:center;padding:16px">✅ Todo en niveles normales</p>'
      } else {
        bajoEl.innerHTML = bajo.map(s => {
          const qty = parseFloat(s.cantidad_disponible || 0)
          const color = qty <= 0 ? 'red' : 'amber'
          return `
          <div style="display:flex;justify-content:space-between;align-items:center;padding:8px 0;border-bottom:1px solid var(--border)">
            <div>
              <div style="font-size:12.5px;font-weight:600;color:var(--text-700)">${(s.product_name || `#${s.product_id}`).substring(0,28)}</div>
            </div>
            <span class="badge badge-${color}">${qty}</span>
          </div>`
        }).join('')
      }
    }

    // Filtros
    document.getElementById('buscar-stock')?.addEventListener('input', (e) => {
      const q = e.target.value.toLowerCase()
      document.querySelectorAll('#stock-tabla tbody tr').forEach(row => {
        row.style.display = row.textContent.toLowerCase().includes(q) ? '' : 'none'
      })
    })

    document.getElementById('filtro-stock')?.addEventListener('change', (e) => {
      const val = e.target.value
      document.querySelectorAll('#stock-tabla tbody tr').forEach(row => {
        if (val === 'todos') { row.style.display = ''; return }
        const alerta = row.dataset.alerta
        row.style.display = alerta === val ? '' : 'none'
      })
    })

    // Ver detalle del stock por producto
    window._verStock = (productId) => {
      openDetailModal(
        'Detalle de Stock',
        () => api.stockProducto(productId),
        (data) => {
          const arr = Array.isArray(data) ? data : [data]
          const main = arr[0] || {}
          const qty = parseFloat(main.cantidad_disponible || 0)
          const res = parseFloat(main.cantidad_reservada || 0)
          const color = qty <= 0 ? 'var(--red)' : qty < 10 ? 'var(--warning)' : 'var(--success)'
          return `
          ${detailSection('Producto', [
            detailRow('Nombre', main.product_name || `#${productId}`),
            detailRow('Cantidad disponible', `<strong style="color:${color}">${fmt(qty, 2)}</strong>`),
            detailRow('Cantidad reservada', fmt(res, 2)),
            detailRow('Cantidad neta', fmt(qty - res, 2)),
          ].join(''))}
          ${arr.length > 1 ? detailSection('Por ubicación', arr.map(s =>
            detailRow(s.ubicacion || 'Sin ubicación', fmt(parseFloat(s.cantidad_disponible || 0), 2))
          ).join('')) : detailSection('Ubicación', [
            detailRow('Almacén', main.ubicacion || 'Sin ubicación'),
          ].join(''))}
          <div style="display:flex;gap:10px;margin-top:16px">
            <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
            <button class="btn btn-primary btn-sm" onclick="alert('Ajuste de inventario — próximamente')">📋 Ajustar</button>
          </div>`
        }
      )
    }

  } catch (err) {
    console.error(err)
    toast('Error al cargar inventario', err.message, 'error')
  }
}
