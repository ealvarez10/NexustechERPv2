import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { fmtMxn, fmtDate, paginationHtml, skeletonTable, toast,
         openDetailModal, detailRow, detailSection } from '../ui.js'
import { api } from '../api.js'
import { editarEmpleado } from './forms/edit_forms.js'

let _page = 1

export async function renderNomina() {
  ensureLayout()
  setBreadcrumb([{label:'Dashboard',href:'dashboard'},{label:'Nómina IMSS'}])
  _page = 1
  await loadNomina()
}

async function loadNomina() {
  setPage(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">👔 Nómina IMSS</h1>
      <p class="page-subtitle" id="nom-sub">Cargando nómina…</p>
    </div>
    <div class="page-actions">
      <input type="text" id="buscar-nom" class="search-input" placeholder="🔍 Buscar empleado…" style="width:220px">
      <button class="btn btn-primary" onclick="alert('Alta de empleado — próximamente')">+ Nuevo Empleado</button>
    </div>
  </div>

  <!-- KPIs -->
  <div style="display:grid;grid-template-columns:repeat(4,1fr);gap:12px;margin-bottom:18px" id="kpi-row" class="anim-2">
    ${[1,2,3,4].map(() => `<div class="data-card" style="padding:16px"><div class="skeleton" style="height:40px"></div></div>`).join('')}
  </div>

  <div class="data-card anim-3">
    <div class="data-card-header">
      <div class="data-card-title">Directorio de Empleados</div>
    </div>
    <div id="nom-tabla">${skeletonTable(8, 5)}</div>
  </div>`)

  try {
    const [kpisRes, listRes] = await Promise.allSettled([
      api.nominaKpis(),
      api.nomina(_page),
    ])

    // KPIs
    const kpis = kpisRes.status === 'fulfilled' ? kpisRes.value?.data : null
    const kpiRow = document.getElementById('kpi-row')
    if (kpiRow) {
      kpiRow.innerHTML = [
        { label: 'Total Empleados',  val: kpis?.total_empleados ?? 0,   tipo:'num', color:'indigo',  icon:'👥' },
        { label: 'Activos',          val: kpis?.activos          ?? 0,   tipo:'num', color:'emerald', icon:'✅' },
        { label: 'Departamentos',    val: kpis?.departamentos     ?? 0,   tipo:'num', color:'violet',  icon:'🏢' },
        { label: 'Nómina Mensual',   val: kpis?.nomina_mensual    ?? 0,   tipo:'mxn', color:'amber',   icon:'💰' },
      ].map(k => `
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${k.icon} ${k.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${k.tipo === 'mxn' ? fmtMxn(parseFloat(k.val)) : Number(k.val).toLocaleString('es-MX')}
        </div>
      </div>`).join('')
    }

    // Tabla
    const empleados = listRes.status === 'fulfilled' ? (listRes.value?.data || []) : []
    const total = listRes.value?.total ?? empleados.length
    const hasMore = empleados.length >= 20

    const sub = document.getElementById('nom-sub')
    if (sub) sub.textContent = `${total} empleados · Página ${_page}`

    const tablaEl = document.getElementById('nom-tabla')
    if (tablaEl) {
      if (empleados.length === 0) {
        tablaEl.innerHTML = `
        <div style="text-align:center;padding:60px 24px">
          <div style="font-size:48px;margin-bottom:16px">👔</div>
          <div style="font-size:16px;font-weight:700;color:var(--text-700);margin-bottom:8px">Sin empleados registrados</div>
          <div style="font-size:13px;color:var(--text-400)">Agrega empleados para gestionar tu nómina</div>
        </div>`
      } else {
        tablaEl.innerHTML = `
        <table class="data-table">
          <thead><tr>
            <th>Empleado</th><th>Puesto</th><th>Departamento</th>
            <th>IMSS</th><th>Estado</th>
          </tr></thead>
          <tbody>
            ${empleados.map(e => {
              const activo = e.active !== false
              const initials = (e.name || '?').split(' ').map(w=>w[0]).slice(0,2).join('')
              return `
              <tr style="cursor:pointer" onclick="window._verEmpleado(${e.id})" title="Ver detalle">
                <td>
                  <div style="display:flex;align-items:center;gap:10px">
                    <div style="width:34px;height:34px;border-radius:50%;background:linear-gradient(135deg,hsl(${(e.id*47)%360},60%,55%),hsl(${(e.id*89)%360},70%,45%));display:flex;align-items:center;justify-content:center;color:white;font-size:12px;font-weight:700;flex-shrink:0">
                      ${initials}
                    </div>
                    <div class="td-primary">${e.name || '—'}</div>
                  </div>
                </td>
                <td style="color:var(--text-600)">${e.job_title || e.job_id_name || '—'}</td>
                <td style="color:var(--text-500);font-size:12px">${e.department_name || e.department_id_name || '—'}</td>
                <td class="td-mono" style="font-size:11px">${e.ssnid || e.imss || '—'}</td>
                <td><span class="badge badge-${activo ? 'emerald' : 'gray'}">${activo ? 'Activo' : 'Baja'}</span></td>
              </tr>`
            }).join('')}
          </tbody>
        </table>
        ${paginationHtml(_page, hasMore, (p) => { _page = p; loadNomina() })}`
      }
    }

    // Búsqueda
    document.getElementById('buscar-nom')?.addEventListener('input', e => {
      const q = e.target.value.toLowerCase()
      document.querySelectorAll('#nom-tabla tbody tr').forEach(r => {
        r.style.display = r.textContent.toLowerCase().includes(q) ? '' : 'none'
      })
    })

    // Ver detalle empleado
    window._verEmpleado = (id) => {
      const emp = empleados.find(e => e.id === id)
      if (!emp) return
      openDetailModal(
        'Detalle de Empleado',
        async () => emp,
        (e) => `
        ${detailSection('Información', [
          detailRow('Nombre completo', e.name),
          detailRow('Puesto', e.job_title || e.job_id_name || '—'),
          detailRow('Departamento', e.department_name || e.department_id_name || '—'),
          detailRow('Estado', `<span class="badge badge-${e.active!==false?'emerald':'gray'}">${e.active!==false?'Activo':'Baja'}</span>`),
        ].join(''))}
        ${detailSection('IMSS / Fiscal', [
          detailRow('N° IMSS', e.ssnid || e.imss || '—'),
          detailRow('RFC', e.rfc || '—'),
          detailRow('CURP', e.curp || '—'),
        ].join(''))}
        ${detailSection('Contacto', [
          detailRow('Email', e.work_email || e.email || '—'),
          detailRow('Teléfono', e.work_phone || e.mobile_phone || '—'),
        ].join(''))}
        <div style="display:flex;gap:10px;margin-top:16px">
          <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
          <button class="btn btn-secondary btn-sm" onclick="window._editarEmpleadoFn(${e.id})">✏️ Editar</button>
          <button class="btn btn-primary btn-sm" onclick="alert('Recibo de nómina — próximamente')">📄 Ver recibo</button>
        </div>`
      )
    }

    window._editarEmpleadoFn = (id) => {
      const emp = empleados.find(e => e.id === id)
      if (emp) editarEmpleado(emp, () => loadNomina())
    }

  } catch (err) {
    console.error(err)
    toast('Error al cargar nómina', err.message, 'error')
  }
}
