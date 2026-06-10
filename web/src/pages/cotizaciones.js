/**
 * cotizaciones.js — Módulo de Cotizaciones para NexusTech ERP v2
 * Gestiona cotizaciones (draft/sent), confirmación, cancelación y líneas de venta
 */
import { api } from '../api.js'
import {
  openDetailModal, detailRow, detailSection,
  openModal, toast, stateBadge, fmtMxn, fmtDate,
  paginationHtml, skeletonTable
} from '../ui.js'
import { setPage, setBreadcrumb } from '../layout.js'

// ─── Estado del módulo ─────────────────────────────────────────────────────────
let _tab = 'draft'   // 'draft' | 'confirmed' | 'nueva'
let _page = 1
let _lineasNueva = []

// ─── Entry point ──────────────────────────────────────────────────────────────
export async function renderCotizaciones() {
  setBreadcrumb([{ label: 'Principal' }, { label: 'Cotizaciones' }])

  setPage(`
    <div class="page-header">
      <div>
        <h1 class="page-title">📝 Cotizaciones</h1>
        <p class="page-subtitle">Gestión de cotizaciones y órdenes de venta</p>
      </div>
    </div>

    <!-- KPI Row -->
    <div id="cot-kpis" class="kpi-row" style="margin-bottom:24px">
      <div class="kpi-card kpi-blue">
        <div class="kpi-label">Borradores</div>
        <div class="kpi-value" id="kpi-borradores">—</div>
        <div class="kpi-sub">En proceso</div>
      </div>
      <div class="kpi-card kpi-violet">
        <div class="kpi-label">Importe Total</div>
        <div class="kpi-value" id="kpi-importe">—</div>
        <div class="kpi-sub">Cotizaciones abiertas</div>
      </div>
      <div class="kpi-card kpi-red">
        <div class="kpi-label">Vencidas</div>
        <div class="kpi-value" id="kpi-vencidas">—</div>
        <div class="kpi-sub">Requieren atención</div>
      </div>
    </div>

    <!-- Tabs -->
    <div class="tabs" style="margin-bottom:20px">
      <button class="tab-btn active" id="tab-draft"     onclick="window._cotTab('draft')">📋 Cotizaciones</button>
      <button class="tab-btn"        id="tab-confirmed" onclick="window._cotTab('confirmed')">✅ Confirmadas</button>
      <button class="tab-btn"        id="tab-nueva"     onclick="window._cotTab('nueva')">➕ Nueva Cotización</button>
    </div>

    <!-- Content area -->
    <div id="cot-content">
      ${skeletonTable(7, 5)}
    </div>
  `)

  // Register global helpers
  window._cotTab    = switchTab
  window._cotPage   = loadTab
  window._cotDetail = openCotDetail
  window._cotConfirm = confirmCot
  window._cotCancel  = cancelCot
  window._cotAddLine = addLinea
  window._cotDelLine = deleteLinea

  loadKpis()
  switchTab('draft')
}

// ─── KPIs ─────────────────────────────────────────────────────────────────────
async function loadKpis() {
  try {
    const res = await api.cotizacionKpis()
    const d = res?.data ?? res
    if (!d) return
    document.getElementById('kpi-borradores').textContent = d.total_borradores ?? '—'
    document.getElementById('kpi-importe').textContent = fmtMxn(d.importe_total)
    document.getElementById('kpi-vencidas').textContent = d.vencidas ?? '0'
  } catch(e) { /* silencioso */ }
}

// ─── Tab switcher ─────────────────────────────────────────────────────────────
function switchTab(tab) {
  _tab = tab
  _page = 1
  document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'))
  const el = document.getElementById('tab-' + tab)
  if (el) el.classList.add('active')
  loadTab(1)
}

async function loadTab(page = 1) {
  _page = page
  const content = document.getElementById('cot-content')
  if (!content) return

  if (_tab === 'nueva') {
    renderFormNueva()
    return
  }

  content.innerHTML = skeletonTable(7, 8)
  try {
    let res
    if (_tab === 'draft') {
      res = await api.cotizaciones(page)
    } else {
      // Confirmadas: usamos ventas (sale/done) — reutilizamos endpoint ventas con filtro
      res = await api.ventas(page)
    }
    const d = res?.data ?? []
    const total = res?.total ?? d.length
    const pp = res?.por_pagina ?? 20
    const hasMore = page * pp < total

    const stateLabel = { draft:'Borrador', sent:'Enviada', sale:'Confirmada', done:'Realizada', cancel:'Cancelada' }

    if (!d.length) {
      content.innerHTML = `<div style="text-align:center;padding:48px;color:var(--text-400)">
        <div style="font-size:48px;margin-bottom:12px">📋</div>
        <p>No hay cotizaciones en esta sección</p>
      </div>`
      return
    }

    content.innerHTML = `
      <div class="table-container">
        <table class="data-table">
          <thead><tr>
            <th>#</th><th>Referencia</th><th>Cliente</th><th>Estado</th>
            <th>Subtotal</th><th>IVA</th><th>Total</th><th>Fecha</th><th>Validez</th><th></th>
          </tr></thead>
          <tbody>
            ${d.map(o => {
              const sLabel = stateLabel[o.state] || o.state
              return `<tr style="cursor:pointer" onclick="window._cotDetail(${o.id})">
                <td style="font-size:11px;color:var(--text-400)">${o.id}</td>
                <td style="font-weight:600;color:var(--primary)">${o.name || '—'}</td>
                <td>${o.partner_name || o.partner_id || '—'}</td>
                <td>${stateBadge(o.state, sLabel)}</td>
                <td>${fmtMxn(o.amount_untaxed)}</td>
                <td>${fmtMxn(o.amount_tax)}</td>
                <td style="font-weight:600">${fmtMxn(o.amount_total)}</td>
                <td style="font-size:12px;color:var(--text-400)">${fmtDate(o.date_order)}</td>
                <td style="font-size:12px;color:var(--text-400)">${fmtDate(o.validity_date)}</td>
                <td onclick="event.stopPropagation()">
                  <button class="btn btn-secondary btn-sm" onclick="window._cotDetail(${o.id})">Ver</button>
                </td>
              </tr>`
            }).join('')}
          </tbody>
        </table>
        ${paginationHtml(page, hasMore, window._cotPage)}
      </div>
    `
  } catch(e) {
    content.innerHTML = `<div class="empty-state"><p style="color:var(--red)">Error cargando cotizaciones: ${e.message}</p></div>`
  }
}

// ─── Detalle de cotización ────────────────────────────────────────────────────
function openCotDetail(id) {
  openDetailModal(`Cotización #${id}`, () => api.cotizacion(id), renderDetalle)
}

function renderDetalle(data) {
  const o = data?.orden ?? data
  const lineas = data?.lineas ?? []
  const stateLabel = { draft:'Borrador', sent:'Enviada', sale:'Confirmada', done:'Realizada', cancel:'Cancelada' }
  const canEdit = ['draft','sent'].includes(o.state)
  const canConfirm = canEdit
  const canCancel = !['cancel','done'].includes(o.state)

  const lineasHtml = lineas.length
    ? `<div class="table-container" style="margin-top:12px">
        <table class="data-table" style="font-size:12px">
          <thead><tr><th>Producto</th><th>Cant.</th><th>Precio U.</th><th>Dto%</th><th>Subtotal</th><th></th></tr></thead>
          <tbody>
            ${lineas.map(l => `<tr>
              <td>${l.name || '—'}</td>
              <td>${l.product_uom_qty}</td>
              <td>${fmtMxn(l.price_unit)}</td>
              <td>${l.discount ? l.discount + '%' : '—'}</td>
              <td style="font-weight:600">${fmtMxn(l.price_subtotal)}</td>
              <td>${canEdit ? `<button class="btn btn-secondary btn-sm" style="color:var(--red)" onclick="window._cotDelLine(${o.id},${l.id})">✕</button>` : ''}</td>
            </tr>`).join('')}
          </tbody>
        </table>
      </div>`
    : `<p style="color:var(--text-400);font-size:13px;padding:8px 0">Sin líneas de venta</p>`

  const addLineForm = canEdit ? `
    <div style="margin-top:16px;padding:16px;background:var(--surface-2);border-radius:10px;border:1px solid var(--border)">
      <div style="font-weight:600;margin-bottom:12px;font-size:13px">➕ Agregar línea</div>
      <div style="display:grid;grid-template-columns:2fr 1fr 1fr 1fr;gap:8px;margin-bottom:8px">
        <input id="linea-name" class="form-control" placeholder="Descripción" style="font-size:13px">
        <input id="linea-qty"  class="form-control" type="number" placeholder="Cantidad" value="1" min="0.01" step="0.01" style="font-size:13px">
        <input id="linea-price" class="form-control" type="number" placeholder="Precio" min="0" step="0.01" style="font-size:13px">
        <input id="linea-dto" class="form-control" type="number" placeholder="Dto %" min="0" max="100" step="0.01" style="font-size:13px">
      </div>
      <button class="btn btn-primary btn-sm" onclick="window._cotAddLine(${o.id})">Agregar línea</button>
    </div>` : ''

  const actionBtns = `
    <div style="display:flex;gap:8px;margin-top:20px;flex-wrap:wrap">
      ${canConfirm ? `<button class="btn btn-primary" onclick="window._cotConfirm(${o.id})">✅ Confirmar pedido</button>` : ''}
      ${canCancel  ? `<button class="btn btn-secondary" style="color:var(--red)" onclick="window._cotCancel(${o.id})">🚫 Cancelar</button>` : ''}
    </div>`

  return `
    ${detailSection('Información General', `
      ${detailRow('Referencia', o.name)}
      ${detailRow('Estado', stateBadge(o.state, stateLabel[o.state] || o.state))}
      ${detailRow('Cliente', o.partner_name || '—')}
      ${detailRow('Referencia cliente', o.client_order_ref || '—')}
      ${detailRow('Fecha', fmtDate(o.date_order))}
      ${detailRow('Validez', fmtDate(o.validity_date))}
      ${detailRow('Estado factura', o.invoice_status || '—')}
    `)}
    ${detailSection('Importes', `
      ${detailRow('Subtotal', fmtMxn(o.amount_untaxed))}
      ${detailRow('IVA', fmtMxn(o.amount_tax))}
      ${detailRow('Total', `<strong style="font-size:16px;color:var(--primary)">${fmtMxn(o.amount_total)}</strong>`)}
    `)}
    ${detailSection('Líneas de venta', lineasHtml + addLineForm)}
    ${o.note ? detailSection('Notas', `<p style="font-size:13px;line-height:1.6">${o.note}</p>`) : ''}
    ${actionBtns}
  `
}

// ─── Acciones sobre cotizaciones ──────────────────────────────────────────────
async function confirmCot(id) {
  if (!confirm('¿Confirmar esta cotización? Pasará a pedido de venta.')) return
  try {
    await api.confirmarCotizacion(id)
    toast('Cotización confirmada', 'El pedido fue confirmado correctamente', 'success')
    window.__closeModal()
    loadKpis()
    loadTab(_page)
  } catch(e) {
    toast('Error', e.message, 'error')
  }
}

async function cancelCot(id) {
  if (!confirm('¿Cancelar esta cotización?')) return
  try {
    await api.cancelarCotizacion(id)
    toast('Cotización cancelada', '', 'info')
    window.__closeModal()
    loadKpis()
    loadTab(_page)
  } catch(e) {
    toast('Error', e.message, 'error')
  }
}

async function addLinea(orderId) {
  const name  = document.getElementById('linea-name')?.value?.trim()
  const qty   = parseFloat(document.getElementById('linea-qty')?.value || '1')
  const price = parseFloat(document.getElementById('linea-price')?.value || '0')
  const dto   = parseFloat(document.getElementById('linea-dto')?.value || '0') || null

  if (!name)  return toast('Campo requerido', 'Escribe una descripción de producto', 'warning')
  if (!price) return toast('Campo requerido', 'Ingresa el precio unitario', 'warning')

  try {
    await api.agregarLinea(orderId, {
      name,
      product_uom_qty: qty,
      price_unit: price,
      discount: dto
    })
    toast('Línea agregada', '', 'success')
    openCotDetail(orderId) // refrescar detalle
  } catch(e) {
    toast('Error al agregar línea', e.message, 'error')
  }
}

async function deleteLinea(orderId, lineaId) {
  if (!confirm('¿Eliminar esta línea?')) return
  try {
    await api.eliminarLinea(orderId, lineaId)
    toast('Línea eliminada', '', 'success')
    openCotDetail(orderId)
  } catch(e) {
    toast('Error', e.message, 'error')
  }
}

// ─── Formulario nueva cotización ──────────────────────────────────────────────
function renderFormNueva() {
  _lineasNueva = []
  const content = document.getElementById('cot-content')
  if (!content) return

  content.innerHTML = `
    <div style="max-width:800px;margin:0 auto">
      <div class="card" style="padding:28px">
        <h2 style="font-size:18px;font-weight:700;margin-bottom:24px;color:var(--text)">Nueva Cotización</h2>

        <div style="display:grid;grid-template-columns:1fr 1fr;gap:16px;margin-bottom:16px">
          <div class="form-group">
            <label class="form-label">Cliente (nombre exacto) *</label>
            <input id="nv-partner" class="form-control" placeholder="Nombre del cliente" autocomplete="off">
          </div>
          <div class="form-group">
            <label class="form-label">Referencia del cliente</label>
            <input id="nv-ref" class="form-control" placeholder="Ej: OC-2024-001">
          </div>
          <div class="form-group">
            <label class="form-label">Fecha de validez</label>
            <input id="nv-validez" class="form-control" type="date">
          </div>
          <div class="form-group">
            <label class="form-label">Notas internas</label>
            <input id="nv-nota" class="form-control" placeholder="Observaciones opcionales">
          </div>
        </div>

        <!-- Sección de líneas -->
        <div style="margin-top:24px">
          <div style="font-weight:700;font-size:14px;margin-bottom:12px;display:flex;align-items:center;justify-content:space-between">
            <span>📦 Líneas de venta</span>
            <button class="btn btn-secondary btn-sm" onclick="window._nvAddRow()">+ Agregar producto</button>
          </div>
          <div id="nv-lineas-list">
            <p style="color:var(--text-400);font-size:13px;padding:16px 0;text-align:center">
              Sin líneas. Agrega productos para calcular el total.
            </p>
          </div>
        </div>

        <!-- Totales -->
        <div style="margin-top:20px;padding:16px;background:var(--surface-2);border-radius:10px;border:1px solid var(--border)">
          <div style="display:flex;justify-content:space-between;padding:6px 0;font-size:13px">
            <span style="color:var(--text-400)">Subtotal</span>
            <strong id="nv-subtotal">$0.00</strong>
          </div>
          <div style="display:flex;justify-content:space-between;padding:6px 0;font-size:13px">
            <span style="color:var(--text-400)">IVA (16%)</span>
            <strong id="nv-iva">$0.00</strong>
          </div>
          <div style="display:flex;justify-content:space-between;padding:10px 0 6px;border-top:2px solid var(--border);margin-top:4px">
            <span style="font-weight:700">Total</span>
            <strong id="nv-total" style="font-size:18px;color:var(--primary)">$0.00</strong>
          </div>
        </div>

        <div style="display:flex;gap:12px;margin-top:24px">
          <button class="btn btn-primary" onclick="window._nvGuardar()">💾 Guardar cotización</button>
          <button class="btn btn-secondary" onclick="window._cotTab('draft')">Cancelar</button>
        </div>
      </div>
    </div>
  `
  window._nvAddRow  = addNuevaRow
  window._nvDelRow  = delNuevaRow
  window._nvGuardar = guardarNueva
  window._nvRecalc  = recalcTotales
}

function addNuevaRow() {
  const idx = _lineasNueva.length
  _lineasNueva.push({ name: '', qty: 1, price: 0, discount: 0 })
  renderNuevaLineas()
}

function delNuevaRow(idx) {
  _lineasNueva.splice(idx, 1)
  renderNuevaLineas()
}

function renderNuevaLineas() {
  const container = document.getElementById('nv-lineas-list')
  if (!container) return
  if (!_lineasNueva.length) {
    container.innerHTML = `<p style="color:var(--text-400);font-size:13px;padding:16px 0;text-align:center">Sin líneas.</p>`
    recalcTotales()
    return
  }
  container.innerHTML = `
    <div class="table-container">
      <table class="data-table" style="font-size:13px">
        <thead><tr>
          <th style="width:40%">Descripción *</th>
          <th>Cant.</th>
          <th>Precio U.</th>
          <th>Dto %</th>
          <th>Subtotal</th>
          <th></th>
        </tr></thead>
        <tbody>
          ${_lineasNueva.map((l, i) => {
            const dto = parseFloat(l.discount) || 0
            const sub = (parseFloat(l.qty)||0) * (parseFloat(l.price)||0) * (1 - dto/100)
            return `<tr>
              <td><input class="form-control" style="font-size:12px" value="${l.name}" oninput="_lineasNueva[${i}].name=this.value" placeholder="Descripción del producto"></td>
              <td><input class="form-control" style="font-size:12px;width:70px" type="number" min="0.01" step="0.01" value="${l.qty}" oninput="_lineasNueva[${i}].qty=this.value;window._nvRecalc()"></td>
              <td><input class="form-control" style="font-size:12px;width:90px" type="number" min="0" step="0.01" value="${l.price}" oninput="_lineasNueva[${i}].price=this.value;window._nvRecalc()"></td>
              <td><input class="form-control" style="font-size:12px;width:65px" type="number" min="0" max="100" step="0.01" value="${l.discount}" oninput="_lineasNueva[${i}].discount=this.value;window._nvRecalc()"></td>
              <td style="font-weight:600">${fmtMxn(sub)}</td>
              <td><button class="btn btn-secondary btn-sm" style="color:var(--red)" onclick="window._nvDelRow(${i})">✕</button></td>
            </tr>`
          }).join('')}
        </tbody>
      </table>
    </div>`
  recalcTotales()
}

function recalcTotales() {
  let subtotal = 0
  _lineasNueva.forEach(l => {
    const dto = parseFloat(l.discount) || 0
    subtotal += (parseFloat(l.qty)||0) * (parseFloat(l.price)||0) * (1 - dto/100)
  })
  const iva = subtotal * 0.16
  const total = subtotal + iva
  const fmt2 = n => n.toLocaleString('es-MX', { minimumFractionDigits:2, maximumFractionDigits:2 })
  const s = document.getElementById('nv-subtotal')
  const iv = document.getElementById('nv-iva')
  const t = document.getElementById('nv-total')
  if (s) s.textContent = '$' + fmt2(subtotal)
  if (iv) iv.textContent = '$' + fmt2(iva)
  if (t) t.textContent = '$' + fmt2(total)
}

async function guardarNueva() {
  const partnerNombre = document.getElementById('nv-partner')?.value?.trim()
  const ref          = document.getElementById('nv-ref')?.value?.trim() || null
  const validez      = document.getElementById('nv-validez')?.value || null
  const nota         = document.getElementById('nv-nota')?.value?.trim() || null

  if (!partnerNombre) return toast('Campo requerido', 'Ingresa el nombre del cliente', 'warning')

  // Buscar partner_id por nombre
  let partnerId = 1
  try {
    // Usamos la API existente de partners para buscar por nombre
    const res = await api.get(`/partners?pagina=1&q=${encodeURIComponent(partnerNombre)}&por_pagina=5`)
    const lista = res?.data ?? []
    const match = lista.find(p => p.name?.toLowerCase() === partnerNombre.toLowerCase())
    if (match) {
      partnerId = match.id
    } else if (lista.length > 0) {
      partnerId = lista[0].id
    } else {
      return toast('Cliente no encontrado', `No se encontró "${partnerNombre}"`, 'warning')
    }
  } catch(e) {
    return toast('Error', 'No se pudo buscar el cliente: ' + e.message, 'error')
  }

  const body = {
    partner_id:          partnerId,
    partner_invoice_id:  partnerId,
    partner_shipping_id: partnerId,
    note:                nota,
    client_order_ref:    ref,
    validity_date:       validez || null,
  }

  try {
    const res = await api.crearCotizacion(body)
    const newId = res?.data?.id ?? res?.id
    toast('Cotización creada', `ID ${newId} — Referencia generada`, 'success')

    // Agregar líneas si las hay
    if (newId && _lineasNueva.length) {
      for (const l of _lineasNueva) {
        if (!l.name) continue
        await api.agregarLinea(newId, {
          name: l.name,
          product_uom_qty: parseFloat(l.qty) || 1,
          price_unit: parseFloat(l.price) || 0,
          discount: parseFloat(l.discount) || null,
        }).catch(() => {})
      }
    }

    _lineasNueva = []
    loadKpis()
    switchTab('draft')
    setTimeout(() => newId && openCotDetail(newId), 600)
  } catch(e) {
    toast('Error al crear cotización', e.message, 'error')
  }
}
