import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { toast } from '../ui.js'

const KEY = 'nexus_config_contabilidad'
const DEFAULTS = {
  multimoneda: false,
  contabilidad_analitica: false,
  presupuestos: false,
  activos_fijos: false,
  ingresos_diferidos: false,
  alertas_facturacion: true,
  auditoria: false
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

export async function renderConfigContabilidad(params = {}) {
  ensureLayout()
  setBreadcrumb([
    { label: 'Contabilidad', onclick: () => window._go('contabilidad') },
    { label: 'Configuración' }
  ])

  const cfg = { ...DEFAULTS, ...JSON.parse(localStorage.getItem(KEY) || '{}') }

  setPage(`<div class="nx-module-page" style="background:var(--bg-app)">
    <div class="o-cp">
      <div class="o-cp-left"></div>
      <div class="o-cp-right">
        <button class="o-btn-primary" onclick="window._saveConfigConta()">💾 Guardar</button>
        <button class="o-btn-secondary" onclick="window._discardConfigConta()">Descartar</button>
      </div>
    </div>

    <div style="max-width:900px;margin:28px auto;padding:0 24px 60px">

      ${section('Operaciones y Monedas', `
        ${cfgCheck('cfgconta-multi', 'Multimoneda',
          'Permite registrar asientos y facturas en monedas extranjeras con tasa de cambio automática',
          cfg.multimoneda)}
        ${cfgCheck('cfgconta-analitica', 'Contabilidad Analítica',
          'Habilita cuentas analíticas y etiquetas para rastrear costos e ingresos por proyectos o departamentos',
          cfg.contabilidad_analitica)}
        ${cfgCheck('cfgconta-presupuestos', 'Presupuestos',
          'Compara los gastos e ingresos reales con metas definidas por periodos',
          cfg.presupuestos)}
      `)}

      ${section('Gestión de Activos e Ingresos', `
        ${cfgCheck('cfgconta-activos', 'Activos Fijos',
          'Calcula y registra automáticamente la depreciación de activos fijos a lo largo del tiempo',
          cfg.activos_fijos)}
        ${cfgCheck('cfgconta-diferidos', 'Ingresos y Gastos Diferidos',
          'Reconoce ingresos o gastos en un periodo de tiempo futuro (ej. suscripciones anuales)',
          cfg.ingresos_diferidos)}
      `)}

      ${section('Control y Auditoría', `
        ${cfgCheck('cfgconta-alertas', 'Alertas de Facturación',
          'Evita la creación de facturas si hay discrepancias de contabilidad no resueltas',
          cfg.alertas_facturacion)}
        ${cfgCheck('cfgconta-auditoria', 'Rastro de Auditoría',
          'Activa un registro inalterable (hash criptográfico) de cada asiento contable para cumplimiento fiscal',
          cfg.auditoria)}
      `)}

      <div style="display:flex;gap:10px;padding-top:8px">
        <button class="o-btn-primary" onclick="window._saveConfigConta()">💾 Guardar Configuración</button>
        <button class="o-btn-secondary" onclick="window._discardConfigConta()">Descartar cambios</button>
      </div>
    </div>
  </div>`)

  window._saveConfigConta = () => {
    const n = (id) => document.getElementById(id)
    const cfg = {
      multimoneda: n('cfgconta-multi')?.checked ?? false,
      contabilidad_analitica: n('cfgconta-analitica')?.checked ?? false,
      presupuestos: n('cfgconta-presupuestos')?.checked ?? false,
      activos_fijos: n('cfgconta-activos')?.checked ?? false,
      ingresos_diferidos: n('cfgconta-diferidos')?.checked ?? false,
      alertas_facturacion: n('cfgconta-alertas')?.checked ?? false,
      auditoria: n('cfgconta-auditoria')?.checked ?? false,
    }
    localStorage.setItem(KEY, JSON.stringify(cfg))
    toast('Guardado', 'Configuración de Contabilidad actualizada', 'success')
  }

  window._discardConfigConta = () => renderConfigContabilidad()
}
