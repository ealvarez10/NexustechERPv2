import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { toast } from '../ui.js'

const KEY = 'nexus_config_crm'
const DEFAULTS = {
  // Actividades
  actividades: true,
  reuniones: true,
  llamadas: true,
  // Pipeline
  etapas_compartidas: false,
  probabilidad_ia: false,
  tiempo_cierre: false,
  // Comunicación
  email_alias: true,
  seguimiento_email: false,
  plantillas_email: true,
  // Estadísticas y pronóstico
  pronostico: false,
  objetivos: false,
  // Leads
  leads: false,
  mineria_leads: false,
  deduplicar: true,
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

export async function renderConfigCRM(params = {}) {
  ensureLayout()
  setBreadcrumb([
    { label: 'CRM', onclick: () => window._go('crm') },
    { label: 'Configuración' }
  ])

  const cfg = { ...DEFAULTS, ...JSON.parse(localStorage.getItem(KEY) || '{}') }

  setPage(`<div class="nx-module-page" style="background:var(--bg-app)">
    <div class="o-cp">
      <div class="o-cp-left"></div>
      <div class="o-cp-right">
        <button class="o-btn-primary" onclick="window._saveConfigCRM()">💾 Guardar</button>
        <button class="o-btn-secondary" onclick="window._discardConfigCRM()">Descartar</button>
      </div>
    </div>

    <div style="max-width:900px;margin:28px auto;padding:0 24px 60px">

      ${section('Leads y Pipeline', `
        ${cfgCheck('crm-leads', 'Leads',
          'Habilita la gestión de leads (prospectos) antes de convertirlos en oportunidades de venta',
          cfg.leads)}
        ${cfgCheck('crm-etapas', 'Etapas del Pipeline Compartidas',
          'Las etapas del pipeline son compartidas entre todos los equipos de ventas. Si se desactiva, cada equipo tiene sus propias etapas',
          cfg.etapas_compartidas)}
        ${cfgCheck('crm-prob', 'Probabilidad con IA',
          'Usa inteligencia artificial para calcular automáticamente la probabilidad de cierre de cada oportunidad',
          cfg.probabilidad_ia)}
        ${cfgCheck('crm-tiempo', 'Tiempo de Cierre',
          'Registra el tiempo desde la creación hasta el cierre de cada oportunidad para métricas de rendimiento',
          cfg.tiempo_cierre)}
        ${cfgCheck('crm-pronostico', 'Pronóstico',
          'Proyecta los ingresos esperados según la probabilidad de cierre del pipeline activo',
          cfg.pronostico)}
        ${cfgCheck('crm-objetivos', 'Objetivos de Venta',
          'Define objetivos de venta por vendedor o equipo y monitorea el avance en tiempo real',
          cfg.objetivos)}
      `)}

      ${section('Actividades', `
        ${cfgCheck('crm-act', 'Actividades',
          'Planifica y registra actividades de seguimiento como llamadas, emails y tareas para cada oportunidad',
          cfg.actividades)}
        ${cfgCheck('crm-meet', 'Reuniones',
          'Agenda reuniones con prospectos y clientes directamente desde la oportunidad. Se sincroniza con el calendario',
          cfg.reuniones)}
        ${cfgCheck('crm-call', 'Llamadas VoIP',
          'Realiza y registra llamadas telefónicas directamente desde las oportunidades mediante integración VoIP',
          cfg.llamadas)}
      `)}

      ${section('Comunicación', `
        ${cfgCheck('crm-alias', 'Alias de Correo',
          'Crea una dirección de email única para el equipo. Los correos recibidos generan automáticamente nuevas oportunidades',
          cfg.email_alias)}
        ${cfgCheck('crm-track', 'Seguimiento de Email',
          'Registra automáticamente cuándo el cliente abre los correos enviados desde las oportunidades',
          cfg.seguimiento_email)}
        ${cfgCheck('crm-tmpl', 'Plantillas de Email',
          'Crea y reutiliza plantillas de correo para comunicaciones frecuentes con prospectos y clientes',
          cfg.plantillas_email)}
      `)}

      ${section('Leads Automáticos', `
        ${cfgCheck('crm-mining', 'Minería de Leads',
          'Genera leads automáticamente a partir de criterios de búsqueda como industria, ubicación y tamaño de empresa',
          cfg.mineria_leads)}
        ${cfgCheck('crm-dedup', 'Deduplicación de Leads',
          'Detecta y fusiona automáticamente leads o oportunidades duplicadas basándose en nombre, email o teléfono',
          cfg.deduplicar)}
      `)}

      <div style="display:flex;gap:10px;padding-top:8px">
        <button class="o-btn-primary" onclick="window._saveConfigCRM()">💾 Guardar Configuración</button>
        <button class="o-btn-secondary" onclick="window._discardConfigCRM()">Descartar cambios</button>
      </div>
    </div>
  </div>`)

  window._saveConfigCRM = () => {
    const n = (id) => document.getElementById(id)
    const cfg = {
      leads:             n('crm-leads')?.checked    ?? false,
      etapas_compartidas:n('crm-etapas')?.checked   ?? false,
      probabilidad_ia:   n('crm-prob')?.checked     ?? false,
      tiempo_cierre:     n('crm-tiempo')?.checked   ?? false,
      pronostico:        n('crm-pronostico')?.checked?? false,
      objetivos:         n('crm-objetivos')?.checked ?? false,
      actividades:       n('crm-act')?.checked      ?? true,
      reuniones:         n('crm-meet')?.checked     ?? true,
      llamadas:          n('crm-call')?.checked     ?? true,
      email_alias:       n('crm-alias')?.checked    ?? true,
      seguimiento_email: n('crm-track')?.checked    ?? false,
      plantillas_email:  n('crm-tmpl')?.checked     ?? true,
      mineria_leads:     n('crm-mining')?.checked   ?? false,
      deduplicar:        n('crm-dedup')?.checked    ?? true,
    }
    localStorage.setItem(KEY, JSON.stringify(cfg))
    toast('Guardado', 'Configuración de CRM actualizada correctamente', 'success')
  }

  window._discardConfigCRM = () => renderConfigCRM()
}
