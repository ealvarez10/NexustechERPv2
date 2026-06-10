import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { toast } from '../ui.js'
import { api } from '../api.js'

let _lastResults = []
let _indexStatus = null

export async function renderSearch() {
  ensureLayout()
  setBreadcrumb([{label:'Dashboard',href:'dashboard'},{label:'NexusSearch'}])
  await loadSearch()
}

async function loadSearch() {
  setPage(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">🔍 NexusSearch</h1>
      <p class="page-subtitle">Motor de búsqueda de alta velocidad</p>
    </div>
    <div class="page-actions">
      <button class="btn btn-secondary" id="btn-status" onclick="window._checkStatus()">📡 Estado</button>
      <button class="btn btn-primary" id="btn-sync" onclick="window._syncSearch()">⚡ Sincronizar Índices</button>
    </div>
  </div>

  <!-- Buscador principal -->
  <div class="data-card anim-2" style="padding:28px">
    <div style="max-width:600px;margin:0 auto">
      <div style="font-size:13px;color:var(--text-400);text-align:center;margin-bottom:16px;font-weight:600">
        Busca en toda la base de datos — productos, clientes, facturas
      </div>
      <div style="display:flex;gap:10px;align-items:center">
        <input id="search-query" class="search-input" placeholder="Escribe para buscar…"
          style="flex:1;font-size:15px;padding:12px 16px"
          autofocus>
        <button class="btn btn-primary" style="padding:12px 20px" onclick="window._buscar()">
          Buscar
        </button>
      </div>
      <div id="search-tabs" style="display:flex;gap:8px;margin-top:14px;flex-wrap:wrap"></div>
    </div>
  </div>

  <!-- Resultados -->
  <div id="search-results" class="anim-3" style="margin-top:16px"></div>

  <!-- Estado del índice -->
  <div id="index-status" class="anim-4" style="margin-top:16px"></div>`)

  // Enter key trigger
  document.getElementById('search-query')?.addEventListener('keydown', e => {
    if (e.key === 'Enter') window._buscar()
  })

  // Auto-buscar al tipear (debounced)
  let timer
  document.getElementById('search-query')?.addEventListener('input', e => {
    clearTimeout(timer)
    if (e.target.value.length < 2) return
    timer = setTimeout(() => window._buscar(), 500)
  })

  // Cargar estado del índice
  await checkIndexStatus()

  window._buscar = buscar
  window._checkStatus = checkIndexStatus
  window._syncSearch = syncSearch
}

async function buscar() {
  const q = document.getElementById('search-query')?.value?.trim()
  if (!q || q.length < 2) return

  const results = document.getElementById('search-results')
  if (results) results.innerHTML = `
    <div class="data-card" style="padding:20px;text-align:center;color:var(--text-400)">
      <div class="spinner" style="margin:0 auto 8px"></div>
      <div>Buscando "${q}"…</div>
    </div>`

  try {
    // Buscar en los endpoints disponibles en paralelo
    const [ventasRes, productsRes, partnersRes] = await Promise.allSettled([
      api.ventas(1).then(r => (r?.data || []).filter(v =>
        (v.name||'').toLowerCase().includes(q.toLowerCase()) ||
        (v.partner_name||'').toLowerCase().includes(q.toLowerCase())
      ).map(v => ({ tipo:'Venta', icon:'💰', titulo: v.name, sub: v.partner_name, meta: `$${v.amount_total}`, href:'ventas' }))),
      api.productos(1, q).then(r => (r?.data || []).map(p => ({
        tipo:'Producto', icon:'📦',
        titulo: typeof p.name === 'object' ? (p.name?.es_MX || p.name?.en_US || '') : (p.name || ''),
        sub: p.categ_name || '',
        meta: '',
        href:'productos'
      }))),
      api.partners(1).then(r => (r?.data || []).filter(p =>
        (p.name||'').toLowerCase().includes(q.toLowerCase()) ||
        (p.email||'').toLowerCase().includes(q.toLowerCase())
      ).map(p => ({ tipo:'Contacto', icon:'👥', titulo: p.name, sub: p.email || '', meta: '', href:'partners' }))),
    ])

    const allResults = [
      ...(ventasRes.status === 'fulfilled' ? ventasRes.value : []),
      ...(productsRes.status === 'fulfilled' ? productsRes.value : []),
      ...(partnersRes.status === 'fulfilled' ? partnersRes.value : []),
    ]

    if (!results) return

    if (allResults.length === 0) {
      results.innerHTML = `
      <div class="data-card" style="padding:40px;text-align:center">
        <div style="font-size:36px;margin-bottom:12px">🔍</div>
        <div style="font-weight:700;color:var(--text-700)">Sin resultados para "${q}"</div>
        <div style="color:var(--text-400);font-size:13px;margin-top:6px">Prueba con otro término</div>
      </div>`
      return
    }

    results.innerHTML = `
    <div class="data-card">
      <div class="data-card-header">
        <div class="data-card-title">${allResults.length} resultados para "${q}"</div>
      </div>
      <div style="padding:0 4px">
        ${allResults.slice(0, 30).map(r => `
        <div style="display:flex;align-items:center;gap:12px;padding:12px 8px;
          border-bottom:1px solid var(--border);cursor:pointer;border-radius:8px;
          transition:background var(--t1)" 
          onmouseover="this.style.background='var(--primary-light)'"
          onmouseout="this.style.background=''"
          onclick="window._go('${r.href}')">
          <div style="width:36px;height:36px;border-radius:10px;background:var(--primary-light);
            display:flex;align-items:center;justify-content:center;font-size:18px;flex-shrink:0">
            ${r.icon}
          </div>
          <div style="flex:1">
            <div style="font-weight:600;color:var(--text-800);font-size:13px">${r.titulo}</div>
            <div style="font-size:11px;color:var(--text-400)">${r.sub}</div>
          </div>
          <div style="display:flex;align-items:center;gap:8px">
            ${r.meta ? `<span style="font-size:12px;font-weight:700;color:var(--text-700)">${r.meta}</span>` : ''}
            <span class="badge badge-${r.tipo==='Venta'?'indigo':r.tipo==='Producto'?'emerald':'violet'}">${r.tipo}</span>
          </div>
        </div>`).join('')}
      </div>
    </div>`

  } catch (err) {
    console.error(err)
    if (results) results.innerHTML = `<p style="color:var(--red);padding:20px">Error: ${err.message}</p>`
  }
}

async function checkIndexStatus() {
  const statusEl = document.getElementById('index-status')
  try {
    const res = await api.searchStatus().catch(() => null)
    _indexStatus = res?.data || res

    if (statusEl && _indexStatus) {
      statusEl.innerHTML = `
      <div class="data-card" style="padding:16px">
        <div class="data-card-header" style="padding:0 0 12px">
          <div class="data-card-title">📡 Estado del Motor de Búsqueda</div>
        </div>
        <div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(180px,1fr));gap:12px">
          ${Object.entries(_indexStatus).map(([k, v]) => `
          <div style="padding:12px;background:var(--bg);border-radius:10px;border:1px solid var(--border)">
            <div style="font-size:10px;color:var(--text-400);font-weight:700;text-transform:uppercase;margin-bottom:4px">${k}</div>
            <div style="font-size:13px;font-weight:600;color:var(--text-800)">${JSON.stringify(v)}</div>
          </div>`).join('')}
        </div>
      </div>`
    }
  } catch (e) {
    if (statusEl) statusEl.innerHTML = ''
  }
}

async function syncSearch() {
  const btn = document.getElementById('btn-sync')
  if (btn) { btn.textContent = '⏳ Sincronizando…'; btn.disabled = true }
  try {
    const res = await api.searchSync()
    toast('Sincronización iniciada', res?.message || 'Los índices se están actualizando', 'success')
  } catch (e) {
    toast('Error de sincronización', e.message, 'error')
  } finally {
    if (btn) { btn.textContent = '⚡ Sincronizar Índices'; btn.disabled = false }
  }
}
