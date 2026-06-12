import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { toast } from '../ui.js'

const KEY = 'nexus_config_inventario'
const DEFAULTS = {
  // Operaciones
  lotes_series: false,
  multi_almacen: false,
  rutas_multietapa: false,
  // Trazabilidad
  paquetes: false,
  advertencias: false,
  // Productos
  unidades_medida: false,
  variantes: false,
  // Escáner
  codigo_barras: false
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

export async function renderConfigInventario(params = {}) {
  ensureLayout()
  setBreadcrumb([
    { label: 'Inventario', onclick: () => window._go('stock') },
    { label: 'Configuración' }
  ])

  const cfg = { ...DEFAULTS, ...JSON.parse(localStorage.getItem(KEY) || '{}') }

  setPage(`<div class="nx-module-page" style="background:var(--bg-app)">
    <div class="o-cp">
      <div class="o-cp-left"></div>
      <div class="o-cp-right">
        <button class="o-btn-primary" onclick="window._saveConfigInv()">💾 Guardar</button>
        <button class="o-btn-secondary" onclick="window._discardConfigInv()">Descartar</button>
      </div>
    </div>

    <div style="max-width:900px;margin:28px auto;padding:0 24px 60px">

      ${section('Operaciones', `
        ${cfgCheck('cfgi-lotes', 'Lotes y Números de Serie',
          'Rastrear inventario a nivel de lote o número de serie individual',
          cfg.lotes_series)}
        ${cfgCheck('cfgi-multi', 'Múltiples Almacenes',
          'Gestionar inventario en más de un almacén físico',
          cfg.multi_almacen)}
        ${cfgCheck('cfgi-rutas', 'Rutas Multietapa',
          'Permitir reglas de enrutamiento complejas (ej: Recibir -> Control de Calidad -> Stock)',
          cfg.rutas_multietapa)}
      `)}

      ${section('Trazabilidad', `
        ${cfgCheck('cfgi-paq', 'Paquetes',
          'Agrupar productos en paquetes o pallets (Cajas, Tarimas) con número de rastreo propio',
          cfg.paquetes)}
        ${cfgCheck('cfgi-adv', 'Advertencias',
          'Mostrar advertencias al hacer movimientos de stock de ciertos productos',
          cfg.advertencias)}
      `)}

      ${section('Productos', `
        ${cfgCheck('cfgi-uom', 'Unidades de Medida',
          'Comprar, vender y almacenar en diferentes unidades de medida (Ej: Cajas vs Piezas)',
          cfg.unidades_medida)}
        ${cfgCheck('cfgi-var', 'Variantes',
          'Habilitar opciones de producto como Talla o Color',
          cfg.variantes)}
      `)}

      ${section('Escáner de Códigos', `
        ${cfgCheck('cfgi-bar', 'Lector de Códigos de Barras',
          'Procesar transferencias de stock, ajustes e inventarios físicos escaneando códigos de barras',
          cfg.codigo_barras)}
      `)}

      <div style="display:flex;gap:10px;padding-top:8px">
        <button class="o-btn-primary" onclick="window._saveConfigInv()">💾 Guardar Configuración</button>
        <button class="o-btn-secondary" onclick="window._discardConfigInv()">Descartar cambios</button>
      </div>
    </div>
  </div>`)

  window._saveConfigInv = () => {
    const n = (id) => document.getElementById(id)
    const cfg = {
      lotes_series:     n('cfgi-lotes')?.checked ?? false,
      multi_almacen:    n('cfgi-multi')?.checked ?? false,
      rutas_multietapa: n('cfgi-rutas')?.checked ?? false,
      paquetes:         n('cfgi-paq')?.checked ?? false,
      advertencias:     n('cfgi-adv')?.checked ?? false,
      unidades_medida:  n('cfgi-uom')?.checked ?? false,
      variantes:        n('cfgi-var')?.checked ?? false,
      codigo_barras:    n('cfgi-bar')?.checked ?? false,
    }
    localStorage.setItem(KEY, JSON.stringify(cfg))
    toast('Guardado', 'Configuración de Inventario actualizada', 'success')
  }

  window._discardConfigInv = () => renderConfigInventario()
}
