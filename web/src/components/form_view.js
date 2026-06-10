/**
 * FormView — Vista de formulario tipo Odoo Enterprise
 * Status bar + Smart buttons + Sheet con grupos + Notebook/Tabs + Chatter
 */
import { setPage, setBreadcrumb } from '../layout.js'
import { toast, openModal } from '../ui.js'

// Mapa de colores por estado
const STATE_COLORS = {
  draft:    { bg: '#fff7ed', color: '#c2410c', label: 'Borrador' },
  sent:     { bg: '#eff6ff', color: '#1d4ed8', label: 'Enviado' },
  sale:     { bg: '#f0fdf4', color: '#15803d', label: 'Confirmado' },
  done:     { bg: '#f0fdf4', color: '#166534', label: 'Realizado' },
  cancel:   { bg: '#fef2f2', color: '#b91c1c', label: 'Cancelado' },
  posted:   { bg: '#f0fdf4', color: '#15803d', label: 'Publicada' },
  open:     { bg: '#eff6ff', color: '#1d4ed8', label: 'Abierta' },
  paid:     { bg: '#f0fdf4', color: '#166534', label: 'Pagada' },
  in_payment: { bg: '#fdf4ff', color: '#7e22ce', label: 'En Pago' },
  purchase: { bg: '#f0fdf4', color: '#15803d', label: 'Orden de Compra' },
  purchase_order: { bg: '#f0fdf4', color: '#15803d', label: 'Confirmada' },
}

export function getStateInfo(state) {
  return STATE_COLORS[state] || { bg: '#f8fafc', color: '#475569', label: state }
}

/**
 * Renderiza una página de formulario completa al estilo Odoo
 * @param {Object} opts
 */
export function renderFormPage(opts) {
  const {
    breadcrumb = [],         // [{label, hash}] para el breadcrumb
    title = '',              // Título del registro
    subtitle = '',           // Subtítulo (ej: nombre empresa)
    statusSteps = [],        // [{key, label, icon?}]
    currentStatus = '',      // key del estado actual
    statusButtons = [],      // [{label, primary?, danger?, onClick, visible?}]
    smartButtons = [],       // [{icon, count, label, onClick}]
    fieldGroups = [],        // [{title?, cols?, fields: [{label, value, type?, colspan?}]}]
    tabs = [],               // [{label, id, content: fn()->html, badge?}]
    chatterMessages = [],    // [{author, initials, date, text, type?}]
    extraHtml = '',          // HTML adicional al final del sheet
    onEdit = null,           // fn: entra en modo edición
    onSave = null,           // fn: guarda cambios
    editing = false,         // si está en modo edición
    id = '',                 // ID del registro
  } = opts

  // Construir breadcrumb
  const fullBreadcrumb = [
    ...breadcrumb.map(b => ({ label: b.label, href: b.hash ? `#${b.hash}` : undefined })),
    { label: title || 'Nuevo' }
  ]
  setBreadcrumb(fullBreadcrumb)

  // Generar tabs
  const tabsHtml = tabs.length ? `
    <div class="o-notebook">
      <div class="o-tabs" role="tablist">
        ${tabs.map((t, i) => `
          <button class="o-tab ${i === 0 ? 'active' : ''}"
            role="tab"
            id="tab-btn-${t.id || i}"
            onclick="window._switchTab('${t.id || i}')">
            ${t.label}
            ${t.badge ? `<span class="o-tab-badge">${t.badge}</span>` : ''}
          </button>
        `).join('')}
      </div>
      ${tabs.map((t, i) => `
        <div class="o-tab-panel ${i === 0 ? 'active' : ''}" id="tab-panel-${t.id || i}">
          ${typeof t.content === 'function' ? t.content() : (t.content || '')}
        </div>
      `).join('')}
    </div>
  ` : ''

  // Generar grupos de campos
  const fieldsHtml = fieldGroups.map(group => fieldGroupHtml(group)).join('')

  // Smart buttons
  const sbHtml = smartButtons.length ? `
    <div class="o-smart-buttons">
      ${smartButtons.map(b => `
        <button class="o-smart-btn" onclick="${b.onClick || 'void 0'}">
          <span class="o-count">${b.count ?? 0}</span>
          <span class="o-label">${b.icon || ''} ${b.label}</span>
        </button>
      `).join('')}
    </div>
  ` : ''

  // Status bar
  const statusHtml = `
    <div class="o-statusbar">
      <div class="o-statusbar-status">
        ${statusSteps.map(s => {
          const currentIdx = statusSteps.findIndex(x => x.key === currentStatus)
          const stepIdx = statusSteps.findIndex(x => x.key === s.key)
          const isDone = currentIdx > stepIdx
          const isActive = s.key === currentStatus
          return `
            <div class="o-status-step ${isActive ? 'active' : ''} ${isDone ? 'done' : ''}">
              ${isDone ? '✔️ ' : ''}${s.label}
            </div>
          `
        }).join('<span class="o-status-arrow">›</span>')}
      </div>
      <div class="o-statusbar-buttons">
        ${statusButtons.filter(b => b.visible !== false).map(b => `
          <button class="btn ${b.danger ? 'btn-danger' : b.primary ? 'btn-primary' : 'btn-secondary'} btn-sm"
            onclick="${b.onClick}">
            ${b.icon || ''}${b.label}
          </button>
        `).join('')}
        ${!editing && onEdit ? `
          <button class="btn btn-secondary btn-sm" onclick="window._formEdit?.()">
            ✏️ Editar
          </button>
        ` : ''}
        ${editing ? `
          <button class="btn btn-primary btn-sm" onclick="window._formSave?.()">💾 Guardar</button>
          <button class="btn btn-secondary btn-sm" onclick="window._formDiscard?.()">✕ Descartar</button>
        ` : ''}
      </div>
    </div>
  `

  // Chatter
  const chatterHtml = `
    <div class="o-chatter">
      <div class="o-chatter-topbar">
        <button class="o-chatter-btn" onclick="window._sendMsg?.('${id}')">✉️ Enviar mensaje</button>
        <button class="o-chatter-btn" onclick="window._addNote?.('${id}')">📋 Nota interna</button>
        <button class="o-chatter-btn">📎 Adjuntar</button>
      </div>
      <div class="o-chatter-thread">
        ${chatterMessages.length ? chatterMessages.map(m => `
          <div class="o-message ${m.type === 'note' ? 'o-message-note' : ''}">
            <div class="o-msg-avatar" style="background:${stringToColor(m.author)}">${m.initials || m.author?.[0]?.toUpperCase() || '?'}</div>
            <div class="o-msg-content">
              <div class="o-msg-header">
                <span class="o-msg-author">${m.author}</span>
                <span class="o-msg-date">${m.date}</span>
                ${m.type === 'note' ? '<span class="o-msg-note-badge">Nota interna</span>' : ''}
              </div>
              <div class="o-msg-text">${m.text}</div>
            </div>
          </div>
        `).join('') : `
          <div class="o-chatter-empty">
            <p>💬 Sin actividad en este registro.</p>
          </div>
        `}
      </div>
    </div>
  `

  const html = `
    <div class="o-form-view ${editing ? 'editing' : ''}" id="form-view-root">
      ${statusHtml}
      ${sbHtml}
      <div class="o-form-sheet">
        <div class="o-form-title-row">
          <h1 class="o-form-record-title">${title}</h1>
          ${subtitle ? `<span class="o-form-subtitle">${subtitle}</span>` : ''}
        </div>
        ${fieldsHtml}
        ${tabsHtml}
        ${extraHtml}
      </div>
      ${chatterHtml}
    </div>
  `

  setPage(html)

  // Inicializar tabs
  window._switchTab = (tabId) => {
    document.querySelectorAll('.o-tab').forEach(t => t.classList.remove('active'))
    document.querySelectorAll('.o-tab-panel').forEach(p => p.classList.remove('active'))
    document.getElementById('tab-btn-' + tabId)?.classList.add('active')
    document.getElementById('tab-panel-' + tabId)?.classList.add('active')
  }

  // Helpers chatter
  window._sendMsg = (recordId) => {
    openModal('Enviar mensaje', `
      <div style="display:flex;flex-direction:column;gap:14px">
        <textarea id="chat-msg" style="width:100%;min-height:120px;padding:12px;border:1px solid var(--border);border-radius:10px;font-size:14px;resize:vertical;font-family:inherit" placeholder="Escribe tu mensaje..."></textarea>
        <div style="display:flex;gap:8px;justify-content:flex-end">
          <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cancelar</button>
          <button class="btn btn-primary btn-sm" onclick="
            const msg=document.getElementById('chat-msg')?.value.trim();
            if(msg){window.__closeModal();toast('Enviado','Mensaje registrado en el historial','success');}">
            ✉️ Enviar
          </button>
        </div>
      </div>
    `)
  }

  window._addNote = (recordId) => {
    openModal('Nota interna', `
      <div style="display:flex;flex-direction:column;gap:14px">
        <div style="padding:8px 12px;background:#FEF9C3;border-radius:8px;font-size:12px;color:#854D0E">
          ⚠️ Solo visible para el equipo interno
        </div>
        <textarea id="note-msg" style="width:100%;min-height:100px;padding:12px;border:1.5px dashed #D97706;border-radius:10px;font-size:14px;resize:vertical;font-family:inherit;background:#FFFBEB" placeholder="Nota interna..."></textarea>
        <div style="display:flex;gap:8px;justify-content:flex-end">
          <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cancelar</button>
          <button class="btn btn-sm" style="background:#D97706;color:#fff;border:none;border-radius:8px;padding:6px 16px;font-weight:600;cursor:pointer" onclick="window.__closeModal();">
            📋 Guardar nota
          </button>
        </div>
      </div>
    `)
  }

  if (onEdit) window._formEdit = onEdit
  if (onSave) window._formSave = onSave
  window._formDiscard = () => history.back()
}

/**
 * Renderiza un grupo de campos (2 columnas por defecto)
 */
export function fieldGroupHtml({ title, cols = 2, fields = [] }) {
  if (!fields.length) return ''

  const half = Math.ceil(fields.length / cols)
  const col1 = fields.slice(0, half)
  const col2 = cols === 2 ? fields.slice(half) : []

  const renderField = (f) => `
    <div class="o-field-row">
      <div class="o-field-label">${f.label}</div>
      <div class="o-field-value">
        ${renderFieldValue(f)}
      </div>
    </div>
  `

  const renderFieldValue = (f) => {
    if (f.value === null || f.value === undefined || f.value === '') {
      return '<span class="o-field-empty">—</span>'
    }
    switch (f.type) {
      case 'money': return `<span class="o-field-money">${f.value}</span>`
      case 'badge': return `<span class="o-state-badge" style="background:${f.bg || '#f1f5f9'};color:${f.color || '#475569'}">${f.value}</span>`
      case 'link': return `<a href="${f.href || '#'}" class="o-field-link">${f.value}</a>`
      case 'email': return `<a href="mailto:${f.value}" class="o-field-link">${f.value}</a>`
      case 'phone': return `<a href="tel:${f.value}" class="o-field-link">${f.value}</a>`
      case 'date': return `<span>${f.value}</span>`
      case 'boolean': return f.value ? '✅ Sí' : '❌ No'
      default: return `<span>${f.value}</span>`
    }
  }

  return `
    <div class="o-form-group-wrapper">
      ${title ? `<div class="o-group-title">${title}</div>` : ''}
      <div class="o-form-group">
        <div class="o-form-col">${col1.map(renderField).join('')}</div>
        ${cols === 2 ? `<div class="o-form-col">${col2.map(renderField).join('')}</div>` : ''}
      </div>
    </div>
  `
}

/**
 * Tabla editable de líneas (como notebook de Odoo)
 */
export function editableLinesHtml({ columns, rows, onAddLine, onDeleteLine, idField = 'id' }) {
  return `
    <div class="o-lines-table-wrapper">
      <table class="o-editable-table">
        <thead>
          <tr>
            ${columns.map(c => `<th>${c.label}</th>`).join('')}
            <th style="width:40px"></th>
          </tr>
        </thead>
        <tbody id="lines-tbody">
          ${rows.length ? rows.map((row, i) => `
            <tr class="o-line-row" data-id="${row[idField] || i}">
              ${columns.map(c => `
                <td class="${c.align === 'right' ? 'text-right' : ''}">${
                  c.render ? c.render(row) : (row[c.field] ?? '—')
                }</td>
              `).join('')}
              <td>
                <button class="o-del-line-btn" onclick="${onDeleteLine}(${row[idField] || i})" title="Eliminar">×</button>
              </td>
            </tr>
          `).join('') : `
            <tr><td colspan="${columns.length + 1}" style="text-align:center;padding:20px;color:var(--text-400)">Sin líneas</td></tr>
          `}
        </tbody>
      </table>
      ${onAddLine ? `
        <div class="o-add-line" onclick="${onAddLine}()">
          <span>+</span> Agregar una línea
        </div>
      ` : ''}
    </div>
  `
}

// ─── Legacy compatibility exports ────────────────────────────────────────────

/**
 * Crea el HTML de la barra de estado (legacy)
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
 * Genera HTML de smart buttons (legacy)
 */
export function smartButtonsHtml(buttons = []) {
  if (!buttons.length) return ''
  return `
    <div class="o-smart-buttons">
      ${buttons.map(b => `
        <button class="o-smart-btn" onclick="${b.onClick || ''}">
          <span class="o-count">${b.count ?? 0}</span>
          <span class="o-label">${b.icon || ''} ${b.label}</span>
        </button>
      `).join('')}
    </div>
  `
}

/**
 * Genera HTML de grupo de campos Odoo - legacy API (fields[], cols)
 */
export function fieldGroupHtmlLegacy(fields = [], cols = 2) {
  return fieldGroupHtml({ fields, cols })
}

/**
 * Genera el chatter HTML con mensajes de log (legacy)
 */
export function chatterHtml(messages = [], recordName = '') {
  const threadHtml = messages.length > 0
    ? messages.map(m => `
      <div class="o-message">
        <div class="o-msg-avatar" style="background:${stringToColor(m.author)}">${m.initials || '?'}</div>
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
 * Renderiza un FormView Odoo completo en el contenedor dado (legacy API)
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
      <div class="o-statusbar">
        <div class="o-statusbar-status">
          ${statusBarHtml(statusSteps, currentStatus)}
        </div>
        <div class="o-statusbar-buttons">
          ${statusBtns}
        </div>
      </div>
      ${smartButtonsHtml(smartButtons)}
      <div class="o-form-sheet">
        <div class="o-form-header">
          ${title ? `<h2 style="font-family:'Plus Jakarta Sans',sans-serif;font-size:20px;font-weight:800;color:var(--text-900);margin-bottom:16px">${title}</h2>` : ''}
        </div>
        ${groups.map(g => fieldGroupHtml({ fields: g.fields, cols: g.cols ?? 2 })).join('')}
        ${tabsHtml}
      </div>
      ${chatterHtml(messages, title)}
    </div>
  `

  if (containerEl) {
    containerEl.innerHTML = formHtml
  }

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
 * Abre un formulario Odoo a pantalla completa usando setPage (legacy API)
 */
export function openFormPage(opts = {}) {
  const { backLabel = 'Volver', backHref = '', pageTitle = opts.title || 'Detalle' } = opts

  setBreadcrumb([
    ...(backHref ? [{ label: backLabel, href: backHref }] : [{ label: backLabel }]),
    { label: pageTitle },
  ])

  const tmp = document.createElement('div')
  renderFormView(tmp, opts)
  setPage(tmp.innerHTML)

  window._switchTab = (btn, idx) => {
    const form = btn.closest('.o-form-view')
    if (!form) return
    form.querySelectorAll('.o-tab').forEach(t => t.classList.remove('active'))
    form.querySelectorAll('.o-tab-panel').forEach(p => p.classList.remove('active'))
    btn.classList.add('active')
    form.querySelector(`.o-tab-panel[data-panel="${idx}"]`)?.classList.add('active')
  }
}

// Chatter modal helpers (legacy)
window._chatterMessage = (recordName) => {
  openModal('Enviar mensaje', `
    <div style="display:flex;flex-direction:column;gap:12px">
      <label style="font-size:13px;font-weight:600;color:var(--text-600)">Mensaje</label>
      <textarea id="chatter-msg" style="width:100%;min-height:100px;padding:10px;border:1px solid var(--border);border-radius:8px;font-size:13px;resize:vertical;font-family:inherit"
        placeholder="Escribe tu mensaje..."></textarea>
      <div style="display:flex;gap:8px;justify-content:flex-end">
        <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cancelar</button>
        <button class="btn btn-primary btn-sm" onclick="(() => {
          const msg = document.getElementById('chatter-msg')?.value;
          if (msg) { window.__closeModal(); }
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

function stringToColor(str = '') {
  let hash = 0
  for (let i = 0; i < str.length; i++) hash = str.charCodeAt(i) + ((hash << 5) - hash)
  const h = hash % 360
  return `hsl(${Math.abs(h)}, 65%, 45%)`
}
