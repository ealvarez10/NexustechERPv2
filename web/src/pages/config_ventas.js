import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { toast } from '../ui.js'

const KEY = 'nexus_config_ventas'
const DEFAULTS = {
  // Catálogo de Productos
  variantes: false,
  unidades_medida: false,
  empaquetado: false,
  // Precios
  descuentos: true,
  listas_precios: false,
  descuento_precio: false,
  margenes: false,
  // Presupuestos y Pedidos
  firma_online: false,
  pago_online: false,
  validez_cotizacion: 30,
  bloquear_confirmado: true,
  advertencias: false,
  plantillas_presupuesto: false,
  compra_online: false,
  notas_cierre: false,
  // Envío
  costos_envio: false,
  fecha_entrega: false,
  aviso_stock: false,
  // Facturación
  politica_facturacion: 'cantidad_pedida',
  // Términos
  terminos: ''
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

export async function renderConfigVentas(params = {}) {
  ensureLayout()
  setBreadcrumb([
    { label: 'Ventas', onclick: () => window._go('ventas') },
    { label: 'Configuración' }
  ])

  const cfg = { ...DEFAULTS, ...JSON.parse(localStorage.getItem(KEY) || '{}') }

  setPage(`<div class="nx-module-page" style="background:var(--bg-app)">

    <!-- Control Panel -->
    <div class="o-cp">
      <div class="o-cp-left"></div>
      <div class="o-cp-right">
        <button class="o-btn-primary" onclick="window._saveConfigV()">💾 Guardar</button>
        <button class="o-btn-secondary" onclick="window._discardConfigV()">Descartar</button>
      </div>
    </div>

    <div style="max-width:900px;margin:28px auto;padding:0 24px 60px">

      ${section('Catálogo de Productos', `
        ${cfgCheck('cfg-var', 'Variantes',
          'Permite crear variantes de producto (talla, color, etc.) desde una sola ficha de producto',
          cfg.variantes)}
        ${cfgCheck('cfg-udm', 'Unidades de Medida',
          'Habilita múltiples unidades de medida y conversiones automáticas entre ellas. Muestra la columna UdM en las líneas de pedido',
          cfg.unidades_medida)}
        ${cfgCheck('cfg-pack', 'Empaquetado de Producto',
          'Define distintas presentaciones de empaque (caja x12, paquete x6, etc.) para los productos',
          cfg.empaquetado)}
      `)}

      ${section('Precios', `
        ${cfgCheck('cfg-desc', 'Descuentos',
          'Permite aplicar descuentos por línea en las órdenes de venta. Muestra la columna Descuento en las líneas de pedido',
          cfg.descuentos)}
        ${cfgCheck('cfg-pricelist', 'Listas de Precios',
          'Habilita múltiples listas de precios para asignar tarifas personalizadas a clientes, grupos o canales de venta',
          cfg.listas_precios)}
        <div id="row-pricelist-link" style="display:${cfg.listas_precios ? '' : 'none'};padding:8px 20px 12px 48px;border-bottom:1px solid var(--border)">
          <a href="#precios" style="font-size:12px;font-weight:600;color:var(--primary);text-decoration:none">→ Administrar listas de precios</a>
          <div style="font-size:11px;color:var(--text-400);margin-top:2px">Las listas activas aparecen en el selector "Lista de Precios" del pedido y aplican su descuento a las líneas</div>
        </div>
        <div id="row-descprice" style="display:${cfg.listas_precios ? '' : 'none'}">
          ${cfgCheck('cfg-descprice', 'Descuentos de Lista de Precios',
            'Muestra el descuento aplicado en la línea de la factura (precio de lista vs precio real)',
            cfg.descuento_precio)}
        </div>
        ${cfgCheck('cfg-marg', 'Márgenes',
          'Muestra el margen de ganancia en cada línea y en los totales de las órdenes de venta',
          cfg.margenes)}
      `)}

      ${section('Presupuestos y Pedidos', `
        ${cfgCheck('cfg-firma', 'Firma en Línea',
          'Permite que los clientes firmen digitalmente las cotizaciones para confirmarlas',
          cfg.firma_online)}
        ${cfgCheck('cfg-pago', 'Pago en Línea',
          'Permite que los clientes paguen sus cotizaciones en línea con tarjeta o transferencia',
          cfg.pago_online)}
        <div style="display:flex;align-items:center;justify-content:space-between;padding:14px 20px;border-bottom:1px solid var(--border)">
          <div>
            <div style="font-weight:600;font-size:14px;color:var(--text-900)">Validez Predeterminada del Presupuesto</div>
            <div style="font-size:12px;color:var(--text-400);margin-top:2px">Número de días que una cotización permanece válida. 0 = sin expiración</div>
          </div>
          <div style="display:flex;align-items:center;gap:6px">
            <input type="number" id="cfg-valid" value="${cfg.validez_cotizacion}" min="0" max="365"
              style="width:80px;text-align:center;padding:6px 10px;border:1px solid var(--border);border-radius:8px;font-size:14px">
            <span style="font-size:13px;color:var(--text-400)">días</span>
          </div>
        </div>
        ${cfgCheck('cfg-bloq', 'Bloquear Pedido Confirmado',
          'Impide editar un pedido después de confirmarlo. Para modificarlo se debe crear un pedido de devolución o cancelarlo',
          cfg.bloquear_confirmado)}
        ${cfgCheck('cfg-warn', 'Advertencias',
          'Muestra advertencias al vendedor al confirmar cotizaciones o pedidos para clientes o productos específicos',
          cfg.advertencias)}
        ${cfgCheck('cfg-tmpl', 'Plantillas de Presupuesto',
          'Crea plantillas reutilizables para los presupuestos más comunes y aplícalas con un clic',
          cfg.plantillas_presupuesto)}
        ${cfgCheck('cfg-online', 'Compra en Línea',
          'Permite a los clientes ver y confirmar sus cotizaciones en un portal en línea',
          cfg.compra_online)}
        ${cfgCheck('cfg-notas', 'Notas de Cierre',
          'Agrega notas personalizadas al final de las cotizaciones y pedidos de venta confirmados',
          cfg.notas_cierre)}
      `)}

      ${section('Envío', `
        ${cfgCheck('cfg-ship', 'Costos de Envío',
          'Permite agregar costos de envío a las cotizaciones. Se integra con los métodos de entrega configurados',
          cfg.costos_envio)}
        ${cfgCheck('cfg-fecha', 'Fechas de Entrega',
          'Muestra la fecha de entrega comprometida al cliente (Fecha Compromiso) en las cotizaciones y pedidos',
          cfg.fecha_entrega)}
        ${cfgCheck('cfg-stock', 'Advertencia de Stock',
          'Muestra una advertencia al confirmar un pedido si no hay suficiente stock disponible',
          cfg.aviso_stock)}
      `)}

      ${section('Facturación', `
        <div style="display:flex;align-items:center;justify-content:space-between;padding:14px 20px;border-bottom:1px solid var(--border)">
          <div>
            <div style="font-weight:600;font-size:14px;color:var(--text-900)">Política de Facturación</div>
            <div style="font-size:12px;color:var(--text-400);margin-top:2px">Define cuándo se puede facturar al cliente: al confirmar el pedido o al entregar los productos</div>
          </div>
          <select id="cfg-pol" style="min-width:230px;padding:7px 12px;border:1px solid var(--border);border-radius:8px;font-size:13px;background:var(--bg-card);color:var(--text-900)">
            <option value="cantidad_pedida" ${cfg.politica_facturacion === 'cantidad_pedida' ? 'selected' : ''}>Cantidades pedidas</option>
            <option value="cantidad_entregada" ${cfg.politica_facturacion === 'cantidad_entregada' ? 'selected' : ''}>Cantidades entregadas</option>
          </select>
        </div>
      `)}

      ${section('Términos y Condiciones', `
        <div style="padding:16px 20px">
          <div style="font-size:12px;color:var(--text-400);margin-bottom:10px">
            Texto que aparece al pie de cada cotización y pedido de venta. Puedes incluir políticas de devolución, formas de pago, etc.
          </div>
          <textarea id="cfg-terms" rows="5"
            style="width:100%;padding:10px 14px;border:1px solid var(--border);border-radius:8px;font-size:13px;background:var(--bg-card);color:var(--text-900);resize:vertical;font-family:inherit;line-height:1.6;box-sizing:border-box"
            placeholder="Ej. Los precios no incluyen IVA. Válido por 30 días. Entrega sujeta a disponibilidad de stock.">${cfg.terminos}</textarea>
        </div>
      `)}

      <div style="display:flex;gap:10px;padding-top:8px">
        <button class="o-btn-primary" onclick="window._saveConfigV()">💾 Guardar Configuración</button>
        <button class="o-btn-secondary" onclick="window._discardConfigV()">Descartar cambios</button>
      </div>
    </div>
  </div>`)

  // Mostrar/ocultar descuento de lista y enlace de administración al activar listas de precios
  document.getElementById('cfg-pricelist')?.addEventListener('change', e => {
    const visible = e.target.checked ? '' : 'none'
    document.getElementById('row-descprice').style.display = visible
    document.getElementById('row-pricelist-link').style.display = visible
  })

  window._saveConfigV = () => {
    const n = (id) => document.getElementById(id)
    const cfg = {
      variantes:              n('cfg-var')?.checked       ?? false,
      unidades_medida:        n('cfg-udm')?.checked       ?? false,
      empaquetado:            n('cfg-pack')?.checked      ?? false,
      descuentos:             n('cfg-desc')?.checked      ?? true,
      listas_precios:         n('cfg-pricelist')?.checked ?? false,
      descuento_precio:       n('cfg-descprice')?.checked ?? false,
      margenes:               n('cfg-marg')?.checked      ?? false,
      firma_online:           n('cfg-firma')?.checked     ?? false,
      pago_online:            n('cfg-pago')?.checked      ?? false,
      validez_cotizacion:     Math.max(0, parseInt(n('cfg-valid')?.value, 10) || 0),
      bloquear_confirmado:    n('cfg-bloq')?.checked      ?? true,
      advertencias:           n('cfg-warn')?.checked      ?? false,
      plantillas_presupuesto: n('cfg-tmpl')?.checked      ?? false,
      compra_online:          n('cfg-online')?.checked    ?? false,
      notas_cierre:           n('cfg-notas')?.checked     ?? false,
      costos_envio:           n('cfg-ship')?.checked      ?? false,
      fecha_entrega:          n('cfg-fecha')?.checked     ?? false,
      aviso_stock:            n('cfg-stock')?.checked     ?? false,
      politica_facturacion:   n('cfg-pol')?.value         || 'cantidad_pedida',
      terminos:               n('cfg-terms')?.value       || ''
    }
    localStorage.setItem(KEY, JSON.stringify(cfg))
    toast('Guardado', 'Configuración de Ventas actualizada correctamente', 'success')
  }

  window._discardConfigV = () => {
    renderConfigVentas()
  }
}
