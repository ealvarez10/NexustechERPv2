/**
 * Formulario modal: Nueva Venta / Nuevo Contacto / Nueva Factura
 * F1: openNuevoContacto ahora llama al endpoint real POST /partners
 */
import { openModal, closeModal, toast } from '../../ui.js'
import { api } from '../../api.js'

// ─── openNuevoContacto — guarda en BD via API real ───────────────────────────
// onSuccess(partner) se invoca con el nuevo partner { id, name } tras guardarlo
export function openNuevoContacto(onSuccess) {
  openModal('Nuevo Contacto', `
  <form id="form-nuevo-contacto" onsubmit="event.preventDefault();window._submitContacto()">
    <div class="modal-form-grid">
      <div class="modal-form-full">
        <label class="modal-form-label">Nombre completo <span style="color:#DC2626">*</span></label>
        <input id="nc-nombre" class="modal-form-input" placeholder="Empresa SA de CV" required autofocus>
      </div>
      <div>
        <label class="modal-form-label">Tipo</label>
        <select id="nc-tipo" class="modal-form-select">
          <option value="company">Empresa</option>
          <option value="person">Persona física</option>
        </select>
      </div>
      <div>
        <label class="modal-form-label">RFC</label>
        <input id="nc-rfc" class="modal-form-input" placeholder="XAXX010101000" maxlength="13">
      </div>
      <div>
        <label class="modal-form-label">Email</label>
        <input id="nc-email" type="email" class="modal-form-input" placeholder="contacto@empresa.com">
      </div>
      <div>
        <label class="modal-form-label">Teléfono</label>
        <input id="nc-tel" class="modal-form-input" placeholder="+52 81 0000 0000">
      </div>
      <div>
        <label class="modal-form-label">Ciudad</label>
        <input id="nc-ciudad" class="modal-form-input" placeholder="Monterrey">
      </div>
      <div class="modal-form-full">
        <label class="modal-form-label">Rol</label>
        <div style="display:flex;gap:12px;margin-top:4px">
          <label style="display:flex;align-items:center;gap:6px;font-size:13px">
            <input type="checkbox" id="nc-es-cliente" checked> Cliente
          </label>
          <label style="display:flex;align-items:center;gap:6px;font-size:13px">
            <input type="checkbox" id="nc-es-proveedor"> Proveedor
          </label>
        </div>
      </div>
    </div>
    <div class="modal-actions">
      <button type="button" class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cancelar</button>
      <button type="submit" class="btn btn-primary btn-sm" id="btn-guardar-contacto">💾 Guardar</button>
    </div>
    <div id="contacto-result" style="margin-top:12px"></div>
  </form>`)

  window._submitContacto = async () => {
    const btn = document.getElementById('btn-guardar-contacto')
    const nombre = document.getElementById('nc-nombre')?.value?.trim()
    if (!nombre) {
      toast('Error', 'El nombre es requerido', 'error')
      return
    }

    btn.textContent = '⏳ Guardando…'
    btn.disabled = true
    const res = document.getElementById('contacto-result')

    try {
      const payload = {
        name:             nombre,
        is_company:       document.getElementById('nc-tipo')?.value === 'company',
        vat:              document.getElementById('nc-rfc')?.value?.trim()   || null,
        email:            document.getElementById('nc-email')?.value?.trim() || null,
        phone:            document.getElementById('nc-tel')?.value?.trim()   || null,
        city:             document.getElementById('nc-ciudad')?.value?.trim()|| null,
        customer_rank:    document.getElementById('nc-es-cliente')?.checked   ? 1 : 0,
        supplier_rank:    document.getElementById('nc-es-proveedor')?.checked ? 1 : 0,
      }

      const response = await api.post('/partners', payload)
      const partner  = response?.data   // { id, name, ... }

      if (!partner?.id) throw new Error('Respuesta inválida del servidor')

      res.innerHTML = `<div style="background:#F0FDF4;border:1.5px solid #10B981;border-radius:10px;padding:12px;color:#065F46">
        ✅ Contacto <strong>${partner.name}</strong> creado exitosamente.</div>`

      toast('Contacto creado', partner.name, 'success')

      // Invocar callback con el nuevo partner para que el campo M2O se actualice
      setTimeout(() => {
        closeModal()
        if (typeof onSuccess === 'function') onSuccess(partner)
      }, 900)

    } catch (e) {
      res.innerHTML = `<p style="color:#DC2626;font-size:13px">❌ Error: ${e.message || 'No se pudo guardar el contacto'}</p>`
      btn.textContent = '💾 Guardar'
      btn.disabled = false
    }
  }
}

// ─── openNuevoContacto — usado desde el dropdown de búsqueda de clientes ─────
// La firma acepta el mismo onSuccess para rellenar el campo tras crear
export { openNuevoContacto as nuevoPartner }

// ─── openNuevaFactura — redirige al módulo CFDI ──────────────────────────────
export function openNuevaFactura() {
  openModal('Nueva Factura / CFDI', `
  <div style="text-align:center;padding:24px">
    <div style="font-size:48px;margin-bottom:12px">🔏</div>
    <div style="font-size:15px;font-weight:700;color:var(--text-800);margin-bottom:8px">
      Generar CFDI 4.0
    </div>
    <div style="font-size:13px;color:var(--text-400);margin-bottom:20px">
      Para emitir facturas electrónicas válidas, usa el módulo CFDI 4.0
    </div>
    <div style="display:flex;gap:10px;justify-content:center">
      <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
      <button class="btn btn-primary" onclick="window.__closeModal();window._go('cfdi')">
        🔏 Ir a CFDI 4.0
      </button>
    </div>
  </div>`)
}
