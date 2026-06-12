import{t as i,a as s}from"./index-CIpkvgrW.js";function c(t,l,a,o){var r;(r=document.getElementById("nx-modal-overlay"))==null||r.remove();const e=document.createElement("div");e.id="nx-modal-overlay",e.innerHTML=`
  <div class="nx-modal-backdrop" onclick="window._closeNxModal()"></div>
  <div class="nx-create-modal" role="dialog" aria-modal="true">
    <div class="nx-modal-header">
      <h2 class="nx-modal-title">${l}</h2>
      <button class="nx-modal-close" onclick="window._closeNxModal()">✕</button>
    </div>
    <div class="nx-modal-body" id="nx-modal-body-${t}">${a}</div>
    <div class="nx-modal-footer">
      <button class="o-btn-secondary" onclick="window._closeNxModal()">Cancelar</button>
      <button class="o-btn-primary" id="nx-save-btn" onclick="window._nxSave()">Guardar</button>
    </div>
  </div>`,document.body.appendChild(e),window._closeNxModal=()=>e.remove(),window._nxSave=o,setTimeout(()=>{var p;return(p=e.querySelector("input,select,textarea"))==null?void 0:p.focus()},100)}function n(t){const l=document.getElementById(t);return l?l.type==="checkbox"?l.checked:l.value.trim():""}function d(t){const l=document.getElementById("nx-save-btn");l&&(l.disabled=t,l.textContent=t?"⏳ Guardando…":"Guardar")}function v(t){c("venta","➕ Nueva Orden de Venta",`
  <div class="nx-form-grid">
    <div class="nx-field">
      <label class="nx-label">Cliente <span class="nx-req">*</span></label>
      <input id="nv-partner" class="nx-input" type="text" placeholder="Nombre del cliente" required>
      <small class="nx-hint">Escribe el nombre del cliente (se buscará en el sistema)</small>
    </div>
    <div class="nx-field">
      <label class="nx-label">Fecha del Pedido</label>
      <input id="nv-fecha" class="nx-input" type="date" value="${new Date().toISOString().slice(0,10)}">
    </div>
    <div class="nx-field">
      <label class="nx-label">Referencia del Cliente</label>
      <input id="nv-ref" class="nx-input" type="text" placeholder="PO-2024-001 (opcional)">
    </div>
    <div class="nx-field">
      <label class="nx-label">Validez (días)</label>
      <input id="nv-validez" class="nx-input" type="number" value="30" min="1" max="365">
    </div>
    <div class="nx-field nx-field-full">
      <label class="nx-label">Notas / Condiciones</label>
      <textarea id="nv-notas" class="nx-input nx-textarea" rows="3" placeholder="Términos y condiciones, notas del pedido…"></textarea>
    </div>
  </div>`,async()=>{const l=n("nv-partner");if(!l){i("Requerido","Ingresa el nombre del cliente","warning");return}d(!0);try{const a=await s.get(`/partners?q=${encodeURIComponent(l)}&pagina=1`),o=(a==null?void 0:a.data)||[];if(!o.length){i("Cliente no encontrado",`No existe cliente con nombre "${l}". Créalo primero en Contactos.`,"warning"),d(!1);return}const e=o[0].id,r=await s.post("/ventas",{partner_id:e,client_order_ref:n("nv-ref"),note:n("nv-notas"),validity_date:n("nv-validez")?new Date(Date.now()+parseInt(n("nv-validez"))*864e5).toISOString().slice(0,10):null});if(r!=null&&r.ok||r!=null&&r.id)i("✅","Venta creada","success"),window._closeNxModal(),t==null||t(r);else throw new Error((r==null?void 0:r.error)||"Error al crear")}catch(a){i("Error",a.message,"error")}d(!1)})}function x(t){c("factura","🧾 Nueva Factura",`
  <div class="nx-form-grid">
    <div class="nx-field">
      <label class="nx-label">Cliente <span class="nx-req">*</span></label>
      <input id="nf-partner" class="nx-input" type="text" placeholder="Nombre del cliente" required>
    </div>
    <div class="nx-field">
      <label class="nx-label">Tipo de Documento</label>
      <select id="nf-tipo" class="nx-input">
        <option value="out_invoice">Factura de Cliente</option>
        <option value="out_refund">Nota de Crédito</option>
        <option value="in_invoice">Factura de Proveedor</option>
      </select>
    </div>
    <div class="nx-field">
      <label class="nx-label">Fecha de Factura</label>
      <input id="nf-fecha" class="nx-input" type="date" value="${new Date().toISOString().slice(0,10)}">
    </div>
    <div class="nx-field">
      <label class="nx-label">Fecha de Vencimiento</label>
      <input id="nf-vence" class="nx-input" type="date" value="${new Date(Date.now()+30*864e5).toISOString().slice(0,10)}">
    </div>
    <div class="nx-field">
      <label class="nx-label">Referencia / Número</label>
      <input id="nf-ref" class="nx-input" type="text" placeholder="INV/2024/00001">
    </div>
    <div class="nx-field">
      <label class="nx-label">Uso CFDI</label>
      <select id="nf-uso" class="nx-input">
        <option value="G03">G03 — Gastos en general</option>
        <option value="G01">G01 — Adquisición de mercancias</option>
        <option value="P01">P01 — Por definir</option>
        <option value="S01">S01 — Sin efectos fiscales</option>
      </select>
    </div>
  </div>`,async()=>{const l=n("nf-partner");if(!l){i("Requerido","Ingresa el cliente","warning");return}d(!0);try{const a=await s.get(`/partners?q=${encodeURIComponent(l)}&pagina=1`),o=(a==null?void 0:a.data)||[];if(!o.length){i("Cliente no encontrado","Créalo primero en Contactos","warning"),d(!1);return}const e=await s.post("/facturas",{partner_id:o[0].id,move_type:n("nf-tipo"),invoice_date:n("nf-fecha"),invoice_date_due:n("nf-vence"),ref:n("nf-ref"),l10n_mx_edi_usage:n("nf-uso")});if(e!=null&&e.ok||e!=null&&e.id)i("✅","Factura creada","success"),window._closeNxModal(),t==null||t(e);else throw new Error((e==null?void 0:e.error)||"Error")}catch(a){i("Error",a.message,"error")}d(!1)})}function b(t){c("producto","📦 Nuevo Producto",`
  <div class="nx-form-grid">
    <div class="nx-field nx-field-full">
      <label class="nx-label">Nombre <span class="nx-req">*</span></label>
      <input id="np-nombre" class="nx-input" type="text" placeholder="Nombre del producto" required>
    </div>
    <div class="nx-field">
      <label class="nx-label">Código Interno (SKU)</label>
      <input id="np-sku" class="nx-input" type="text" placeholder="PROD-001">
    </div>
    <div class="nx-field">
      <label class="nx-label">Tipo de Producto</label>
      <select id="np-tipo" class="nx-input">
        <option value="consu">Consumible</option>
        <option value="product">Almacenable</option>
        <option value="service">Servicio</option>
      </select>
    </div>
    <div class="nx-field">
      <label class="nx-label">Precio de Venta</label>
      <input id="np-precio" class="nx-input" type="number" step="0.01" value="0.00" min="0">
    </div>
    <div class="nx-field">
      <label class="nx-label">Costo</label>
      <input id="np-costo" class="nx-input" type="number" step="0.01" value="0.00" min="0">
    </div>
    <div class="nx-field">
      <label class="nx-label">Unidad de Medida</label>
      <select id="np-uom" class="nx-input">
        <option value="1">Unidad</option>
        <option value="2">Kilogramo</option>
        <option value="3">Litro</option>
        <option value="4">Metro</option>
        <option value="5">Caja</option>
        <option value="6">Paquete</option>
      </select>
    </div>
    <div class="nx-field nx-field-full">
      <label class="nx-label">Descripción</label>
      <textarea id="np-desc" class="nx-input nx-textarea" rows="2" placeholder="Descripción del producto…"></textarea>
    </div>
  </div>`,async()=>{const l=n("np-nombre");if(!l){i("Requerido","Ingresa el nombre","warning");return}d(!0);try{const a=await s.post("/productos",{name:l,default_code:n("np-sku"),type:n("np-tipo"),list_price:parseFloat(n("np-precio"))||0,standard_price:parseFloat(n("np-costo"))||0,uom_id:parseInt(n("np-uom"))||1,description_sale:n("np-desc"),active:!0});if(a!=null&&a.ok||a!=null&&a.id)i("✅","Producto creado","success"),window._closeNxModal(),t==null||t(a);else throw new Error((a==null?void 0:a.error)||"Error")}catch(a){i("Error",a.message,"error")}d(!1)})}function f(t="cliente",l){const a=t!=="proveedor";c("partner",a?"👤 Nuevo Cliente":"🏭 Nuevo Proveedor",`
  <div class="nx-form-grid">
    <div class="nx-field nx-field-full">
      <label class="nx-label">Nombre / Razón Social <span class="nx-req">*</span></label>
      <input id="npa-nombre" class="nx-input" type="text" placeholder="Empresa S.A. de C.V." required>
    </div>
    <div class="nx-field">
      <label class="nx-label">RFC</label>
      <input id="npa-rfc" class="nx-input" type="text" placeholder="XAXX010101000" maxlength="13" style="text-transform:uppercase">
    </div>
    <div class="nx-field">
      <label class="nx-label">Tipo</label>
      <select id="npa-tipo" class="nx-input">
        <option value="empresa" ${a?"":"selected"}>Empresa</option>
        <option value="persona">Persona Física</option>
      </select>
    </div>
    <div class="nx-field">
      <label class="nx-label">Teléfono</label>
      <input id="npa-tel" class="nx-input" type="tel" placeholder="+52 55 1234 5678">
    </div>
    <div class="nx-field">
      <label class="nx-label">Email</label>
      <input id="npa-email" class="nx-input" type="email" placeholder="contacto@empresa.com">
    </div>
    <div class="nx-field">
      <label class="nx-label">Ciudad</label>
      <input id="npa-ciudad" class="nx-input" type="text" placeholder="Ciudad de México">
    </div>
    <div class="nx-field nx-field-full">
      <label class="nx-label">Dirección</label>
      <input id="npa-dir" class="nx-input" type="text" placeholder="Calle, No., Col., CP">
    </div>
  </div>`,async()=>{const o=n("npa-nombre");if(!o){i("Requerido","Ingresa el nombre","warning");return}d(!0);try{const e=await s.post("/partners",{name:o,vat:n("npa-rfc").toUpperCase(),phone:n("npa-tel"),email:n("npa-email"),city:n("npa-ciudad"),street:n("npa-dir"),is_company:n("npa-tipo")==="empresa",customer_rank:a?1:0,supplier_rank:a?0:1,active:!0});if(e!=null&&e.ok||e!=null&&e.id)i("✅",`${a?"Cliente":"Proveedor"} creado`,"success"),window._closeNxModal(),l==null||l(e);else throw new Error((e==null?void 0:e.error)||"Error")}catch(e){i("Error",e.message,"error")}d(!1)})}function m(t){c("compra","🛒 Nueva Orden de Compra",`
  <div class="nx-form-grid">
    <div class="nx-field">
      <label class="nx-label">Proveedor <span class="nx-req">*</span></label>
      <input id="nc-prov" class="nx-input" type="text" placeholder="Nombre del proveedor" required>
    </div>
    <div class="nx-field">
      <label class="nx-label">Fecha de Orden</label>
      <input id="nc-fecha" class="nx-input" type="date" value="${new Date().toISOString().slice(0,10)}">
    </div>
    <div class="nx-field">
      <label class="nx-label">Fecha de Entrega Esperada</label>
      <input id="nc-entrega" class="nx-input" type="date" value="${new Date(Date.now()+7*864e5).toISOString().slice(0,10)}">
    </div>
    <div class="nx-field">
      <label class="nx-label">Referencia del Proveedor</label>
      <input id="nc-ref" class="nx-input" type="text" placeholder="Número de cotización del proveedor">
    </div>
    <div class="nx-field nx-field-full">
      <label class="nx-label">Notas</label>
      <textarea id="nc-notas" class="nx-input nx-textarea" rows="2" placeholder="Instrucciones especiales…"></textarea>
    </div>
  </div>`,async()=>{const l=n("nc-prov");if(!l){i("Requerido","Ingresa el proveedor","warning");return}d(!0);try{const a=await s.get(`/partners?q=${encodeURIComponent(l)}&pagina=1`),o=(a==null?void 0:a.data)||[];if(!o.length){i("Proveedor no encontrado","Créalo primero en Contactos","warning"),d(!1);return}const e=await s.post("/compras",{partner_id:o[0].id,date_order:n("nc-fecha"),date_planned:n("nc-entrega"),partner_ref:n("nc-ref"),notes:n("nc-notas")});if(e!=null&&e.ok||e!=null&&e.id)i("✅","Compra creada","success"),window._closeNxModal(),t==null||t(e);else throw new Error((e==null?void 0:e.error)||"Error")}catch(a){i("Error",a.message,"error")}d(!1)})}export{m as nuevaCompra,x as nuevaFactura,v as nuevaVenta,f as nuevoPartner,b as nuevoProducto};
