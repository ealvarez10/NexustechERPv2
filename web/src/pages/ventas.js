import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { fmtMxn, fmtDate, paginationHtml, skeletonTable, toast,
         stateBadge, openDetailModal, detailRow, detailSection } from '../ui.js'
import { api } from '../api.js'

const ESTADO_MAP = {
  sale:   { lbl:'Confirmada', color:'indigo'  },
  done:   { lbl:'Entregada',  color:'emerald' },
  draft:  { lbl:'Borrador',   color:'gray'    },
  cancel: { lbl:'Cancelada',  color:'red'     },
  sent:   { lbl:'Enviada',    color:'sky'     },
}

let _page = 1
let _total = 0

export async function renderVentas() {
  ensureLayout()
  setBreadcrumb([{label:'Dashboard',href:'dashboard'},{label:'Ventas'}])
  _page = 1
  await loadVentas()
}

async function loadVentas() {
  setPage(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">Ventas</h1>
      <p class="page-subtitle" id="ventas-sub">Cargando…</p>
    </div>
    <div class="page-actions">
      <input type="text" id="buscar-venta" class="search-input" placeholder="🔍 Buscar por folio o cliente…" style="width:240px">
      <button class="btn btn-secondary" id="btn-exportar">📥 Exportar</button>
      <button class="btn btn-primary" onclick="window._nuevaVenta()">+ Nueva Venta</button>
    </div>
  </div>

  <!-- KPI row -->
  <div style="display:grid;grid-template-columns:repeat(4,1fr);gap:12px;margin-bottom:18px" id="kpi-row" class="anim-2">
    ${[1,2,3,4].map(() => `<div class="data-card" style="padding:16px"><div class="skeleton" style="height:40px"></div></div>`).join('')}
  </div>

  <div class="data-card anim-3">
    <div class="data-card-header">
      <div class="data-card-title">Órdenes de Venta</div>
    </div>
    <div id="ventas-tabla">${skeletonTable(8, 6)}</div>
  </div>`)

  try {
    const [kpisRes, listRes] = await Promise.allSettled([
      api.ventaKpis(),
      api.ventas(_page),
    ])

    // ─── KPIs ────────────────────────────────────────────────────────────────
    const kpis = kpisRes.status === 'fulfilled' ? kpisRes.value?.data : null
    const kpiRow = document.getElementById('kpi-row')
    if (kpiRow && kpis) {
      kpiRow.innerHTML = [
        { label: 'Total Órdenes',    val: kpis.ordenes_confirmadas ?? kpis.total_ordenes ?? 0,   tipo:'num', color:'indigo'  },
        { label: 'Facturado Total',  val: kpis.total_facturado    ?? 0,                           tipo:'mxn', color:'emerald' },
        { label: 'Ticket Promedio',  val: kpis.ticket_promedio    ?? 0,                           tipo:'mxn', color:'violet'  },
        { label: 'Este Mes',         val: kpis.ordenes_este_mes   ?? 0,                           tipo:'num', color:'amber'   },
      ].map(k => `
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:6px">${k.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${k.tipo === 'mxn' ? fmtMxn(parseFloat(k.val)) : Number(k.val).toLocaleString('es-MX')}
        </div>
      </div>`).join('')
    }

    // ─── Lista ───────────────────────────────────────────────────────────────
    const ventas = listRes.status === 'fulfilled' ? (listRes.value?.data || []) : []
    _total = listRes.value?.total ?? ventas.length
    const hasMore = ventas.length >= 20

    const sub = document.getElementById('ventas-sub')
    if (sub) sub.textContent = `${_total} registros · Página ${_page}`

    const tablaEl = document.getElementById('ventas-tabla')
    if (tablaEl) {
      if (ventas.length === 0) {
        tablaEl.innerHTML = '<p style="text-align:center;padding:32px;color:var(--text-400)">Sin ventas en esta página</p>'
      } else {
        tablaEl.innerHTML = `
        <table class="data-table">
          <thead><tr>
            <th>Folio</th>
            <th>Cliente</th>
            <th>Fecha</th>
            <th>Subtotal</th>
            <th>Total</th>
            <th>Factura</th>
            <th>Estado</th>
          </tr></thead>
          <tbody>
            ${ventas.map(v => {
              const e = ESTADO_MAP[v.state] || { lbl: v.state || '—', color:'gray' }
              const fecha = v.date_order ? fmtDate(v.date_order) : '—'
              const inv = v.invoice_status === 'invoiced' ? 'Facturada' :
                          v.invoice_status === 'to invoice' ? 'Por facturar' : '—'
              return `
              <tr style="cursor:pointer" onclick="window._verVenta(${v.id})" title="Ver detalle">
                <td class="td-mono">${v.name || `#${v.id}`}</td>
                <td class="td-primary">${v.partner_name || '—'}</td>
                <td>${fecha}</td>
                <td class="td-amount">${fmtMxn(parseFloat(v.amount_untaxed || 0))}</td>
                <td class="td-amount" style="font-weight:700">${fmtMxn(parseFloat(v.amount_total || 0))}</td>
                <td><span class="badge badge-${inv==='Facturada'?'emerald':inv==='Por facturar'?'amber':'gray'}" style="font-size:10px">${inv}</span></td>
                <td>${stateBadge(v.state, e.lbl)}</td>
              </tr>`
            }).join('')}
          </tbody>
        </table>
        ${paginationHtml(_page, hasMore, (p) => { _page = p; loadVentas() })}`
      }
    }

    // Búsqueda
    document.getElementById('buscar-venta')?.addEventListener('input', (e) => {
      const q = e.target.value.toLowerCase()
      document.querySelectorAll('#ventas-tabla tbody tr').forEach(row => {
        row.style.display = row.textContent.toLowerCase().includes(q) ? '' : 'none'
      })
    })

    // Ver detalle
    window._verVenta = (id) => {
      openDetailModal(
        'Detalle de Orden de Venta',
        () => api.get(`/ventas/${id}`),
        (v) => {
          const e = ESTADO_MAP[v.state] || { lbl: v.state, color:'gray' }
          return `
          ${detailSection('Información General', [
            detailRow('Folio', v.name),
            detailRow('Estado', stateBadge(v.state, e.lbl)),
            detailRow('Cliente', v.partner_name || v.partner_id),
            detailRow('Fecha', fmtDate(v.date_order)),
            detailRow('Estado Factura', v.invoice_status || '—'),
            detailRow('Política entrega', v.picking_policy || '—'),
          ].join(''))}
          ${detailSection('Importes', [
            detailRow('Subtotal', fmtMxn(parseFloat(v.amount_untaxed || 0))),
            detailRow('IVA', fmtMxn(parseFloat(v.amount_tax || 0))),
            detailRow('Total', `<strong>${fmtMxn(parseFloat(v.amount_total || 0))}</strong>`, {color:'var(--primary)'}),
          ].join(''))}
          <div style="display:flex;gap:10px;margin-top:16px">
            <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
            <button class="btn btn-primary btn-sm" onclick="alert('Crear CFDI — próximamente')">🔏 Timbrar CFDI</button>
          </div>`
        }
      )
    }

    // Nueva venta
    window._nuevaVenta = () => {
      toast('Próximamente', 'Formulario de nueva venta en desarrollo', 'info')
    }

  } catch (err) {
    console.error(err)
    toast('Error al cargar ventas', err.message, 'error')
    const tablaEl = document.getElementById('ventas-tabla')
    if (tablaEl) tablaEl.innerHTML = `<p style="text-align:center;padding:32px;color:var(--red)">Error de conexión: ${err.message}</p>`
  }
}
