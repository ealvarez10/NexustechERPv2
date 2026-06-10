/**
 * ControlPanel — Barra superior tipo Odoo para vistas de lista/kanban
 * Incluye: botón Nuevo, buscador, Filtros, Agrupar Por, switcher de vista
 */

/**
 * Genera el HTML del control panel
 * @param {Object} opts
 * @param {string} opts.title - Título del módulo (ej: "Ventas")
 * @param {string} opts.newLabel - Texto botón nuevo (ej: "Nueva Venta")
 * @param {string} opts.newOnClick - JS onclick para el botón nuevo
 * @param {string[]} opts.views - ['list','kanban','form'] para mostrar en switcher
 * @param {string} opts.activeView - vista activa actual
 * @param {string} opts.onViewSwitch - fn JS para cambiar vista (ej: 'window._switchView')
 * @param {number} opts.total - total de registros
 * @param {Object[]} opts.filters - [{label, onClick}] opciones de filtro
 * @param {Object[]} opts.groupBy - [{label, onClick}] opciones agrupar
 * @returns {string} HTML
 */
export function controlPanelHtml(opts) {
  const {
    title = '',
    newLabel = 'Nuevo',
    newOnClick = '',
    views = ['list'],
    activeView = 'list',
    onViewSwitch = '',
    total = 0,
    filters = [],
    groupBy = [],
  } = opts

  const viewIcons = { list: '☰', kanban: '⬜', form: '📑', calendar: '📅' }

  const filterMenu = filters.length ? `
    <div class="o-dropdown" id="dd-filters">
      <button class="o-btn-filter" onclick="window._toggleDropdown('dd-filters')">
        📂 Filtros <span class="dd-arrow">▾</span>
      </button>
      <div class="o-dropdown-menu" id="dd-filters-menu">
        ${filters.map(f => `
          <div class="o-dropdown-item" onclick="${f.onClick};window._closeDropdowns()">${f.label}</div>
        `).join('')}
        <div class="o-dropdown-divider"></div>
        <div class="o-dropdown-item" onclick="window._clearFilters?.()">❌ Quitar filtros</div>
      </div>
    </div>
  ` : ''

  const groupMenu = groupBy.length ? `
    <div class="o-dropdown" id="dd-group">
      <button class="o-btn-filter" onclick="window._toggleDropdown('dd-group')">
        🗂️ Agrupar Por <span class="dd-arrow">▾</span>
      </button>
      <div class="o-dropdown-menu" id="dd-group-menu">
        ${groupBy.map(g => `
          <div class="o-dropdown-item" onclick="${g.onClick};window._closeDropdowns()">${g.label}</div>
        `).join('')}
      </div>
    </div>
  ` : ''

  return `
    <div class="o-control-panel">
      <div class="o-cp-left">
        <button class="o-btn-new" onclick="${newOnClick}">
          <span style="font-size:16px">+</span> ${newLabel}
        </button>
        <div class="o-cp-sep"></div>
        ${filterMenu}
        ${groupMenu}
        <div class="o-search-box">
          <span style="color:var(--text-400);font-size:14px">🔍</span>
          <input type="search" placeholder="Buscar..." id="cp-search"
            oninput="window._cpSearch?.(this.value)" onkeydown="if(event.key==='Escape')this.value=''">
          <span id="cp-search-tag" style="display:none" class="cp-tag">
            <span id="cp-tag-text"></span>
            <span style="cursor:pointer;margin-left:4px" onclick="document.getElementById('cp-search').value='';window._cpSearch?.('')">×</span>
          </span>
        </div>
        <span class="o-record-count" id="cp-count">${total > 0 ? total + ' registros' : ''}</span>
      </div>
      <div class="o-cp-right">
        ${views.length > 1 ? `
          <div class="o-view-switcher">
            ${views.map(v => `
              <button class="o-view-btn ${v === activeView ? 'active' : ''}"
                title="Vista ${v}"
                onclick="${onViewSwitch}('${v}')">
                ${viewIcons[v] || v}
              </button>
            `).join('')}
          </div>
        ` : ''}
      </div>
    </div>
  `
}

// Inicializar helpers de dropdown (llamar una vez en la página)
export function initDropdowns() {
  window._toggleDropdown = (id) => {
    const menu = document.getElementById(id + '-menu')
    if (!menu) return
    const isOpen = menu.classList.contains('open')
    window._closeDropdowns()
    if (!isOpen) menu.classList.add('open')
  }
  window._closeDropdowns = () => {
    document.querySelectorAll('.o-dropdown-menu.open').forEach(m => m.classList.remove('open'))
  }
  document.addEventListener('click', (e) => {
    if (!e.target.closest('.o-dropdown')) window._closeDropdowns()
  })
}
