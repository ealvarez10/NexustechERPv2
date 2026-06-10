/**
 * KanbanView — Vista tablero tipo Odoo
 */

/**
 * Genera el HTML de la vista kanban
 * @param {Object} opts
 * @param {Object[]} opts.columns - [{key, label, color?}]
 * @param {Object[]} opts.records - registros con campo state/stage
 * @param {string} opts.stateField - campo que determina la columna (default: 'state')
 * @param {Function} opts.cardRender - fn(record) => html del card
 * @param {string} opts.onCardClick - onclick del card (ej: 'window._verVenta')
 * @param {string} opts.onNewCard - crear nuevo en columna
 */
export function kanbanViewHtml(opts) {
  const {
    columns = [],
    records = [],
    stateField = 'state',
    cardRender = null,
    onCardClick = '',
    onNewCard = '',
  } = opts

  const grouped = {}
  columns.forEach(c => { grouped[c.key] = [] })
  records.forEach(r => {
    const col = r[stateField]
    if (grouped[col] !== undefined) {
      grouped[col].push(r)
    } else {
      // Si no hay columna para este estado, va a la primera
      const first = columns[0]?.key
      if (first) grouped[first].push(r)
    }
  })

  return `
    <div class="o-kanban-view">
      ${columns.map(col => {
        const colRecords = grouped[col.key] || []
        return `
          <div class="o-kanban-col">
            <div class="o-kanban-col-header" style="border-top:3px solid ${col.color || 'var(--primary)'}">
              <span>${col.label}</span>
              <span class="o-kanban-col-count">${colRecords.length}</span>
            </div>
            <div class="o-kanban-cards">
              ${colRecords.map(r => cardRender ? cardRender(r) : defaultCard(r, onCardClick)).join('')}
              ${colRecords.length === 0 ? `
                <div style="text-align:center;padding:20px;color:var(--text-300);font-size:12px">
                  Sin registros
                </div>
              ` : ''}
            </div>
            ${onNewCard ? `
              <div class="o-kanban-add-btn" onclick="${onNewCard}('${col.key}')">
                + Agregar
              </div>
            ` : ''}
          </div>
        `
      }).join('')}
    </div>
  `
}

function defaultCard(r, onCardClick) {
  return `
    <div class="o-kanban-card" onclick="${onCardClick}(${r.id})">
      <div class="o-kanban-card-title">${r.name || r.display_name || '#' + r.id}</div>
      <div class="o-kanban-card-meta">
        <span>${r.partner_name || r.partner_id || ''}</span>
        ${r.amount_total ? `<span class="o-kanban-card-amount">$${Number(r.amount_total).toLocaleString('es-MX', { minimumFractionDigits: 2 })}</span>` : ''}
      </div>
      ${r.date_order ? `<div style="font-size:11px;color:var(--text-400);margin-top:6px">${r.date_order?.slice(0, 10) || ''}</div>` : ''}
    </div>
  `
}
