import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { fmtMxn, fmtDate, paginationHtml, skeletonTable, toast,
         openModal, openDetailModal, detailRow, detailSection, stateBadge } from '../ui.js'
import { api } from '../api.js'

let _page = 1
let _tab  = 'historial'  // 'historial' | 'timbrar'

export async function renderCfdi() {
  ensureLayout()
  setBreadcrumb([{label:'Dashboard',href:'dashboard'},{label:'CFDI 4.0'}])
  _page = 1
  await loadCfdi()
}

async function loadCfdi() {
  setPage(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">🔏 CFDI 4.0</h1>
      <p class="page-subtitle" id="cfdi-sub">Comprobantes Fiscales Digitales</p>
    </div>
    <div class="page-actions">
      <button class="btn ${_tab==='historial'?'btn-primary':'btn-secondary'}"
        onclick="window._cfdiTab('historial')">📋 Historial</button>
      <button class="btn ${_tab==='timbrar'?'btn-primary':'btn-secondary'}"
        onclick="window._cfdiTab('timbrar')">➕ Timbrar</button>
    </div>
  </div>

  <!-- KPI row -->
  <div style="display:grid;grid-template-columns:repeat(4,1fr);gap:12px;margin-bottom:18px" id="kpi-row" class="anim-2">
    ${[1,2,3,4].map(() => `<div class="data-card" style="padding:16px"><div class="skeleton" style="height:40px"></div></div>`).join('')}
  </div>

  <div class="data-card anim-3" id="cfdi-content">
    <div id="cfdi-body">${skeletonTable(6, 6)}</div>
  </div>`)

  window._cfdiTab = (tab) => { _tab = tab; loadCfdi() }

  try {
    // KPIs
    const kpisRes = await api.cfdiKpis().catch(() => null)
    const kpis = kpisRes?.data
    const kpiRow = document.getElementById('kpi-row')
    if (kpiRow) {
      kpiRow.innerHTML = [
        { label: 'Total Timbrados', val: kpis?.total_timbrados ?? 0,  tipo:'num', color:'indigo',  icon:'🧾' },
        { label: 'Vigentes',        val: kpis?.vigentes         ?? 0,  tipo:'num', color:'emerald', icon:'✅' },
        { label: 'Cancelados',      val: kpis?.cancelados       ?? 0,  tipo:'num', color:'red',     icon:'❌' },
        { label: 'Monto Total',     val: kpis?.monto_total      ?? 0,  tipo:'mxn', color:'violet',  icon:'💰' },
      ].map(k => `
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${k.icon} ${k.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${k.tipo === 'mxn' ? fmtMxn(parseFloat(k.val)) : Number(k.val).toLocaleString('es-MX')}
        </div>
      </div>`).join('')
    }

    if (_tab === 'historial') {
      await loadHistorial()
    } else {
      loadFormTimbrado()
    }
  } catch (err) {
    console.error(err)
    toast('Error CFDI', err.message, 'error')
  }
}

async function loadHistorial() {
  const body = document.getElementById('cfdi-body')
  if (body) body.innerHTML = skeletonTable(6, 6)

  const res = await api.cfdiTimbrados(_page).catch(() => ({ data: [], total: 0 }))
  const items = res?.data || []
  const total = res?.total ?? items.length
  const hasMore = items.length >= 20

  const sub = document.getElementById('cfdi-sub')
  if (sub) sub.textContent = `${total} CFDIs timbrados · Página ${_page}`

  if (!body) return
  if (items.length === 0) {
    body.innerHTML = `
    <div style="text-align:center;padding:60px 24px">
      <div style="font-size:48px;margin-bottom:16px">🧾</div>
      <div style="font-size:16px;font-weight:700;color:var(--text-700);margin-bottom:8px">Sin CFDIs timbrados</div>
      <div style="font-size:13px;color:var(--text-400);margin-bottom:20px">Usa el botón "Timbrar" para crear tu primer comprobante</div>
      <button class="btn btn-primary" onclick="window._cfdiTab('timbrar')">➕ Timbrar CFDI</button>
    </div>`
    return
  }

  body.innerHTML = `
  <div class="data-card-header">
    <div class="data-card-title">Historial de CFDIs</div>
  </div>
  <table class="data-table">
    <thead><tr>
      <th>UUID</th><th>Folio</th><th>Receptor</th>
      <th>Total</th><th>Tipo</th><th>Estado</th><th>Fecha</th>
    </tr></thead>
    <tbody>
      ${items.map(c => {
        const color = c.estado === 'vigente' ? 'emerald' : c.estado === 'cancelado' ? 'red' : 'gray'
        return `
        <tr style="cursor:pointer" onclick="window._verCfdi('${c.uuid}')" title="Ver detalle">
          <td class="td-mono" style="font-size:11px">${c.uuid.substring(0,18)}…</td>
          <td class="td-mono">${c.serie || ''}${c.folio || '—'}</td>
          <td class="td-primary">${c.nombre_receptor || c.rfc_receptor}</td>
          <td class="td-amount" style="font-weight:700">${fmtMxn(parseFloat(c.total || 0))}</td>
          <td><span class="badge badge-sky">${c.tipo_cfdi === 'I' ? 'Ingreso' : c.tipo_cfdi === 'E' ? 'Egreso' : c.tipo_cfdi || '—'}</span></td>
          <td><span class="badge badge-${color}">${c.estado || '—'}</span></td>
          <td style="font-size:12px">${fmtDate(c.fecha_timbrado || c.created_at)}</td>
        </tr>`
      }).join('')}
    </tbody>
  </table>
  ${paginationHtml(_page, hasMore, (p) => { _page = p; loadHistorial() })}`

  window._verCfdi = (uuid) => {
    openDetailModal(
      'Detalle CFDI',
      () => api.cfdiTimbrado(uuid),
      (c) => `
      ${detailSection('Comprobante', [
        detailRow('UUID', `<span style="font-family:monospace;font-size:11px">${c.uuid}</span>`),
        detailRow('Serie / Folio', `${c.serie || ''}${c.folio || '—'}`),
        detailRow('Tipo', c.tipo_cfdi === 'I' ? 'Ingreso' : c.tipo_cfdi === 'E' ? 'Egreso' : c.tipo_cfdi),
        detailRow('Estado', `<span class="badge badge-${c.estado==='vigente'?'emerald':'red'}">${c.estado}</span>`),
        detailRow('Fecha emisión', fmtDate(c.fecha_emision)),
        detailRow('Fecha timbrado', fmtDate(c.fecha_timbrado)),
      ].join(''))}
      ${detailSection('Partes', [
        detailRow('RFC Emisor', c.rfc_emisor),
        detailRow('Emisor', c.nombre_emisor || '—'),
        detailRow('RFC Receptor', c.rfc_receptor),
        detailRow('Receptor', c.nombre_receptor || '—'),
      ].join(''))}
      ${detailSection('Importes', [
        detailRow('Total', `<strong>${fmtMxn(parseFloat(c.total || 0))}</strong>`, {color:'var(--primary)'}),
      ].join(''))}
      <div style="display:flex;gap:10px;margin-top:16px">
        <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
        ${c.estado === 'vigente' ? `<button class="btn btn-danger btn-sm" onclick="window._cancelarCfdi('${c.uuid}')">❌ Cancelar</button>` : ''}
      </div>`
    )
  }

  window._cancelarCfdi = async (uuid) => {
    if (!confirm(`¿Cancelar el CFDI ${uuid.substring(0,18)}…?`)) return
    try {
      await api.cancelarCfdi({ uuid, rfc_emisor: '', motivo: '02' })
      toast('CFDI cancelado', uuid, 'success')
      window.__closeModal()
      loadHistorial()
    } catch (e) {
      toast('Error al cancelar', e.message, 'error')
    }
  }
}

function loadFormTimbrado() {
  const body = document.getElementById('cfdi-body')
  if (!body) return

  body.innerHTML = `
  <div class="data-card-header">
    <div class="data-card-title">Timbrar CFDI 4.0</div>
  </div>
  <div style="padding:8px 0">
    <div class="modal-form-grid" style="grid-template-columns:1fr 1fr 1fr;">
      <div>
        <label class="modal-form-label">Serie</label>
        <input id="f-serie" class="modal-form-input" placeholder="A" value="A">
      </div>
      <div>
        <label class="modal-form-label">Folio</label>
        <input id="f-folio" class="modal-form-input" placeholder="001" value="001">
      </div>
      <div>
        <label class="modal-form-label">Tipo CFDI</label>
        <select id="f-tipo" class="modal-form-select">
          <option value="I">Ingreso</option>
          <option value="E">Egreso</option>
          <option value="P">Pago</option>
        </select>
      </div>
      <div>
        <label class="modal-form-label">RFC Emisor</label>
        <input id="f-rfc-emisor" class="modal-form-input" placeholder="XAXX010101000">
      </div>
      <div>
        <label class="modal-form-label">Nombre Emisor</label>
        <input id="f-nombre-emisor" class="modal-form-input" placeholder="Mi Empresa SA de CV">
      </div>
      <div>
        <label class="modal-form-label">Régimen Fiscal</label>
        <select id="f-regimen" class="modal-form-select">
          <option value="601">601 — General Ley Personas Morales</option>
          <option value="603">603 — Personas Morales con Fines no Lucrativos</option>
          <option value="612">612 — Personas Físicas con Actividades Empresariales</option>
          <option value="626">626 — Simplificado de Confianza</option>
        </select>
      </div>
      <div>
        <label class="modal-form-label">RFC Receptor</label>
        <input id="f-rfc-receptor" class="modal-form-input" placeholder="XAXX010101000">
      </div>
      <div>
        <label class="modal-form-label">Nombre Receptor</label>
        <input id="f-nombre-receptor" class="modal-form-input" placeholder="Cliente SA de CV">
      </div>
      <div>
        <label class="modal-form-label">Uso CFDI (receptor)</label>
        <select id="f-uso" class="modal-form-select">
          <option value="G01">G01 — Adquisición de mercancias</option>
          <option value="G03">G03 — Gastos en general</option>
          <option value="I01">I01 — Construcciones</option>
          <option value="S01">S01 — Sin efectos fiscales</option>
        </select>
      </div>
      <div>
        <label class="modal-form-label">Subtotal</label>
        <input id="f-subtotal" type="number" class="modal-form-input" placeholder="0.00" step="0.01">
      </div>
      <div>
        <label class="modal-form-label">IVA (16%)</label>
        <input id="f-iva" type="number" class="modal-form-input" placeholder="0.00" step="0.01" readonly>
      </div>
      <div>
        <label class="modal-form-label">Total</label>
        <input id="f-total" type="number" class="modal-form-input" placeholder="0.00" readonly style="font-weight:700;color:var(--primary)">
      </div>
      <div style="grid-column:1/-1">
        <label class="modal-form-label">Concepto / Descripción</label>
        <textarea id="f-concepto" class="modal-form-textarea" placeholder="Descripción del producto o servicio facturado…" rows="2"></textarea>
      </div>
      <div>
        <label class="modal-form-label">Certificado (.cer) en Base64</label>
        <input id="f-cer" type="file" accept=".cer" class="modal-form-input" style="padding:6px">
      </div>
      <div>
        <label class="modal-form-label">Clave privada (.key) en Base64</label>
        <input id="f-key" type="file" accept=".key" class="modal-form-input" style="padding:6px">
      </div>
      <div>
        <label class="modal-form-label">Contraseña del CSD</label>
        <input id="f-pwd" type="password" class="modal-form-input" placeholder="••••••••">
      </div>
    </div>
    <div style="display:flex;gap:10px;margin-top:8px;justify-content:flex-end">
      <button class="btn btn-secondary" onclick="window._cfdiTab('historial')">Cancelar</button>
      <button class="btn btn-primary" id="btn-timbrar" onclick="window._timbrar()">🔏 Timbrar CFDI</button>
    </div>
    <div id="cfdi-resultado" style="margin-top:16px"></div>
  </div>`

  // Auto-calcular IVA y total
  document.getElementById('f-subtotal')?.addEventListener('input', e => {
    const sub = parseFloat(e.target.value) || 0
    const iva = sub * 0.16
    document.getElementById('f-iva').value = iva.toFixed(2)
    document.getElementById('f-total').value = (sub + iva).toFixed(2)
  })

  window._timbrar = async () => {
    const btn = document.getElementById('btn-timbrar')
    btn.textContent = '⏳ Timbrando…'
    btn.disabled = true
    const result = document.getElementById('cfdi-resultado')

    try {
      const cerFile = document.getElementById('f-cer')?.files[0]
      const keyFile = document.getElementById('f-key')?.files[0]

      const toB64 = (f) => new Promise((res, rej) => {
        if (!f) { res(''); return }
        const r = new FileReader()
        r.onload = e => res(e.target.result.split(',')[1] || '')
        r.onerror = rej
        r.readAsDataURL(f)
      })

      const [certB64, keyB64] = await Promise.all([toB64(cerFile), toB64(keyFile)])

      const sub = parseFloat(document.getElementById('f-subtotal')?.value) || 0
      const iva = sub * 0.16

      const payload = {
        cfdi: {
          serie: document.getElementById('f-serie')?.value || 'A',
          folio: document.getElementById('f-folio')?.value || '1',
          tipo_comprobante: document.getElementById('f-tipo')?.value || 'I',
          emisor: {
            rfc: document.getElementById('f-rfc-emisor')?.value || '',
            nombre: document.getElementById('f-nombre-emisor')?.value || '',
            regimen_fiscal: document.getElementById('f-regimen')?.value || '601',
          },
          receptor: {
            rfc: document.getElementById('f-rfc-receptor')?.value || '',
            nombre: document.getElementById('f-nombre-receptor')?.value || '',
            uso_cfdi: document.getElementById('f-uso')?.value || 'G03',
            domicilio_fiscal_receptor: '64000',
            regimen_fiscal_receptor: '601',
          },
          conceptos: [{
            clave_prod_serv: '84111506',
            descripcion: document.getElementById('f-concepto')?.value || 'Servicio',
            cantidad: '1',
            unidad: 'ACT',
            valor_unitario: sub.toFixed(2),
            importe: sub.toFixed(2),
            impuestos: { traslados: [{ base: sub.toFixed(2), impuesto: '002', tipo_factor: 'Tasa', tasa: '0.160000', importe: iva.toFixed(2) }] },
          }],
          subtotal: sub.toFixed(2),
          total: (sub + iva).toFixed(2),
          moneda: 'MXN',
          lugar_expedicion: '64000',
        },
        cert_b64: certB64,
        key_b64: keyB64,
        password: document.getElementById('f-pwd')?.value || '',
      }

      const res = await api.timbrar(payload)
      if (res?.success) {
        result.innerHTML = `
        <div style="background:var(--success-light);border:1.5px solid var(--success);border-radius:12px;padding:16px">
          <div style="font-weight:700;color:var(--success);margin-bottom:8px">✅ CFDI Timbrado</div>
          <div style="font-size:12px;font-family:monospace;word-break:break-all;color:var(--text-600)">UUID: ${res.uuid}</div>
          <div style="font-size:12px;color:var(--text-500);margin-top:4px">Fecha: ${fmtDate(res.fecha_timbrado)}</div>
        </div>`
        toast('CFDI timbrado', `UUID: ${res.uuid}`, 'success')
      } else {
        result.innerHTML = `<div style="background:var(--danger-light,#fee2e2);border:1.5px solid var(--red);border-radius:12px;padding:16px;color:var(--red)">
          ❌ Error: ${res?.error || 'Error desconocido'}</div>`
      }
    } catch (e) {
      result.innerHTML = `<div style="background:#fee2e2;border:1.5px solid var(--red);border-radius:12px;padding:16px;color:var(--red)">❌ ${e.message}</div>`
    } finally {
      btn.textContent = '🔏 Timbrar CFDI'
      btn.disabled = false
    }
  }
}
