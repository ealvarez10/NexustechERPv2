import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { toast } from '../ui.js'

const KEY = 'nexus_config_compras'
const DEFAULTS = {
  // Pedidos
  bloquear_confirmado: false,
  advertencias: false,
  precio_compra: true,
  descuentos: false,
  // Facturación
  politica_facturacion: 'cantidad_pedida',
  bloquear_factura: false,
  // Productos
  variantes: false,
  unidades_medida: false,
  empaquetado: false,
  // Avanzado
  presupuesto_solicitud: false,
  recordatorio_recepcion: 0,
  // Envío
  costos_aterrizaje: false,
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

export async function renderConfigCompras(params = {}) {
  ensureLayout()
  setBreadcrumb([
    { label: 'Compras', onclick: () => window._go('compras') },
    { label: 'Configuración' }
  ])

  const cfg = { ...DEFAULTS, ...JSON.parse(localStorage.getItem(KEY) || '{}') }

  setPage(`<div class="nx-module-page" style="background:var(--bg-app)">
    <div class="o-cp">
      <div class="o-cp-left"></div>
      <div class="o-cp-right">
        <button class="o-btn-primary" onclick="window._saveConfigC()">💾 Guardar</button>
        <button class="o-btn-secondary" onclick="window._discardConfigC()">Descartar</button>
      </div>
    </div>

    <div style="max-width:900px;margin:28px auto;padding:0 24px 60px">

      ${section('Pedidos', `
        ${cfgCheck('cfc-bloq', 'Bloquear Pedido Confirmado',
          'Impide editar una orden de compra después de confirmarla. Se debe crear una nueva orden para modificar',
          cfg.bloquear_confirmado)}
        ${cfgCheck('cfc-warn', 'Advertencias',
          'Muestra advertencias al comprador al crear pedidos para proveedores o productos con observaciones especiales',
          cfg.advertencias)}
        ${cfgCheck('cfc-precio', 'Precio de Compra',
          'Muestra el precio de compra del proveedor en las líneas de pedido para comparación con el precio de catálogo',
          cfg.precio_compra)}
        ${cfgCheck('cfc-desc', 'Descuentos',
          'Permite aplicar descuentos por línea en las órdenes de compra',
          cfg.descuentos)}
      `)}

      ${section('Facturación', `
        <div style="display:flex;align-items:center;justify-content:space-between;padding:14px 20px;border-bottom:1px solid var(--border)">
          <div>
            <div style="font-weight:600;font-size:14px;color:var(--text-900)">Política de Control de Facturas</div>
            <div style="font-size:12px;color:var(--text-400);margin-top:2px">Define si se puede facturar al recibir el pedido o después de validar la recepción</div>
          </div>
          <select id="cfc-pol" style="min-width:230px;padding:7px 12px;border:1px solid var(--border);border-radius:8px;font-size:13px;background:var(--bg-card);color:var(--text-900)">
            <option value="cantidad_pedida" ${cfg.politica_facturacion === 'cantidad_pedida' ? 'selected' : ''}>Cantidades pedidas</option>
            <option value="cantidad_recibida" ${cfg.politica_facturacion === 'cantidad_recibida' ? 'selected' : ''}>Cantidades recibidas</option>
          </select>
        </div>
        ${cfgCheck('cfc-bloq-fact', 'Bloquear Facturas',
          'Impide modificar facturas de proveedor después de validarlas. Requiere una nota de crédito para correcciones',
          cfg.bloquear_factura)}
      `)}

      ${section('Productos', `
        ${cfgCheck('cfc-var', 'Variantes',
          'Habilita variantes de producto (talla, color, material) agrupadas bajo una misma referencia',
          cfg.variantes)}
        ${cfgCheck('cfc-udm', 'Unidades de Medida',
          'Permite comprar en una unidad diferente a la unidad de stock, con conversión automática',
          cfg.unidades_medida)}
        ${cfgCheck('cfc-pack', 'Empaquetado',
          'Define presentaciones de empaque del proveedor (caja de 12, pallet de 100, etc.)',
          cfg.empaquetado)}
      `)}

      ${section('Avanzado', `
        ${cfgCheck('cfc-rfq', 'Solicitud de Presupuesto a Proveedores',
          'Envía solicitudes de cotización a múltiples proveedores para comparar precios antes de confirmar la compra',
          cfg.presupuesto_solicitud)}
        <div style="display:flex;align-items:center;justify-content:space-between;padding:14px 20px;border-bottom:1px solid var(--border)">
          <div>
            <div style="font-weight:600;font-size:14px;color:var(--text-900)">Recordatorio de Recepción</div>
            <div style="font-size:12px;color:var(--text-400);margin-top:2px">Envía un recordatorio N días antes de la fecha de entrega esperada. 0 = desactivado</div>
          </div>
          <div style="display:flex;align-items:center;gap:6px">
            <input type="number" id="cfc-rec" value="${cfg.recordatorio_recepcion}" min="0" max="30"
              style="width:70px;text-align:center;padding:6px 10px;border:1px solid var(--border);border-radius:8px;font-size:14px">
            <span style="font-size:13px;color:var(--text-400)">días</span>
          </div>
        </div>
      `)}

      ${section('Costos de Aterrizaje', `
        ${cfgCheck('cfc-land', 'Costos de Aterrizaje',
          'Permite distribuir costos adicionales de importación (flete, aduanas, seguros) entre los productos recibidos',
          cfg.costos_aterrizaje)}
      `)}

      <div style="display:flex;gap:10px;padding-top:8px">
        <button class="o-btn-primary" onclick="window._saveConfigC()">💾 Guardar Configuración</button>
        <button class="o-btn-secondary" onclick="window._discardConfigC()">Descartar cambios</button>
      </div>
    </div>
  </div>`)

  window._saveConfigC = () => {
    const n = (id) => document.getElementById(id)
    const cfg = {
      bloquear_confirmado:   n('cfc-bloq')?.checked      ?? false,
      advertencias:          n('cfc-warn')?.checked      ?? false,
      precio_compra:         n('cfc-precio')?.checked    ?? true,
      descuentos:            n('cfc-desc')?.checked      ?? false,
      politica_facturacion:  n('cfc-pol')?.value         || 'cantidad_pedida',
      bloquear_factura:      n('cfc-bloq-fact')?.checked ?? false,
      variantes:             n('cfc-var')?.checked       ?? false,
      unidades_medida:       n('cfc-udm')?.checked       ?? false,
      empaquetado:           n('cfc-pack')?.checked      ?? false,
      presupuesto_solicitud: n('cfc-rfq')?.checked       ?? false,
      recordatorio_recepcion: parseInt(n('cfc-rec')?.value) || 0,
      costos_aterrizaje:     n('cfc-land')?.checked      ?? false,
    }
    localStorage.setItem(KEY, JSON.stringify(cfg))
    toast('Guardado', 'Configuración de Compras actualizada correctamente', 'success')
  }

  window._discardConfigC = () => renderConfigCompras()
}
