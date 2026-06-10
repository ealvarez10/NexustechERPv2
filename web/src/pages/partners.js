import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { fmtMxn, fmtNum, paginationHtml, skeletonTable, toast,
         openDetailModal, detailRow, detailSection } from '../ui.js'
import { api } from '../api.js'

let _page = 1
let _filtro = '' // 'clientes' | 'proveedores' | ''

export async function renderPartners() {
  ensureLayout()
  setBreadcrumb([{ label: 'Dashboard', href: 'dashboard' }, { label: 'Clientes / Proveedores' }])
  _page = 1
  _filtro = ''
  await loadPartners()
}

async function loadPartners() {
  setPage(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">Clientes y Proveedores</h1>
      <p class="page-subtitle" id="part-sub">Cargando directorio…</p>
    </div>
    <div class="page-actions">
      <input type="text" id="buscar-part" class="search-input" placeholder="🔍 Buscar por nombre…" style="width:220px">
      <div style="display:flex;gap:6px">
        <button class="btn ${_filtro===''?'btn-primary':'btn-secondary'}" id="btn-todos" onclick="window._filterPart('')">Todos</button>
        <button class="btn ${_filtro==='clientes'?'btn-primary':'btn-secondary'}" id="btn-cli" onclick="window._filterPart('clientes')">👥 Clientes</button>
        <button class="btn ${_filtro==='proveedores'?'btn-primary':'btn-secondary'}" id="btn-prov" onclick="window._filterPart('proveedores')">🏭 Proveedores</button>
      </div>
      <button class="btn btn-primary">+ Nuevo Contacto</button>
    </div>
  </div>

  <!-- Stats row -->
  <div style="display:grid;grid-template-columns:repeat(3,1fr);gap:12px;margin-bottom:18px" id="stats-row" class="anim-2">
    ${[1,2,3].map(() => `<div class="data-card" style="padding:16px"><div class="skeleton" style="height:40px"></div></div>`).join('')}
  </div>

  <div class="data-card anim-3">
    <div class="data-card-header">
      <div class="data-card-title">${_filtro === 'clientes' ? '👥 Clientes' : _filtro === 'proveedores' ? '🏭 Proveedores' : '📋 Directorio'}</div>
    </div>
    <div id="part-tabla">${skeletonTable(10, 5)}</div>
  </div>`)

  // Botón filtro handler
  window._filterPart = (f) => { _filtro = f; _page = 1; loadPartners() }

  try {
    let fetchFn
    if (_filtro === 'clientes')    fetchFn = api.clientes(_page)
    else if (_filtro === 'proveedores') fetchFn = api.proveedores(_page)
    else fetchFn = api.partners(_page)

    const [partRes, allRes] = await Promise.allSettled([
      fetchFn,
      api.partners(1),
    ])

    const partners = partRes.status === 'fulfilled' ? (partRes.value?.data || []) : []
    const allPartners = allRes.status === 'fulfilled' ? (allRes.value?.data || []) : partners
    const hasMore = partners.length >= 20

    // Stats
    const statsRow = document.getElementById('stats-row')
    if (statsRow) {
      const clientes = allPartners.filter(p => (p.customer_rank || 0) > 0).length
      const proveedores = allPartners.filter(p => (p.supplier_rank || 0) > 0).length
      statsRow.innerHTML = [
        { label: 'Total Contactos',  val: allPartners.length,  color: 'indigo',  icon: '📋' },
        { label: 'Clientes',         val: clientes,            color: 'emerald', icon: '👥' },
        { label: 'Proveedores',      val: proveedores,         color: 'violet',  icon: '🏭' },
      ].map(s => `
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${s.icon} ${s.label}</div>
        <div style="font-size:26px;font-weight:800;color:var(--text-900)">${fmtNum(s.val)}</div>
      </div>`).join('')
    }

    const sub = document.getElementById('part-sub')
    if (sub) sub.textContent = `${partners.length} contactos · Página ${_page}`

    const tablaEl = document.getElementById('part-tabla')
    if (tablaEl) {
      if (partners.length === 0) {
        tablaEl.innerHTML = '<p style="text-align:center;padding:32px;color:var(--text-400)">Sin contactos registrados</p>'
      } else {
        tablaEl.innerHTML = `
        <table class="data-table">
          <thead><tr>
            <th>Nombre</th><th>Tipo</th><th>Email</th><th>Teléfono</th><th>Tags</th>
          </tr></thead>
          <tbody>
            ${partners.map(p => {
              const esCliente   = (p.customer_rank || 0) > 0
              const esProveedor = (p.supplier_rank || 0) > 0
              const esEmpresa   = p.is_company
              return `
              <tr style="cursor:pointer" onclick="window._verPartner(${p.id})" title="Ver detalle">
                <td>
                  <div style="display:flex;align-items:center;gap:8px">
                    <div style="width:32px;height:32px;border-radius:50%;background:linear-gradient(135deg,hsl(${(p.id * 37) % 360},60%,55%),hsl(${(p.id * 71) % 360},70%,45%));display:flex;align-items:center;justify-content:center;color:white;font-size:13px;font-weight:700;flex-shrink:0">
                      ${(p.name || p.nombre || '?')[0].toUpperCase()}
                    </div>
                    <div>
                      <div class="td-primary">${p.name || p.nombre || '—'}</div>
                      ${esEmpresa ? '<div style="font-size:11px;color:var(--text-400)">Empresa</div>' : ''}
                    </div>
                  </div>
                </td>
                <td>
                  ${esCliente   ? '<span class="badge badge-emerald">Cliente</span>' : ''}
                  ${esProveedor ? '<span class="badge badge-violet" style="margin-left:2px">Proveedor</span>' : ''}
                  ${!esCliente && !esProveedor ? '<span class="badge badge-gray">Contacto</span>' : ''}
                </td>
                <td style="color:var(--text-500);font-size:12.5px">${p.email || '—'}</td>
                <td style="color:var(--text-500);font-size:12.5px">${p.phone || '—'}</td>
                <td>${esEmpresa ? '<span class="badge badge-sky">Empresa</span>' : '<span class="badge badge-gray">Persona</span>'}</td>
              </tr>`
            }).join('')}
          </tbody>
        </table>
        ${paginationHtml(_page, hasMore, (p) => { _page = p; loadPartners() })}`
      }
    }

    // Búsqueda cliente-side
    document.getElementById('buscar-part')?.addEventListener('input', (e) => {
      const q = e.target.value.toLowerCase()
      document.querySelectorAll('#part-tabla tbody tr').forEach(r => {
        r.style.display = r.textContent.toLowerCase().includes(q) ? '' : 'none'
      })
    })

    // Ver detalle del contacto
    window._verPartner = (id) => {
      openDetailModal(
        'Detalle de Contacto',
        () => api.partner(id),
        (p) => {
          const esCliente   = (p.customer_rank || 0) > 0
          const esProveedor = (p.supplier_rank || 0) > 0
          return `
          ${detailSection('Información General', [
            detailRow('Nombre', p.name),
            detailRow('Tipo', p.is_company ? 'Empresa' : 'Persona física'),
            detailRow('Rol', [esCliente ? 'Cliente' : '', esProveedor ? 'Proveedor' : ''].filter(Boolean).join(', ') || 'Contacto'),
            detailRow('RFC', p.vat || '—'),
            detailRow('Website', p.website || '—'),
          ].join(''))}
          ${detailSection('Contacto', [
            detailRow('Email', p.email ? `<a href="mailto:${p.email}" style="color:var(--primary)">${p.email}</a>` : '—'),
            detailRow('Teléfono', p.phone || '—'),
            detailRow('Móvil', p.mobile || '—'),
            detailRow('Ciudad', p.city || '—'),
            detailRow('País', p.country_name || '—'),
          ].join(''))}
          <div style="display:flex;gap:10px;margin-top:16px">
            <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
            <button class="btn btn-primary btn-sm" onclick="alert('Editar contacto — próximamente')">✏️ Editar</button>
          </div>`
        }
      )
    }

  } catch (err) {
    console.error(err)
    toast('Error al cargar contactos', err.message, 'error')
  }
}
