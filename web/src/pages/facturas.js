import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { fmtMxn, fmtDate, fmtNum, paginationHtml, skeletonTable, toast,
         stateBadge, openDetailModal, detailRow, detailSection } from '../ui.js'
import { api } from '../api.js'
import { verFacturaDetalle } from './forms/edit_forms.js'

const ESTADO_MAP = {
  posted:     { lbl: 'Publicada',  color: 'emerald' },
  draft:      { lbl: 'Borrador',   color: 'gray'    },
  in_payment: { lbl: 'En cobro',   color: 'violet'  },
  paid:       { lbl: 'Pagada',     color: 'sky'     },
  cancel:     { lbl: 'Cancelada',  color: 'red'     },
}

let _page = 1

export async function renderFacturas() {
  ensureLayout()
  setBreadcrumb([{ label: 'Dashboard', href: 'dashboard' }, { label: 'Facturación' }])
  _page = 1
  await loadFacturas()
}

async function loadFacturas() {
  setPage(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">Facturación</h1>
      <p class="page-subtitle" id="fact-sub">Cargando…</p>
    </div>
    <div class="page-actions">
      <input type="text" id="buscar-fact" class="search-input" placeholder="🔍 Buscar folio o cliente…" style="width:230px">
      <button class="btn btn-primary" onclick="window._go('cfdi')">🧾 Nueva Factura CFDI</button>
    </div>
  </div>

  <!-- KPIs -->
  <div style="display:grid;grid-template-columns:repeat(4,1fr);gap:12px;margin-bottom:18px" id="kpi-row" class="anim-2">
    ${[1,2,3,4].map(() => `<div class="data-card" style="padding:16px"><div class="skeleton" style="height:40px"></div></div>`).join('')}
  </div>

  <!-- Por cobrar widget -->
  <div style="display:grid;grid-template-columns:2fr 1fr;gap:16px;margin-bottom:16px" class="anim-3">
    <div class="data-card">
      <div class="data-card-header">
        <div class="data-card-title">Facturas</div>
        <select id="filtro-estado" class="search-input" style="width:150px;font-size:12px">
          <option value="">Todos los estados</option>
          <option value="posted">Publicadas</option>
          <option value="draft">Borradores</option>
          <option value="cancel">Canceladas</option>
        </select>
      </div>
      <div id="fact-tabla">${skeletonTable(8, 5)}</div>
    </div>

    <!-- Panel por cobrar -->
    <div class="data-card" style="padding:16px">
      <div style="font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.06em;color:var(--text-400);margin-bottom:12px">📋 Por Cobrar</div>
      <div id="por-cobrar-lista">${[1,2,3,4].map(() => `<div class="skeleton" style="height:38px;margin-bottom:8px;border-radius:8px"></div>`).join('')}</div>
    </div>
  </div>`)

  try {
    const [kpisRes, listRes, cobrarRes] = await Promise.allSettled([
      api.factKpis(),
      api.facturas(_page),
      api.porCobrar(),
    ])

    // ─── KPIs ────────────────────────────────────────────────────────────────
    const kpis = kpisRes.status === 'fulfilled' ? kpisRes.value?.data : null
    const kpiRow = document.getElementById('kpi-row')
    if (kpiRow) {
      kpiRow.innerHTML = [
        { label: 'Total Facturas',      val: kpis?.total_facturas  || 0, tipo: 'num', color: 'indigo',  icon: '🧾' },
        { label: 'Monto Facturado',     val: kpis?.monto_total     || 0, tipo: 'mxn', color: 'emerald', icon: '💰' },
        { label: 'Por Cobrar',          val: kpis?.por_cobrar      || 0, tipo: 'mxn', color: 'amber',   icon: '📋' },
        { label: 'Facturas Vencidas',   val: kpis?.facturas_vencidas || 0, tipo: 'num', color: 'red',   icon: '⚠️' },
      ].map(k => `
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${k.icon} ${k.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${k.tipo === 'mxn' ? fmtMxn(parseFloat(k.val)) : fmtNum(parseInt(k.val))}
        </div>
      </div>`).join('')
    }

    // ─── Tabla ───────────────────────────────────────────────────────────────
    const facturas = listRes.status === 'fulfilled' ? (listRes.value?.data || []) : []
    const hasMore = facturas.length >= 20

    const sub = document.getElementById('fact-sub')
    if (sub) sub.textContent = `${facturas.length} registros · Página ${_page}`

    const tablaEl = document.getElementById('fact-tabla')
    if (tablaEl) {
      if (facturas.length === 0) {
        tablaEl.innerHTML = '<p style="text-align:center;padding:32px;color:var(--text-400)">Sin facturas registradas</p>'
      } else {
        tablaEl.innerHTML = `
        <table class="data-table">
          <thead><tr>
            <th>Folio</th><th>Cliente</th><th>Fecha</th>
            <th>Subtotal</th><th>Total</th><th>Estado</th>
          </tr></thead>
          <tbody>
            ${facturas.map(f => {
              const e = ESTADO_MAP[f.state] || { lbl: f.state || '—', color: 'gray' }
              const fecha = f.invoice_date || f.date ? fmtDate(f.invoice_date || f.date) : '—'
              // partner_name viene del JOIN; partner_id es el fallback numérico
              const cliente = f.partner_name && isNaN(f.partner_name) ? f.partner_name : (f.customer_name || `Cliente #${f.partner_id}`)
              return `
              <tr data-estado="${f.state || ''}" style="cursor:pointer" onclick="window._verFactura(${f.id})" title="Ver detalle">
                <td class="td-mono">${f.name || `#${f.id}`}</td>
                <td class="td-primary">${cliente}</td>
                <td>${fecha}</td>
                <td class="td-amount">${fmtMxn(parseFloat(f.amount_untaxed || 0))}</td>
                <td class="td-amount" style="font-weight:700">${fmtMxn(parseFloat(f.amount_total || 0))}</td>
                <td>${stateBadge(f.state, e.lbl)}</td>
              </tr>`
            }).join('')}
          </tbody>
        </table>
        ${paginationHtml(_page, hasMore, (p) => { _page = p; loadFacturas() })}`
      }
    }

    // ─── Por cobrar ───────────────────────────────────────────────────────────
    const cobrar = cobrarRes.status === 'fulfilled' ? (cobrarRes.value?.data || []) : []
    const cobrarEl = document.getElementById('por-cobrar-lista')
    if (cobrarEl) {
      if (cobrar.length === 0) {
        cobrarEl.innerHTML = '<p style="color:var(--emerald);font-size:13px;text-align:center;padding:20px">✅ Sin saldo pendiente</p>'
      } else {
        cobrarEl.innerHTML = cobrar.slice(0, 8).map(f => {
          const vencida = f.invoice_date_due && new Date(f.invoice_date_due) < new Date()
          return `
          <div style="display:flex;justify-content:space-between;align-items:center;padding:8px 0;border-bottom:1px solid var(--border)">
            <div>
              <div style="font-size:12px;font-weight:600;color:var(--text-700)">${(f.partner_name || f.name || '—').substring(0,22)}</div>
              <div style="font-size:11px;color:${vencida ? 'var(--red)' : 'var(--text-400)'}">${vencida ? '🔴 Vencida' : '🟡 Pendiente'}</div>
            </div>
            <span class="badge badge-${vencida ? 'red' : 'amber'}">${fmtMxn(parseFloat(f.amount_residual || f.amount_total || 0))}</span>
          </div>`
        }).join('')
      }
    }

    // Filtros cliente-side
    document.getElementById('buscar-fact')?.addEventListener('input', (e) => {
      const q = e.target.value.toLowerCase()
      document.querySelectorAll('#fact-tabla tbody tr').forEach(r => {
        r.style.display = r.textContent.toLowerCase().includes(q) ? '' : 'none'
      })
    })
    document.getElementById('filtro-estado')?.addEventListener('change', (e) => {
      const val = e.target.value
      document.querySelectorAll('#fact-tabla tbody tr').forEach(r => {
        r.style.display = !val || r.dataset.estado === val ? '' : 'none'
      })
    })

    // Ver detalle factura
    window._verFactura = (id) => {
      openDetailModal(
        'Detalle de Factura',
        () => api.factura(id),
        (f) => {
          // Abrir el form de detalle con botones de acción
          setTimeout(() => verFacturaDetalle(f), 0)
          return '<div style="padding:24px;text-align:center;color:var(--text-400)">Cargando…</div>'
        }
      )
    }

  } catch (err) {
    console.error(err)
    toast('Error al cargar facturas', err.message, 'error')
  }
}
