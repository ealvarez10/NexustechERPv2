// ui.js — Componentes de UI compartidos

export function fmt(n, d=0) {
  if (n == null || n === '') return '—'
  return Number(n).toLocaleString('es-MX', { minimumFractionDigits: d, maximumFractionDigits: d })
}

export function fmtMxn(n) {
  if (n == null) return '—'
  n = parseFloat(n) || 0
  if (Math.abs(n) >= 1_000_000) return `$${(n/1_000_000).toFixed(2)}M`
  if (Math.abs(n) >= 1_000) return `$${(n/1_000).toFixed(1)}k`
  return `$${fmt(n, 2)}`
}

export function fmtNum(n) {
  if (n == null) return '—'
  return Number(n).toLocaleString('es-MX')
}

export function fmtDate(s) {
  if (!s) return '—'
  return new Date(s).toLocaleDateString('es-MX', { day:'2-digit', month:'short', year:'numeric' })
}

// Toast notification
export function toast(title, msg='', type='info') {
  const icons = { success:'✅', error:'❌', info:'ℹ️', warning:'⚠️' }
  let c = document.getElementById('__toasts')
  if (!c) {
    c = document.createElement('div')
    c.id = '__toasts'
    c.className = 'toast-container'
    document.body.appendChild(c)
  }
  const t = document.createElement('div')
  t.className = `toast ${type}`
  t.innerHTML = `
    <span class="toast-icon">${icons[type]||'ℹ️'}</span>
    <div><div class="toast-title">${title}</div>${msg ? `<div class="toast-msg">${msg}</div>` : ''}</div>`
  c.appendChild(t)
  requestAnimationFrame(() => t.classList.add('show'))
  setTimeout(() => { t.classList.remove('show'); setTimeout(() => t.remove(), 400) }, 3800)
}

// Animate counter with easing
export function animCount(el, target, duration=900, prefix='', suffix='') {
  if (!el) return
  const start = performance.now()
  const isFloat = String(target).includes('.')
  function tick(now) {
    const p = Math.min((now - start) / duration, 1)
    const e = 1 - Math.pow(1 - p, 3)
    const v = target * e
    el.textContent = prefix + (isFloat
      ? v.toLocaleString('es-MX', {minimumFractionDigits:2,maximumFractionDigits:2})
      : Math.round(v).toLocaleString('es-MX')) + suffix
    if (p < 1) requestAnimationFrame(tick)
  }
  requestAnimationFrame(tick)
}

// Sparkline CSS bars
export function sparkline(values) {
  if (!values?.length) return ''
  const max = Math.max(...values, 1)
  return `<div class="sparkline">${
    values.map((v, i) => `<div class="spark-bar${i===values.length-1?' active':''}" style="height:${Math.max(4, Math.round((v/max)*100))}%"></div>`).join('')
  }</div>`
}

// Skeleton rows for tables
export function skeletonRows(cols=5, rows=6) {
  return `<tbody>${Array.from({length:rows}, () =>
    `<tr>${Array.from({length:cols}, () =>
      `<td><div class="skeleton" style="height:14px;width:${60+Math.random()*30}%"></div></td>`
    ).join('')}</tr>`
  ).join('')}</tbody>`
}

// Full skeleton table with header
export function skeletonTable(rows=5, cols=4) {
  return `<table class="data-table"><thead><tr>${Array.from({length:cols}, () =>
    `<th><div class="skeleton" style="height:12px;width:${40+Math.random()*40}%"></div></th>`
  ).join('')}</tr></thead>${skeletonRows(cols, rows)}</table>`
}

// Skeleton KPI cards
export function skeletonKpis(count=5) {
  return Array.from({length:count}, () => `
  <div class="kpi-card kpi-gray">
    <div class="kpi-label"><div class="skeleton" style="height:13px;width:60%"></div></div>
    <div class="kpi-value"><div class="skeleton" style="height:28px;width:70%"></div></div>
    <div><div class="skeleton" style="height:11px;width:40%;margin-top:6px"></div></div>
  </div>`).join('')
}

// Badge helper
const BADGE_MAP = {
  'sale':'emerald','done':'indigo','draft':'gray','sent':'sky',
  'cancel':'red','posted':'emerald','in_payment':'violet','paid':'emerald','partial':'amber',
}

export function stateBadge(state, label) {
  const cls = BADGE_MAP[state] || 'gray'
  return `<span class="badge badge-${cls} badge-dot">${label}</span>`
}

// Pagination
export function paginationHtml(page, hasMore, onNav) {
  window.__pagNav = onNav
  return `
  <div class="data-table-footer">
    <span style="color:var(--text-400)">Página ${page}</span>
    <div class="pagination">
      <button class="pag-btn" ${page<=1?'disabled':''} onclick="window.__pagNav(${page-1})">&#8592; Anterior</button>
      <span class="pag-btn active">${page}</span>
      <button class="pag-btn" ${!hasMore?'disabled':''} onclick="window.__pagNav(${page+1})">Siguiente &#8594;</button>
    </div>
  </div>`
}
