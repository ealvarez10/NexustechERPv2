import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'

export function renderGeneric(id, title, subtitle, icon) {
  ensureLayout()
  setBreadcrumb([{label:'Dashboard',href:'dashboard'},{label:title}])
  setPage(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">${icon} ${title}</h1>
      <p class="page-subtitle">${subtitle}</p>
    </div>
  </div>
  <div class="data-card anim-2">
    <div class="empty-state">
      <div class="empty-state-icon">${icon}</div>
      <div class="empty-state-title">Módulo ${title} en construcción</div>
      <div class="empty-state-desc">Este módulo estará disponible próximamente en NexusTech ERP v2.0</div>
      <button class="btn btn-primary" onclick="window._go('dashboard')">← Volver al Dashboard</button>
    </div>
  </div>`)
}
