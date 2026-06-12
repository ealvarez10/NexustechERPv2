import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { toast } from '../ui.js'

const KEY = 'nexus_config_contactos'
const DEFAULTS = {
  // Información
  geolocalizacion: false,
  validar_vat: true,
  limite_credito: false,
  alerta_credito: false,
  // Ventas/Compras
  niveles_partner: false,
  comisiones: false
}

function cfgCheck(id, label, desc, checked) {
  return `
  <label style="display:flex;align-items:flex-start;gap:12px;cursor:pointer;padding:14px 20px;border-bottom:1px solid var(--border)">
    <input type="checkbox" id="${id}" ${checked ? 'checked' : ''}
      style="margin-top:2px;accent-color:var(--primary);width:16px;height:16px;flex-shrink:0">
    <div>
      <div style="font-weight:600;font-size:14px;color:var(--text-900)">${label}</div>
      <div style="font-size:12px;color:var(--text-400);margin-top:2px;line-height:1.5">${desc}</div>
    </div>
  </label>`
}

function section(title, content) {
  return `
  <div style="background:var(--bg-card);border:1px solid var(--border);border-radius:12px;margin-bottom:24px;overflow:hidden">
    <div style="padding:12px 20px;background:var(--bg-app);border-bottom:1px solid var(--border);
      font-size:11px;font-weight:800;text-transform:uppercase;color:var(--text-500);letter-spacing:.08em">
      ${title}
    </div>
    ${content}
  </div>`
}

export async function renderConfigContactos(params = {}) {
  ensureLayout()
  setBreadcrumb([
    { label: 'Contactos', onclick: () => window._go('partners') },
    { label: 'Configuración' }
  ])

  const cfg = { ...DEFAULTS, ...JSON.parse(localStorage.getItem(KEY) || '{}') }

  setPage(`<div class="nx-module-page" style="background:var(--bg-app)">
    <div class="o-cp">
      <div class="o-cp-left"></div>
      <div class="o-cp-right">
        <button class="o-btn-primary" onclick="window._saveConfigContact()">💾 Guardar</button>
        <button class="o-btn-secondary" onclick="window._discardConfigContact()">Descartar</button>
      </div>
    </div>

    <div style="max-width:900px;margin:28px auto;padding:0 24px 60px">

      ${section('Información del Contacto', `
        ${cfgCheck('cfgc-geo', 'Geolocalización',
          'Calcula coordenadas de longitud/latitud en base a la dirección para rutas en mapa',
          cfg.geolocalizacion)}
        ${cfgCheck('cfgc-vat', 'Validar RFC / RUT',
          'Verifica la estructura y validez fiscal del documento de identidad ante el servicio de impuestos local',
          cfg.validar_vat)}
      `)}

      ${section('Límites Financieros', `
        ${cfgCheck('cfgc-limite', 'Límite de Crédito',
          'Permite asignar un monto máximo de crédito a clientes (detiene ventas si excede)',
          cfg.limite_credito)}
        ${cfgCheck('cfgc-alerta', 'Alerta de Cartera Vencida',
          'Muestra una advertencia roja en las ventas cuando el cliente tiene facturas atrasadas',
          cfg.alerta_credito)}
      `)}

      ${section('Asociaciones y Comisiones', `
        ${cfgCheck('cfgc-niveles', 'Niveles de Partner',
          'Clasifica a clientes y distribuidores por nivel (Plata, Oro, Platino)',
          cfg.niveles_partner)}
        ${cfgCheck('cfgc-comisiones', 'Comisiones de Referidos',
          'Asigna comisiones a los partners por atraer nuevos clientes al ERP',
          cfg.comisiones)}
      `)}

      <div style="display:flex;gap:10px;padding-top:8px">
        <button class="o-btn-primary" onclick="window._saveConfigContact()">💾 Guardar Configuración</button>
        <button class="o-btn-secondary" onclick="window._discardConfigContact()">Descartar cambios</button>
      </div>
    </div>
  </div>`)

  window._saveConfigContact = () => {
    const n = (id) => document.getElementById(id)
    const cfg = {
      geolocalizacion: n('cfgc-geo')?.checked ?? false,
      validar_vat:     n('cfgc-vat')?.checked ?? true,
      limite_credito:  n('cfgc-limite')?.checked ?? false,
      alerta_credito:  n('cfgc-alerta')?.checked ?? false,
      niveles_partner: n('cfgc-niveles')?.checked ?? false,
      comisiones:      n('cfgc-comisiones')?.checked ?? false,
    }
    localStorage.setItem(KEY, JSON.stringify(cfg))
    toast('Guardado', 'Configuración de Contactos actualizada', 'success')
  }

  window._discardConfigContact = () => renderConfigContactos()
}
