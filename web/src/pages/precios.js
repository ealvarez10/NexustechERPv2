import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { api } from '../api.js'
import { toast, fmtMxn, skeletonTable } from '../ui.js'

const MOCK = [
  { id:1, name:'Tarifa General', currency:'MXN', type:'Porcentaje', active:true, discount:0 },
  { id:2, name:'Distribuidores', currency:'MXN', type:'Porcentaje', active:true, discount:10 },
  { id:3, name:'Exportación USD', currency:'USD', type:'Fijo', active:false, discount:0 },
]

// Las listas se comparten con el formulario de Ventas vía localStorage (selector "Lista de Precios")
const STORE_KEY = 'nexus_pricelists'

let _records = [], _search = ''

function _persistir() {
  localStorage.setItem(STORE_KEY, JSON.stringify(_records))
}

export async function renderPrecios(params = {}) {
  ensureLayout()
  setBreadcrumb([{ label:'Ventas', href:'#ventas' }, { label:'Precios Especiales' }])
  setPage(`<div class="nx-module-page">
    <div class="o-cp">
      <div class="o-cp-left">
        <button class="o-btn-primary" onclick="window._nprecio()">+ Nueva Lista</button>
      </div>
      <div class="o-cp-center">
        <div class="o-search-bar">
          <span class="o-search-icon">🔍</span>
          <input class="o-search-input" placeholder="Buscar lista de precios..." oninput="window._sprecio(this.value)">
        </div>
      </div>
      <div class="o-cp-right">
        <span id="pc-count" style="font-size:12px;color:var(--text-400)"></span>
      </div>
    </div>
    <div id="pc-content">${skeletonTable(3,5)}</div>
  </div>`)

  let guardadas = null
  try { guardadas = JSON.parse(localStorage.getItem(STORE_KEY) || 'null') } catch { guardadas = null }
  if (Array.isArray(guardadas) && guardadas.length) {
    _records = guardadas
  } else {
    try {
      const res = await api.get('/precios')
      _records = res?.data?.length ? res.data : MOCK
    } catch { _records = MOCK }
    _persistir()
  }

  window._sprecio = (q) => { _search = q; _renderPrecios() }
  window._nprecio = _modalNueva
  window._editprecio = _verDetalle
  _renderPrecios()
}

function _renderPrecios() {
  const c = document.getElementById('pc-content')
  if (!c) return
  const cnt = document.getElementById('pc-count')
  const rows = _search ? _records.filter(r => r.name.toLowerCase().includes(_search.toLowerCase())) : _records
  if (cnt) cnt.textContent = `${rows.length} registros`
  if (!rows.length) {
    c.innerHTML = `<div style="padding:60px;text-align:center;color:var(--text-400)"><div style="font-size:48px;margin-bottom:12px">🏷️</div><div style="font-size:16px;font-weight:600">Sin listas de precios</div><div style="font-size:13px;margin-top:8px">Crea la primera con + Nueva Lista</div></div>`
    return
  }
  c.innerHTML = `
    <div class="o-list-view">
      <table class="o-list-table">
        <thead><tr>
          <th>Nombre</th><th>Moneda</th><th>Tipo</th><th>Descuento %</th><th>Estado</th>
        </tr></thead>
        <tbody>
          ${rows.map(r => `
          <tr class="o-list-row" onclick="window._editprecio(${r.id})" style="cursor:pointer">
            <td><strong>${r.name}</strong></td>
            <td>${r.currency || 'MXN'}</td>
            <td>${r.type || 'Porcentaje'}</td>
            <td>${r.discount ?? 0}%</td>
            <td><span class="o-badge ${r.active ? 'o-badge-success' : 'o-badge-gray'}">${r.active ? 'Activa' : 'Inactiva'}</span></td>
          </tr>`).join('')}
        </tbody>
      </table>
    </div>`
}

function _verDetalle(id) {
  const r = _records.find(x => x.id === id)
  if (!r) return
  const c = document.getElementById('pc-content')
  if (!c) return
  c.innerHTML = `
    <div style="max-width:760px;margin:24px auto;background:var(--bg-card);border-radius:12px;border:1px solid var(--border);padding:28px">
      <div style="display:flex;align-items:center;gap:12px;margin-bottom:20px">
        <button onclick="window._go('precios')" class="o-btn-secondary o-btn-sm">← Volver</button>
        <h2 style="margin:0;font-size:18px;font-weight:700">${r.name}</h2>
      </div>
      <div style="display:grid;grid-template-columns:1fr 1fr;gap:16px;margin-bottom:20px">
        <div class="o-field-row"><div class="o-field-label">Nombre</div><div class="o-field-value"><input class="o-input" id="pc-name" value="${r.name}"></div></div>
        <div class="o-field-row"><div class="o-field-label">Moneda</div><div class="o-field-value">
          <select class="o-select" id="pc-cur"><option ${r.currency==='MXN'?'selected':''}>MXN</option><option ${r.currency==='USD'?'selected':''}>USD</option></select>
        </div></div>
        <div class="o-field-row"><div class="o-field-label">Tipo</div><div class="o-field-value">
          <select class="o-select" id="pc-type"><option ${r.type==='Porcentaje'?'selected':''}>Porcentaje</option><option ${r.type==='Fijo'?'selected':''}>Fijo</option></select>
        </div></div>
        <div class="o-field-row"><div class="o-field-label">Descuento %</div><div class="o-field-value"><input type="number" class="o-input" id="pc-disc" value="${r.discount??0}" min="0" max="100"></div></div>
        <div class="o-field-row"><div class="o-field-label">Activa</div><div class="o-field-value"><input type="checkbox" id="pc-active" ${r.active?'checked':''}></div></div>
      </div>
      <div style="display:flex;gap:8px">
        <button class="o-btn-primary" onclick="window._saveprecio(${r.id})">💾 Guardar</button>
        <button class="o-btn-secondary o-btn-sm" onclick="window._go('precios')">Descartar</button>
      </div>
    </div>`
  window._saveprecio = (id) => {
    const idx = _records.findIndex(x => x.id === id)
    if (idx < 0) return
    _records[idx].name = document.getElementById('pc-name').value
    _records[idx].currency = document.getElementById('pc-cur').value
    _records[idx].type = document.getElementById('pc-type').value
    _records[idx].discount = parseFloat(document.getElementById('pc-disc').value)||0
    _records[idx].active = document.getElementById('pc-active').checked
    _persistir()
    toast('Guardado', 'Lista de precios actualizada', 'success')
    window._go('precios')
  }
}

function _modalNueva() {
  const m = document.createElement('div')
  m.style.cssText = 'position:fixed;inset:0;z-index:950;background:rgba(0,0,0,.45);display:flex;align-items:center;justify-content:center;padding:16px'
  m.innerHTML = `
    <div style="background:var(--bg-card);border-radius:14px;border:1px solid var(--border);width:100%;max-width:440px;box-shadow:0 24px 64px rgba(0,0,0,.18)">
      <div style="padding:16px 20px;border-bottom:1px solid var(--border);display:flex;justify-content:space-between;align-items:center">
        <h3 style="margin:0;font-size:15px;font-weight:700">Nueva Lista de Precios</h3>
        <button onclick="this.closest('[style*=fixed]').remove()" style="background:none;border:none;cursor:pointer;font-size:20px">×</button>
      </div>
      <div style="padding:20px;display:flex;flex-direction:column;gap:14px">
        <div><label style="font-size:12px;font-weight:600;color:var(--text-500);text-transform:uppercase">Nombre *</label><input id="np-name" class="o-input" style="margin-top:4px" placeholder="Ej. Tarifa VIP"></div>
        <div><label style="font-size:12px;font-weight:600;color:var(--text-500);text-transform:uppercase">Moneda</label><select id="np-cur" class="o-select" style="margin-top:4px"><option>MXN</option><option>USD</option></select></div>
        <div><label style="font-size:12px;font-weight:600;color:var(--text-500);text-transform:uppercase">Tipo</label><select id="np-type" class="o-select" style="margin-top:4px"><option>Porcentaje</option><option>Fijo</option></select></div>
        <div><label style="font-size:12px;font-weight:600;color:var(--text-500);text-transform:uppercase">Descuento %</label><input id="np-disc" type="number" class="o-input" value="0" min="0" max="100" style="margin-top:4px"></div>
      </div>
      <div style="padding:12px 20px;border-top:1px solid var(--border);display:flex;gap:8px;justify-content:flex-end">
        <button class="o-btn-secondary o-btn-sm" onclick="this.closest('[style*=fixed]').remove()">Cancelar</button>
        <button class="o-btn-primary" onclick="window._crearPrecio()">Crear</button>
      </div>
    </div>`
  document.body.appendChild(m)
  m.onclick = e => { if (e.target === m) m.remove() }
  window._crearPrecio = () => {
    const name = document.getElementById('np-name')?.value?.trim()
    if (!name) { toast('Error', 'El nombre es obligatorio', 'error'); return }
    const nuevo = {
      id: Date.now(),
      name,
      currency: document.getElementById('np-cur').value,
      type: document.getElementById('np-type').value,
      discount: parseFloat(document.getElementById('np-disc').value)||0,
      active: true
    }
    _records.push(nuevo)
    _persistir()
    m.remove()
    toast('Creado', `Lista "${name}" creada`, 'success')
    _renderPrecios()
  }
}
