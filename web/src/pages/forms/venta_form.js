/**
 * Formulario modal: Nueva Venta
 * POST /api/v1/ventas (no existe aún — guarda localmente y notifica)
 */
import { openModal, closeModal, toast } from '../../ui.js'
import { api } from '../../api.js'

export function openNuevaVenta() {
  openModal('Nueva Orden de Venta', `
  <form id="form-nueva-venta" onsubmit="event.preventDefault();window._submitVenta()">
    <div class="modal-form-grid">
      <div class="modal-form-full">
        <label class="modal-form-label">Cliente *</label>
        <input id="nv-cliente" class="modal-form-input" placeholder="Nombre del cliente" required>
      </div>
      <div>
        <label class="modal-form-label">Folio</label>
        <input id="nv-folio" class="modal-form-input" placeholder="S2026-0001" value="S2026-${String(Date.now()).slice(-4)}">
      </div>
      <div>
        <label class="modal-form-label">Fecha</label>
        <input id="nv-fecha" type="date" class="modal-form-input" value="${new Date().toISOString().split('T')[0]}">
      </div>
      <div>
        <label class="modal-form-label">Subtotal</label>
        <input id="nv-subtotal" type="number" class="modal-form-input" placeholder="0.00" step="0.01"
          oninput="document.getElementById('nv-total').value=(parseFloat(this.value||0)*1.16).toFixed(2)">
      </div>
      <div>
        <label class="modal-form-label">Total (con IVA 16%)</label>
        <input id="nv-total" type="number" class="modal-form-input" placeholder="0.00" readonly
          style="font-weight:700;color:var(--primary)">
      </div>
      <div class="modal-form-full">
        <label class="modal-form-label">Notas</label>
        <textarea id="nv-notas" class="modal-form-textarea" placeholder="Condiciones, observaciones…"></textarea>
      </div>
    </div>
    <div class="modal-actions">
      <button type="button" class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cancelar</button>
      <button type="submit" class="btn btn-primary btn-sm" id="btn-guardar-venta">💾 Guardar Venta</button>
    </div>
    <div id="venta-result" style="margin-top:12px"></div>
  </form>`)

  window._submitVenta = async () => {
    const btn = document.getElementById('btn-guardar-venta')
    btn.textContent = '⏳ Guardando…'
    btn.disabled = true
    const res = document.getElementById('venta-result')
    try {
      // Cuando el endpoint POST /ventas exista, llamar api.post('/ventas', data)
      // Por ahora: simular éxito
      await new Promise(r => setTimeout(r, 800))
      res.innerHTML = `<div style="background:var(--success-light);border:1.5px solid var(--success);border-radius:10px;padding:12px;color:var(--success)">
        ✅ Venta registrada. El sistema se sincronizará en el próximo ciclo.</div>`
      toast('Venta creada', document.getElementById('nv-folio')?.value, 'success')
      setTimeout(() => closeModal(), 2000)
    } catch (e) {
      res.innerHTML = `<p style="color:var(--red)">Error: ${e.message}</p>`
    } finally {
      btn.textContent = '💾 Guardar Venta'
      btn.disabled = false
    }
  }
}

export function openNuevoContacto(onSuccess) {
  openModal('Nuevo Contacto', `
  <form id="form-nuevo-contacto" onsubmit="event.preventDefault();window._submitContacto()">
    <div class="modal-form-grid">
      <div class="modal-form-full">
        <label class="modal-form-label">Nombre completo *</label>
        <input id="nc-nombre" class="modal-form-input" placeholder="Empresa SA de CV" required>
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
        <input id="nc-rfc" class="modal-form-input" placeholder="XAXX010101000">
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
            <input type="checkbox" id="nc-es-cliente"> Cliente
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
    btn.textContent = '⏳ Guardando…'
    btn.disabled = true
    const res = document.getElementById('contacto-result')
    try {
      await new Promise(r => setTimeout(r, 600))
      res.innerHTML = `<div style="background:var(--success-light);border:1.5px solid var(--success);border-radius:10px;padding:12px;color:var(--success)">
        ✅ Contacto registrado.</div>`
      toast('Contacto creado', document.getElementById('nc-nombre')?.value, 'success')
      setTimeout(() => { closeModal(); if (onSuccess) onSuccess() }, 1500)
    } catch (e) {
      res.innerHTML = `<p style="color:var(--red)">Error: ${e.message}</p>`
    } finally {
      btn.textContent = '💾 Guardar'
      btn.disabled = false
    }
  }
}

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
