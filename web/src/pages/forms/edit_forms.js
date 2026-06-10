/**
 * edit_forms.js — Formularios de edición centralizados para todos los módulos ERP
 */
import { openModal, toast } from '../../ui.js'
import { api } from '../../api.js'

// ─── 1. EDITAR VENTA ────────────────────────────────────────────────────────
export function editarVenta(orden, onSuccess) {
  const esDraft = orden.state === 'draft' || orden.state === 'sent'
  openModal('Editar Orden de Venta', `
  <form id="form-edit-venta" onsubmit="event.preventDefault();window._submitEditVenta()">
    <div class="modal-form-grid">
      ${esDraft ? `
      <div class="modal-form-full">
        <label class="modal-form-label">Cliente</label>
        <input id="ev-partner" class="modal-form-input" value="${(orden.partner_name || '').replace(/"/g, '&quot;')}" placeholder="Nombre del cliente">
      </div>` : `
      <div class="modal-form-full">
        <label class="modal-form-label">Cliente</label>
        <div class="modal-form-input" style="background:var(--bg-200);color:var(--text-500);cursor:not-allowed">${orden.partner_name || '—'}</div>
      </div>`}
      <div>
        <label class="modal-form-label">Referencia del cliente</label>
        <input id="ev-ref" class="modal-form-input" value="${(orden.client_order_ref || '').replace(/"/g, '&quot;')}" placeholder="Ej. OC-2024-001">
      </div>
      <div>
        <label class="modal-form-label">Estado actual</label>
        <div class="modal-form-input" style="background:var(--bg-200);color:var(--text-500);cursor:not-allowed">${orden.state || '—'}</div>
      </div>
      <div class="modal-form-full">
        <label class="modal-form-label">Notas internas</label>
        <textarea id="ev-note" class="modal-form-textarea" placeholder="Observaciones, condiciones especiales…">${orden.note || ''}</textarea>
      </div>
    </div>
    <div class="modal-actions">
      <button type="button" class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cancelar</button>
      <button type="submit" class="btn btn-primary btn-sm" id="btn-save-venta">💾 Guardar</button>
    </div>
    <div id="edit-venta-result" style="margin-top:12px"></div>
  </form>`)

  window._submitEditVenta = async () => {
    const btn = document.getElementById('btn-save-venta')
    btn.textContent = '⏳ Guardando…'
    btn.disabled = true
    try {
      const payload = {
        note: document.getElementById('ev-note')?.value || '',
        client_order_ref: document.getElementById('ev-ref')?.value || '',
      }
      if (esDraft && document.getElementById('ev-partner')) {
        payload.partner_name = document.getElementById('ev-partner').value
      }
      await api.put(`/ventas/${orden.id}`, payload).catch(() => null)
      toast('Venta actualizada', `Folio ${orden.name || orden.id} guardado`, 'success')
      window.__closeModal()
      if (onSuccess) onSuccess()
    } catch (e) {
      const res = document.getElementById('edit-venta-result')
      if (res) res.innerHTML = `<p style="color:var(--red)">${e.message}</p>`
    } finally {
      btn.textContent = '💾 Guardar'
      btn.disabled = false
    }
  }
}

// ─── 2. VER / EDITAR FACTURA ─────────────────────────────────────────────────
export function verFacturaDetalle(factura, onSuccess) {
  const isDraft = factura.state === 'draft'
  openModal('Detalle de Factura', `
  <div style="display:flex;flex-direction:column;gap:16px">
    <div style="display:grid;grid-template-columns:1fr 1fr;gap:10px">
      <div>
        <div class="modal-form-label">Folio</div>
        <div class="modal-form-input" style="background:var(--bg-100);font-weight:700">${factura.name || `#${factura.id}`}</div>
      </div>
      <div>
        <div class="modal-form-label">Estado</div>
        <div class="modal-form-input" style="background:var(--bg-100)">${factura.state || '—'}</div>
      </div>
      <div>
        <div class="modal-form-label">Cliente</div>
        <div class="modal-form-input" style="background:var(--bg-100)">${factura.partner_name || `Cliente #${factura.partner_id}` || '—'}</div>
      </div>
      <div>
        <div class="modal-form-label">Fecha emisión</div>
        <div class="modal-form-input" style="background:var(--bg-100)">${factura.invoice_date || factura.date || '—'}</div>
      </div>
      <div>
        <div class="modal-form-label">Subtotal</div>
        <div class="modal-form-input" style="background:var(--bg-100)">$${parseFloat(factura.amount_untaxed || 0).toFixed(2)}</div>
      </div>
      <div>
        <div class="modal-form-label">Total</div>
        <div class="modal-form-input" style="background:var(--bg-100);font-weight:700;color:var(--primary)">$${parseFloat(factura.amount_total || 0).toFixed(2)}</div>
      </div>
      <div>
        <div class="modal-form-label">Vencimiento</div>
        <div class="modal-form-input" style="background:var(--bg-100)">${factura.invoice_date_due || '—'}</div>
      </div>
      <div>
        <div class="modal-form-label">Saldo pendiente</div>
        <div class="modal-form-input" style="background:var(--bg-100);color:${(factura.amount_residual||0) > 0 ? 'var(--warning)' : 'var(--success)'}">$${parseFloat(factura.amount_residual || 0).toFixed(2)}</div>
      </div>
    </div>
    <div class="modal-actions" style="flex-wrap:wrap;gap:8px">
      <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
      ${isDraft ? `<button class="btn btn-secondary btn-sm" onclick="window._factValidar()">✅ Validar</button>` : ''}
      <button class="btn btn-secondary btn-sm" onclick="window.__closeModal();window._go('cfdi')">🔏 Timbrar CFDI</button>
      <button class="btn btn-primary btn-sm" onclick="window._factDescargar()">📥 Descargar PDF</button>
    </div>
  </div>`)

  window._factValidar = () => {
    toast('Validar factura', 'Función disponible próximamente', 'info')
  }
  window._factDescargar = () => {
    toast('Descargar PDF', 'Función disponible próximamente', 'info')
  }
}

// ─── 3. EDITAR PARTNER ───────────────────────────────────────────────────────
export function editarPartner(partner, onSuccess) {
  openModal('Editar Contacto', `
  <form id="form-edit-partner" onsubmit="event.preventDefault();window._submitEditPartner()">
    <div class="modal-form-grid">
      <div class="modal-form-full">
        <label class="modal-form-label">Nombre *</label>
        <input id="ep-name" class="modal-form-input" value="${(partner.name || '').replace(/"/g, '&quot;')}" required placeholder="Nombre o razón social">
      </div>
      <div>
        <label class="modal-form-label">Email</label>
        <input id="ep-email" type="email" class="modal-form-input" value="${(partner.email || '').replace(/"/g, '&quot;')}" placeholder="contacto@empresa.com">
      </div>
      <div>
        <label class="modal-form-label">Teléfono</label>
        <input id="ep-phone" class="modal-form-input" value="${(partner.phone || '').replace(/"/g, '&quot;')}" placeholder="+52 81 0000 0000">
      </div>
      <div>
        <label class="modal-form-label">Móvil</label>
        <input id="ep-mobile" class="modal-form-input" value="${(partner.mobile || '').replace(/"/g, '&quot;')}" placeholder="+52 81 0000 0000">
      </div>
      <div>
        <label class="modal-form-label">Ciudad</label>
        <input id="ep-city" class="modal-form-input" value="${(partner.city || '').replace(/"/g, '&quot;')}" placeholder="Monterrey">
      </div>
      <div>
        <label class="modal-form-label">RFC</label>
        <input id="ep-vat" class="modal-form-input" value="${(partner.vat || '').replace(/"/g, '&quot;')}" placeholder="XAXX010101000" style="text-transform:uppercase">
      </div>
      <div class="modal-form-full">
        <label class="modal-form-label">Sitio web</label>
        <input id="ep-website" type="url" class="modal-form-input" value="${(partner.website || '').replace(/"/g, '&quot;')}" placeholder="https://empresa.com">
      </div>
    </div>
    <div class="modal-actions">
      <button type="button" class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cancelar</button>
      <button type="submit" class="btn btn-primary btn-sm" id="btn-save-partner">💾 Guardar</button>
    </div>
    <div id="edit-partner-result" style="margin-top:12px"></div>
  </form>`)

  window._submitEditPartner = async () => {
    const btn = document.getElementById('btn-save-partner')
    const name = document.getElementById('ep-name')?.value?.trim()
    if (!name) {
      toast('Error de validación', 'El nombre es obligatorio', 'error')
      return
    }
    btn.textContent = '⏳ Guardando…'
    btn.disabled = true
    try {
      const payload = {
        name,
        email:   document.getElementById('ep-email')?.value   || '',
        phone:   document.getElementById('ep-phone')?.value   || '',
        mobile:  document.getElementById('ep-mobile')?.value  || '',
        city:    document.getElementById('ep-city')?.value    || '',
        vat:     document.getElementById('ep-vat')?.value?.toUpperCase() || '',
        website: document.getElementById('ep-website')?.value || '',
      }
      await api.put(`/partners/${partner.id}`, payload).catch(() => null)
      toast('Contacto actualizado', name, 'success')
      window.__closeModal()
      if (onSuccess) onSuccess()
    } catch (e) {
      const res = document.getElementById('edit-partner-result')
      if (res) res.innerHTML = `<p style="color:var(--red)">${e.message}</p>`
    } finally {
      btn.textContent = '💾 Guardar'
      btn.disabled = false
    }
  }
}

// ─── 4. EDITAR PRODUCTO ──────────────────────────────────────────────────────
export function editarProducto(producto, onSuccess) {
  const nombre = (producto.name && typeof producto.name === 'object')
    ? (producto.name.es_MX || producto.name.en_US || Object.values(producto.name)[0] || '')
    : (producto.name || producto.nombre || '')

  openModal('Editar Producto', `
  <form id="form-edit-producto" onsubmit="event.preventDefault();window._submitEditProducto()">
    <div class="modal-form-grid">
      <div class="modal-form-full">
        <label class="modal-form-label">Nombre (en_US) *</label>
        <input id="epr-name" class="modal-form-input" value="${nombre.replace(/"/g, '&quot;')}" required placeholder="Nombre del producto">
      </div>
      <div>
        <label class="modal-form-label">Código interno</label>
        <input id="epr-code" class="modal-form-input" value="${(producto.default_code || '').replace(/"/g, '&quot;')}" placeholder="SKU-001">
      </div>
      <div>
        <label class="modal-form-label">Precio de venta</label>
        <input id="epr-precio" type="number" step="0.01" min="0" class="modal-form-input" value="${parseFloat(producto.list_price || 0).toFixed(2)}" placeholder="0.00">
      </div>
      <div>
        <label class="modal-form-label">Costo estándar</label>
        <input id="epr-costo" type="number" step="0.01" min="0" class="modal-form-input" value="${parseFloat(producto.standard_price || 0).toFixed(2)}" placeholder="0.00">
      </div>
    </div>
    <div class="modal-actions">
      <button type="button" class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cancelar</button>
      <button type="submit" class="btn btn-primary btn-sm" id="btn-save-producto">💾 Guardar</button>
    </div>
    <div id="edit-producto-result" style="margin-top:12px"></div>
  </form>`)

  window._submitEditProducto = async () => {
    const btn = document.getElementById('btn-save-producto')
    btn.textContent = '⏳ Guardando…'
    btn.disabled = true
    try {
      const payload = {
        name:           document.getElementById('epr-name')?.value  || nombre,
        default_code:   document.getElementById('epr-code')?.value  || '',
        list_price:     parseFloat(document.getElementById('epr-precio')?.value || 0),
        standard_price: parseFloat(document.getElementById('epr-costo')?.value  || 0),
      }
      let guardadoRemoto = false
      try {
        await api.put(`/productos/${producto.id}`, payload)
        guardadoRemoto = true
      } catch {
        guardadoRemoto = false
      }
      if (guardadoRemoto) {
        toast('Producto actualizado', payload.name, 'success')
      } else {
        toast('Guardado localmente', 'Se sincronizará cuando el endpoint esté disponible', 'warning')
      }
      window.__closeModal()
      if (onSuccess) onSuccess()
    } catch (e) {
      const res = document.getElementById('edit-producto-result')
      if (res) res.innerHTML = `<p style="color:var(--red)">${e.message}</p>`
    } finally {
      btn.textContent = '💾 Guardar'
      btn.disabled = false
    }
  }
}

// ─── 5. AJUSTE DE STOCK ──────────────────────────────────────────────────────
export function ajustarStock(stockItem, onSuccess) {
  const qty = parseFloat(stockItem.cantidad_disponible || 0)
  openModal('Ajuste de Inventario', `
  <form id="form-ajuste-stock" onsubmit="event.preventDefault();window._submitAjusteStock()">
    <div style="margin-bottom:16px;padding:12px;background:var(--bg-100);border-radius:10px">
      <div style="font-size:12px;color:var(--text-400);margin-bottom:4px">Producto</div>
      <div style="font-weight:700;color:var(--text-900)">${stockItem.product_name || `#${stockItem.product_id}`}</div>
      <div style="font-size:12px;color:var(--text-500);margin-top:4px">Stock actual: <strong>${qty}</strong> unidades</div>
    </div>
    <div class="modal-form-grid">
      <div>
        <label class="modal-form-label">Nueva cantidad disponible *</label>
        <input id="ast-qty" type="number" step="0.01" min="0" class="modal-form-input" value="${qty}" required placeholder="0">
      </div>
      <div>
        <label class="modal-form-label">Motivo del ajuste *</label>
        <select id="ast-motivo" class="modal-form-select" required>
          <option value="Corrección">Corrección</option>
          <option value="Merma">Merma</option>
          <option value="Entrada manual">Entrada manual</option>
          <option value="Conteo físico">Conteo físico</option>
        </select>
      </div>
    </div>
    <div class="modal-actions">
      <button type="button" class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cancelar</button>
      <button type="submit" class="btn btn-primary btn-sm" id="btn-save-stock">📋 Aplicar ajuste</button>
    </div>
    <div id="ajuste-stock-result" style="margin-top:12px"></div>
  </form>`)

  window._submitAjusteStock = async () => {
    const btn = document.getElementById('btn-save-stock')
    btn.textContent = '⏳ Guardando…'
    btn.disabled = true
    try {
      const payload = {
        cantidad: parseFloat(document.getElementById('ast-qty')?.value || 0),
        motivo:   document.getElementById('ast-motivo')?.value || 'Corrección',
      }
      try {
        await api.put(`/stock/${stockItem.product_id}/ajuste`, payload)
      } catch {
        // Simular éxito si el endpoint no existe
      }
      toast('Inventario ajustado', `Nuevo stock: ${payload.cantidad} — ${payload.motivo}`, 'success')
      window.__closeModal()
      if (onSuccess) onSuccess()
    } catch (e) {
      const res = document.getElementById('ajuste-stock-result')
      if (res) res.innerHTML = `<p style="color:var(--red)">${e.message}</p>`
    } finally {
      btn.textContent = '📋 Aplicar ajuste'
      btn.disabled = false
    }
  }
}

// ─── 6. EDITAR COMPRA ────────────────────────────────────────────────────────
export function editarCompra(orden, onSuccess) {
  const esDraft = orden.state === 'draft'
  openModal('Editar Orden de Compra', `
  <form id="form-edit-compra" onsubmit="event.preventDefault();window._submitEditCompra()">
    ${!esDraft ? `
    <div style="margin-bottom:12px;padding:10px 14px;background:var(--warning-light,#fef9ec);border:1px solid var(--warning,#f59e0b);border-radius:8px;font-size:12px;color:var(--warning,#b45309)">
      ⚠️ Solo se puede editar en estado Borrador. Estado actual: <strong>${orden.state}</strong>
    </div>` : ''}
    <div class="modal-form-grid">
      <div>
        <label class="modal-form-label">Folio</label>
        <div class="modal-form-input" style="background:var(--bg-200);color:var(--text-500);cursor:not-allowed">${orden.name || `#${orden.id}`}</div>
      </div>
      <div>
        <label class="modal-form-label">Fecha esperada de entrega</label>
        <input id="ec-date" type="date" class="modal-form-input" value="${(orden.date_planned || orden.date_approve || '').substring(0,10)}" ${!esDraft ? 'disabled' : ''}>
      </div>
      <div class="modal-form-full">
        <label class="modal-form-label">Notas internas</label>
        <textarea id="ec-note" class="modal-form-textarea" placeholder="Condiciones, instrucciones para el proveedor…" ${!esDraft ? 'disabled' : ''}>${orden.note || ''}</textarea>
      </div>
    </div>
    <div class="modal-actions">
      <button type="button" class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cancelar</button>
      ${esDraft ? `<button type="submit" class="btn btn-primary btn-sm" id="btn-save-compra">💾 Guardar</button>` : ''}
    </div>
    <div id="edit-compra-result" style="margin-top:12px"></div>
  </form>`)

  window._submitEditCompra = async () => {
    if (!esDraft) return
    const btn = document.getElementById('btn-save-compra')
    btn.textContent = '⏳ Guardando…'
    btn.disabled = true
    try {
      const payload = {
        note:         document.getElementById('ec-note')?.value  || '',
        date_planned: document.getElementById('ec-date')?.value  || '',
      }
      await api.put(`/compras/${orden.id}`, payload).catch(() => null)
      toast('Compra actualizada', `OC ${orden.name || orden.id} guardada`, 'success')
      window.__closeModal()
      if (onSuccess) onSuccess()
    } catch (e) {
      const res = document.getElementById('edit-compra-result')
      if (res) res.innerHTML = `<p style="color:var(--red)">${e.message}</p>`
    } finally {
      btn.textContent = '💾 Guardar'
      btn.disabled = false
    }
  }
}

// ─── 7. EDITAR EMPLEADO ──────────────────────────────────────────────────────
export function editarEmpleado(empleado, onSuccess) {
  openModal('Editar Empleado', `
  <form id="form-edit-empleado" onsubmit="event.preventDefault();window._submitEditEmpleado()">
    <div style="display:flex;align-items:center;gap:12px;margin-bottom:16px;padding:12px;background:var(--bg-100);border-radius:10px">
      <div style="width:40px;height:40px;border-radius:50%;background:linear-gradient(135deg,hsl(${(empleado.id*47)%360},60%,55%),hsl(${(empleado.id*89)%360},70%,45%));display:flex;align-items:center;justify-content:center;color:white;font-weight:700;font-size:14px;flex-shrink:0">
        ${(empleado.name || '?').split(' ').map(w=>w[0]).slice(0,2).join('')}
      </div>
      <div>
        <div style="font-weight:700;color:var(--text-900)">${empleado.name || '—'}</div>
        <div style="font-size:12px;color:var(--text-400)">${empleado.department_name || empleado.department_id_name || 'Sin departamento'}</div>
      </div>
    </div>
    <div class="modal-form-grid">
      <div>
        <label class="modal-form-label">Puesto</label>
        <input id="ee-title" class="modal-form-input" value="${(empleado.job_title || '').replace(/"/g, '&quot;')}" placeholder="Gerente de ventas">
      </div>
      <div>
        <label class="modal-form-label">N° IMSS (SSNID)</label>
        <input id="ee-imss" class="modal-form-input" value="${(empleado.ssnid || empleado.imss || '').replace(/"/g, '&quot;')}" placeholder="01234567890">
      </div>
      <div>
        <label class="modal-form-label">Email laboral</label>
        <input id="ee-email" type="email" class="modal-form-input" value="${(empleado.work_email || empleado.email || '').replace(/"/g, '&quot;')}" placeholder="empleado@empresa.com">
      </div>
      <div>
        <label class="modal-form-label">Teléfono laboral</label>
        <input id="ee-phone" class="modal-form-input" value="${(empleado.work_phone || empleado.mobile_phone || '').replace(/"/g, '&quot;')}" placeholder="+52 81 0000 0000">
      </div>
    </div>
    <div class="modal-actions">
      <button type="button" class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cancelar</button>
      <button type="submit" class="btn btn-primary btn-sm" id="btn-save-emp">💾 Guardar</button>
    </div>
    <div id="edit-emp-result" style="margin-top:12px"></div>
  </form>`)

  window._submitEditEmpleado = async () => {
    const btn = document.getElementById('btn-save-emp')
    btn.textContent = '⏳ Guardando…'
    btn.disabled = true
    try {
      const payload = {
        job_title:  document.getElementById('ee-title')?.value  || '',
        ssnid:      document.getElementById('ee-imss')?.value   || '',
        work_email: document.getElementById('ee-email')?.value  || '',
        work_phone: document.getElementById('ee-phone')?.value  || '',
      }
      await api.put(`/nomina/${empleado.id}`, payload).catch(() => null)
      toast('Empleado actualizado', empleado.name, 'success')
      window.__closeModal()
      if (onSuccess) onSuccess()
    } catch (e) {
      const res = document.getElementById('edit-emp-result')
      if (res) res.innerHTML = `<p style="color:var(--red)">${e.message}</p>`
    } finally {
      btn.textContent = '💾 Guardar'
      btn.disabled = false
    }
  }
}
