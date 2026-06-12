import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { toast } from '../ui.js'

const KEY = 'nexus_config_facturacion'
const DEFAULTS = {
  // Impuestos
  impuestos_ventas: true,
  impuestos_compras: true,
  redondeo: false, // Cash rounding
  // Pagos
  pagos_online: false,
  descuentos_pronto_pago: false,
  // Facturas de clientes
  terminos_default: '',
  alertas_cliente: false,
  // Localización (CFDI)
  cfdi_auto: false,
  cancelacion_directa: false
}

// ── Helper para renderizar una opción de checkbox estilo Odoo ─────────────
function cfgCheck(id, label, desc, checked) {
  return `
  <label style="display:flex;align-items:flex-start;gap:12px;cursor:pointer;padding:14px 20px;border-bottom:1px solid var(--border)" id="row-${id}">
    <input type="checkbox" id="${id}" ${checked ? 'checked' : ''}
      style="margin-top:2px;accent-color:var(--primary);width:16px;height:16px;flex-shrink:0">
    <div>
      <div style="font-weight:600;font-size:14px;color:var(--text-900)">${label}</div>
      <div style="font-size:12px;color:var(--text-400);margin-top:2px;line-height:1.5">${desc}</div>
    </div>
  </label>`
}

// ── Helper para sección ──────────────────────────────────────────────────
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

export async function renderConfigFacturacion(params = {}) {
  ensureLayout()
  setBreadcrumb([
    { label: 'Facturación', onclick: () => window._go('facturas') },
    { label: 'Configuración' }
  ])

  const cfg = { ...DEFAULTS, ...JSON.parse(localStorage.getItem(KEY) || '{}') }

  setPage(`<div class="nx-module-page" style="background:var(--bg-app)">

    <!-- Control Panel -->
    <div class="o-cp">
      <div class="o-cp-left"></div>
      <div class="o-cp-right">
        <button class="o-btn-primary" onclick="window._saveConfigF()">💾 Guardar</button>
        <button class="o-btn-secondary" onclick="window._discardConfigF()">Descartar</button>
      </div>
    </div>

    <div style="max-width:900px;margin:28px auto;padding:0 24px 60px">

      ${section('Impuestos', `
        ${cfgCheck('cfg-tax-v', 'Impuestos de Ventas',
          'Aplica automáticamente el impuesto configurado en las facturas de venta',
          cfg.impuestos_ventas)}
        ${cfgCheck('cfg-tax-c', 'Impuestos de Compras',
          'Aplica automáticamente el impuesto configurado en las facturas de proveedores',
          cfg.impuestos_compras)}
        ${cfgCheck('cfg-redondeo', 'Redondeo de Efectivo',
          'Añade una línea de ajuste en el subtotal para redondear el total a la fracción más cercana (Ej. 0.05)',
          cfg.redondeo)}
      `)}

      ${section('Pagos de Clientes', `
        ${cfgCheck('cfg-pago-online', 'Pagos en Línea',
          'Permite a los clientes pagar sus facturas en línea usando tarjetas de crédito o pasarelas de pago',
          cfg.pagos_online)}
        ${cfgCheck('cfg-desc-pago', 'Descuentos por Pronto Pago',
          'Habilita la configuración de descuentos condicionales si el cliente paga antes de cierta fecha',
          cfg.descuentos_pronto_pago)}
      `)}

      ${section('Facturas de Clientes', `
        ${cfgCheck('cfg-alertas', 'Alertas de Cliente',
          'Muestra alertas al seleccionar un cliente en la factura (ej. Si tiene deuda pendiente)',
          cfg.alertas_cliente)}
      `)}

      ${section('Términos y Condiciones por Defecto', `
        <div style="padding:16px 20px">
          <div style="font-size:12px;color:var(--text-400);margin-bottom:10px">
            Texto predeterminado que aparecerá en el campo "Términos y condiciones" de las nuevas facturas.
          </div>
          <textarea id="cfg-terminos" rows="5"
            style="width:100%;padding:10px 14px;border:1px solid var(--border);border-radius:8px;font-size:13px;background:var(--bg-card);color:var(--text-900);resize:vertical;font-family:inherit;line-height:1.6;box-sizing:border-box"
            placeholder="Ej. El pago debe realizarse a 30 días netos. Interés moratorio del 2% mensual.">${cfg.terminos_default}</textarea>
        </div>
      `)}

      ${section('Localización (México / CFDI)', `
        ${cfgCheck('cfg-cfdi-auto', 'Timbrado Automático al Publicar',
          'El sistema enviará el CFDI al PAC automáticamente en cuanto se publique/confirme la factura',
          cfg.cfdi_auto)}
        ${cfgCheck('cfg-canc-directa', 'Cancelación Directa',
          'Permite cancelar directamente facturas en el ERP ignorando el estatus del CFDI en el SAT (solo usar si manejas la cancelación externamente)',
          cfg.cancelacion_directa)}
      `)}

      <div style="display:flex;gap:10px;padding-top:8px">
        <button class="o-btn-primary" onclick="window._saveConfigF()">💾 Guardar Configuración</button>
        <button class="o-btn-secondary" onclick="window._discardConfigF()">Descartar cambios</button>
      </div>
    </div>
  </div>`)

  window._saveConfigF = () => {
    const n = (id) => document.getElementById(id)
    const cfg = {
      impuestos_ventas:       n('cfg-tax-v')?.checked       ?? true,
      impuestos_compras:      n('cfg-tax-c')?.checked       ?? true,
      redondeo:               n('cfg-redondeo')?.checked    ?? false,
      pagos_online:           n('cfg-pago-online')?.checked ?? false,
      descuentos_pronto_pago: n('cfg-desc-pago')?.checked   ?? false,
      alertas_cliente:        n('cfg-alertas')?.checked     ?? false,
      cfdi_auto:              n('cfg-cfdi-auto')?.checked   ?? false,
      cancelacion_directa:    n('cfg-canc-directa')?.checked ?? false,
      terminos_default:       n('cfg-terminos')?.value      || ''
    }
    localStorage.setItem(KEY, JSON.stringify(cfg))
    toast('Guardado', 'Configuración de Facturación actualizada correctamente', 'success')
  }

  window._discardConfigF = () => {
    renderConfigFacturacion()
  }
}
