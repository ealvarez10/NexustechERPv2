/**
 * FormView — Componente de formulario tipo Odoo Enterprise
 * Genera el HTML de un formulario con status bar, smart buttons,
 * notebook y chatter.
 */
import { openModal, toast } from '../ui.js'
import { setPage, setBreadcrumb } from '../layout.js'

/**
 * Crea el HTML de la barra de estado
 * @param {Array} steps - [{key, label, done?}]
 * @param {string} current - key del estado activo
 */
export function statusBarHtml(steps, current) {
  return steps.map(s => `
    <div class="o-status-step ${s.key === current ? 'active' : (s.done ? 'done' : '')}"
         data-status="${s.key}">
      ${s.label}
    </div>
  `).join('')
}

/**
 * Crea el HTML de smart buttons
 * @param {Array} buttons - [{icon, count, label, onClick}]
 */
export function smartButtonsHtml(buttons) {
  if (!buttons?.length) return ''
  return `
    <div class="o-smart-buttons">
      ${buttons.map(b => `
        <button class="o-smart-btn" onclick="${b.onClick || ''}">
          <span class="o-count">${b.count ?? '—'}</span>
          <span class="o-label">${b.icon || ''} ${b.label}</span>
        </button>
      `).join('')}
    </div>
  `
}

/**
 * Genera HTML de grupo de campos Odoo (1 o 2 columnas)
 * @param {Array} fields - [{label, value, editable?, type?}]
 * @param {number} cols - 1 o 2
 */
export function fieldGroupHtml(fields, cols = 2) {
  const half = Math.ceil(fields.length / cols)
  const col1 = fields.slice(0, half)
  const col2 = cols === 2 ? fields.slice(half) : []

  const renderField = (f) => `
    <div class="o-field-row">
      <div class="o-field-label">${f.label}</div>
      <div class="o-field-value">
        ${(f.value !== null && f.value !== undefined && f.value !== '')
          ? `<span>${f.value}</span>`
          : '<span style="color:var(--text-300)">—</span>'}
      </div>
    </div>
  `

  const renderCol = (fields) => fields.map(renderField).join('')

  return `
    <div class="o-form-group${cols === 1 ? ' full' : ''}">
      <div class="o-form-col">${renderCol(col1)}</div>
      ${cols === 2 ? `<div class="o-form-col">${renderCol(col2)}</div>` : ''}
    </div>
  `
}

/**
 * Genera el chatter HTML con mensajes de log
 * @param {Array} messages - [{author, initials, date, text}]
 * @param {string} recordName - nombre del registro para contexto
 */
export function chatterHtml(messages = [], recordName = '') {
  const threadHtml = messages.length > 0
    ? messages.map(m => `
      <div class="o-message">
        <div class="o-msg-avatar">${m.initials || '?'}</div>
        <div class="o-msg-content">
          <div class="o-msg-header">
            <span class="o-msg-author">${m.author}</span>
            <span class="o-msg-date">${m.date}</span>
          </div>
          <div class="o-msg-text">${m.text}</div>
        </div>
      </div>
    `).join('')
    : `<div class="o-chatter-empty">Sin actividad registrada en este documento.</div>`

  const safe = (recordName || '').replace(/'/g, "\\'")
  return `
    <div class="o-chatter">
      <div class="o-chatter-topbar">
        <button class="o-chatter-btn" onclick="window._chatterMessage('${safe}')">✉️ Enviar mensaje</button>
        <button class="o-chatter-btn" onclick="window._chatterNote('${safe}')">📋 Nota interna</button>
        <button class="o-chatter-btn">📎 Adjuntar</button>
      </div>
      <div class="o-chatter-thread">${threadHtml}</div>
    </div>
  `
}

/**
 * Renderiza un FormView Odoo completo en el contenedor dado
 * @param {HTMLElement} containerEl - Elemento donde montar el form
 * @param {Object} opts
 */
export function renderFormView(containerEl, opts = {}) {
  const {
    title = '',
    statusSteps = [],
    currentStatus = '',
    smartButtons = [],
    statusButtons = [],
    groups = [],
    tabs = [],
    messages = [],
    editable = false,
  } = opts

  const statusBtns = statusButtons
    .filter(b => b.visible !== false)
    .map(b => `
      <button class="btn ${b.primary ? 'btn-primary' : 'btn-secondary'} btn-sm"
              onclick="${b.onClick || ''}">
        ${b.label}
      </button>
    `).join('')

  const tabsHtml = tabs.length > 0 ? `
    <div class="o-notebook">
      <div class="o-tabs" role="tablist">
        ${tabs.map((t, i) => `
          <button class="o-tab${i === 0 ? ' active' : ''}"
                  role="tab"
                  data-tab="${i}"
                  onclick="window._switchTab(this, ${i})">
            ${t.label}
          </button>
        `).join('')}
      </div>
      ${tabs.map((t, i) => `
        <div class="o-tab-panel${i === 0 ? ' active' : ''}" data-panel="${i}">
          ${t.content || ''}
        </div>
      `).join('')}
    </div>
  ` : ''

  const formHtml = `
    <div class="o-form-view${editable ? ' editing' : ''}">
      <!-- Status Bar -->
      <div class="o-statusbar">
        <div class="o-statusbar-status">
          ${statusBarHtml(statusSteps, currentStatus)}
        </div>
        <div class="o-statusbar-buttons">
          ${statusBtns}
        </div>
      </div>

      <!-- Smart Buttons -->
      ${smartButtonsHtml(smartButtons)}

      <!-- Form Sheet -->
      <div class="o-form-sheet">
        <div class="o-form-header">
          ${title ? `<h2 style="font-family:'Plus Jakarta Sans',sans-serif;font-size:20px;font-weight:800;color:var(--text-900);margin-bottom:16px">${title}</h2>` : ''}
        </div>

        ${groups.map(g => fieldGroupHtml(g.fields, g.cols ?? 2)).join('')}

        ${tabsHtml}
      </div>

      <!-- Chatter -->
      ${chatterHtml(messages, title)}
    </div>
  `

  if (containerEl) {
    containerEl.innerHTML = formHtml
  }

  // Tab switching helper
  window._switchTab = (btn, idx) => {
    const form = btn.closest('.o-form-view')
    form.querySelectorAll('.o-tab').forEach(t => t.classList.remove('active'))
    form.querySelectorAll('.o-tab-panel').forEach(p => p.classList.remove('active'))
    btn.classList.add('active')
    form.querySelector(`.o-tab-panel[data-panel="${idx}"]`)?.classList.add('active')
  }

  return formHtml
}

/**
 * Abre un formulario Odoo a pantalla completa usando setPage
 * @param {Object} opts - Mismas opciones que renderFormView + backLabel/backHref para breadcrumb
 */
export function openFormPage(opts = {}) {
  const { backLabel = 'Volver', backHref = '', pageTitle = opts.title || 'Detalle' } = opts

  setBreadcrumb([
    ...(backHref ? [{ label: backLabel, href: backHref }] : [{ label: backLabel }]),
    { label: pageTitle },
  ])

  // Contenedor temporal
  const tmp = document.createElement('div')
  renderFormView(tmp, opts)
  setPage(tmp.innerHTML)

  // Re-bind tab switching after setPage (innerHTML swap)
  window._switchTab = (btn, idx) => {
    const form = btn.closest('.o-form-view')
    if (!form) return
    form.querySelectorAll('.o-tab').forEach(t => t.classList.remove('active'))
    form.querySelectorAll('.o-tab-panel').forEach(p => p.classList.remove('active'))
    btn.classList.add('active')
    form.querySelector(`.o-tab-panel[data-panel="${idx}"]`)?.classList.add('active')
  }
}

// ─── Chatter modal helpers ────────────────────────────────────────────────────

window._chatterMessage = (recordName) => {
  openModal('Enviar mensaje', `
    <div style="display:flex;flex-direction:column;gap:12px">
      <label style="font-size:13px;font-weight:600;color:var(--text-600)">Mensaje</label>
      <textarea id="chatter-msg" style="width:100%;min-height:100px;padding:10px;border:1px solid var(--border);border-radius:8px;font-size:13px;resize:vertical;font-family:inherit"
        placeholder="Escribe tu mensaje..."></textarea>
      <div style="display:flex;gap:8px;justify-content:flex-end">
        <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cancelar</button>
        <button class="btn btn-primary btn-sm" onclick="(()=>{
          const msg = document.getElementById('chatter-msg')?.value;
          if(msg){window.__closeModal();window._toast&&window._toast('Mensaje enviado','Registrado en el historial','success');}
        })()">✉️ Enviar</button>
      </div>
    </div>
  `)
}

window._chatterNote = (recordName) => {
  openModal('Nota interna', `
    <div style="display:flex;flex-direction:column;gap:12px">
      <label style="font-size:13px;font-weight:600;color:var(--text-600)">Nota interna <small style="color:var(--text-400)">(solo visible para el equipo)</small></label>
      <textarea id="chatter-note" style="width:100%;min-height:80px;padding:10px;border:1.5px dashed var(--warning);border-radius:8px;font-size:13px;resize:vertical;font-family:inherit;background:#FFFBEB"
        placeholder="Nota interna..."></textarea>
      <div style="display:flex;gap:8px;justify-content:flex-end">
        <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cancelar</button>
        <button class="btn btn-sm" style="background:#D97706;color:#fff;border:none;font-weight:600;cursor:pointer"
          onclick="window.__closeModal()">📋 Guardar nota</button>
      </div>
    </div>
  `)
}
