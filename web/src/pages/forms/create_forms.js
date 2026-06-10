/**
 * create_forms.js — Formularios de creación de registros
 * Modales tipo Odoo: campo por campo, validación, POST al backend
 */
import { api } from '../../api.js'
import { toast } from '../../ui.js'

/* ─── Utilidades de modal ─── */
function _modal(id, title, bodyHtml, onSave) {
  document.getElementById('nx-modal-overlay')?.remove()
  const overlay = document.createElement('div')
  overlay.id = 'nx-modal-overlay'
  overlay.innerHTML = `
  <div class="nx-modal-backdrop" onclick="window._closeNxModal()"></div>
  <div class="nx-create-modal" role="dialog" aria-modal="true">
    <div class="nx-modal-header">
      <h2 class="nx-modal-title">${title}</h2>
      <button class="nx-modal-close" onclick="window._closeNxModal()">✕</button>
    </div>
    <div class="nx-modal-body" id="nx-modal-body-${id}">${bodyHtml}</div>
    <div class="nx-modal-footer">
      <button class="o-btn-secondary" onclick="window._closeNxModal()">Cancelar</button>
      <button class="o-btn-primary" id="nx-save-btn" onclick="window._nxSave()">Guardar</button>
    </div>
  </div>`
  document.body.appendChild(overlay)
  window._closeNxModal = () => overlay.remove()
  window._nxSave = onSave
  // Focus primer input
  setTimeout(() => overlay.querySelector('input,select,textarea')?.focus(), 100)
}

function _val(id) {
  const el = document.getElementById(id)
  if (!el) return ''
  return el.type === 'checkbox' ? el.checked : el.value.trim()
}

function _setSaving(saving) {
  const btn = document.getElementById('nx-save-btn')
  if (btn) { btn.disabled = saving; btn.textContent = saving ? '⏳ Guardando…' : 'Guardar' }
}

/* ═══════════════════════════════════════════════
   NUEVA VENTA / ORDEN DE VENTA
   ═══════════════════════════════════════════════ */
export function nuevaVenta(onSuccess) {
  _modal('venta', '➕ Nueva Orden de Venta', `
  <div class="nx-form-grid">
    <div class="nx-field">
      <label class="nx-label">Cliente <span class="nx-req">*</span></label>
      <input id="nv-partner" class="nx-input" type="text" placeholder="Nombre del cliente" required>
      <small class="nx-hint">Escribe el nombre del cliente (se buscará en el sistema)</small>
    </div>
    <div class="nx-field">
      <label class="nx-label">Fecha del Pedido</label>
      <input id="nv-fecha" class="nx-input" type="date" value="${new Date().toISOString().slice(0,10)}">
    </div>
    <div class="nx-field">
      <label class="nx-label">Referencia del Cliente</label>
      <input id="nv-ref" class="nx-input" type="text" placeholder="PO-2024-001 (opcional)">
    </div>
    <div class="nx-field">
      <label class="nx-label">Validez (días)</label>
      <input id="nv-validez" class="nx-input" type="number" value="30" min="1" max="365">
    </div>
    <div class="nx-field nx-field-full">
      <label class="nx-label">Notas / Condiciones</label>
      <textarea id="nv-notas" class="nx-input nx-textarea" rows="3" placeholder="Términos y condiciones, notas del pedido…"></textarea>
    </div>
  </div>`, async () => {
    const partner = _val('nv-partner')
    if (!partner) { toast('Requerido', 'Ingresa el nombre del cliente', 'warning'); return }
    _setSaving(true)
    try {
      // Buscar partner_id por nombre
      const pr = await api.get(`/partners?q=${encodeURIComponent(partner)}&pagina=1`)
      const partners = pr?.data || []
      if (!partners.length) {
        toast('Cliente no encontrado', `No existe cliente con nombre "${partner}". Créalo primero en Contactos.`, 'warning')
        _setSaving(false); return
      }
      const partnerId = partners[0].id
      const res = await api.post('/ventas', {
        partner_id: partnerId,
        client_order_ref: _val('nv-ref'),
        note: _val('nv-notas'),
        validity_date: _val('nv-validez')
          ? new Date(Date.now() + parseInt(_val('nv-validez')) * 86400000).toISOString().slice(0, 10)
          : null,
      })
      if (res?.ok || res?.id) {
        toast('✅', 'Venta creada', 'success')
        window._closeNxModal()
        onSuccess?.(res)
      } else {
        throw new Error(res?.error || 'Error al crear')
      }
    } catch (e) { toast('Error', e.message, 'error') }
    _setSaving(false)
  })
}

/* ═══════════════════════════════════════════════
   NUEVA FACTURA
   ═══════════════════════════════════════════════ */
export function nuevaFactura(onSuccess) {
  _modal('factura', '🧾 Nueva Factura', `
  <div class="nx-form-grid">
    <div class="nx-field">
      <label class="nx-label">Cliente <span class="nx-req">*</span></label>
      <input id="nf-partner" class="nx-input" type="text" placeholder="Nombre del cliente" required>
    </div>
    <div class="nx-field">
      <label class="nx-label">Tipo de Documento</label>
      <select id="nf-tipo" class="nx-input">
        <option value="out_invoice">Factura de Cliente</option>
        <option value="out_refund">Nota de Crédito</option>
        <option value="in_invoice">Factura de Proveedor</option>
      </select>
    </div>
    <div class="nx-field">
      <label class="nx-label">Fecha de Factura</label>
      <input id="nf-fecha" class="nx-input" type="date" value="${new Date().toISOString().slice(0, 10)}">
    </div>
    <div class="nx-field">
      <label class="nx-label">Fecha de Vencimiento</label>
      <input id="nf-vence" class="nx-input" type="date" value="${new Date(Date.now() + 30 * 86400000).toISOString().slice(0, 10)}">
    </div>
    <div class="nx-field">
      <label class="nx-label">Referencia / Número</label>
      <input id="nf-ref" class="nx-input" type="text" placeholder="INV/2024/00001">
    </div>
    <div class="nx-field">
      <label class="nx-label">Uso CFDI</label>
      <select id="nf-uso" class="nx-input">
        <option value="G03">G03 — Gastos en general</option>
        <option value="G01">G01 — Adquisición de mercancias</option>
        <option value="P01">P01 — Por definir</option>
        <option value="S01">S01 — Sin efectos fiscales</option>
      </select>
    </div>
  </div>`, async () => {
    const partner = _val('nf-partner')
    if (!partner) { toast('Requerido', 'Ingresa el cliente', 'warning'); return }
    _setSaving(true)
    try {
      const pr = await api.get(`/partners?q=${encodeURIComponent(partner)}&pagina=1`)
      const partners = pr?.data || []
      if (!partners.length) { toast('Cliente no encontrado', 'Créalo primero en Contactos', 'warning'); _setSaving(false); return }
      const res = await api.post('/facturas', {
        partner_id: partners[0].id,
        move_type: _val('nf-tipo'),
        invoice_date: _val('nf-fecha'),
        invoice_date_due: _val('nf-vence'),
        ref: _val('nf-ref'),
        l10n_mx_edi_usage: _val('nf-uso'),
      })
      if (res?.ok || res?.id) {
        toast('✅', 'Factura creada', 'success')
        window._closeNxModal()
        onSuccess?.(res)
      } else throw new Error(res?.error || 'Error')
    } catch (e) { toast('Error', e.message, 'error') }
    _setSaving(false)
  })
}

/* ═══════════════════════════════════════════════
   NUEVO PRODUCTO
   ═══════════════════════════════════════════════ */
export function nuevoProducto(onSuccess) {
  _modal('producto', '📦 Nuevo Producto', `
  <div class="nx-form-grid">
    <div class="nx-field nx-field-full">
      <label class="nx-label">Nombre <span class="nx-req">*</span></label>
      <input id="np-nombre" class="nx-input" type="text" placeholder="Nombre del producto" required>
    </div>
    <div class="nx-field">
      <label class="nx-label">Código Interno (SKU)</label>
      <input id="np-sku" class="nx-input" type="text" placeholder="PROD-001">
    </div>
    <div class="nx-field">
      <label class="nx-label">Tipo de Producto</label>
      <select id="np-tipo" class="nx-input">
        <option value="consu">Consumible</option>
        <option value="product">Almacenable</option>
        <option value="service">Servicio</option>
      </select>
    </div>
    <div class="nx-field">
      <label class="nx-label">Precio de Venta</label>
      <input id="np-precio" class="nx-input" type="number" step="0.01" value="0.00" min="0">
    </div>
    <div class="nx-field">
      <label class="nx-label">Costo</label>
      <input id="np-costo" class="nx-input" type="number" step="0.01" value="0.00" min="0">
    </div>
    <div class="nx-field">
      <label class="nx-label">Unidad de Medida</label>
      <select id="np-uom" class="nx-input">
        <option value="1">Unidad</option>
        <option value="2">Kilogramo</option>
        <option value="3">Litro</option>
        <option value="4">Metro</option>
        <option value="5">Caja</option>
        <option value="6">Paquete</option>
      </select>
    </div>
    <div class="nx-field nx-field-full">
      <label class="nx-label">Descripción</label>
      <textarea id="np-desc" class="nx-input nx-textarea" rows="2" placeholder="Descripción del producto…"></textarea>
    </div>
  </div>`, async () => {
    const nombre = _val('np-nombre')
    if (!nombre) { toast('Requerido', 'Ingresa el nombre', 'warning'); return }
    _setSaving(true)
    try {
      const res = await api.post('/productos', {
        name: nombre,
        default_code: _val('np-sku'),
        type: _val('np-tipo'),
        list_price: parseFloat(_val('np-precio')) || 0,
        standard_price: parseFloat(_val('np-costo')) || 0,
        uom_id: parseInt(_val('np-uom')) || 1,
        description_sale: _val('np-desc'),
        active: true,
      })
      if (res?.ok || res?.id) {
        toast('✅', 'Producto creado', 'success')
        window._closeNxModal()
        onSuccess?.(res)
      } else throw new Error(res?.error || 'Error')
    } catch (e) { toast('Error', e.message, 'error') }
    _setSaving(false)
  })
}

/* ═══════════════════════════════════════════════
   NUEVO CONTACTO / CLIENTE / PROVEEDOR
   ═══════════════════════════════════════════════ */
export function nuevoPartner(tipo = 'cliente', onSuccess) {
  const isCliente = tipo !== 'proveedor'
  _modal('partner', isCliente ? '👤 Nuevo Cliente' : '🏭 Nuevo Proveedor', `
  <div class="nx-form-grid">
    <div class="nx-field nx-field-full">
      <label class="nx-label">Nombre / Razón Social <span class="nx-req">*</span></label>
      <input id="npa-nombre" class="nx-input" type="text" placeholder="Empresa S.A. de C.V." required>
    </div>
    <div class="nx-field">
      <label class="nx-label">RFC</label>
      <input id="npa-rfc" class="nx-input" type="text" placeholder="XAXX010101000" maxlength="13" style="text-transform:uppercase">
    </div>
    <div class="nx-field">
      <label class="nx-label">Tipo</label>
      <select id="npa-tipo" class="nx-input">
        <option value="empresa" ${isCliente ? '' : 'selected'}>Empresa</option>
        <option value="persona">Persona Física</option>
      </select>
    </div>
    <div class="nx-field">
      <label class="nx-label">Teléfono</label>
      <input id="npa-tel" class="nx-input" type="tel" placeholder="+52 55 1234 5678">
    </div>
    <div class="nx-field">
      <label class="nx-label">Email</label>
      <input id="npa-email" class="nx-input" type="email" placeholder="contacto@empresa.com">
    </div>
    <div class="nx-field">
      <label class="nx-label">Ciudad</label>
      <input id="npa-ciudad" class="nx-input" type="text" placeholder="Ciudad de México">
    </div>
    <div class="nx-field nx-field-full">
      <label class="nx-label">Dirección</label>
      <input id="npa-dir" class="nx-input" type="text" placeholder="Calle, No., Col., CP">
    </div>
  </div>`, async () => {
    const nombre = _val('npa-nombre')
    if (!nombre) { toast('Requerido', 'Ingresa el nombre', 'warning'); return }
    _setSaving(true)
    try {
      const res = await api.post('/partners', {
        name: nombre,
        vat: _val('npa-rfc').toUpperCase(),
        phone: _val('npa-tel'),
        email: _val('npa-email'),
        city: _val('npa-ciudad'),
        street: _val('npa-dir'),
        is_company: _val('npa-tipo') === 'empresa',
        customer_rank: isCliente ? 1 : 0,
        supplier_rank: isCliente ? 0 : 1,
        active: true,
      })
      if (res?.ok || res?.id) {
        toast('✅', `${isCliente ? 'Cliente' : 'Proveedor'} creado`, 'success')
        window._closeNxModal()
        onSuccess?.(res)
      } else throw new Error(res?.error || 'Error')
    } catch (e) { toast('Error', e.message, 'error') }
    _setSaving(false)
  })
}

/* ═══════════════════════════════════════════════
   NUEVA ORDEN DE COMPRA
   ═══════════════════════════════════════════════ */
export function nuevaCompra(onSuccess) {
  _modal('compra', '🛒 Nueva Orden de Compra', `
  <div class="nx-form-grid">
    <div class="nx-field">
      <label class="nx-label">Proveedor <span class="nx-req">*</span></label>
      <input id="nc-prov" class="nx-input" type="text" placeholder="Nombre del proveedor" required>
    </div>
    <div class="nx-field">
      <label class="nx-label">Fecha de Orden</label>
      <input id="nc-fecha" class="nx-input" type="date" value="${new Date().toISOString().slice(0, 10)}">
    </div>
    <div class="nx-field">
      <label class="nx-label">Fecha de Entrega Esperada</label>
      <input id="nc-entrega" class="nx-input" type="date" value="${new Date(Date.now() + 7 * 86400000).toISOString().slice(0, 10)}">
    </div>
    <div class="nx-field">
      <label class="nx-label">Referencia del Proveedor</label>
      <input id="nc-ref" class="nx-input" type="text" placeholder="Número de cotización del proveedor">
    </div>
    <div class="nx-field nx-field-full">
      <label class="nx-label">Notas</label>
      <textarea id="nc-notas" class="nx-input nx-textarea" rows="2" placeholder="Instrucciones especiales…"></textarea>
    </div>
  </div>`, async () => {
    const prov = _val('nc-prov')
    if (!prov) { toast('Requerido', 'Ingresa el proveedor', 'warning'); return }
    _setSaving(true)
    try {
      const pr = await api.get(`/partners?q=${encodeURIComponent(prov)}&pagina=1`)
      const provs = pr?.data || []
      if (!provs.length) { toast('Proveedor no encontrado', 'Créalo primero en Contactos', 'warning'); _setSaving(false); return }
      const res = await api.post('/compras', {
        partner_id: provs[0].id,
        date_order: _val('nc-fecha'),
        date_planned: _val('nc-entrega'),
        partner_ref: _val('nc-ref'),
        notes: _val('nc-notas'),
      })
      if (res?.ok || res?.id) {
        toast('✅', 'Compra creada', 'success')
        window._closeNxModal()
        onSuccess?.(res)
      } else throw new Error(res?.error || 'Error')
    } catch (e) { toast('Error', e.message, 'error') }
    _setSaving(false)
  })
}
