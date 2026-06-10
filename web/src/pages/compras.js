import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { fmtMxn, fmtDate, paginationHtml, skeletonTable, toast,
         openDetailModal, detailRow, detailSection, stateBadge } from '../ui.js'
import { api } from '../api.js'
import { editarCompra } from './forms/edit_forms.js'

const ESTADO_MAP = {
  purchase: { lbl:'Confirmada', color:'indigo'  },
  done:     { lbl:'Recibida',   color:'emerald' },
  draft:    { lbl:'Borrador',   color:'gray'    },
  cancel:   { lbl:'Cancelada',  color:'red'     },
  sent:     { lbl:'Enviada',    color:'sky'     },
}

let _page = 1

export async function renderCompras() {
  ensureLayout()
  setBreadcrumb([{label:'Dashboard',href:'dashboard'},{label:'Compras'}])
  _page = 1
  await loadCompras()
}

async function loadCompras() {
  setPage(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">🛒 Órdenes de Compra</h1>
      <p class="page-subtitle" id="comp-sub">Cargando…</p>
    </div>
    <div class="page-actions">
      <input type="text" id="buscar-comp" class="search-input" placeholder="🔍 Buscar folio o proveedor…" style="width:240px">
      <button class="btn btn-primary" onclick="alert('Nueva OC — próximamente')">+ Nueva Orden</button>
    </div>
  </div>

  <!-- KPIs -->
  <div style="display:grid;grid-template-columns:repeat(4,1fr);gap:12px;margin-bottom:18px" id="kpi-row" class="anim-2">
    ${[1,2,3,4].map(() => `<div class="data-card" style="padding:16px"><div class="skeleton" style="height:40px"></div></div>`).join('')}
  </div>

  <div class="data-card anim-3">
    <div class="data-card-header">
      <div class="data-card-title">Órdenes de Compra</div>
    </div>
    <div id="comp-tabla">${skeletonTable(8, 5)}</div>
  </div>`)

  try {
    const [kpisRes, listRes] = await Promise.allSettled([
      api.comprasKpis(),
      api.compras(_page),
    ])

    // KPIs
    const kpis = kpisRes.status === 'fulfilled' ? kpisRes.value?.data : null
    const kpiRow = document.getElementById('kpi-row')
    if (kpiRow) {
      kpiRow.innerHTML = [
        { label: 'Total OC',        val: kpis?.total        ?? 0, tipo:'num', color:'indigo',  icon:'📋' },
        { label: 'Confirmadas',     val: kpis?.confirmadas  ?? 0, tipo:'num', color:'emerald', icon:'✅' },
        { label: 'Monto Total',     val: kpis?.monto_total  ?? 0, tipo:'mxn', color:'violet',  icon:'💰' },
        { label: 'Completadas',     val: kpis?.completadas  ?? 0, tipo:'num', color:'amber',   icon:'📦' },
      ].map(k => `
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${k.icon} ${k.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${k.tipo === 'mxn' ? fmtMxn(parseFloat(k.val)) : Number(k.val).toLocaleString('es-MX')}
        </div>
      </div>`).join('')
    }

    // Tabla
    const compras = listRes.status === 'fulfilled' ? (listRes.value?.data || []) : []
    const total = listRes.value?.total ?? compras.length
    const hasMore = compras.length >= 20

    const sub = document.getElementById('comp-sub')
    if (sub) sub.textContent = `${total} órdenes · Página ${_page}`

    const tablaEl = document.getElementById('comp-tabla')
    if (tablaEl) {
      if (compras.length === 0) {
        tablaEl.innerHTML = `<div style="text-align:center;padding:60px;color:var(--text-400)">Sin órdenes de compra registradas</div>`
      } else {
        tablaEl.innerHTML = `
        <table class="data-table">
          <thead><tr>
            <th>Folio</th><th>Proveedor</th><th>Fecha</th>
            <th>Total</th><th>Estado</th>
          </tr></thead>
          <tbody>
            ${compras.map(c => {
              const e = ESTADO_MAP[c.state] || { lbl: c.state || '—', color:'gray' }
              return `
              <tr style="cursor:pointer" onclick="window._verCompra(${c.id})" title="Ver detalle">
                <td class="td-mono">${c.name || `#${c.id}`}</td>
                <td class="td-primary">${c.partner_name || '—'}</td>
                <td>${fmtDate(c.date_order)}</td>
                <td class="td-amount" style="font-weight:700">${fmtMxn(parseFloat(c.amount_total || 0))}</td>
                <td>${stateBadge(c.state, e.lbl)}</td>
              </tr>`
            }).join('')}
          </tbody>
        </table>
        ${paginationHtml(_page, hasMore, (p) => { _page = p; loadCompras() })}`
      }
    }

    // Búsqueda
    document.getElementById('buscar-comp')?.addEventListener('input', e => {
      const q = e.target.value.toLowerCase()
      document.querySelectorAll('#comp-tabla tbody tr').forEach(r => {
        r.style.display = r.textContent.toLowerCase().includes(q) ? '' : 'none'
      })
    })

    // Ver detalle
    window._verCompra = (id) => {
      const c = compras.find(x => x.id === id)
      if (!c) return
      openDetailModal(
        'Detalle Orden de Compra',
        async () => c,
        (c) => {
          const e = ESTADO_MAP[c.state] || { lbl: c.state, color: 'gray' }
          return `
          ${detailSection('Orden', [
            detailRow('Folio', c.name),
            detailRow('Estado', stateBadge(c.state, e.lbl)),
            detailRow('Proveedor', c.partner_name || '—'),
            detailRow('Fecha', fmtDate(c.date_order)),
            detailRow('Fecha entrega', fmtDate(c.date_planned)),
          ].join(''))}
          ${detailSection('Importes', [
            detailRow('Subtotal', fmtMxn(parseFloat(c.amount_untaxed || 0))),
            detailRow('IVA', fmtMxn(parseFloat(c.amount_tax || 0))),
            detailRow('Total', `<strong>${fmtMxn(parseFloat(c.amount_total || 0))}</strong>`, {color:'var(--primary)'}),
          ].join(''))}
          <div style="display:flex;gap:10px;margin-top:16px">
            <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
            <button class="btn btn-secondary btn-sm" onclick="window._editarCompraFn(${c.id})">✏️ Editar</button>
            <button class="btn btn-primary btn-sm" onclick="alert('Recibir mercancía — próximamente')">📦 Recibir</button>
          </div>`
        }
      )
    }

    window._editarCompraFn = (id) => {
      const c = compras.find(x => x.id === id)
      if (c) editarCompra(c, () => loadCompras())
    }

  } catch (err) {
    console.error(err)
    toast('Error al cargar compras', err.message, 'error')
  }
}
