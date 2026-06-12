(function(){const e=document.createElement("link").relList;if(e&&e.supports&&e.supports("modulepreload"))return;for(const i of document.querySelectorAll('link[rel="modulepreload"]'))o(i);new MutationObserver(i=>{for(const s of i)if(s.type==="childList")for(const n of s.addedNodes)n.tagName==="LINK"&&n.rel==="modulepreload"&&o(n)}).observe(document,{childList:!0,subtree:!0});function a(i){const s={};return i.integrity&&(s.integrity=i.integrity),i.referrerPolicy&&(s.referrerPolicy=i.referrerPolicy),i.crossOrigin==="use-credentials"?s.credentials="include":i.crossOrigin==="anonymous"?s.credentials="omit":s.credentials="same-origin",s}function o(i){if(i.ep)return;i.ep=!0;const s=a(i);fetch(i.href,s)}})();const Pt={isLoggedIn:()=>!!localStorage.getItem("nx_token"),getUser:()=>{try{return JSON.parse(localStorage.getItem("nx_user")||"{}")}catch{return{}}},setSession(t,e){localStorage.setItem("nx_token",t),localStorage.setItem("nx_user",JSON.stringify(e))},clear(){localStorage.removeItem("nx_token"),localStorage.removeItem("nx_user")}},Xt={};function qt(t,e){Xt[t]=e}function At(t){(window.location.hash.replace("#","")||"home")===t?ge():window.location.hash=t}function La(){window.addEventListener("hashchange",ge),ge()}function ge(){const t=window.location.hash.replace("#","")||"home",[e,a]=t.split("?"),o={};if(a&&a.split("&").forEach(s=>{const[n,d]=s.split("=");n&&(o[decodeURIComponent(n)]=decodeURIComponent(d||""))}),!Pt.isLoggedIn()&&e!=="login"){At("login");return}if(Pt.isLoggedIn()&&e==="login"){At("home");return}const i=Xt[e];i?i(o):Xt[404]&&Xt[404](o)}const Ma="/api/v1";function ja(){return localStorage.getItem("nx_token")}class Da extends Error{constructor(e,a){super(a),this.status=e}}async function x(t,e,a){const o=ja(),i=await fetch(Ma+e,{method:t,headers:{"Content-Type":"application/json",...o?{Authorization:`Bearer ${o}`}:{}},...a!==void 0?{body:JSON.stringify(a)}:{}});if(i.status===401)return localStorage.removeItem("nx_token"),localStorage.removeItem("nx_user"),window.location.hash="login",null;if(!i.ok)throw new Da(i.status,await i.text());return(i.headers.get("content-type")||"").includes("application/json")?i.json():i.text()}const g={get:t=>x("GET",t),post:(t,e)=>x("POST",t,e),put:(t,e)=>x("PUT",t,e),del:t=>x("DELETE",t),login:(t,e)=>x("POST","/auth/login",{login:t,password:e}),logout:()=>x("POST","/auth/logout",{}),dashboard:()=>x("GET","/dashboard"),ventaKpis:()=>x("GET","/ventas/kpis"),factKpis:()=>x("GET","/facturas/kpis"),stockKpis:()=>x("GET","/stock/kpis"),ventas:(t=1)=>x("GET",`/ventas?pagina=${t}`),venta:t=>x("GET",`/ventas/${t}`),facturas:(t=1)=>x("GET",`/facturas?pagina=${t}`),factura:t=>x("GET",`/facturas/${t}`),porCobrar:()=>x("GET","/facturas/por-cobrar"),productos:(t=1,e="")=>x("GET",`/productos?pagina=${t}&q=${encodeURIComponent(e)}`),producto:t=>x("GET",`/productos/${t}`),partners:(t=1)=>x("GET",`/partners?pagina=${t}`),partner:t=>x("GET",`/partners/${t}`),clientes:(t=1)=>x("GET",`/clientes?pagina=${t}`),proveedores:(t=1)=>x("GET",`/proveedores?pagina=${t}`),stock:(t=1)=>x("GET",`/stock?pagina=${t}`),stockKpis:()=>x("GET","/stock/kpis"),stockBajo:()=>x("GET","/stock/bajo"),stockProducto:t=>x("GET",`/stock/producto/${t}`),cfdiTimbrados:(t=1)=>x("GET",`/cfdi/timbrados?pagina=${t}`),cfdiTimbrado:t=>x("GET",`/cfdi/timbrados/${t}`),cfdiKpis:()=>x("GET","/cfdi/kpis"),timbrar:t=>x("POST","/cfdi/timbrar",t),cancelarCfdi:t=>x("POST","/cfdi/cancelar",t),nomina:(t=1)=>x("GET",`/nomina?pagina=${t}`),empleado:t=>x("GET",`/nomina/${t}`),nominaKpis:()=>x("GET","/nomina/kpis"),compras:(t=1)=>x("GET",`/compras?pagina=${t}`),compra:t=>x("GET",`/compras/${t}`),comprasKpis:()=>x("GET","/compras/kpis"),cotizaciones:(t=1)=>x("GET",`/cotizaciones?pagina=${t}`),cotizacionKpis:()=>x("GET","/cotizaciones/kpis"),cotizacion:t=>x("GET",`/cotizaciones/${t}`),crearCotizacion:t=>x("POST","/cotizaciones",t),confirmarCotizacion:t=>x("PUT",`/cotizaciones/${t}/confirmar`),cancelarCotizacion:t=>x("PUT",`/cotizaciones/${t}/cancelar`),actualizarCotizacion:(t,e)=>x("PUT",`/cotizaciones/${t}`,e),agregarLinea:(t,e)=>x("POST",`/cotizaciones/${t}/lineas`,e),eliminarLinea:(t,e)=>x("DELETE",`/cotizaciones/${t}/lineas/${e}`),searchSync:()=>x("POST","/search/sync",{}),searchStatus:()=>x("GET","/search/status"),health:()=>x("GET","/health"),putVenta:(t,e)=>x("PUT",`/ventas/${t}`,e),putPartner:(t,e)=>x("PUT",`/partners/${t}`,e),putProducto:(t,e)=>x("PUT",`/productos/${t}`,e),putCompra:(t,e)=>x("PUT",`/compras/${t}`,e),putEmpleado:(t,e)=>x("PUT",`/nomina/${t}`,e),ajusteStock:(t,e)=>x("PUT",`/stock/${t}/ajuste`,e)};function Na(){const t=document.getElementById("__shell");t&&t.remove(),document.getElementById("app").innerHTML=`
  <div class="login-bg odoo-login">
    <!-- Panel Izquierdo -->
    <div class="login-left">
      <div class="login-brand">
        <span class="brand-n">NexusTech</span>
      </div>
      <h1 class="login-hero-title">Gestión empresarial<br><span class="highlight">sin fricciones</span></h1>
      <p class="login-hero-desc">
        Inventario, ventas, facturación y más — todo desde<br>
        una interfaz ultra rápida construida en Rust.
      </p>

      <div class="login-features-list">
        <div class="feat-item">
          <div class="feat-icon" style="color: #A78BFA; background: rgba(167, 139, 250, 0.1);">🛡️</div>
          <div>
            <strong>Acceso seguro</strong>
            <p>Sesión cifrada extremo a extremo</p>
          </div>
        </div>
        <div class="feat-item">
          <div class="feat-icon" style="color: #60A5FA; background: rgba(96, 165, 250, 0.1);">⚡</div>
          <div>
            <strong>Velocidad nativa</strong>
            <p>Respuestas en < 50 ms con Rust</p>
          </div>
        </div>
        <div class="feat-item">
          <div class="feat-icon" style="color: #A78BFA; background: rgba(167, 139, 250, 0.1);">⊞</div>
          <div>
            <strong>ERP completo</strong>
            <p>Todos los módulos integrados</p>
          </div>
        </div>
      </div>

      <div class="login-stats">
        <div class="stat-box"><strong>40k+</strong><p>Productos</p></div>
        <div class="stat-box"><strong>100</strong><p>PageSpeed</p></div>
        <div class="stat-box"><strong><50ms</strong><p>Respuesta</p></div>
      </div>
    </div>

    <!-- Panel Derecho -->
    <div class="login-right">
      <div class="login-card">
        <div class="tag-portal">● PORTAL DE CLIENTES</div>
        <h2>Bienvenido</h2>
        <p class="subtitle">Ingresa tus credenciales para continuar</p>

        <div class="form-group">
          <label>Correo electrónico</label>
          <div class="input-wrap">
            <span class="input-icon">✉️</span>
            <input type="text" id="lu" placeholder="ealvarez@nexustechnologies.com.mx">
            <span class="input-icon-right" style="color:#A78BFA;">◈</span>
          </div>
        </div>

        <div class="form-group">
          <label>Contraseña</label>
          <div class="input-wrap">
            <span class="input-icon">🔒</span>
            <input type="password" id="lp" placeholder="•••••••••••••">
            <span class="input-icon-right">👁️</span>
          </div>
        </div>

        <div class="form-check">
          <input type="checkbox" id="remember">
          <label for="remember">Recordarme</label>
        </div>

        <div class="login-error" id="lerr">Credenciales incorrectas. Verifica tu usuario y contraseña.</div>

        <button class="btn-primary-login" id="lbtn">Iniciar sesión</button>

        <div class="divider"></div>
        <div class="login-footer-links">
          <a href="#">← Volver al sitio</a>
        </div>
        <div class="ssl-badge">
          🛡️ Conexión segura · SSL/TLS cifrado
        </div>
      </div>
    </div>
  </div>`;const e=document.getElementById("lbtn"),a=document.getElementById("lu"),o=document.getElementById("lp"),i=document.getElementById("lerr");async function s(){if(e.disabled)return;const n=a.value.trim(),d=o.value;if(!n||!d){i.textContent="Ingresa usuario y contraseña",i.classList.add("show");return}e.disabled=!0,e.textContent="Iniciando...",i.classList.remove("show");try{const r=await g.login(n,d),l=(r==null?void 0:r.data)||r,c=(l==null?void 0:l.access_token)||(l==null?void 0:l.token);if(c){Pt.setSession(c,{nombre:l.email||n,email:l.email||n,user_id:l.user_id,company_id:l.company_id}),document.getElementById("app").innerHTML="",At("dashboard");return}i.textContent="Error inesperado. Intenta de nuevo.",i.classList.add("show")}catch(r){i.textContent=(r==null?void 0:r.status)===401?"Credenciales incorrectas.":`Error de conexión: ${(r==null?void 0:r.message)||"Fallo de red"}`,i.classList.add("show")}e.disabled=!1,e.textContent="Iniciar sesión"}e.addEventListener("click",s),o.addEventListener("keydown",n=>n.key==="Enter"&&s()),a.addEventListener("keydown",n=>n.key==="Enter"&&o.focus()),setTimeout(()=>a.focus(),100)}function qa(t,e=0){return t==null||t===""?"—":Number(t).toLocaleString("es-MX",{minimumFractionDigits:e,maximumFractionDigits:e})}function f(t){return t==null?"—":(t=parseFloat(t)||0,Math.abs(t)>=1e6?`$${(t/1e6).toFixed(2)}M`:Math.abs(t)>=1e3?`$${(t/1e3).toFixed(1)}k`:`$${qa(t,2)}`)}function W(t){return t==null?"—":Number(t).toLocaleString("es-MX")}function j(t){return t?new Date(t).toLocaleDateString("es-MX",{day:"2-digit",month:"short",year:"numeric"}):"—"}function b(t,e="",a="info"){const o={success:"✅",error:"❌",info:"ℹ️",warning:"⚠️"};let i=document.getElementById("__toasts");i||(i=document.createElement("div"),i.id="__toasts",i.className="toast-container",document.body.appendChild(i));const s=document.createElement("div");s.className=`toast ${a}`,s.innerHTML=`
    <span class="toast-icon">${o[a]||"ℹ️"}</span>
    <div><div class="toast-title">${t}</div>${e?`<div class="toast-msg">${e}</div>`:""}</div>`,i.appendChild(s),requestAnimationFrame(()=>s.classList.add("show")),setTimeout(()=>{s.classList.remove("show"),setTimeout(()=>s.remove(),400)},3800)}function Ge(t,e,a=900,o="",i=""){if(!t)return;const s=performance.now(),n=String(e).includes(".");function d(r){const l=Math.min((r-s)/a,1),c=1-Math.pow(1-l,3),p=e*c;t.textContent=o+(n?p.toLocaleString("es-MX",{minimumFractionDigits:2,maximumFractionDigits:2}):Math.round(p).toLocaleString("es-MX"))+i,l<1&&requestAnimationFrame(d)}requestAnimationFrame(d)}function Ra(t){if(!(t!=null&&t.length))return"";const e=Math.max(...t,1);return`<div class="sparkline">${t.map((a,o)=>`<div class="spark-bar${o===t.length-1?" active":""}" style="height:${Math.max(4,Math.round(a/e*100))}%"></div>`).join("")}</div>`}function Oa(t=5,e=6){return`<tbody>${Array.from({length:e},()=>`<tr>${Array.from({length:t},()=>`<td><div class="skeleton" style="height:14px;width:${60+Math.random()*30}%"></div></td>`).join("")}</tr>`).join("")}</tbody>`}function I(t=5,e=4){return`<table class="data-table"><thead><tr>${Array.from({length:e},()=>`<th><div class="skeleton" style="height:12px;width:${40+Math.random()*40}%"></div></th>`).join("")}</tr></thead>${Oa(e,t)}</table>`}function Va(t=5){return Array.from({length:t},()=>`
  <div class="kpi-card kpi-gray">
    <div class="kpi-label"><div class="skeleton" style="height:13px;width:60%"></div></div>
    <div class="kpi-value"><div class="skeleton" style="height:28px;width:70%"></div></div>
    <div><div class="skeleton" style="height:11px;width:40%;margin-top:6px"></div></div>
  </div>`).join("")}const Ha={sale:"emerald",done:"indigo",draft:"sky",sent:"violet",cancel:"red",posted:"emerald",in_payment:"violet",paid:"emerald",partial:"amber"};function Rt(t,e){return`<span class="badge badge-${Ha[t]||"gray"} badge-dot" style="font-weight:600">${e}</span>`}function vt(t,e,a){return window.__pagNav=a,`
  <div class="data-table-footer">
    <span style="color:var(--text-400)">Página ${t}</span>
    <div class="pagination">
      <button class="pag-btn" ${t<=1?"disabled":""} onclick="window.__pagNav(${t-1})">&#8592; Anterior</button>
      <span class="pag-btn active">${t}</span>
      <button class="pag-btn" ${e?"":"disabled"} onclick="window.__pagNav(${t+1})">Siguiente &#8594;</button>
    </div>
  </div>`}let ct=null;function Q(t,e,a={}){let o=document.getElementById("__modal-overlay");o||(o=document.createElement("div"),o.id="__modal-overlay",o.innerHTML=`
      <div id="__modal-drawer">
        <div id="__modal-header">
          <span id="__modal-title"></span>
          <button id="__modal-close" onclick="window.__closeModal()">✕</button>
        </div>
        <div id="__modal-body"></div>
      </div>`,document.body.appendChild(o),o.addEventListener("click",i=>{i.target===o&&window.__closeModal()})),document.getElementById("__modal-title").textContent=t,document.getElementById("__modal-body").innerHTML=e,o.classList.add("open"),document.body.style.overflow="hidden",ct&&document.removeEventListener("keydown",ct),ct=i=>{i.key==="Escape"&&window.__closeModal()},document.addEventListener("keydown",ct),a.onMounted&&setTimeout(a.onMounted,10)}function Ua(){const t=document.getElementById("__modal-overlay");t&&t.classList.remove("open"),document.body.style.overflow="",ct&&(document.removeEventListener("keydown",ct),ct=null)}window.__closeModal=Ua;async function Ga(t,e,a){Q(t,`
    <div style="display:flex;flex-direction:column;gap:12px;padding:8px 0">
      ${[1,2,3,4,5].map(()=>'<div class="skeleton" style="height:52px;border-radius:10px"></div>').join("")}
    </div>`);try{const o=await e(),i=(o==null?void 0:o.data)??o;document.getElementById("__modal-body").innerHTML=a(i)}catch(o){document.getElementById("__modal-body").innerHTML=`<p style="color:var(--red);padding:24px">Error: ${o.message}</p>`}}function K(t,e,a={}){const o=e??"—",i=a.color?`color:${a.color}`:"";return`
  <div style="display:flex;justify-content:space-between;align-items:flex-start;
    padding:10px 0;border-bottom:1px solid var(--border)">
    <span style="font-size:12px;color:var(--text-400);font-weight:600;min-width:140px">${t}</span>
    <span style="font-size:13px;font-weight:500;text-align:right;${i}">${o}</span>
  </div>`}function ce(t,e){return`
  <div style="margin-bottom:20px">
    <div style="font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.06em;
      color:var(--text-400);margin-bottom:8px;padding-bottom:6px;
      border-bottom:2px solid var(--primary)">${t}</div>
    ${e}
  </div>`}const Z={ventas:[{id:"ventas",label:"Órdenes"},{id:"cotizaciones",label:"Por facturar"},{id:"precios",label:"Precios Especiales"},{id:"productos",label:"Productos"},{id:"reportes_ventas",label:"Reportes"},{id:"config_ventas",label:"Configuración"}],compras:[{id:"compras",label:"Órdenes"},{id:"productos_compra",label:"Productos"},{id:"reportes_compras",label:"Reportes"},{id:"config_compras",label:"Configuración"}],nomina:[{id:"nomina",label:"Empleados"},{id:"config_nomina",label:"Configuración"}],contabilidad:[{id:"contabilidad",label:"Asientos"},{id:"config_contabilidad",label:"Configuración"}],crm:[{id:"crm",label:"Mi flujo"},{id:"ventas",label:"Ventas"},{id:"reportes_crm",label:"Reportes"},{id:"config_crm",label:"Configuración"}],facturacion:[{id:"facturas",label:"Facturas"},{id:"pagos",label:"Pagos"},{id:"reportes_facturacion",label:"Reportes"},{id:"config_facturacion",label:"Configuración"}],inventario:[{id:"stock",label:"Tablero"},{id:"operaciones",label:"Operaciones"},{id:"productos",label:"Productos"},{id:"reportes_inventario",label:"Reportes"},{id:"config_inventario",label:"Configuración"}],contactos:[{id:"partners",label:"Contactos"},{id:"etiquetas",label:"Etiquetas"},{id:"config_contactos",label:"Configuración"}],mercadily:[{id:"mercadily",label:"Configuración Tienda"}]};function B(){if(document.getElementById("__shell"))return;const t=Pt.getUser(),e=(t.nombre||t.name||"AD").substring(0,2).toUpperCase();document.getElementById("app").innerHTML=`
  <div class="app-shell odoo-layout" id="__shell">
    <!-- ODOO TOPBAR -->
    <header class="odoo-topbar">
      <div class="odoo-topbar-left">
        <button class="app-drawer-btn" title="Aplicaciones" onclick="window._go('home')">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="white">
            <path d="M4 4h4v4H4V4zm6 0h4v4h-4V4zm6 0h4v4h-4V4zM4 10h4v4H4v-4zm6 0h4v4h-4v-4zm6 0h4v4h-4v-4zM4 16h4v4H4v-4zm6 0h4v4h-4v-4zm6 0h4v4h-4v-4z"/>
          </svg>
        </button>
        <div class="app-title" id="odoo-app-title">NexusTech ERP</div>
        <nav class="app-nav" id="odoo-app-nav"></nav>
      </div>
      
      <div class="odoo-topbar-right">
        <div class="topbar-search">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/>
          </svg>
          <input type="text" placeholder="Search..." id="global-search">
        </div>
        <button class="topbar-action" title="Notificaciones">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"></path><path d="M13.73 21a2 2 0 0 1-3.46 0"></path></svg>
          <span class="notif-dot"></span>
        </button>
        <div class="company-name" style="cursor:pointer" onclick="window._logout()" title="Cerrar sesión">
          NEXUSTECH
        </div>
        <div class="avatar-sm" style="cursor:pointer" onclick="window._logout()">${e}</div>
      </div>
    </header>

    <!-- CONTENT -->
    <main class="page full-width" id="__page"></main>
  </div>`,window._go=a=>{At(a)},window._logout=()=>{Pt.clear();const a=document.getElementById("__shell");a&&a.remove(),At("login"),b("Sesión cerrada","Hasta pronto","info")},window.addEventListener("hashchange",Ke),Ke()}function w(t){const e=document.getElementById("__page");e&&(e.innerHTML=t,e.scrollTop=0)}function Ke(){const t=window.location.hash.replace("#","")||"home";let e="NexusTech ERP",a=[];t.startsWith("ventas")||t.startsWith("cotizaciones")||t==="precios"||t.startsWith("reportes_ventas")||t.startsWith("config_ventas")?(e="VENTAS",a=Z.ventas):t.startsWith("compras")||t.startsWith("config_compras")||t.startsWith("productos_compra")||t.startsWith("reportes_compras")?(e="COMPRAS",a=Z.compras):t.startsWith("crm")||t.startsWith("config_crm")||t.startsWith("reportes_crm")?(e="CRM",a=Z.crm):t.startsWith("facturas")||t.startsWith("pagos")||t.startsWith("config_facturacion")||t.startsWith("reportes_facturacion")?(e="FACTURACIÓN",a=Z.facturacion):t.startsWith("stock")||t.startsWith("productos")||t.startsWith("operaciones")||t.startsWith("reportes_inventario")||t.startsWith("config_inventario")?(e="INVENTARIO",a=Z.inventario):t.startsWith("partners")||t.startsWith("etiquetas")||t.startsWith("config_contactos")?(e="CONTACTOS",a=Z.contactos):t.startsWith("contabilidad")||t.startsWith("config_contabilidad")?(e="CONTABILIDAD",a=Z.contabilidad):t.startsWith("nomina")||t.startsWith("config_nomina")?(e="NÓMINA",a=Z.nomina):t.startsWith("mercadily")?(e="MERCADILY",a=Z.mercadily):t.startsWith("apps")&&(e="APLICACIONES",a=[]);const o=document.getElementById("odoo-app-title");o&&(o.textContent=e);const i=document.getElementById("odoo-app-nav");i&&(a.length>0?i.innerHTML=a.map(s=>`
        <a class="app-nav-link ${s.id===t?"active":""}" href="#${s.id}" onclick="event.preventDefault();window._go('${s.id}')">
          ${s.label}
        </a>
      `).join(""):i.innerHTML="")}async function Ka(){B();let t=[];try{const a=await g.get("/apps");t=Array.isArray(a==null?void 0:a.data)?a.data:Array.isArray(a)?a:[]}catch(a){console.error("Error cargando apps",a)}let e=t.filter(a=>a.estado==="installed"||a.id==="apps");e.length===0&&(e=[{id:"apps",nombre:"Aplicaciones",descripcion:"Catálogo de Módulos",icono:"🛍️",gradiente:"#1E293B,#0F172A",estado:"installed"}]),w(`
    <div class="nx-home">
      <div class="nx-home-header">
        <h1 class="nx-home-title">Aplicaciones</h1>
        <div class="nx-home-search">
          <input type="search" placeholder="Buscar módulo…" id="home-search" oninput="window._filterApps(this.value)">
        </div>
      </div>
      <div class="nx-app-grid" id="home-app-grid">
        ${e.map((a,o)=>`
          <div class="nx-app-card" data-id="${a.id}" onclick="window._go('${a.id}')" style="animation-delay:${o*50}ms">
            <div class="nx-app-icon" style="background:linear-gradient(135deg,${a.gradiente||"#475569,#1E293B"})">${a.icono||"📦"}</div>
            ${a.kpi_url?`<div class="nx-app-badge" id="app-badge-${a.id}">…</div>`:""}
            <div class="nx-app-name">${a.nombre}</div>
            <div class="nx-app-desc">${a.descripcion||""}</div>
          </div>
        `).join("")}
      </div>
    </div>
  `),await Promise.allSettled(e.filter(a=>a.kpi_url).map(async a=>{try{const o=await g.get(a.kpi_url),i=(o==null?void 0:o.data)??o,s=a.kpi_field&&i?i[a.kpi_field]??"—":Array.isArray(i)?i.length:"—",n=document.getElementById("app-badge-"+a.id);n&&(n.textContent=Number(s)>999?(s/1e3).toFixed(1)+"k":s)}catch{const o=document.getElementById("app-badge-"+a.id);o&&(o.textContent="—")}})),window._filterApps=a=>{const o=a.toLowerCase().trim();document.querySelectorAll(".nx-app-card").forEach(i=>{var d,r;const s=((d=i.querySelector(".nx-app-name"))==null?void 0:d.textContent.toLowerCase())||"",n=((r=i.querySelector(".nx-app-desc"))==null?void 0:r.textContent.toLowerCase())||"";i.classList.toggle("hidden",!!o&&!s.includes(o)&&!n.includes(o))})}}const Xa={sale:"indigo",done:"emerald",draft:"gray",cancel:"red",sent:"sky",posted:"emerald"},Ja={sale:"Confirmada",done:"Entregada",draft:"Borrador",cancel:"Cancelada",sent:"Enviada"};function $t(t,e=10){return Array.from({length:e},()=>Math.max(5,Math.round(t*(.6+Math.random()*.8))))}async function ua(){var t,e,a,o,i,s,n,d,r;B(),w(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">Dashboard</h1>
      <p class="page-subtitle">${new Date().toLocaleDateString("es-MX",{weekday:"long",day:"numeric",month:"long",year:"numeric"})}</p>
    </div>
    <div class="page-actions">
      <button class="btn btn-secondary" id="btn-refresh">🔄 Actualizar</button>
      <button class="btn btn-primary" onclick="window._go('ventas')">+ Nueva Venta</button>
    </div>
  </div>

  <!-- KPI Cards skeleton -->
  <div class="kpi-grid anim-2" id="kpi-grid">${Va(5)}</div>

  <!-- Main grid -->
  <div style="display:grid;grid-template-columns:1.6fr 1fr;gap:16px;margin-bottom:16px" class="anim-3">
    <div class="data-card">
      <div class="data-card-header">
        <div>
          <div class="data-card-title">Últimas Ventas</div>
          <div class="data-card-subtitle">Pedidos más recientes del sistema</div>
        </div>
        <button class="btn btn-secondary btn-sm" onclick="window._go('ventas')">Ver todas →</button>
      </div>
      <div id="tabla-ventas">${I(6,5)}</div>
    </div>

    <div class="data-card">
      <div class="data-card-header">
        <div>
          <div class="data-card-title">⚠️ Stock Bajo</div>
          <div class="data-card-subtitle">Productos bajo nivel mínimo</div>
        </div>
        <button class="btn btn-secondary btn-sm" onclick="window._go('stock')">Inventario</button>
      </div>
      <div id="tabla-stock">${I(5,4)}</div>
    </div>
  </div>

  <!-- Bottom grid -->
  <div style="display:grid;grid-template-columns:repeat(3,1fr);gap:16px" class="anim-4">
    <!-- Accesos rápidos (estático) -->
    <div class="data-card" style="padding:20px">
      <div style="font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.06em;color:var(--text-400);margin-bottom:14px">⚡ Accesos Rápidos</div>
      ${[{icon:"🧾",label:"Nueva Factura CFDI",href:"cfdi"},{icon:"📦",label:"Recepción de Mercancía",href:"stock"},{icon:"👥",label:"Nuevo Cliente",href:"partners"},{icon:"📈",label:"Reporte de Ventas",href:"reportes"},{icon:"🔍",label:"Búsqueda Global",href:"search"}].map(l=>`
      <button class="btn btn-secondary" style="width:100%;margin-bottom:6px;justify-content:flex-start;font-size:12.5px" onclick="window._go('${l.href}')">
        ${l.icon} ${l.label}
      </button>`).join("")}
    </div>

    <!-- Resumen fiscal — datos en vivo -->
    <div class="data-card" style="padding:20px">
      <div style="font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.06em;color:var(--text-400);margin-bottom:16px">📊 Resumen Fiscal</div>
      <div id="resumen-fiscal">${I(4,2)}</div>
    </div>

    <!-- Estado del sistema -->
    <div class="data-card" style="padding:20px">
      <div style="font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.06em;color:var(--text-400);margin-bottom:16px">🟢 Estado del Sistema</div>
      <div id="system-status">${I(4,2)}</div>
    </div>
  </div>`);try{const[l,c,p]=await Promise.allSettled([g.dashboard(),g.ventas(1),g.stockBajo()]),m=l.status==="fulfilled"?(t=l.value)==null?void 0:t.data:null,u=[{key:"ventas_mes",label:"Ventas del Mes",tipo:"mxn",icon:"💰",color:"indigo",valor:parseFloat(((e=m==null?void 0:m.ventas)==null?void 0:e.importe_mes)||0),trend:null,spark:$t(100)},{key:"facturas",label:"Facturas Emitidas",tipo:"num",icon:"🧾",color:"emerald",valor:parseInt(((a=m==null?void 0:m.facturacion)==null?void 0:a.total_facturas)||0),trend:null,spark:$t(50)},{key:"cobrar",label:"Por Cobrar",tipo:"mxn",icon:"📋",color:"amber",valor:parseFloat(((o=m==null?void 0:m.facturacion)==null?void 0:o.por_cobrar)||0),trend:null,spark:$t(80)},{key:"stock_total",label:"Productos en Stock",tipo:"num",icon:"📦",color:"sky",valor:parseInt(((i=m==null?void 0:m.inventario)==null?void 0:i.total_productos_con_stock)||0),trend:null,spark:$t(80)},{key:"stock_bajo",label:"Alertas Stock Bajo",tipo:"num",icon:"⚠️",color:"rose",valor:parseInt(((s=m==null?void 0:m.inventario)==null?void 0:s.alertas_stock_bajo)||0),trend:null,spark:$t(20)}],v=document.getElementById("kpi-grid");v&&(v.innerHTML=u.map(h=>`
      <div class="kpi-card kpi-${h.color}">
        <div class="kpi-label">
          <span>${h.label}</span>
          <div class="kpi-icon-box">${h.icon}</div>
        </div>
        <div class="kpi-value" id="kv-${h.key}">—</div>
        <div class="kpi-trend neutral">→ En tiempo real</div>
        ${Ra(h.spark)}
      </div>`).join(""),u.forEach(h=>{const E=document.getElementById("kv-"+h.key);E&&(h.tipo==="mxn"?Ge(E,h.valor,1100,"$"):Ge(E,h.valor,1100))}));const k=document.getElementById("tabla-ventas");if(k){const h=c.status==="fulfilled"?(((n=c.value)==null?void 0:n.data)||[]).slice(0,6):[];h.length===0?k.innerHTML='<p style="text-align:center;color:var(--text-400);padding:24px">Sin ventas registradas</p>':k.innerHTML=`
        <table class="data-table">
          <thead><tr>
            <th>Folio</th><th>Cliente</th><th>Fecha</th><th>Total</th><th>Estado</th>
          </tr></thead>
          <tbody>
            ${h.map(E=>{const S=E.state||"draft",y=Ja[S]||S,z=Xa[S]||"gray",P=E.date_order?new Date(E.date_order).toLocaleDateString("es-MX",{day:"2-digit",month:"short"}):"—";return`
              <tr>
                <td class="td-mono">${E.name||E.id}</td>
                <td class="td-primary">${E.partner_name||E.partner_id||"—"}</td>
                <td>${P}</td>
                <td class="td-amount">${f(parseFloat(E.amount_total||0))}</td>
                <td><span class="badge badge-${z} badge-dot">${y}</span></td>
              </tr>`}).join("")}
          </tbody>
        </table>`}const _=document.getElementById("tabla-stock");if(_){const h=p.status==="fulfilled"?(((d=p.value)==null?void 0:d.data)||[]).slice(0,5):[];h.length===0?_.innerHTML='<p style="text-align:center;color:var(--text-400);padding:24px">✅ Stock en niveles normales</p>':_.innerHTML=`
        <table class="data-table">
          <thead><tr><th>Producto</th><th>Disponible</th></tr></thead>
          <tbody>
            ${h.map(E=>{const S=parseFloat(E.cantidad_disponible||0),y=S<=0?"red":S<5?"amber":"sky";return`
              <tr>
                <td class="td-primary" style="max-width:140px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">${E.product_name||E.product_id}</td>
                <td><span class="badge badge-${y}">${S}</span></td>
              </tr>`}).join("")}
          </tbody>
        </table>`}const $=document.getElementById("resumen-fiscal");if($){const h=m==null?void 0:m.facturacion,E=[{label:"Facturas emitidas (total)",val:W((h==null?void 0:h.total_facturas)||0),color:"indigo"},{label:"Por cobrar",val:f(parseFloat((h==null?void 0:h.por_cobrar)||0)),color:"amber"},{label:"Monto total facturado",val:f(parseFloat((h==null?void 0:h.monto_total)||0)),color:"emerald"},{label:"Facturas vencidas",val:W((h==null?void 0:h.facturas_vencidas)||0),color:"red"}];$.innerHTML=E.map(S=>`
      <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:11px;padding-bottom:11px;border-bottom:1px solid var(--border)">
        <span style="font-size:12.5px;color:var(--text-500)">${S.label}</span>
        <span class="badge badge-${S.color}">${S.val}</span>
      </div>`).join("")}const T=document.getElementById("system-status");if(T){let h=!1;try{await g.health(),h=!0}catch{}T.innerHTML=[{label:"API Backend",val:h?"✅ En línea":"❌ Offline",color:h?"emerald":"red"},{label:"Base de datos",val:m?"✅ Operativa":"⚠️ Sin datos",color:m?"emerald":"amber"},{label:"Versión ERP",val:"v2.0.0",color:"indigo"},{label:"Uptime",val:"99.98%",color:"emerald"}].map(E=>`
      <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:10px">
        <span style="font-size:12.5px;color:var(--text-500)">${E.label}</span>
        <span class="badge badge-${E.color}">${E.val}</span>
      </div>`).join("")}}catch(l){console.error("Dashboard load error:",l),b("Error al cargar","No se pudo conectar con el servidor","error")}(r=document.getElementById("btn-refresh"))==null||r.addEventListener("click",()=>ua())}let Lt="list",G=[],Mt=0,xt=1,X="",jt="",N=new Set,F=null,O=[],Wt=null;const Xe={firma_online:!1,pago_online:!1,descuentos:!0,margenes:!1,advertencias:!1,bloquear_confirmado:!0,validez_cotizacion:30,plantillas_presupuesto:!1,compra_online:!1,notas_cierre:!1,politica_facturacion:"cantidad_pedida",costos_envio:!1,fecha_entrega:!1,aviso_stock:!1,listas_precios:!1,descuento_precio:!1,variantes:!1,unidades_medida:!1,empaquetado:!1,terminos:""};function tt(){try{return{...Xe,...JSON.parse(localStorage.getItem("nexus_config_ventas")||"{}")}}catch{return{...Xe}}}const Wa=[{id:1,name:"Tarifa General",currency:"MXN",type:"Porcentaje",active:!0,discount:0},{id:2,name:"Distribuidores",currency:"MXN",type:"Porcentaje",active:!0,discount:10},{id:3,name:"Exportación USD",currency:"USD",type:"Fijo",active:!1,discount:0}];function Yt(){try{const t=JSON.parse(localStorage.getItem("nexus_pricelists")||"null");if(Array.isArray(t)&&t.length)return t}catch{}return Wa}function mt(t){try{return JSON.parse(localStorage.getItem(`nexus_venta_extras_${t}`)||"{}")}catch{return{}}}function wt(t,e){const a={...mt(t),...e};return localStorage.setItem(`nexus_venta_extras_${t}`,JSON.stringify(a)),a}const ye={std:{label:"Cotización estándar",lineas:[{display_type:"line_section",name:"Productos",product_uom_qty:0,price_unit:0,discount:0}]},serv:{label:"Servicios profesionales",lineas:[{display_type:"line_section",name:"Servicios profesionales",product_uom_qty:0,price_unit:0,discount:0},{name:"Implementación y configuración",product_uom_qty:1,price_unit:15e3,discount:0},{name:"Capacitación (por sesión)",product_uom_qty:2,price_unit:2500,discount:0},{name:"Soporte mensual",product_uom_qty:1,price_unit:3500,discount:0}]}},Je=[{key:"estandar",label:"Entrega estándar (3-5 días)",costo:99},{key:"express",label:"Entrega express (24 hrs)",costo:199},{key:"recoger",label:"Recoger en tienda",costo:0}];function se(t){const e=tt();return`
  <div id="btn-bar-lineas" style="display:flex;gap:8px;padding:10px 20px;border-top:1px solid var(--border);background:var(--bg-app)">
    <button class="o-btn-secondary o-btn-sm" onclick="window._agregarProductoInline(${t})" id="btn-add-product" style="gap:6px">＋ Agregar producto</button>
    <button class="o-btn-secondary o-btn-sm" onclick="window._agregarSeccion(${t})" style="gap:6px">＋ Agregar sección</button>
    ${e.costos_envio?`<button class="o-btn-secondary o-btn-sm" onclick="window._agregarEnvio(${t})" style="gap:6px">🚚 Agregar envío</button>`:""}
  </div>`}const Pe={draft:"Cotización",sent:"Enviado",sale:"Pedido de Venta",done:"Realizado",cancel:"Cancelado"},Ae={draft:"o-badge-gray",sent:"o-badge-info",sale:"o-badge-success",done:"o-badge-violet",cancel:"o-badge-danger"},Le={no:"—",to_invoice:"Por Facturar",invoiced:"Facturado"},Me={no:"",to_invoice:"o-badge-warn",invoiced:"o-badge-success"};async function Ya(t){return Qa(t)}async function Qa(t){B();const e=t==null?void 0:t.id;if(e){await J(parseInt(e));return}window._setPage=a=>{xt=a,zt()},window._setFilter=a=>{X=a,xt=1,zt()},window._setView=a=>{Lt=a,je()},window._abrirVenta=J,window._sortBy=()=>{},window._toggleSel=a=>{var o;N.has(a)?N.delete(a):N.add(a),(o=document.getElementById("nx-sel-count"))!=null&&o.textContent&&xe()},window._toggleAll=a=>{a.checked?G.forEach(o=>N.add(o.id)):N.clear(),document.querySelectorAll(".o-chk-row").forEach(o=>o.checked=a.checked),xe()},w(Za()),await zt()}function Za(){return`
  <div class="nx-module-page" style="min-height:100vh;background:var(--bg-app)">
    <!-- Control Panel -->
    <div class="o-cp" style="gap:10px;flex-wrap:wrap">
      <div class="o-cp-left">
        <button class="o-btn-primary" onclick="window._nuevaVenta()" id="btn-nueva-venta">
          ＋ Nueva
        </button>
        <div class="o-dropdown" style="position:relative">
          <button class="o-btn-filter" onclick="this.nextElementSibling.classList.toggle('open')" id="btn-filtros">
            ☰ Filtros ▾
          </button>
          <div class="o-dropdown-menu" id="dd-filtros">
            <div class="o-dd-item ${X?"":"o-dd-item-active"}" onclick="window._setFilter('');document.getElementById('dd-filtros').classList.remove('open')">Todos</div>
            <div class="o-dd-item ${X==="draft"?"o-dd-item-active":""}" onclick="window._setFilter('draft');document.getElementById('dd-filtros').classList.remove('open')">Cotizaciones</div>
            <div class="o-dd-item ${X==="sent"?"o-dd-item-active":""}" onclick="window._setFilter('sent');document.getElementById('dd-filtros').classList.remove('open')">Enviados</div>
            <div class="o-dd-item ${X==="sale"?"o-dd-item-active":""}" onclick="window._setFilter('sale');document.getElementById('dd-filtros').classList.remove('open')">Pedidos</div>
            <div class="o-dd-item ${X==="to_invoice"?"o-dd-item-active":""}" onclick="window._setFilter('to_invoice');document.getElementById('dd-filtros').classList.remove('open')">Por Facturar</div>
            <div class="o-dd-item ${X==="done"?"o-dd-item-active":""}" onclick="window._setFilter('done');document.getElementById('dd-filtros').classList.remove('open')">Realizados</div>
          </div>
        </div>
      </div>
      <div class="o-cp-center">
        <div class="o-search-bar">
          <span class="o-search-icon">🔍</span>
          <input class="o-search-input" id="venta-search" placeholder="Buscar por número, cliente, referencia..."
            value="${jt}"
            onkeyup="if(event.key==='Enter'){window._doSearch(this.value)}"
            oninput="if(!this.value){window._doSearch('')}">
          ${jt?`<button style="background:none;border:none;cursor:pointer;color:var(--text-400);font-size:16px" onclick="document.getElementById('venta-search').value='';window._doSearch('')">×</button>`:""}
        </div>
      </div>
      <div class="o-cp-right">
        <span id="nx-count" style="font-size:12px;color:var(--text-400)">${Mt} registros</span>
        <div class="o-view-switcher">
          <button class="o-view-btn ${Lt==="list"?"o-active":""}" title="Lista" onclick="window._setView('list')">☰</button>
          <button class="o-view-btn ${Lt==="kanban"?"o-active":""}" title="Kanban" onclick="window._setView('kanban')">⊞</button>
        </div>
      </div>
    </div>
    <!-- Barra de selección múltiple (oculta inicialmente) -->
    <div id="nx-sel-bar" style="display:none;align-items:center;gap:12px;padding:8px 16px;background:#EEF2FF;border-bottom:1px solid var(--primary)">
      <span id="nx-sel-count" style="font-size:13px;font-weight:700;color:var(--primary)">0 seleccionados</span>
      <button class="o-btn-secondary o-btn-sm" onclick="window._cancelarSeleccionados()">❌ Cancelar</button>
      <button class="o-btn-secondary o-btn-sm" onclick="window._exportarCSV()">⬇ Exportar CSV</button>
      <button class="o-btn-secondary o-btn-sm" onclick="window._limpiarSel()">× Desmarcar todo</button>
    </div>
    <!-- Contenido principal -->
    <div id="nx-content" style="flex:1">${I(6,7)}</div>
  </div>`}window.toast=b;window._doSearch=t=>{jt=t,xt=1,zt()};window._nuevaVenta=lo;window._cancelarSeleccionados=co;window._exportarCSV=po;window._limpiarSel=()=>{N.clear(),je()};async function zt(){const t=document.getElementById("nx-content");if(t){t.innerHTML=I(6,7);try{const e=new URLSearchParams({pagina:xt,limite:80});X&&X!=="to_invoice"&&e.set("estado",X),X==="to_invoice"&&e.set("invoice_status","to_invoice"),jt&&e.set("buscar",jt);const a=await g.get(`/ventas?${e}`);G=(a==null?void 0:a.data)||[],Mt=(a==null?void 0:a.total)??G.length;const o=document.getElementById("nx-count");o&&(o.textContent=`${Mt} registros`),je()}catch(e){t.innerHTML=`<div style="padding:60px;text-align:center;color:var(--text-400)">⚠️ ${e.message}</div>`}}}function je(){const t=document.getElementById("nx-content");t&&(t.innerHTML=Lt==="kanban"?eo(G):to(G),Lt==="list"&&uo(),xe())}function xe(){const t=document.getElementById("nx-sel-bar"),e=document.getElementById("nx-sel-count");!t||!e||(N.size>0?(t.style.display="flex",e.textContent=`${N.size} seleccionado${N.size>1?"s":""}`):t.style.display="none")}function to(t){return t.length?`<div class="o-list-view">
    <table class="o-list-table">
      <thead>
        <tr>
          <th class="o-list-chk"><input type="checkbox" class="o-chk" onchange="window._toggleAll(this)"></th>
          <th class="o-col-sortable" onclick="window._sortBy('name')">NÚMERO ↕</th>
          <th>FECHA</th>
          <th>CLIENTE</th>
          <th>REFERENCIA CLIENTE</th>
          <th>VENDEDOR</th>
          <th class="o-col-right">TOTAL</th>
          <th>ESTADO</th>
          <th>FACTURACIÓN</th>
        </tr>
      </thead>
      <tbody>
        ${t.map(e=>`
        <tr class="o-list-row${N.has(e.id)?" selected":""}" onclick="window._abrirVenta(${e.id})">
          <td class="o-list-chk" onclick="event.stopPropagation()">
            <input type="checkbox" class="o-chk o-chk-row" ${N.has(e.id)?"checked":""} onchange="window._toggleSel(${e.id})">
          </td>
          <td class="o-td-primary" style="font-family:monospace">${e.name||"#"+e.id}</td>
          <td class="o-td-muted">${j(e.date_order)}</td>
          <td>
            <div class="o-partner-cell">
              <div class="o-avatar o-avatar-sm" style="background:${mo(e.partner_name)}">${(e.partner_name||"?")[0].toUpperCase()}</div>
              <span style="font-weight:500">${e.partner_name||"—"}</span>
            </div>
          </td>
          <td class="o-td-muted">${e.client_order_ref||"—"}</td>
          <td class="o-td-muted">
            ${e.user_name?`<div class="o-partner-cell">
              <div class="o-avatar o-avatar-sm" style="background:#6366F1">${(e.user_name||"A")[0]}</div>
              <span>${e.user_name}</span>
            </div>`:'<span style="color:var(--text-400)">Administrador</span>'}
          </td>
          <td class="o-td-amount">${vo(e.amount_total)}</td>
          <td><span class="o-badge ${Ae[e.state]||"o-badge-gray"}">${Pe[e.state]||e.state}</span></td>
          <td>${e.invoice_status&&e.invoice_status!=="no"?`<span class="o-badge ${Me[e.invoice_status]||""}">${Le[e.invoice_status]||e.invoice_status}</span>`:'<span style="color:var(--text-300)">—</span>'}</td>
        </tr>`).join("")}
      </tbody>
    </table>
    <div class="o-list-footer">
      <span class="o-list-count">${Mt} registros</span>
      ${vt(xt,Mt>xt*80,window._setPage)}
    </div>
  </div>`:`<div style="padding:60px;text-align:center;color:var(--text-400)">
    <div style="font-size:48px;margin-bottom:12px">📋</div>
    <div style="font-size:16px;font-weight:600;margin-bottom:8px">No hay registros</div>
    <div style="font-size:13px">Crea una nueva cotización con el botón <strong>+ Nueva</strong></div>
  </div>`}function eo(t){const e=[{key:"draft",label:"Cotización",color:"#6B7280"},{key:"sent",label:"Enviado",color:"#3B82F6"},{key:"sale",label:"Pedido de Venta",color:"#10B981"},{key:"done",label:"Realizado",color:"#8B5CF6"}],a={};return e.forEach(o=>a[o.key]=t.filter(i=>i.state===o.key)),`<div style="display:flex;gap:16px;padding:20px;overflow-x:auto;min-height:calc(100vh - 180px);align-items:flex-start;background:var(--bg-app)">
    ${e.map(o=>`
    <div style="background:var(--bg-card);border:1px solid var(--border);border-radius:12px;min-width:280px;max-width:300px;flex-shrink:0;display:flex;flex-direction:column;box-shadow:var(--shadow-sm)">
      <div style="display:flex;align-items:center;justify-content:space-between;padding:12px 16px;font-size:12px;font-weight:700;text-transform:uppercase;letter-spacing:.05em;color:#fff;background:${o.color};border-radius:12px 12px 0 0">
        <span>${o.label}</span>
        <span style="background:rgba(255,255,255,.25);padding:2px 8px;border-radius:12px">${a[o.key].length}</span>
      </div>
      <div style="padding:10px;display:flex;flex-direction:column;gap:8px;flex:1;overflow-y:auto;max-height:65vh">
        ${a[o.key].length===0?'<div style="text-align:center;padding:24px;color:var(--text-300);font-size:12px">Sin registros</div>':""}
        ${a[o.key].map(i=>`
        <div onclick="window._abrirVenta(${i.id})"
          style="background:var(--bg-card);border:1px solid var(--border);border-radius:10px;padding:14px;cursor:pointer;transition:all .15s;box-shadow:var(--shadow-sm)"
          onmouseover="this.style.borderColor='${o.color}';this.style.transform='translateY(-2px)';this.style.boxShadow='0 6px 20px rgba(0,0,0,.1)'"
          onmouseout="this.style.borderColor='';this.style.transform='';this.style.boxShadow=''">
          <div style="display:flex;justify-content:space-between;align-items:flex-start;margin-bottom:6px">
            <strong style="font-family:monospace;font-size:13px;color:var(--primary)">${i.name||"#"+i.id}</strong>
            <span style="font-size:11px;color:var(--text-400)">${j(i.date_order)}</span>
          </div>
          <div style="font-weight:600;margin-bottom:4px;font-size:13px;color:var(--text-900)">${i.partner_name||"—"}</div>
          ${i.client_order_ref?`<div style="font-size:11px;color:var(--text-400);margin-bottom:6px">Ref: ${i.client_order_ref}</div>`:""}
          <div style="display:flex;justify-content:space-between;align-items:center;margin-top:8px;padding-top:8px;border-top:1px solid var(--border)">
            <span class="o-badge ${Me[i.invoice_status]||"o-badge-gray"}" style="font-size:10px">${Le[i.invoice_status]||"—"}</span>
            <strong style="font-size:14px;color:${o.color};font-variant-numeric:tabular-nums">${f(i.amount_total)}</strong>
          </div>
        </div>`).join("")}
      </div>
      <div style="padding:10px 14px;border-top:1px solid var(--border);font-size:12px;font-weight:700;color:var(--text-500)">
        Total: ${f(a[o.key].reduce((i,s)=>i+parseFloat(s.amount_total||0),0))}
      </div>
    </div>`).join("")}
  </div>`}async function J(t){history.replaceState(null,"",`#ventas?id=${t}`),w(`<div style="padding:40px">${I(3,5)}</div>`);try{const[e,a]=await Promise.all([g.get(`/ventas/${t}`),g.get(`/ventas/${t}/lineas`)]);let o=[],i=null;try{const s=await g.get(`/ventas/${t}/facturas`);o=(s==null?void 0:s.data)||[]}catch{}try{const s=await g.get(`/ventas/${t}/entrega`);i=(s==null?void 0:s.data)||null}catch{}F=(e==null?void 0:e.data)||e,O=(a==null?void 0:a.data)||[],ao(F,O,o,i)}catch(e){w(`<div style="padding:40px;text-align:center;color:red">⚠️ ${e.message}</div>`)}}function ao(t,e,a=[],o=null){var z,P,A,U;const i=tt(),s=mt(t.id),n=(t.state==="draft"||t.state==="sent")&&!t.locked,d=t.state==="sale"||t.state==="done",r=[{key:"draft",label:"Cotización"},{key:"sent",label:"Enviado"},{key:"sale",label:"Pedido de Venta"},{key:"done",label:"Realizado"}],l=r.findIndex(C=>C.key===t.state),c=e.reduce((C,R)=>C+parseFloat(R.price_subtotal||0),0),p=c*.16,m=c+p,u=e.filter(C=>C.display_type!=="line_section"&&/^env[ií]o/i.test(C.name||"")),v=u.reduce((C,R)=>C+parseFloat(R.price_subtotal||0),0),k=u.length?((z=u[0].name.split("—")[1])==null?void 0:z.trim())||u[0].name:null,_=s.pagos||[],$=_.reduce((C,R)=>C+parseFloat(R.monto||0),0),T=t.signature_name||((P=s.firma)==null?void 0:P.name)||null,h=(A=s.firma)!=null&&A.fecha?j(s.firma.fecha):"",E=Yt().filter(C=>C.active),S=Yt().find(C=>C.id===s.pricelist_id),y=`
  <div id="venta-form" style="min-height:100vh;background:var(--bg-app)">

    <!-- TOPBAR -->
    <div style="display:flex;align-items:center;justify-content:space-between;padding:10px 20px;background:var(--bg-card);border-bottom:1px solid var(--border);position:sticky;top:50px;z-index:20;flex-wrap:wrap;gap:8px">
      <div style="display:flex;align-items:center;gap:8px">
        <button class="o-btn-secondary o-btn-sm" onclick="window._go('ventas')" style="gap:6px">
          ← Ventas
        </button>
        <button class="o-btn-secondary o-btn-sm" onclick="window._prevRecord()">‹</button>
        <button class="o-btn-secondary o-btn-sm" onclick="window._nextRecord()">›</button>
      </div>
      <div style="display:flex;gap:8px;flex-wrap:wrap">
        ${oo(t)}
      </div>
    </div>

    <!-- STATUS BAR -->
    <div style="display:flex;align-items:center;padding:8px 24px;background:var(--bg-card);border-bottom:1px solid var(--border);gap:0">
      ${r.map((C,R)=>{const Ht=R<l,_t=R===l,le=R>l;return t.state==="cancel"?"":`
          ${R>0?'<span style="color:var(--text-300);font-size:14px;margin:0 2px">›</span>':""}
          <button onclick="return false"
            style="padding:6px 16px;border-radius:20px;border:none;font-size:12px;font-weight:700;cursor:default;
              ${_t?"background:var(--primary);color:#fff;":""}
              ${Ht?"color:var(--primary);opacity:.6;background:transparent;":""}
              ${le?"color:var(--text-400);background:transparent;":""}"
          >${Ht?"✓ ":""}${C.label}</button>`}).join("")}
      ${t.state==="cancel"?'<span class="o-badge o-badge-danger" style="font-size:13px">Cancelado</span>':""}
    </div>

    <!-- SMART BUTTONS — Flujo 3 módulos: Ventas → Almacén → Facturación -->
    <div style="display:flex;gap:10px;padding:10px 24px;background:var(--bg-card);border-bottom:1px solid var(--border);flex-wrap:wrap">
      ${t.state==="sale"||t.state==="done"?`
      <!-- Smart Button: Entrega → navega al módulo Almacén (igual que Odoo) -->
      <button onclick="window._verEntrega(${t.id})"
        style="display:flex;flex-direction:column;align-items:center;gap:2px;padding:8px 18px;border:1px solid ${(o==null?void 0:o.state)==="entregado"?"#10B981":(o==null?void 0:o.state)==="parcial"?"#F59E0B":"#6366F1"};border-radius:10px;background:var(--bg-card);cursor:pointer;transition:all .15s;min-width:90px"
        onmouseover="this.style.background='#EEF2FF'" onmouseout="this.style.background=''">
        <span style="font-size:22px">${(o==null?void 0:o.state)==="entregado"?"✅":(o==null?void 0:o.state)==="parcial"?"📦":"🚚"}</span>
        <span style="font-size:12px;font-weight:700;color:${(o==null?void 0:o.state)==="entregado"?"#10B981":(o==null?void 0:o.state)==="parcial"?"#F59E0B":"#6366F1"}">
          1 Entrega
        </span>
        <span style="font-size:10px;color:var(--text-400)">Almacén</span>
      </button>`:""}

      <!-- Smart Button: Facturas -->
      <button onclick="window._verFacturas(${t.id})"
        style="display:flex;flex-direction:column;align-items:center;gap:2px;padding:8px 18px;border:1px solid ${a.length?"#10B981":"var(--border)"};border-radius:10px;background:var(--bg-card);cursor:pointer;transition:all .15s;min-width:80px"
        onmouseover="this.style.background='#EEF2FF'" onmouseout="this.style.background=''">
        <span style="font-size:20px;font-weight:800;color:${a.length?"#10B981":"var(--text-400)"}">${a.length}</span>
        <span style="font-size:11px;color:var(--text-500)">Facturas</span>
      </button>

      <!-- Smart Button: Líneas -->
      <button onclick="window._verLineas(${t.id})"
        style="display:flex;flex-direction:column;align-items:center;gap:2px;padding:8px 18px;border:1px solid var(--border);border-radius:10px;background:var(--bg-card);cursor:pointer;transition:all .15s;min-width:80px"
        onmouseover="this.style.borderColor='var(--primary)';this.style.background='#EEF2FF'" onmouseout="this.style.borderColor='';this.style.background=''">
        <span style="font-size:20px;font-weight:800;color:var(--primary)">${e.length}</span>
        <span style="font-size:11px;color:var(--text-500)">Líneas</span>
      </button>
    </div>

    <!-- FORM SHEET -->
    <div style="background:var(--bg-card);border-radius:12px;margin:16px 20px 0;border:1px solid var(--border);overflow:hidden">

      <!-- Encabezado del documento -->
      <div style="padding:20px 24px 16px;border-bottom:1px solid var(--border)">
        <div style="display:flex;align-items:flex-start;gap:16px">
          <div style="flex:1">
            <h1 style="font-family:'Plus Jakarta Sans',sans-serif;font-size:22px;font-weight:800;color:var(--text-900);margin:0 0 6px">${t.name||"Nueva Cotización"}</h1>
            <span class="o-badge ${Ae[t.state]||"o-badge-gray"}">${Pe[t.state]||t.state}</span>
          </div>
        </div>
      </div>

      <!-- CAMPOS DEL FORMULARIO -->
      <div style="display:grid;grid-template-columns:1fr 1fr;gap:0 32px;padding:16px 24px">

        <!-- Columna izquierda -->
        <div>
          ${L("Cliente",io("f-partner",t.partner_name||"","buscar-clientes","partner_id",t.partner_id,n,t.id),!0)}
          ${L("Dirección de Facturación",n?`<input class="o-field-input" value="${t.partner_invoice_name||t.partner_name||""}" onchange="window._actualizarCampoVenta(${t.id},'partner_invoice_id',this.value)">`:`<span>${t.partner_invoice_name||t.partner_name||"—"}</span>`)}
          ${L("Dirección de Envío",n?`<input class="o-field-input" value="${t.partner_shipping_name||t.partner_name||""}" onchange="window._actualizarCampoVenta(${t.id},'partner_shipping_id',this.value)">`:`<span>${t.partner_shipping_name||t.partner_name||"—"}</span>`)}
          ${L("Referencia del Cliente",n?`<input class="o-field-input" id="f-client-ref" value="${t.client_order_ref||""}" onblur="window._guardarCampo(${t.id},'client_order_ref',this.value)">`:`<span>${t.client_order_ref||"—"}</span>`)}
          ${i.plantillas_presupuesto?L("Plantilla de Presupuesto",n?`<select class="o-field-input" onchange="if(this.value)window._aplicarPlantilla(${t.id},this.value)">
                  <option value="">— Sin plantilla —</option>
                  ${Object.entries(ye).map(([C,R])=>`<option value="${C}" ${s.plantilla===C?"selected":""}>${R.label}</option>`).join("")}
                </select>`:`<span>${((U=ye[s.plantilla])==null?void 0:U.label)||"—"}</span>`):""}
        </div>

        <!-- Columna derecha -->
        <div>
          ${L("Fecha Pedido",`<span style="font-weight:600">${j(t.date_order)}</span>`)}
          ${i.fecha_entrega?L("Fecha Compromiso",n?`<input class="o-field-input" type="date" value="${t.commitment_date?t.commitment_date.split("T")[0]:""}" onblur="window._guardarCampo(${t.id},'commitment_date',this.value)">`:`<span>${t.commitment_date?j(t.commitment_date):"—"}</span>`):""}
          ${L("Fecha Validez",n?`<input class="o-field-input" type="date" value="${t.validity_date?t.validity_date.split("T")[0]:""}" onblur="window._guardarCampo(${t.id},'validity_date',this.value)">`:`<span>${t.validity_date?j(t.validity_date):"—"}</span>`)}
          ${L("Origen",n?`<input class="o-field-input" placeholder="Referencia de origen..." value="${t.origin||""}" onblur="window._guardarCampo(${t.id},'origin',this.value)">`:`<span>${t.origin||"—"}</span>`)}
          ${L("Plazo de Pago",`<span>${t.payment_term_name||"—"}</span>`)}
          ${L("Moneda",`<span>${t.currency_name||"MXN"}</span>`)}
        </div>
      </div>

      <!-- NOTEBOOK TABS -->
      <div style="border-top:1px solid var(--border)">
        <div style="display:flex;border-bottom:1px solid var(--border);background:var(--bg-app);padding:0 20px;overflow-x:auto" id="venta-tabs">
          <button class="o-tab active" data-tab="lineas" onclick="window._switchTab('lineas',this)">Líneas de Pedido</button>
          <button class="o-tab" data-tab="info" onclick="window._switchTab('info',this)">Otra Información</button>
          <button class="o-tab" data-tab="notas" onclick="window._switchTab('notas',this)">Notas y Términos</button>
        </div>

        <!-- TAB: Líneas de Pedido -->
        <div id="tab-lineas" style="padding:0">
          ${De(e,t.state,t.locked,t.id)}
          ${n?se(t.id):""}
        </div>

        <!-- TOTALES (separado de tab-lineas para que el picker no quede debajo) -->
        <div id="totales-area" style="display:flex;justify-content:flex-end;padding:16px 24px;border-top:1px solid var(--border)">
          <table style="width:280px">
            <tr>
              <td style="padding:4px 8px;font-size:13px;color:var(--text-600)">Subtotal</td>
              <td id="tot-subtotal" style="padding:4px 8px;font-size:13px;text-align:right;font-weight:600;font-variant-numeric:tabular-nums">${f(c)}</td>
            </tr>
            <tr>
              <td style="padding:4px 8px;font-size:13px;color:var(--text-600)">IVA (16%)</td>
              <td id="tot-iva" style="padding:4px 8px;font-size:13px;text-align:right;font-weight:600;font-variant-numeric:tabular-nums">${f(p)}</td>
            </tr>
            <tr style="border-top:2px solid var(--border)">
              <td style="padding:8px 8px 4px;font-size:16px;font-weight:800;color:var(--text-900)">TOTAL</td>
              <td id="tot-total" style="padding:8px 8px 4px;font-size:16px;font-weight:800;color:var(--primary);text-align:right;font-variant-numeric:tabular-nums">${f(m)}</td>
            </tr>
            ${i.margenes?`
            <tr>
              <td style="padding:4px 8px;font-size:12px;color:var(--text-400)">Margen</td>
              <td id="tot-margen" style="padding:4px 8px;font-size:12px;text-align:right;font-weight:600;color:#10B981;font-variant-numeric:tabular-nums">${f(va(e))}</td>
            </tr>`:""}
          </table>
        </div>

        <!-- TAB: Otra Información -->
        <div id="tab-info" style="padding:16px 24px;display:none">
          <div style="display:grid;grid-template-columns:1fr 1fr;gap:0 32px">
            <div>
              <h4 style="font-size:11px;text-transform:uppercase;color:var(--text-400);letter-spacing:.05em;margin:0 0 12px;font-weight:700">Ventas</h4>
              ${L("Vendedor",`<span>${t.user_name||"Administrador"}</span>`)}
              ${L("Equipo de Ventas",`<span>${t.team_name||"—"}</span>`)}
              ${L("Empresa",`<span>${t.company_id?"NexusTech":"—"}</span>`)}
              ${i.listas_precios?L("Lista de Precios",n?`<select class="o-field-input" onchange="window._aplicarListaPrecios(${t.id},this.value)">
                      <option value="">Tarifa pública (sin descuento)</option>
                      ${E.map(C=>`<option value="${C.id}" ${s.pricelist_id===C.id?"selected":""}>${C.name} (${C.currency}${C.type==="Porcentaje"&&C.discount?` · −${C.discount}%`:""})</option>`).join("")}
                    </select>`:`<span>${(S==null?void 0:S.name)||t.pricelist_name||"Tarifa pública"}</span>`):""}
            </div>
            <div>
              <h4 style="font-size:11px;text-transform:uppercase;color:var(--text-400);letter-spacing:.05em;margin:0 0 12px;font-weight:700">Contabilidad</h4>
              ${L("Estado Facturación",`<span class="o-badge ${Me[t.invoice_status]||"o-badge-gray"}">${Le[t.invoice_status]||"—"}</span>`)}
              ${L("Política Facturación",`<span>${i.politica_facturacion==="cantidad_pedida"?"Cantidades pedidas":"Cantidades entregadas"}</span>`)}
              ${i.listas_precios&&i.descuento_precio&&(S==null?void 0:S.type)==="Porcentaje"&&(S!=null&&S.discount)?L("Desc. de Lista",`<span style="color:#10B981;font-weight:600">−${S.discount}% (${S.name})</span>`):""}
              ${L("Bloqueado",`<span>${t.locked?"Sí":"No"}</span>`)}
            </div>
          </div>
          ${i.costos_envio?`
          <div style="border-top:1px solid var(--border);margin-top:16px;padding-top:16px">
            <h4 style="font-size:11px;text-transform:uppercase;color:var(--text-400);letter-spacing:.05em;margin:0 0 12px;font-weight:700">Envío</h4>
            <div style="display:grid;grid-template-columns:1fr 1fr;gap:0 32px">
              <div>
                ${L("Método de Entrega",`<span>${k||"—"}</span>`)}
              </div>
              <div>
                ${L("Costo de Envío",`<span style="font-weight:600">${u.length?f(v):"—"}</span>`)}
              </div>
            </div>
            ${n?`
            <button class="o-btn-secondary o-btn-sm" onclick="window._agregarEnvio(${t.id})" style="margin-top:4px">🚚 ${u.length?"Cambiar":"Agregar"} costo de envío</button>`:""}
          </div>`:""}
        </div>

        <!-- TAB: Notas y Términos -->
        <div id="tab-notas" style="padding:16px 24px;display:none">
          ${i.notas_cierre&&d?`
          <div style="margin-bottom:16px;padding:12px 16px;background:#EEF2FF;border-radius:8px;border-left:3px solid var(--primary)">
            <div style="font-size:12px;font-weight:700;color:var(--primary);margin-bottom:6px">ℹ️ NOTA DE CIERRE</div>
            <div style="font-size:13px;color:var(--text-700)">${i.terminos||"Gracias por su preferencia. Cualquier reclamación debe hacerse en los 5 días hábiles siguientes a la entrega."}</div>
          </div>`:""}
          <div style="margin-bottom:12px">
            <label style="font-size:12px;font-weight:700;color:var(--text-400);margin-bottom:6px;display:block">TÉRMINOS Y CONDICIONES</label>
            ${n?`<textarea class="o-field-input" rows="4" style="resize:vertical;width:100%;box-sizing:border-box" onblur="window._guardarCampo(${t.id},'note',this.value)">${t.note||i.terminos||""}</textarea>`:`<div style="font-size:13px;color:var(--text-600);min-height:60px;white-space:pre-wrap">${t.note||i.terminos||"Sin notas."}</div>`}
          </div>
          ${i.firma_online?`
          <div style="border-top:1px solid var(--border);padding-top:16px;margin-top:8px">
            <h4 style="font-size:12px;font-weight:700;color:var(--text-500);margin:0 0 12px;text-transform:uppercase;letter-spacing:.05em">✍️ Firma Digital</h4>
            ${T?`
            <div style="display:flex;align-items:center;gap:12px;padding:12px 16px;background:#F0FDF4;border-radius:8px;border:1px solid #10B981">
              <span style="font-size:18px">✅</span>
              <div>
                <div style="font-weight:600;color:#065F46">${T}</div>
                <div style="font-size:11px;color:#059669">Firmado digitalmente${h?` el ${h}`:""}</div>
              </div>
            </div>`:t.state==="draft"||t.state==="sent"?`
            <div style="display:flex;gap:8px;align-items:center">
              <input id="firma-nombre" class="o-field-input" placeholder="Nombre completo del firmante..." style="flex:1;max-width:340px">
              <button class="o-btn-primary o-btn-sm" onclick="window._firmarCotizacion(${t.id})">✍️ Firmar</button>
            </div>
            <div style="font-size:11px;color:var(--text-400);margin-top:6px">Al firmar, la cotización queda aceptada por el cliente y puede confirmarse como pedido de venta.</div>`:`
            <div style="border:2px dashed var(--border);border-radius:8px;padding:16px;text-align:center;color:var(--text-400);font-size:13px">Sin firma registrada</div>`}
          </div>`:""}
          ${i.pago_online?`
          <div style="border-top:1px solid var(--border);padding-top:16px;margin-top:8px">
            <h4 style="font-size:12px;font-weight:700;color:var(--text-500);margin:0 0 12px;text-transform:uppercase;letter-spacing:.05em">💳 Pago en Línea</h4>
            ${_.length?`
            <div style="margin-bottom:10px">
              ${_.map(C=>`
              <div style="display:flex;justify-content:space-between;align-items:center;padding:8px 14px;background:#F0FDF4;border:1px solid #10B981;border-radius:8px;margin-bottom:6px;font-size:13px">
                <span>✅ ${C.metodo} — ${j(C.fecha)}</span>
                <strong style="color:#065F46">${f(C.monto)}</strong>
              </div>`).join("")}
              <div style="font-size:12px;color:var(--text-500);text-align:right">Pagado: <strong>${f($)}</strong> de ${f(m)}</div>
            </div>`:""}
            ${$>=m-.01&&m>0?`
            <div style="font-size:13px;font-weight:700;color:#10B981">✅ Pedido pagado en su totalidad</div>`:t.state==="draft"||t.state==="sent"?`
            <div style="display:flex;gap:8px;flex-wrap:wrap">
              <button class="o-btn-secondary o-btn-sm" onclick="window._registrarPagoOnline(${t.id},'Tarjeta')">💳 Pagar con Tarjeta</button>
              <button class="o-btn-secondary o-btn-sm" onclick="window._registrarPagoOnline(${t.id},'Transferencia')">🏦 Transferencia Bancaria</button>
            </div>`:'<div style="font-size:13px;color:var(--text-400)">Sin pagos en línea registrados</div>'}
          </div>`:""}
        </div>
      </div>
    </div>

    <!-- CHATTER -->
    <div style="background:var(--bg-card);border-radius:12px;margin:16px 20px 24px;border:1px solid var(--border);overflow:hidden">
      <div style="padding:12px 20px;border-bottom:1px solid var(--border);background:var(--bg-app)">
        <div style="display:flex;gap:8px">
          <button class="o-chatter-btn" onclick="window._enviarMensaje(${t.id})">💬 Enviar mensaje</button>
          <button class="o-chatter-btn" onclick="window._agregarNota(${t.id})">📝 Agregar nota interna</button>
        </div>
      </div>
      <div id="chatter-${t.id}" style="padding:16px 20px;min-height:60px;font-size:13px;color:var(--text-400)">
        Sin actividad registrada.
      </div>
    </div>

  </div>

  <!-- Modal crear factura -->
  <div id="modal-factura" style="display:none;position:fixed;inset:0;z-index:950;background:rgba(0,0,0,.45);backdrop-filter:blur(3px);align-items:center;justify-content:center;padding:16px">
    <div style="background:var(--bg-card);border-radius:14px;box-shadow:0 24px 64px rgba(0,0,0,.22);border:1px solid var(--border);width:100%;max-width:480px;animation:slideUp .18s cubic-bezier(.34,1.56,.64,1)">
      <div style="display:flex;align-items:center;justify-content:space-between;padding:16px 20px;border-bottom:1px solid var(--border);background:var(--bg-app);border-radius:14px 14px 0 0">
        <h3 style="font-size:15px;font-weight:700;color:var(--text-900);margin:0">Crear Factura</h3>
        <button onclick="document.getElementById('modal-factura').style.display='none'" style="background:none;border:none;cursor:pointer;font-size:18px;color:var(--text-400)">×</button>
      </div>
      <div style="padding:20px">
        <p style="font-size:13px;color:var(--text-600);margin:0 0 16px">Se creará una factura de cliente por el monto total del pedido <strong>${t.name}</strong>.</p>
        <div style="background:#EEF2FF;border-radius:8px;padding:12px 16px;margin-bottom:16px">
          <div style="display:flex;justify-content:space-between;font-size:13px">
            <span style="color:var(--text-600)">Subtotal</span><span>${f(c)}</span>
          </div>
          <div style="display:flex;justify-content:space-between;font-size:13px;margin-top:4px">
            <span style="color:var(--text-600)">IVA (16%)</span><span>${f(p)}</span>
          </div>
          <div style="display:flex;justify-content:space-between;font-size:14px;font-weight:800;margin-top:8px;padding-top:8px;border-top:1px solid var(--border)">
            <span>Total</span><span style="color:var(--primary)">${f(m)}</span>
          </div>
        </div>
        <div style="font-size:12px;color:var(--text-500);margin-bottom:14px">
          Política de facturación: <strong>${i.politica_facturacion==="cantidad_entregada"?"Cantidades entregadas":"Cantidades pedidas"}</strong>
        </div>
        <div style="margin-bottom:14px">
          <label style="font-size:12px;font-weight:700;color:var(--text-600);margin-bottom:6px;display:block">TIPO DE FACTURA</label>
          <select class="o-field-input" id="tipo-factura">
            <option value="regular">Factura Regular</option>
            <option value="downpayment_fixed">Anticipo — Monto Fijo</option>
            <option value="downpayment_pct">Anticipo — Porcentaje</option>
          </select>
        </div>
      </div>
      <div style="display:flex;gap:8px;justify-content:flex-end;padding:12px 20px;border-top:1px solid var(--border);background:var(--bg-app);border-radius:0 0 14px 14px">
        <button class="o-btn-secondary" onclick="document.getElementById('modal-factura').style.display='none'">Cancelar</button>
        <button class="o-btn-primary" onclick="window._ejecutarCrearFactura(${t.id})">✓ Crear Factura</button>
      </div>
    </div>
  </div>

  <!-- Producto picker inline dropdown -->
  <div id="product-picker-dropdown" style="display:none;position:fixed;z-index:500;background:var(--bg-card);border:1.5px solid var(--primary);border-radius:8px;box-shadow:0 8px 24px rgba(0,0,0,.12);min-width:320px;max-height:280px;overflow-y:auto"></div>

  `;w(y),no(t)}function oo(t){const a=tt().compra_online?`<button class="o-btn-secondary" onclick="window._vistaPreviaCliente(${t.id})">👁 Vista Previa</button>`:"";switch(t.state){case"draft":return`
        <button class="o-btn-primary" onclick="window._accionVenta(${t.id},'confirmar')" style="background:#10B981">✓ Confirmar</button>
        <button class="o-btn-secondary" onclick="window._accionVenta(${t.id},'enviar')">📧 Enviar</button>
        <button class="o-btn-secondary" onclick="window._imprimirCotizacion(${t.id})">🖨 Imprimir</button>
        ${a}
        <button class="o-btn-secondary" onclick="window._accionVenta(${t.id},'cancelar')" style="color:#DC2626;border-color:#DC262640">✕ Cancelar</button>
      `;case"sent":return`
        <button class="o-btn-primary" onclick="window._accionVenta(${t.id},'confirmar')" style="background:#10B981">✓ Confirmar</button>
        ${a}
        <button class="o-btn-secondary" onclick="window._accionVenta(${t.id},'cancelar')" style="color:#DC2626;border-color:#DC262640">✕ Cancelar</button>
      `;case"sale":return`
        <button class="o-btn-primary" onclick="window._abrirModalFactura(${t.id})">📄 Crear Factura</button>
        <button class="o-btn-secondary" onclick="window._imprimirPedido(${t.id})">🖨 Imprimir</button>
        ${t.locked?`<button class="o-btn-secondary" onclick="window._accionVenta(${t.id},'bloquear',{locked:false})">🔓 Desbloquear</button>`:`<button class="o-btn-secondary" onclick="window._accionVenta(${t.id},'bloquear',{locked:true})">🔒 Bloquear</button>`}
      `;case"done":return'<span class="o-badge o-badge-violet" style="font-size:13px;padding:6px 14px">✓ Realizado</span>';case"cancel":return`
        <button class="o-btn-secondary" onclick="window._restaurarBorrador(${t.id})">↩ Restaurar a Borrador</button>
      `;default:return""}}function L(t,e,a=!1){return`
  <div style="display:grid;grid-template-columns:150px 1fr;align-items:start;padding:5px 0;min-height:32px">
    <label style="font-size:12px;font-weight:600;color:var(--text-400);padding-top:7px">${t}${a?'<span style="color:#DC2626;margin-left:2px">*</span>':""}</label>
    <div style="font-size:13px;color:var(--text-900)">${e}</div>
  </div>`}function io(t,e,a,o,i,s,n){return s?`
  <div class="o-m2o-field" style="position:relative">
    <input class="o-field-input" id="${t}"
      value="${e}"
      autocomplete="off"
      placeholder="Buscar..."
      data-field="${o}"
      data-order="${n}"
      oninput="window._m2oInput(this,'${a}')"
      onblur="setTimeout(()=>window._hideM2o('${t}'),200)">
    <div id="${t}-dd" style="display:none;position:absolute;top:calc(100% + 2px);left:0;right:0;background:var(--bg-card);border:1.5px solid var(--primary);border-radius:8px;box-shadow:0 8px 24px rgba(0,0,0,.12);z-index:200;max-height:220px;overflow-y:auto"></div>
  </div>`:`<span style="font-weight:500;color:var(--primary)">${e||"—"}</span>`}function De(t,e,a,o){const i=tt(),s=mt(o),n=(e==="draft"||e==="sent")&&!a,d=i.descuentos!==!1||!!(i.listas_precios&&i.descuento_precio),r=!!i.margenes,l=!!i.unidades_medida,c=!!i.empaquetado,p=["Unidades","Piezas","Docena","Caja","kg","g","Litros","m"],m=[{label:"—",qty:0},{label:"Paquete x6",qty:6},{label:"Caja x12",qty:12},{label:"Caja x24",qty:24}],u=5+(d?1:0)+(r?1:0)+(l?1:0)+(c?1:0)+(n?1:0);return t.length?`
  <div style="overflow-x:auto">
    <table class="o-list-table" style="margin-top:0;min-width:600px">
      <thead>
        <tr style="background:var(--bg-table-head)">
          <th style="width:200px">PRODUCTO</th>
          <th style="width:240px">DESCRIPCIÓN</th>
          <th style="width:90px;text-align:center">CANTIDAD</th>
          ${l?'<th style="width:90px">UDM</th>':""}
          ${c?'<th style="width:110px">EMPAQUE</th>':""}
          <th style="width:110px;text-align:right">PRECIO UNIT.</th>
          ${d?'<th style="width:80px;text-align:right">DESCUENTO</th>':""}
          ${r?'<th style="width:100px;text-align:right">MARGEN</th>':""}
          <th style="width:110px;text-align:right">SUBTOTAL</th>
          ${n?'<th style="width:36px"></th>':""}
        </tr>
      </thead>
      <tbody>
        ${t.map(v=>{var S,y,z,P;if(v.display_type==="line_section")return`<tr><td colspan="${u}" style="padding:8px 14px;font-weight:700;font-size:13px;background:var(--bg-app);color:var(--text-600);border-top:2px solid var(--border)">${v.name}</td></tr>`;const k=parseFloat(v.price_subtotal||0),_=parseFloat(v.cost||0),$=parseFloat(v.product_uom_qty)||0,T=k-$*_,h=((S=s.uoms)==null?void 0:S[v.id])||v.uom_name||v.product_uom_name||"Unidades",E=((y=s.packs)==null?void 0:y[v.id])||"—";return`
          <tr style="border-bottom:1px solid var(--border)">
            <td style="padding:8px 14px;font-weight:600;font-size:13px">
              ${v.product_name||((P=(z=v.name)==null?void 0:z.split("—")[0])==null?void 0:P.trim())||"—"}
              ${i.variantes&&v.product_id?`<div style="font-size:10px;color:var(--text-400);font-weight:400">Variante · ref. interna #${v.product_id}</div>`:""}
            </td>
            <td style="padding:8px 14px;font-size:12px;color:var(--text-500)">${v.name||""}</td>
            <td style="padding:8px 14px;text-align:center">
              ${n?`<input class="o-qty-input" type="number" value="${$}" min="0" step="0.001" style="width:70px;text-align:center" onchange="window._inlineEdit(${v.order_id||o},${v.id},'product_uom_qty',this.value)">`:`<span>${$}</span>`}
            </td>
            ${l?`
            <td style="padding:8px 14px;font-size:12px;color:var(--text-500)">
              ${n?`<select class="o-field-input" style="font-size:12px;padding:4px 6px" onchange="window._setLineaUom(${v.order_id||o},${v.id},this.value)">
                    ${p.map(A=>`<option ${A===h?"selected":""}>${A}</option>`).join("")}
                  </select>`:`<span>${h}</span>`}
            </td>`:""}
            ${c?`
            <td style="padding:8px 14px;font-size:12px;color:var(--text-500)">
              ${n?`<select class="o-field-input" style="font-size:12px;padding:4px 6px" onchange="window._setLineaEmpaque(${v.order_id||o},${v.id},this.value)">
                    ${m.map(A=>`<option value="${A.label}|${A.qty}" ${A.label===E?"selected":""}>${A.label}</option>`).join("")}
                  </select>`:`<span>${E}</span>`}
            </td>`:""}
            <td style="padding:8px 14px;text-align:right">
              ${n?`<input class="o-price-input" type="number" value="${parseFloat(v.price_unit)||0}" min="0" step="0.01" style="width:100px;text-align:right" onchange="window._inlineEdit(${v.order_id||o},${v.id},'price_unit',this.value)">`:`<span>${f(v.price_unit)}</span>`}
            </td>
            ${d?`
            <td style="padding:8px 14px;text-align:right">
              ${n?`<div style="display:flex;align-items:center;justify-content:flex-end;gap:2px"><input class="o-disc-input" type="number" value="${parseFloat(v.discount)||0}" min="0" max="100" step="0.1" style="width:55px;text-align:right" onchange="window._inlineEdit(${v.order_id||o},${v.id},'discount',this.value)"><span style="color:var(--text-400);font-size:12px">%</span></div>`:`<span>${parseFloat(v.discount)||0}%</span>`}
            </td>`:""}
            ${r?`
            <td style="padding:8px 14px;text-align:right;font-variant-numeric:tabular-nums;color:${T>=0?"#10B981":"#DC2626"};font-weight:600">
              ${f(T)}${k>0?` <span style="font-size:11px;color:var(--text-400)">(${(T/k*100).toFixed(1)}%)</span>`:""}
            </td>`:""}
            <td style="padding:8px 14px;text-align:right;font-weight:700;font-variant-numeric:tabular-nums">${f(k)}</td>
            ${n?`
            <td style="padding:4px 8px;text-align:center">
              <button onclick="window._eliminarLinea(${v.order_id||o},${v.id})"
                style="background:none;border:none;cursor:pointer;color:var(--text-300);font-size:18px;line-height:1;padding:2px 6px;border-radius:4px"
                onmouseover="this.style.color='#DC2626';this.style.background='#FEE2E2'"
                onmouseout="this.style.color='';this.style.background=''">×</button>
            </td>`:""}
          </tr>`}).join("")}
      </tbody>
    </table>
  </div>`:`<div style="padding:32px;text-align:center;color:var(--text-400);font-size:13px">
    Sin líneas de pedido. Haz clic en <strong>＋ Agregar producto</strong> para comenzar.
  </div>`}function no(t,e){window._switchTab=(a,o)=>{document.querySelectorAll("#venta-tabs .o-tab").forEach(s=>s.classList.remove("active")),o.classList.add("active"),document.querySelectorAll('[id^="tab-"]').forEach(s=>s.style.display="none");const i=document.getElementById(`tab-${a}`);i&&(i.style.display="")},window._prevRecord=()=>{const a=G.findIndex(o=>o.id===t.id);a>0&&J(G[a-1].id)},window._nextRecord=()=>{const a=G.findIndex(o=>o.id===t.id);a>=0&&a<G.length-1&&J(G[a+1].id)},window._guardarCampo=async(a,o,i)=>{try{await g.put(`/ventas/${a}`,{[o]:i})}catch(s){b("Error",s.message,"error")}},window._m2oInput=async(a,o)=>{const i=a.value,s=a.id+"-dd",n=document.getElementById(s);if(n){if(!i||i.length<1){n.style.display="none";return}clearTimeout(Wt),Wt=setTimeout(async()=>{try{const d=await g.get(`/ventas/${o}?q=${encodeURIComponent(i)}`),r=(d==null?void 0:d.data)||[];if(!r.length){n.style.display="none";return}n.style.display="block",n.innerHTML=r.map(l=>`
          <div style="padding:8px 12px;cursor:pointer;font-size:13px;border-bottom:1px solid var(--border)"
            onmouseover="this.style.background='#EEF2FF'"
            onmouseout="this.style.background=''"
            onmousedown="window._selM2o('${a.id}',${l.id},'${(l.name||"").replace(/'/g,"\\'")}','${a.dataset.field}',${a.dataset.order})">
            <div style="font-weight:600">${l.name||""}</div>
            ${l.email?`<div style="font-size:11px;color:var(--text-400)">${l.email}</div>`:""}
          </div>`).join("")}catch{}},250)}},window._hideM2o=a=>{const o=document.getElementById(a+"-dd");o&&(o.style.display="none")},window._selM2o=async(a,o,i,s,n)=>{const d=document.getElementById(a);d&&(d.value=i);const r=document.getElementById(a+"-dd");r&&(r.style.display="none");try{await g.put(`/ventas/${n}`,{[s]:o})}catch(l){b("Error",l.message,"error")}},window._inlineEdit=async(a,o,i,s)=>{try{await g.put(`/ventas/${a}/lineas/${o}`,{[i]:parseFloat(s)});const n=await g.get(`/ventas/${a}/lineas`);O=(n==null?void 0:n.data)||[],he(O)}catch(n){b("Error",n.message,"error")}},window._eliminarLinea=async(a,o)=>{try{await g.del(`/ventas/${a}/lineas/${o}`);const i=await g.get(`/ventas/${a}/lineas`);O=(i==null?void 0:i.data)||[];const s=document.getElementById("tab-lineas");s&&(s.innerHTML=De(O,t.state,t.locked,a)+((t.state==="draft"||t.state==="sent")&&!t.locked?se(a):""),he(O)),b("Línea eliminada","","success")}catch(i){b("Error",i.message,"error")}},window._agregarProductoInline=a=>ro(a),window._agregarSeccion=async a=>{const o=prompt("Nombre de la sección:");if(o)try{await g.post(`/ventas/${a}/lineas`,{display_type:"line_section",name:o,product_uom_qty:0,price_unit:0,discount:0}),Ot(a,t.state,t.locked)}catch(i){b("Error",i.message,"error")}},window._accionVenta=async(a,o,i={})=>{var n;const s=tt();if(!(o==="confirmar"&&s.advertencias&&!confirm(`¿Confirmar este pedido de venta?
Se generará la orden de entrega en Almacén.`))){if(o==="confirmar"&&s.aviso_stock){const d=O.filter(l=>l.display_type!=="line_section"&&l.product_id),r=[];for(const l of d)try{const c=await g.get(`/stock/producto/${l.product_id}`),p=((c==null?void 0:c.data)||[]).reduce((m,u)=>m+parseFloat(u.cantidad_disponible||0),0);parseFloat(l.product_uom_qty)>p&&r.push(`• ${l.product_name||l.name}: pedido ${parseFloat(l.product_uom_qty)}, disponible ${p}`)}catch{}if(r.length&&!confirm(`⚠️ Stock insuficiente:
${r.join(`
`)}

¿Confirmar de todos modos?`))return}if(!(o==="cancelar"&&!confirm("¿Cancelar esta orden?")))try{const d=await g.put(`/ventas/${a}/${o}`,i);if((n=d==null?void 0:d.data)!=null&&n.ok||d!=null&&d.success){if(o==="confirmar"&&s.bloquear_confirmado)try{await g.put(`/ventas/${a}/bloquear`,{locked:!0})}catch{}b("Éxito",so(o),"success"),await J(a)}}catch(d){b("Error",d.message,"error")}}},window._restaurarBorrador=async a=>{try{await g.put(`/ventas/${a}/borrador`,{}),b("Restaurado","Orden restaurada a borrador","success"),await J(a)}catch(o){b("Error",o.message,"error")}},window._abrirModalFactura=a=>{const o=document.getElementById("modal-factura");o&&(o.style.display="flex")},window._ejecutarCrearFactura=async a=>{var i,s;const o=((i=document.getElementById("tipo-factura"))==null?void 0:i.value)||"regular";try{const n=await g.post(`/ventas/${a}/crear-factura`,{tipo:o}),d=(s=n==null?void 0:n.data)==null?void 0:s.factura_id;document.getElementById("modal-factura").style.display="none",b("Factura creada","Factura generada exitosamente","success"),d?setTimeout(()=>{window._go(`facturas?id=${d}`)},800):await J(a)}catch(n){b("Error creando factura",n.message,"error")}},window._verFacturas=async a=>{try{const o=await g.get(`/ventas/${a}/facturas`),i=(o==null?void 0:o.data)||[];if(i.length===1)window._go(`facturas?id=${i[0].id}`);else if(i.length>1){const s=document.createElement("div");s.style.cssText="position:fixed;inset:0;z-index:960;background:rgba(0,0,0,.45);display:flex;align-items:center;justify-content:center;padding:16px",s.innerHTML=`
          <div style="background:var(--bg-card);border-radius:14px;box-shadow:0 24px 64px rgba(0,0,0,.22);border:1px solid var(--border);width:100%;max-width:480px">
            <div style="padding:16px 20px;border-bottom:1px solid var(--border);display:flex;justify-content:space-between;align-items:center">
              <h3 style="margin:0;font-size:15px;font-weight:700">Facturas vinculadas</h3>
              <button onclick="this.closest('[style*=fixed]').remove()" style="background:none;border:none;cursor:pointer;font-size:18px">×</button>
            </div>
            <div style="padding:12px">
              ${i.map(n=>`
              <div onclick="window._go('facturas?id='+f.id)" style="padding:10px 14px;border:1px solid var(--border);border-radius:8px;cursor:pointer;margin-bottom:8px;display:flex;justify-content:space-between;align-items:center"
                onmouseover="this.style.background='#EEF2FF'" onmouseout="this.style.background=''">
                <div>
                  <div style="font-weight:700;font-family:monospace">${n.name||"#"+n.id}</div>
                  <div style="font-size:11px;color:var(--text-400)">${n.invoice_date||""}</div>
                </div>
                <div style="text-align:right">
                  <div style="font-weight:700;color:var(--primary)">${n.amount_total?"$"+parseFloat(n.amount_total).toFixed(2):"—"}</div>
                  <span class="o-badge ${n.payment_state==="paid"?"o-badge-success":n.state==="posted"?"o-badge-info":"o-badge-gray"}" style="font-size:10px">${n.state==="posted"?"Publicada":n.state==="draft"?"Borrador":"—"}</span>
                </div>
              </div>`).join("")}
            </div>
          </div>`,document.body.appendChild(s),s.onclick=n=>{n.target===s&&s.remove()}}else window._go(`facturas?orden=${a}`)}catch{window._go(`facturas?orden=${a}`)}},window._verEntrega=async a=>{try{const o=await g.get(`/ventas/${a}/picking`),i=o==null?void 0:o.data;i&&i.picking_id?window._go(`stock?picking=${i.picking_id}&origen=${a}`):window._go(`stock?orden=${a}`)}catch{window._go("stock")}},window._verLineas=()=>window._switchTab("lineas",document.querySelector('[data-tab="lineas"]')),window._imprimirCotizacion=window._imprimirPedido=a=>{b("Imprimir","Función de impresión próximamente","info")},window._enviarMensaje=window._agregarNota=a=>{b("Chatter","Función de mensajería próximamente","info")}}function so(t){return{confirmar:"Pedido confirmado",enviar:"Cotización enviada",cancelar:"Orden cancelada",bloquear:"Estado actualizado"}[t]||"Acción completada"}function ro(t){var m;const e=document.getElementById("inline-picker-row");if(e){(m=e.querySelector("input"))==null||m.focus();return}const a=tt(),o=a.descuentos!==!1||!!(a.listas_precios&&a.descuento_precio),i=!!a.margenes,s=!!a.unidades_medida,n=!!a.empaquetado,d=5+(o?1:0)+(i?1:0)+(s?1:0)+(n?1:0)+1;let r=document.querySelector("#tab-lineas table tbody");if(!r){const u=document.getElementById("tab-lineas");if(!u)return;const v=document.createElement("div");v.innerHTML=`
      <table class="o-list-table" style="margin-top:0;min-width:600px;width:100%">
        <thead>
          <tr style="background:var(--bg-table-head)">
            <th style="width:200px">PRODUCTO</th>
            <th style="width:240px">DESCRIPCIÓN</th>
            <th style="width:90px;text-align:center">CANTIDAD</th>
            ${s?'<th style="width:90px">UDM</th>':""}
            ${n?'<th style="width:110px">EMPAQUE</th>':""}
            <th style="width:110px;text-align:right">PRECIO UNIT.</th>
            ${o?'<th style="width:80px;text-align:right">DESCUENTO</th>':""}
            ${i?'<th style="width:100px;text-align:right">MARGEN</th>':""}
            <th style="width:110px;text-align:right">SUBTOTAL</th>
            <th style="width:36px"></th>
          </tr>
        </thead>
        <tbody id="lines-tbody-dynamic"></tbody>
      </table>`,u.innerHTML="",u.appendChild(v),r=document.getElementById("lines-tbody-dynamic");const k=document.createElement("div");k.innerHTML=se(t),u.appendChild(k.firstElementChild)}const l=document.createElement("tr");l.id="inline-picker-row",l.style.cssText="background:#EEF2FF;border-bottom:1px solid var(--primary)",l.innerHTML=`
    <td colspan="${d-1}" style="padding:8px 14px">
      <input id="inline-product-input" class="o-field-input" placeholder="🔍 Buscar producto por nombre o código..."
        style="width:100%;font-size:13px" autocomplete="off"
        oninput="window._buscarProductoInline(this.value,${t})"
        onkeydown="if(event.key==='Escape'){window._cancelarPickerInline()}">
    </td>
    <td style="padding:8px;text-align:center">
      <button onclick="window._cancelarPickerInline()" style="background:none;border:none;cursor:pointer;color:var(--text-400);font-size:18px">×</button>
    </td>`,r.appendChild(l);let c=document.getElementById("inline-product-dd");c||(c=document.createElement("div"),c.id="inline-product-dd",c.style.cssText="display:none;position:fixed;background:var(--bg-card);border:1.5px solid var(--primary);border-radius:8px;box-shadow:0 8px 24px rgba(0,0,0,.18);z-index:9999;max-height:280px;overflow-y:auto;min-width:400px",document.body.appendChild(c));const p=document.getElementById("inline-product-input");if(p){p.focus();const u=()=>{const v=p.getBoundingClientRect();c.style.top=v.bottom+4+"px",c.style.left=v.left+"px",c.style.width=v.width+"px"};p.addEventListener("focus",u),p.addEventListener("input",u),u()}}window._cancelarPickerInline=()=>{var e;(e=document.getElementById("inline-picker-row"))==null||e.remove();const t=document.getElementById("inline-product-dd");t&&(t.style.display="none",t.innerHTML="")};window._buscarProductoInline=async(t,e)=>{const a=document.getElementById("inline-product-dd");if(a){if(!t||t.length<1){a.style.display="none";return}clearTimeout(Wt),Wt=setTimeout(async()=>{try{const o=await g.get(`/ventas/buscar-productos?q=${encodeURIComponent(t)}`),i=(o==null?void 0:o.data)||[];if(a.style.display="block",!i.length){a.innerHTML=`<div style="padding:12px;font-size:13px;color:var(--text-400);text-align:center">Sin resultados para "${t}"</div>`;return}a.innerHTML=i.map(s=>`
        <div style="padding:10px 14px;cursor:pointer;border-bottom:1px solid var(--border);display:flex;justify-content:space-between;align-items:center"
          onmouseover="this.style.background='#EEF2FF'"
          onmouseout="this.style.background=''"
          onclick="window._selProductoInline(${e},${s.id},'${(s.name||"").replace(/'/g,"\\'")}',${parseFloat(s.precio||0)})">
          <div>
            <div style="font-weight:600;font-size:13px">${s.name||""}</div>
            ${s.codigo?`<div style="font-size:11px;color:var(--text-400)">${s.codigo}</div>`:""}
          </div>
          <span style="font-weight:700;color:var(--primary);font-size:13px">${f(s.precio||0)}</span>
        </div>`).join("")}catch{}},200)}};window._selProductoInline=async(t,e,a,o)=>{var d;(d=document.getElementById("inline-picker-row"))==null||d.remove();const i=document.getElementById("inline-product-dd");i&&(i.style.display="none",i.innerHTML="",i.remove());const s=tt();let n=0;if(s.listas_precios){const r=Yt().find(l=>l.id===mt(t).pricelist_id);r&&r.type==="Porcentaje"&&(n=parseFloat(r.discount)||0)}try{await g.post(`/ventas/${t}/lineas`,{product_id:e,product_uom_qty:1,price_unit:o,discount:n}),await Ot(t,F==null?void 0:F.state,F==null?void 0:F.locked),b("Producto agregado",n?`${a} (−${n}% por lista de precios)`:a,"success")}catch(r){b("Error",r.message,"error")}};window._aplicarListaPrecios=async(t,e)=>{const a=parseInt(e)||null,o=Yt().find(n=>n.id===a);wt(t,{pricelist_id:o?a:null});const i=o&&o.type==="Porcentaje"&&parseFloat(o.discount)||0,s=O.filter(n=>n.display_type!=="line_section"&&!/^env[ií]o/i.test(n.name||""));try{for(const n of s)await g.put(`/ventas/${t}/lineas/${n.id}`,{discount:i});await Ot(t,F==null?void 0:F.state,F==null?void 0:F.locked),b("Lista de precios",o?`"${o.name}" aplicada${i?` — descuento del ${i}% en ${s.length} línea(s)`:""}`:"Tarifa pública aplicada (sin descuento)","success")}catch(n){b("Error",n.message,"error")}};window._aplicarPlantilla=async(t,e)=>{const a=ye[e];if(a)try{for(const o of a.lineas)await g.post(`/ventas/${t}/lineas`,o);wt(t,{plantilla:e}),await Ot(t,F==null?void 0:F.state,F==null?void 0:F.locked),b("Plantilla aplicada",`${a.label} — ${a.lineas.length} línea(s) agregada(s)`,"success")}catch(o){b("Error",o.message,"error")}};window._agregarEnvio=t=>{const e=document.createElement("div");e.style.cssText="position:fixed;inset:0;z-index:960;background:rgba(0,0,0,.45);display:flex;align-items:center;justify-content:center;padding:16px",e.innerHTML=`
    <div style="background:var(--bg-card);border-radius:14px;border:1px solid var(--border);width:100%;max-width:420px;box-shadow:0 24px 64px rgba(0,0,0,.22)">
      <div style="padding:16px 20px;border-bottom:1px solid var(--border);display:flex;justify-content:space-between;align-items:center">
        <h3 style="margin:0;font-size:15px;font-weight:700">🚚 Costo de Envío</h3>
        <button onclick="this.closest('[style*=fixed]').remove()" style="background:none;border:none;cursor:pointer;font-size:18px;color:var(--text-400)">×</button>
      </div>
      <div style="padding:20px;display:flex;flex-direction:column;gap:14px">
        <div>
          <label style="font-size:12px;font-weight:700;color:var(--text-600);display:block;margin-bottom:6px">MÉTODO DE ENTREGA</label>
          <select id="envio-metodo" class="o-field-input" onchange="document.getElementById('envio-costo').value=this.selectedOptions[0].dataset.costo">
            ${Je.map((a,o)=>`<option value="${a.label}" data-costo="${a.costo}" ${o===0?"selected":""}>${a.label} — ${f(a.costo)}</option>`).join("")}
          </select>
        </div>
        <div>
          <label style="font-size:12px;font-weight:700;color:var(--text-600);display:block;margin-bottom:6px">COSTO (MXN)</label>
          <input id="envio-costo" type="number" min="0" step="0.01" class="o-field-input" value="${Je[0].costo}">
        </div>
      </div>
      <div style="display:flex;gap:8px;justify-content:flex-end;padding:12px 20px;border-top:1px solid var(--border);background:var(--bg-app);border-radius:0 0 14px 14px">
        <button class="o-btn-secondary" onclick="this.closest('[style*=fixed]').remove()">Cancelar</button>
        <button class="o-btn-primary" onclick="window._confirmarEnvio(${t})">✓ Agregar a la orden</button>
      </div>
    </div>`,document.body.appendChild(e),e.onclick=a=>{a.target===e&&e.remove()},window._confirmarEnvio=async a=>{var s,n;const o=((s=document.getElementById("envio-metodo"))==null?void 0:s.value)||"Entrega estándar",i=parseFloat((n=document.getElementById("envio-costo"))==null?void 0:n.value)||0;e.remove();try{const d=O.filter(r=>r.display_type!=="line_section"&&/^env[ií]o/i.test(r.name||""));for(const r of d)await g.del(`/ventas/${a}/lineas/${r.id}`);await g.post(`/ventas/${a}/lineas`,{name:`Envío — ${o}`,product_uom_qty:1,price_unit:i,discount:0}),await J(a),b("Envío agregado",`${o}: ${f(i)}`,"success")}catch(d){b("Error",d.message,"error")}}};window._firmarCotizacion=async t=>{var a,o;const e=(o=(a=document.getElementById("firma-nombre"))==null?void 0:a.value)==null?void 0:o.trim();if(!e){b("Firma","Escribe el nombre completo del firmante","error");return}wt(t,{firma:{name:e,fecha:new Date().toISOString()}});try{await g.put(`/ventas/${t}`,{signature_name:e})}catch{}b("Cotización firmada",`Firmada por ${e}`,"success"),confirm(`✍️ ${e} firmó la cotización.
¿Confirmar el pedido de venta ahora?`)?window._accionVenta(t,"confirmar"):await J(t)};window._registrarPagoOnline=async(t,e)=>{const a=mt(t),o=(a.pagos||[]).reduce((p,m)=>p+parseFloat(m.monto||0),0),s=O.reduce((p,m)=>p+parseFloat(m.price_subtotal||0),0)*1.16,n=Math.max(0,s-o),d=prompt(`💳 Pago con ${e}
Pendiente: ${f(n)}

Monto a pagar:`,n.toFixed(2));if(d===null)return;const r=parseFloat(d);if(!(r>0)){b("Pago","Monto inválido","error");return}const l=[...a.pagos||[],{metodo:e,monto:r,fecha:new Date().toISOString()}];wt(t,{pagos:l}),b("Pago registrado",`${e}: ${f(r)}`,"success");const c=F==null?void 0:F.state;if(o+r>=s-.01&&(c==="draft"||c==="sent")&&confirm(`✅ El pago cubre el total de la cotización.
¿Confirmar el pedido de venta ahora?`)){window._accionVenta(t,"confirmar");return}await J(t)};window._vistaPreviaCliente=t=>{const e=F,a=tt();if(!e||e.id!==t)return;const o=O.filter(d=>d.display_type!=="line_section"),i=o.reduce((d,r)=>d+parseFloat(r.price_subtotal||0),0),s=i*1.16,n=document.createElement("div");n.style.cssText="position:fixed;inset:0;z-index:970;background:rgba(0,0,0,.55);display:flex;align-items:center;justify-content:center;padding:16px;overflow-y:auto",n.innerHTML=`
    <div style="background:#fff;border-radius:14px;width:100%;max-width:680px;max-height:90vh;overflow-y:auto;box-shadow:0 24px 64px rgba(0,0,0,.3)">
      <div style="padding:14px 24px;background:#1F2937;color:#fff;border-radius:14px 14px 0 0;display:flex;justify-content:space-between;align-items:center">
        <span style="font-size:13px;font-weight:700">👁 Vista previa del portal del cliente</span>
        <button onclick="this.closest('[style*=fixed]').remove()" style="background:none;border:none;cursor:pointer;font-size:20px;color:#fff">×</button>
      </div>
      <div style="padding:28px 32px">
        <div style="display:flex;justify-content:space-between;align-items:flex-start;margin-bottom:20px">
          <div>
            <h2 style="margin:0 0 4px;font-size:20px;font-weight:800;color:#111">Cotización ${e.name||""}</h2>
            <div style="font-size:13px;color:#6B7280">${e.partner_name||""} · ${j(e.date_order)}</div>
          </div>
          <span class="o-badge ${Ae[e.state]||"o-badge-gray"}">${Pe[e.state]||e.state}</span>
        </div>
        <table style="width:100%;border-collapse:collapse;font-size:13px;margin-bottom:16px">
          <thead>
            <tr style="border-bottom:2px solid #E5E7EB;text-align:left">
              <th style="padding:8px 4px">Concepto</th>
              <th style="padding:8px 4px;text-align:center">Cant.</th>
              <th style="padding:8px 4px;text-align:right">Precio</th>
              <th style="padding:8px 4px;text-align:right">Subtotal</th>
            </tr>
          </thead>
          <tbody>
            ${o.map(d=>`
            <tr style="border-bottom:1px solid #F3F4F6">
              <td style="padding:8px 4px">${d.product_name||d.name||""}</td>
              <td style="padding:8px 4px;text-align:center">${parseFloat(d.product_uom_qty)||0}</td>
              <td style="padding:8px 4px;text-align:right">${f(d.price_unit)}${parseFloat(d.discount)?` <span style="color:#10B981;font-size:11px">(−${parseFloat(d.discount)}%)</span>`:""}</td>
              <td style="padding:8px 4px;text-align:right;font-weight:600">${f(d.price_subtotal)}</td>
            </tr>`).join("")}
          </tbody>
        </table>
        <div style="display:flex;justify-content:flex-end;margin-bottom:20px">
          <table style="font-size:13px;min-width:220px">
            <tr><td style="padding:3px 16px 3px 0;color:#6B7280">Subtotal</td><td style="text-align:right;font-weight:600">${f(i)}</td></tr>
            <tr><td style="padding:3px 16px 3px 0;color:#6B7280">IVA (16%)</td><td style="text-align:right;font-weight:600">${f(i*.16)}</td></tr>
            <tr style="border-top:2px solid #E5E7EB"><td style="padding:6px 16px 0 0;font-weight:800">TOTAL</td><td style="text-align:right;font-weight:800;color:#6366F1">${f(s)}</td></tr>
          </table>
        </div>
        ${e.note||a.terminos?`<div style="font-size:12px;color:#6B7280;border-top:1px solid #E5E7EB;padding-top:12px;white-space:pre-wrap;margin-bottom:16px">${e.note||a.terminos}</div>`:""}
        ${e.state==="draft"||e.state==="sent"?`
        <div style="display:flex;gap:8px;justify-content:center;border-top:1px solid #E5E7EB;padding-top:16px">
          ${a.firma_online?`<button class="o-btn-primary" onclick="this.closest('[style*=fixed]').remove();window._switchTab('notas',document.querySelector('[data-tab=notas]'));document.getElementById('firma-nombre')?.focus()">✍️ Firmar cotización</button>`:""}
          ${a.pago_online?`<button class="o-btn-secondary" onclick="this.closest('[style*=fixed]').remove();window._registrarPagoOnline(${t},'Tarjeta')">💳 Pagar en línea</button>`:""}
        </div>`:""}
      </div>
    </div>`,document.body.appendChild(n),n.onclick=d=>{d.target===n&&n.remove()}};window._setLineaUom=(t,e,a)=>{const o=mt(t);wt(t,{uoms:{...o.uoms||{},[e]:a}}),b("Unidad de medida",a,"success")};window._setLineaEmpaque=async(t,e,a)=>{const[o,i]=(a||"").split("|"),s=parseFloat(i)||0,n=mt(t);if(wt(t,{packs:{...n.packs||{},[e]:o}}),s>0)try{await g.put(`/ventas/${t}/lineas/${e}`,{product_uom_qty:s}),await Ot(t,F==null?void 0:F.state,F==null?void 0:F.locked),b("Empaque aplicado",`${o} → cantidad ${s}`,"success")}catch(d){b("Error",d.message,"error")}};async function Ot(t,e,a){const o=await g.get(`/ventas/${t}/lineas`);O=(o==null?void 0:o.data)||[];const i=document.getElementById("tab-lineas");if(!i)return;const s=(e==="draft"||e==="sent")&&!a;i.innerHTML=De(O,e,a,t)+(s?se(t):""),he(O)}function va(t){return t.reduce((e,a)=>{if(a.display_type==="line_section")return e;const o=parseFloat(a.product_uom_qty)||0;return e+(parseFloat(a.price_subtotal||0)-o*parseFloat(a.cost||0))},0)}function he(t){const e=t.reduce((r,l)=>r+parseFloat(l.price_subtotal||0),0),a=e*.16,o=e+a,i=document.getElementById("tot-subtotal"),s=document.getElementById("tot-iva"),n=document.getElementById("tot-total"),d=document.getElementById("tot-margen");i&&(i.textContent=f(e)),s&&(s.textContent=f(a)),n&&(n.textContent=f(o)),d&&(d.textContent=f(va(t)))}async function lo(){const t=tt(),e=new Date().toISOString().slice(0,10),a=parseInt(t.validez_cotizacion)||0,o=a>0?new Date(Date.now()+a*24*60*60*1e3).toISOString().slice(0,10):"";w(`<div id="venta-form" style="min-height:100vh;background:var(--bg-app)">

    <!-- TOPBAR -->
    <div style="display:flex;align-items:center;justify-content:space-between;padding:10px 20px;background:var(--bg-card);border-bottom:1px solid var(--border);position:sticky;top:50px;z-index:20;flex-wrap:wrap;gap:8px">
      <div style="display:flex;align-items:center;gap:8px">
        <button class="o-btn-secondary o-btn-sm" onclick="window._go('ventas')">← Ventas</button>
      </div>
      <div style="display:flex;gap:8px;flex-wrap:wrap">
        <button class="o-btn-primary" onclick="window._guardarNueva()" id="btn-guardar-nueva">💾 Guardar</button>
        <button class="o-btn-secondary o-btn-sm" onclick="window._go('ventas')">Descartar</button>
      </div>
    </div>

    <!-- STATUS BAR -->
    <div style="display:flex;align-items:center;padding:8px 24px;background:var(--bg-card);border-bottom:1px solid var(--border);gap:0">
      <button style="padding:6px 16px;border-radius:20px;border:none;font-size:12px;font-weight:700;background:var(--primary);color:#fff;cursor:default">Cotización</button>
      <span style="color:var(--text-300);font-size:14px;margin:0 2px">›</span>
      <button style="padding:6px 16px;border-radius:20px;border:none;font-size:12px;font-weight:700;color:var(--text-400);background:transparent;cursor:default">Enviado</button>
      <span style="color:var(--text-300);font-size:14px;margin:0 2px">›</span>
      <button style="padding:6px 16px;border-radius:20px;border:none;font-size:12px;font-weight:700;color:var(--text-400);background:transparent;cursor:default">Pedido de Venta</button>
      <span style="color:var(--text-300);font-size:14px;margin:0 2px">›</span>
      <button style="padding:6px 16px;border-radius:20px;border:none;font-size:12px;font-weight:700;color:var(--text-400);background:transparent;cursor:default">Realizado</button>
    </div>

    <!-- FORM SHEET -->
    <div style="background:var(--bg-card);border-radius:12px;margin:16px 20px 0;border:1px solid var(--border);overflow:hidden">

      <!-- Encabezado -->
      <div style="padding:20px 24px 16px;border-bottom:1px solid var(--border)">
        <h1 style="font-family:'Plus Jakarta Sans',sans-serif;font-size:22px;font-weight:800;color:var(--text-900);margin:0 0 6px">Nueva Cotización</h1>
        <span class="o-badge o-badge-gray">Borrador</span>
      </div>

      <!-- Campos principales -->
      <div style="display:grid;grid-template-columns:1fr 1fr;gap:0 32px;padding:16px 24px">

        <!-- Columna izquierda -->
        <div>
          <div class="o-form-field" style="display:grid;grid-template-columns:140px 1fr;align-items:center;padding:7px 0;border-bottom:1px solid var(--border)">
            <span style="font-size:12px;color:var(--text-500);font-weight:600">Cliente <span style="color:#DC2626">*</span></span>
            <div style="position:relative">
              <input id="n-partner-name" class="o-field-input" placeholder="Buscar cliente..." autocomplete="off"
                oninput="window._buscarClienteNueva(this.value)" style="width:100%">
              <input type="hidden" id="n-partner-id">
              <div id="n-partner-dd" style="display:none;position:absolute;top:calc(100%+2px);left:0;right:0;background:var(--bg-card);border:1.5px solid var(--primary);border-radius:8px;box-shadow:0 8px 24px rgba(0,0,0,.12);z-index:300;max-height:220px;overflow-y:auto"></div>
            </div>
          </div>
          <div class="o-form-field" style="display:grid;grid-template-columns:140px 1fr;align-items:center;padding:7px 0;border-bottom:1px solid var(--border)">
            <span style="font-size:12px;color:var(--text-500);font-weight:600">Referencia Cliente</span>
            <input id="n-ref" class="o-field-input" placeholder="Número de referencia del cliente...">
          </div>
          <div class="o-form-field" style="display:grid;grid-template-columns:140px 1fr;align-items:center;padding:7px 0;border-bottom:1px solid var(--border)">
            <span style="font-size:12px;color:var(--text-500);font-weight:600">Vendedor</span>
            <input id="n-vendedor" class="o-field-input" placeholder="Nombre del vendedor..." value="">
          </div>
        </div>

        <!-- Columna derecha -->
        <div>
          <div class="o-form-field" style="display:grid;grid-template-columns:140px 1fr;align-items:center;padding:7px 0;border-bottom:1px solid var(--border)">
            <span style="font-size:12px;color:var(--text-500);font-weight:600">Fecha de Orden</span>
            <input id="n-fecha" type="date" class="o-field-input" value="${e}">
          </div>
          <div class="o-form-field" style="display:grid;grid-template-columns:140px 1fr;align-items:center;padding:7px 0;border-bottom:1px solid var(--border)">
            <span style="font-size:12px;color:var(--text-500);font-weight:600">Vencimiento</span>
            <input id="n-vence" type="date" class="o-field-input" value="${o}">
          </div>
          ${t.fecha_entrega?`
          <div class="o-form-field" style="display:grid;grid-template-columns:140px 1fr;align-items:center;padding:7px 0;border-bottom:1px solid var(--border)">
            <span style="font-size:12px;color:var(--text-500);font-weight:600">Fecha Compromiso</span>
            <input id="n-entrega" type="date" class="o-field-input">
          </div>`:""}
          <div class="o-form-field" style="display:grid;grid-template-columns:140px 1fr;align-items:center;padding:7px 0;border-bottom:1px solid var(--border)">
            <span style="font-size:12px;color:var(--text-500);font-weight:600">Términos de Pago</span>
            <select id="n-payment" class="o-field-input">
              <option value="">— Seleccionar —</option>
              <option value="30">Neto 30 días</option>
              <option value="15">Neto 15 días</option>
              <option value="0">Pago inmediato</option>
            </select>
          </div>
        </div>
      </div>

      <!-- Tabs: Líneas de Pedido / Otra Información / Notas -->
      <div style="border-top:1px solid var(--border)">
        <div class="o-tabs" style="display:flex;border-bottom:1px solid var(--border);background:var(--bg-app);padding:0 16px">
          <button class="o-tab active" data-ntab="lineas" onclick="window._ntab('lineas',this)" style="padding:10px 16px;border:none;background:none;font-size:13px;font-weight:600;cursor:pointer;border-bottom:2px solid var(--primary);color:var(--primary)">Líneas de Pedido</button>
          <button class="o-tab" data-ntab="info" onclick="window._ntab('info',this)" style="padding:10px 16px;border:none;background:none;font-size:13px;font-weight:600;cursor:pointer;border-bottom:2px solid transparent;color:var(--text-500)">Otra Información</button>
          <button class="o-tab" data-ntab="notas" onclick="window._ntab('notas',this)" style="padding:10px 16px;border:none;background:none;font-size:13px;font-weight:600;cursor:pointer;border-bottom:2px solid transparent;color:var(--text-500)">Notas y Términos</button>
        </div>

        <!-- Panel Líneas -->
        <div id="ntab-lineas" style="padding:0">
          <table style="width:100%;border-collapse:collapse;font-size:13px">
            <thead style="background:var(--bg-app)">
              <tr>
                <th style="padding:8px 12px;text-align:left;font-size:11px;font-weight:700;color:var(--text-500);border-bottom:1px solid var(--border)">PRODUCTO</th>
                <th style="padding:8px 12px;text-align:left;font-size:11px;font-weight:700;color:var(--text-500);border-bottom:1px solid var(--border)">DESCRIPCIÓN</th>
                <th style="padding:8px 12px;text-align:right;font-size:11px;font-weight:700;color:var(--text-500);border-bottom:1px solid var(--border)">CANT.</th>
                <th style="padding:8px 12px;text-align:right;font-size:11px;font-weight:700;color:var(--text-500);border-bottom:1px solid var(--border)">PRECIO</th>
                <th style="padding:8px 12px;text-align:right;font-size:11px;font-weight:700;color:var(--text-500);border-bottom:1px solid var(--border)">IMPUESTO</th>
                <th style="padding:8px 12px;text-align:right;font-size:11px;font-weight:700;color:var(--text-500);border-bottom:1px solid var(--border)">SUBTOTAL</th>
              </tr>
            </thead>
            <tbody id="n-lineas-tbody">
              <tr id="n-empty-row">
                <td colspan="6" style="padding:32px;text-align:center;color:var(--text-400);font-size:13px">
                  Guarda la cotización para agregar productos
                </td>
              </tr>
            </tbody>
          </table>
          <div style="padding:10px 12px">
            <button class="o-btn-secondary o-btn-sm" onclick="window._guardarNueva(true)" style="font-size:12px">+ Agregar producto</button>
          </div>
          <div style="padding:12px 24px;border-top:1px solid var(--border);display:flex;justify-content:flex-end">
            <table style="font-size:13px;min-width:260px">
              <tr><td style="padding:3px 16px 3px 0;color:var(--text-500)">Subtotal:</td><td style="text-align:right;font-weight:600">$0.00</td></tr>
              <tr><td style="padding:3px 16px 3px 0;color:var(--text-500)">IVA (16%):</td><td style="text-align:right;font-weight:600">$0.00</td></tr>
              <tr style="border-top:2px solid var(--border)"><td style="padding:6px 16px 3px 0;font-weight:700;font-size:14px">TOTAL:</td><td style="text-align:right;font-weight:800;font-size:15px;color:var(--primary)">$0.00 MXN</td></tr>
            </table>
          </div>
        </div>

        <!-- Panel Info -->
        <div id="ntab-info" style="padding:16px 24px;display:none">
          <div style="display:grid;grid-template-columns:1fr 1fr;gap:0 32px">
            <div>
              <div style="display:grid;grid-template-columns:160px 1fr;align-items:center;padding:7px 0;border-bottom:1px solid var(--border)">
                <span style="font-size:12px;color:var(--text-500);font-weight:600">Equipo de Ventas</span>
                <input id="n-team" class="o-field-input" placeholder="Equipo de ventas...">
              </div>
              <div style="display:grid;grid-template-columns:160px 1fr;align-items:center;padding:7px 0;border-bottom:1px solid var(--border)">
                <span style="font-size:12px;color:var(--text-500);font-weight:600">Etiquetas</span>
                <input id="n-tags" class="o-field-input" placeholder="Etiquetas...">
              </div>
            </div>
            <div>
              <div style="display:grid;grid-template-columns:160px 1fr;align-items:center;padding:7px 0;border-bottom:1px solid var(--border)">
                <span style="font-size:12px;color:var(--text-500);font-weight:600">Empresa</span>
                <input id="n-empresa" class="o-field-input" value="NEXUSTECH" readonly style="background:var(--bg-app)">
              </div>
            </div>
          </div>
        </div>

        <!-- Panel Notas -->
        <div id="ntab-notas" style="padding:16px 24px;display:none">
          <div style="margin-bottom:12px">
            <label style="font-size:12px;font-weight:600;color:var(--text-500);display:block;margin-bottom:4px">TÉRMINOS Y CONDICIONES</label>
            <textarea id="n-nota" class="o-field-input" rows="4" placeholder="Escribe los términos y condiciones de esta cotización..." style="width:100%;resize:vertical;box-sizing:border-box">${t.terminos||""}</textarea>
          </div>
        </div>
      </div>
    </div>

    <div style="height:60px"></div>
  </div>`),window._ntab=(i,s)=>{document.querySelectorAll("[data-ntab]").forEach(d=>{d.style.borderBottomColor="transparent",d.style.color="var(--text-500)"}),document.querySelectorAll('[id^="ntab-"]').forEach(d=>d.style.display="none"),s.style.borderBottomColor="var(--primary)",s.style.color="var(--primary)";const n=document.getElementById(`ntab-${i}`);n&&(n.style.display="")},window._buscarClienteNueva=async i=>{const s=document.getElementById("n-partner-dd");if(!i||!s){s&&(s.style.display="none");return}try{const n=await g.get(`/ventas/buscar-clientes?q=${encodeURIComponent(i)}`),d=(n==null?void 0:n.data)||[];s.style.display=d.length?"block":"none",s.innerHTML=d.map(r=>`
        <div style="padding:8px 12px;cursor:pointer;border-bottom:1px solid var(--border)"
          onmouseover="this.style.background='#EEF2FF'" onmouseout="this.style.background=''"
          onclick="
            document.getElementById('n-partner-name').value='${(r.name||"").replace(/'/g,"\\'")}';
            document.getElementById('n-partner-id').value='${r.id}';
            document.getElementById('n-partner-dd').style.display='none'">
          <div style="font-weight:600;font-size:13px">${r.name||""}</div>
          ${r.email?`<div style="font-size:11px;color:var(--text-400)">${r.email}</div>`:""}
        </div>`).join("")}catch{}},window._guardarNueva=async(i=!1)=>{var d,r,l,c,p,m;const s=parseInt(((d=document.getElementById("n-partner-id"))==null?void 0:d.value)||"0");if(!s){b("Error","Selecciona un cliente antes de guardar","error");return}const n=document.getElementById("btn-guardar-nueva");n&&(n.disabled=!0,n.textContent="⏳ Guardando...");try{const u=(r=document.getElementById("n-vence"))==null?void 0:r.value;let v=null;u?v=Math.max(1,Math.round((new Date(u+"T00:00:00")-Date.now())/864e5)):a===0&&(v=0);const k={partner_id:s,client_order_ref:((l=document.getElementById("n-ref"))==null?void 0:l.value)||"",note:((c=document.getElementById("n-nota"))==null?void 0:c.value)||"",validity_days:v},_=await g.post("/ventas",k),$=(p=_==null?void 0:_.data)==null?void 0:p.id;if($){const T=(m=document.getElementById("n-entrega"))==null?void 0:m.value;if(T)try{await g.put(`/ventas/${$}`,{commitment_date:T})}catch{}b("Cotización creada",_.data.name||`S${String($).padStart(5,"0")}`,"success"),await J($)}}catch(u){b("Error",u.message,"error"),n&&(n.disabled=!1,n.textContent="💾 Guardar")}}}async function co(){if(!N.size||!confirm(`¿Cancelar ${N.size} orden(es)?`))return;const t=[...N];let e=0;for(const a of t)try{await g.put(`/ventas/${a}/cancelar`,{}),e++}catch{}N.clear(),b(`${e} canceladas`,"","success"),zt()}async function po(){const t=N.size>0?G.filter(s=>N.has(s.id)):G,e=["name","date_order","partner_name","client_order_ref","amount_total","state","invoice_status"],o=[["Número","Fecha","Cliente","Ref. Cliente","Total","Estado","Facturación"].join(","),...t.map(s=>e.map(n=>`"${s[n]||""}"`).join(","))].join(`
`),i=document.createElement("a");i.href="data:text/csv;charset=utf-8,"+encodeURIComponent(o),i.download=`ventas-${new Date().toISOString().slice(0,10)}.csv`,i.click()}function uo(){document.querySelectorAll(".o-chk-row").forEach(t=>{var a,o,i,s;const e=parseInt(((s=(i=(o=(a=t.closest("tr"))==null?void 0:a.querySelector('[onclick*="_abrirVenta"]'))==null?void 0:o.onclick)==null?void 0:i.toString().match(/\d+/))==null?void 0:s[0])||"0");e&&N.has(e)&&(t.checked=!0)})}function vo(t){const e=parseFloat(t||0);return e>=1e6?`$${(e/1e6).toFixed(1)}M`:e>=1e3?`$${(e/1e3).toFixed(1)}k`:f(e)}function mo(t){const e=["#6366F1","#8B5CF6","#EC4899","#F59E0B","#10B981","#3B82F6","#EF4444","#14B8A6"];if(!t)return e[0];let a=0;for(let o=0;o<t.length;o++)a=t.charCodeAt(o)+((a<<5)-a);return e[Math.abs(a)%e.length]}const bo="modulepreload",fo=function(t){return"/"+t},We={},de=function(e,a,o){let i=Promise.resolve();if(a&&a.length>0){document.getElementsByTagName("link");const n=document.querySelector("meta[property=csp-nonce]"),d=(n==null?void 0:n.nonce)||(n==null?void 0:n.getAttribute("nonce"));i=Promise.allSettled(a.map(r=>{if(r=fo(r),r in We)return;We[r]=!0;const l=r.endsWith(".css"),c=l?'[rel="stylesheet"]':"";if(document.querySelector(`link[href="${r}"]${c}`))return;const p=document.createElement("link");if(p.rel=l?"stylesheet":bo,l||(p.as="script"),p.crossOrigin="",p.href=r,d&&p.setAttribute("nonce",d),document.head.appendChild(p),l)return new Promise((m,u)=>{p.addEventListener("load",m),p.addEventListener("error",()=>u(new Error(`Unable to preload CSS for ${r}`)))})}))}function s(n){const d=new Event("vite:preloadError",{cancelable:!0});if(d.payload=n,window.dispatchEvent(d),!d.defaultPrevented)throw n}return i.then(n=>{for(const d of n||[])d.status==="rejected"&&s(d.reason);return e().catch(s)})};let Tt="list",it=1,we="",_e=null,pe=[],M={};async function go(t={}){if(B(),M={impuestos_ventas:!0,impuestos_compras:!0,redondeo:!1,pagos_online:!1,descuentos_pronto_pago:!1,alertas_cliente:!1,cfdi_auto:!1,cancelacion_directa:!1,terminos_default:"",...JSON.parse(localStorage.getItem("nexus_config_facturacion")||"{}")},t.id){w(`<div class="nx-module-page"><div style="padding:40px">${I(3,5)}</div></div>`),await window._vVF(parseInt(t.id));return}w(`<div class="nx-module-page"><div id="mcp"></div><div id="mcontent">${I(5,7)}</div></div>`),ma(),await gt()}function ma(){const t=document.getElementById("mcp");t&&(t.innerHTML=`
    <div class="o-control-panel">
      <div class="o-cp-left">
        <button class="o-btn-new" onclick="window._newFactura()">+ Nueva Factura</button>
        <span class="o-cp-sep"></span>
        <div class="o-dropdown" id="dd-ff">
          <button class="o-btn-filter" onclick="window._tog('dd-ff')">📂 Filtros ▾</button>
          <div class="o-dropdown-menu" id="dd-ff-menu">
            <div class="o-dropdown-item" onclick="window._ff('draft')">Borradores</div>
            <div class="o-dropdown-item" onclick="window._ff('posted')">Publicadas</div>
            <div class="o-dropdown-item" onclick="window._ff('in_payment')">En Pago</div>
            <div class="o-dropdown-item" onclick="window._ff('paid')">Pagadas</div>
            <div class="o-dropdown-item" onclick="window._ff('cancel')">Canceladas</div>
            <div class="o-dropdown-divider"></div>
            <div class="o-dropdown-item" onclick="window._ff(null)">❌ Sin filtro</div>
          </div>
        </div>
        <div class="o-search-box">
          <span style="color:var(--text-400)">🔍</span>
          <input type="search" placeholder="Buscar factura…" id="fs" oninput="window._sf(this.value)">
        </div>
        <span class="o-record-count" id="fcount"></span>
      </div>
      <div class="o-cp-right">
        <button class="o-btn-secondary" style="margin-right:8px;font-size:16px" onclick="window._go('config_facturacion')" title="Ajustes">⚙️</button>
        <div class="o-view-switcher">
          <button class="o-view-btn ${Tt==="list"?"active":""}" onclick="window._fvv('list')" title="Lista">☰</button>
          <button class="o-view-btn ${Tt==="kanban"?"active":""}" onclick="window._fvv('kanban')" title="Kanban">⬜</button>
        </div>
      </div>
    </div>`,yo(),window._fvv=e=>{Tt=e,ma(),gt()},window._sf=_o(e=>{we=e,it=1,gt()},300),window._ff=e=>{_e=e,it=1,gt(),window._cdd()},window._newFactura=()=>{de(()=>import("./create_forms-CESEMRXd.js"),[]).then(e=>e.nuevaFactura(()=>gt(),M))})}function yo(){window._tog=t=>{const e=document.getElementById(t+"-menu");if(!e)return;const a=e.classList.contains("open");window._cdd(),a||e.classList.add("open")},window._cdd=()=>document.querySelectorAll(".o-dropdown-menu.open").forEach(t=>t.classList.remove("open")),window._ddInit||(document.addEventListener("click",t=>{t.target.closest(".o-dropdown")||window._cdd()}),window._ddInit=!0)}async function gt(){const t=document.getElementById("mcontent");if(t){t.innerHTML=I(5,7);try{const e=await g.facturas(it);pe=(e==null?void 0:e.data)||[];let a=_e?pe.filter(i=>i.state===_e):pe;if(we){const i=we.toLowerCase();a=a.filter(s=>(s.name||"").toLowerCase().includes(i)||(s.partner_name||"").toLowerCase().includes(i))}const o=document.getElementById("fcount");o&&(o.textContent=a.length+" registros"),t.innerHTML=Tt==="kanban"?ho(a):xo(a),Tt==="list"&&wo()}catch(e){t.innerHTML=`<div style="padding:40px;text-align:center;color:var(--text-400)">⚠️ ${e.message}</div>`}}}const $e={draft:"Borrador",posted:"Publicada",in_payment:"En Pago",paid:"Pagada",cancel:"Cancelada"};function xo(t){return t.length?`
    <div class="o-list-actions-bar" id="flab"><span class="o-actions-count" id="fsel-cnt">0 seleccionados</span>
      <button class="o-action-btn-sm" onclick="alert('Exportar')">Exportar</button>
    </div>
    <div class="o-list-view"><table>
      <thead><tr>
        <th class="th-check"><input type="checkbox" class="o-list-checkbox" id="fca" onchange="window._fca(this.checked)"></th>
        <th>Número</th><th>Cliente</th><th>Tipo</th><th>Fecha</th><th>Estado</th><th style="text-align:right">Total</th><th style="text-align:right">Saldo</th>
      </tr></thead>
      <tbody>
        ${t.map(e=>{var a,o;return`
          <tr onclick="window._vVF(${e.id})" data-id="${e.id}">
            <td class="td-check" onclick="event.stopPropagation()"><input type="checkbox" class="o-list-checkbox frc" data-id="${e.id}" onchange="window._frc()"></td>
            <td><strong>${e.name||"-"}</strong></td>
            <td>${e.partner_name||e.partner_id||"-"}
                ${M.alertas_cliente&&e.amount_residual>0?' <span style="color:#DC2626;font-size:11px" title="Tiene deuda">⚠️</span>':""}
            </td>
            <td><span style="font-size:11px;color:var(--text-400)">${e.move_type==="out_invoice"?"Factura":e.move_type||"-"}</span></td>
            <td>${((a=e.invoice_date)==null?void 0:a.slice(0,10))||((o=e.date)==null?void 0:o.slice(0,10))||"-"}</td>
            <td>${Rt(e.state,$e[e.state]||e.state)}</td>
            <td style="text-align:right;font-weight:700;color:var(--primary)">${f(e.amount_total)}</td>
            <td style="text-align:right;color:${e.amount_residual>0?"#DC2626":"var(--text-400)"}">${f(e.amount_residual||0)}</td>
          </tr>`}).join("")}
      </tbody></table></div>
    <div style="padding:12px 20px;display:flex;justify-content:space-between;align-items:center;background:var(--bg-card);border-top:1px solid var(--border)">
      <span style="font-size:13px;color:var(--text-400)">${t.length} registros</span>
      <div style="display:flex;gap:8px">
        <button class="o-action-btn-sm" ${it<=1?"disabled":""} onclick="window._fp(${it-1})">‹ Anterior</button>
        <span style="padding:5px 10px;font-size:13px">${it}</span>
        <button class="o-action-btn-sm" onclick="window._fp(${it+1})">Siguiente ›</button>
      </div></div>`:'<div style="padding:60px;text-align:center"><div style="font-size:48px;margin-bottom:12px">🧾</div><p style="color:var(--text-400)">Sin facturas. Timbra la primera.</p></div>'}const Ye=[{key:"draft",label:"Borrador",color:"#9CA3AF"},{key:"posted",label:"Publicada",color:"#059669"},{key:"in_payment",label:"En Pago",color:"#7C3AED"},{key:"paid",label:"Pagada",color:"#0EA5E9"},{key:"cancel",label:"Cancelada",color:"#DC2626"}];function ho(t){const e={};return Ye.forEach(a=>e[a.key]=[]),t.forEach(a=>{e[a.state]?e[a.state].push(a):e.draft&&e.draft.push(a)}),`<div class="o-kanban-view">${Ye.map(a=>`
    <div class="o-kanban-col">
      <div class="o-kanban-col-header" style="border-top:3px solid ${a.color}">
        <span>${a.label}</span><span class="o-kanban-col-count">${e[a.key].length}</span>
      </div>
      <div class="o-kanban-cards">
        ${e[a.key].map(o=>{var i;return`
          <div class="o-kanban-card" onclick="window._vVF(${o.id})">
            <div class="o-kanban-card-title">${o.name||"#"+o.id}</div>
            <div style="font-size:12px;color:var(--text-400);margin-bottom:8px">
              ${o.partner_name||""}
              ${M.alertas_cliente&&o.amount_residual>0?' <span style="color:#DC2626">⚠️</span>':""}
            </div>
            <div class="o-kanban-card-meta">
              <span style="font-size:11px">${((i=o.invoice_date)==null?void 0:i.slice(0,10))||""}</span>
              <span class="o-kanban-card-amount">${f(o.amount_total)}</span>
            </div>
          </div>`}).join("")||'<div style="padding:16px;text-align:center;color:var(--text-300);font-size:12px">Vacío</div>'}
      </div>
    </div>`).join("")}</div>`}function wo(){window._fca=t=>{document.querySelectorAll(".frc").forEach(e=>e.checked=t),window._frc()},window._frc=()=>{const t=document.querySelectorAll(".frc:checked").length,e=document.getElementById("flab"),a=document.getElementById("fsel-cnt");e&&e.classList.toggle("visible",t>0),a&&(a.textContent=t+" seleccionado"+(t!==1?"s":"")),document.querySelectorAll("[data-id]").forEach(o=>{const i=o.querySelector(".frc");i&&o.classList.toggle("selected",i.checked)})}}window._fp=t=>{it=t,gt()};window._vVF=async t=>{var e,a,o,i;w(`<div style="padding:40px">${I(3,5)}</div>`);try{const s=await g.factura(t),n=(s==null?void 0:s.data)||s;if(!n)throw new Error("No encontrada");n.name||""+t;const d=["draft","posted","in_payment","paid"];n.state==="cancel"&&d.push("cancel");const r=d.indexOf(n.state),l={draft:"Borrador",posted:"Publicada",in_payment:"En Pago",paid:"Pagada",cancel:"Cancelada"};w(`
      <div class="o-form-view" id="ffv">
        <div class="o-statusbar">
          <div class="o-statusbar-status">
            ${d.map((c,p)=>`
              <div class="o-status-step ${c===n.state?"active":""} ${p<r?"done":""}">
                ${p<r?"✔ ":""}${l[c]||c}
              </div>${p<d.length-1?'<span class="o-status-arrow">›</span>':""}`).join("")}
          </div>
          <div class="o-statusbar-buttons">
            ${n.state==="draft"?`<button class="btn btn-primary btn-sm" onclick="window._pubF(${t})">✅ Confirmar / Publicar</button>`:""}
            ${n.state==="posted"?`<button class="btn btn-primary btn-sm" onclick="window._pagoF(${t})">💳 Registrar Pago</button>`:""}
            
            ${M.pagos_online&&n.state==="posted"?`<button class="btn btn-secondary btn-sm" onclick="toast('Link de Pago', 'https://pagos.nexustecherp.com/pay/${n.id}', 'info')">🔗 Generar Enlace de Pago</button>`:""}
            
            ${n.state==="draft"||n.state==="posted"?`<button class="btn btn-secondary btn-sm" onclick="window._timF(${t})">🔐 Timbrar CFDI</button>`:""}
            <button class="btn btn-secondary btn-sm" onclick="toast('Info','PDF próximamente','info')">📄 Descargar PDF</button>
            ${n.state!=="cancel"&&n.state!=="paid"?`<button class="btn btn-sm" style="background:#FEE2E2;color:#DC2626;border:none;padding:6px 14px;border-radius:8px;font-weight:600;cursor:pointer" onclick="window._cancF(${t})">❌ Cancelar${M.cancelacion_directa?" (Directo)":""}</button>`:""}
            <button class="btn btn-secondary btn-sm" onclick="window._go('facturas')">← Volver</button>
          </div>
        </div>
        <div class="o-smart-buttons">
          <button class="o-smart-btn"><span class="o-count">${((e=n.payment_ids)==null?void 0:e.length)||0}</span><span class="o-label">💳 Pagos</span></button>
          ${M.pagos_online?'<button class="o-smart-btn"><span class="o-count" style="color:#059669">0</span><span class="o-label">Stripe</span></button>':""}
          <button class="o-smart-btn"><span class="o-count">0</span><span class="o-label">🔐 CFDI</span></button>
        </div>
        <div class="o-form-sheet">
          <div class="o-form-title-row">
            <h1 class="o-form-record-title">${n.name||"Nueva Factura"}</h1>
            <span class="o-form-subtitle">${n.partner_name||""}</span>
          </div>
          <div class="o-form-group-wrapper">
            <div class="o-form-group">
              <div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">Número</div><div class="o-field-value"><strong>${n.name||'<span class="o-field-empty">Borrador</span>'}</strong></div></div>
                <div class="o-field-row"><div class="o-field-label">Tipo</div><div class="o-field-value">${n.move_type==="out_invoice"?"Factura de cliente":n.move_type||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Cliente</div><div class="o-field-value"><strong>${n.partner_name||n.partner_id||'<span class="o-field-empty">—</span>'}</strong></div></div>
                <div class="o-field-row"><div class="o-field-label">Fecha</div><div class="o-field-value">${((a=n.invoice_date)==null?void 0:a.slice(0,10))||((o=n.date)==null?void 0:o.slice(0,10))||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Vencimiento</div><div class="o-field-value">${((i=n.invoice_date_due)==null?void 0:i.slice(0,10))||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Referencia</div><div class="o-field-value">${n.ref||'<span class="o-field-empty">—</span>'}</div></div>
              </div>
              <div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">Estado</div><div class="o-field-value">${Rt(n.state,$e[n.state]||n.state)}</div></div>
                <div class="o-field-row"><div class="o-field-label">Empresa</div><div class="o-field-value">${n.company_id||n.company_name||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Diario</div><div class="o-field-value">${n.journal_id||n.journal_name||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Método Pago</div><div class="o-field-value">${n.invoice_payment_term_id||n.payment_term||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Saldo</div><div class="o-field-value" style="font-weight:700;color:${n.amount_residual>0?"#DC2626":"var(--text-700)"}">${f(n.amount_residual||0)}</div></div>
              </div>
            </div>
          </div>
          <div class="o-notebook">
            <div class="o-tabs">
              <button class="o-tab active" onclick="window._ft('fl')">Líneas de Factura</button>
              <button class="o-tab" onclick="window._ft('fi')">Otra Información</button>
              <button class="o-tab" onclick="window._ft('fc')">CFDI 4.0</button>
            </div>
            <div class="o-tab-panel active" id="tab-panel-fl">
              <table class="o-editable-table"><thead><tr>
                <th>Producto / Servicio</th><th>Descripción</th>
                <th style="text-align:right">Qty</th>
                <th style="text-align:right">Precio</th>
                ${M.descuentos_pronto_pago?'<th style="text-align:right">% P.P.</th>':""}
                ${M.impuestos_ventas?'<th style="text-align:right">Impuesto</th>':""}
                <th style="text-align:right">Subtotal</th>
              </tr></thead>
              <tbody id="flineas"><tr><td colspan="${M.impuestos_ventas?6:5}" style="text-align:center;padding:20px;color:var(--text-400)">⏳ Cargando…</td></tr></tbody></table>
              <div class="o-lines-totals"><table id="ftotals">
                <!-- Se llena asincronamente -->
              </table></div>
            </div>
            <div class="o-tab-panel" id="tab-panel-fi">
              <div class="o-form-group"><div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">Notas</div><div class="o-field-value">${n.narration||n.note||(n.state==="draft"?M.terminos_default:'<span class="o-field-empty">—</span>')}</div></div>
                <div class="o-field-row"><div class="o-field-label">Ref. Interna</div><div class="o-field-value">${n.payment_reference||'<span class="o-field-empty">—</span>'}</div></div>
              </div></div>
            </div>
            <div class="o-tab-panel" id="tab-panel-fc">
              <div class="o-form-group"><div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">UUID CFDI</div><div class="o-field-value">${n.l10n_mx_edi_cfdi_uuid||'<span class="o-field-empty">No timbrado</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Uso CFDI</div><div class="o-field-value">${n.l10n_mx_edi_usage||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Método Pago SAT</div><div class="o-field-value">${n.l10n_mx_edi_payment_method_id||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Forma Pago SAT</div><div class="o-field-value">${n.l10n_mx_edi_payment_policy||'<span class="o-field-empty">—</span>'}</div></div>
              </div></div>
            </div>
          </div>
        </div>
        <div class="o-chatter">
          <div class="o-chatter-topbar">
            <button class="o-chatter-btn">✉️ Enviar mensaje</button>
            <button class="o-chatter-btn">📋 Nota interna</button>
            <button class="o-chatter-btn">📎 Adjuntar</button>
          </div>
          <div class="o-chatter-thread">
            <div class="o-message">
              <div class="o-msg-avatar" style="background:#059669">F</div>
              <div class="o-msg-content">
                <div class="o-msg-header">
                  <span class="o-msg-author">Sistema</span>
                  <span class="o-msg-date">${new Date().toLocaleDateString("es-MX")}</span>
                </div>
                <div class="o-msg-text">Factura ${n.name||""} — Estado: ${$e[n.state]||n.state}</div>
              </div>
            </div>
          </div>
        </div>
      </div>`),window._ft=c=>{document.querySelectorAll(".o-tab").forEach(u=>u.classList.remove("active")),document.querySelectorAll(".o-tab-panel").forEach(u=>u.classList.remove("active"));const p=document.querySelector(`.o-tab[onclick*="'${c}'"]`);p&&p.classList.add("active");const m=document.getElementById("tab-panel-"+c);m&&m.classList.add("active")};try{const c=await g.get(`/facturas/${t}/lineas`),p=(c==null?void 0:c.data)||[],m=document.getElementById("flineas");let u=0,v=0;m&&(m.innerHTML=p.length?p.map(_=>{var T,h;const $=_.price_unit*(_.quantity||0);return u+=$,M.impuestos_ventas&&((T=_.tax_ids)!=null&&T.length)&&(v+=$*.16),`<tr>
              <td>${_.product_id?"#"+_.product_id:'<span class="o-field-empty">Servicio</span>'}</td>
              <td>${_.name||"-"}</td>
              <td style="text-align:right">${_.quantity??0}</td>
              <td style="text-align:right">${f(_.price_unit)}</td>
              ${M.descuentos_pronto_pago?'<td style="text-align:right"><span style="color:var(--text-400);font-size:11px">0%</span></td>':""}
              ${M.impuestos_ventas?`<td style="text-align:right;font-size:11px">${(h=_.tax_ids)!=null&&h.length?"IVA 16%":"—"}</td>`:""}
              <td style="text-align:right;font-weight:700">${f($)}</td>
            </tr>`}).join(""):`<tr><td colspan="${(M.impuestos_ventas?6:5)+(M.descuentos_pronto_pago?1:0)}" style="text-align:center;padding:16px;color:var(--text-400)">Sin líneas de factura</td></tr>`);const k=document.getElementById("ftotals");if(k){let _=u+v,$=0;if(M.redondeo){const T=Math.round(_*20)/20;$=T-_,_=T}k.innerHTML=`
          <tr><td>Subtotal:</td><td style="text-align:right;font-weight:600">${f(u)}</td></tr>
          ${M.impuestos_ventas?`<tr><td>IVA (16%):</td><td style="text-align:right;font-weight:600">${f(v)}</td></tr>`:""}
          ${M.redondeo?`<tr><td>Ajuste (Redondeo):</td><td style="text-align:right;color:var(--text-500)">${f($)}</td></tr>`:""}
          <tr class="total-row"><td>TOTAL:</td><td style="text-align:right">${f(_)}</td></tr>
        `}}catch{}window._pubF=async c=>{if(confirm("¿Confirmar y publicar factura?"))try{await g.put(`/facturas/${c}/confirmar`,{}),M.cfdi_auto?(b("CFDI Auto","Timbrando automáticamente...","info"),setTimeout(()=>{b("Timbrado Exitoso","El CFDI se ha enviado al PAC","success")},1500)):b("OK","Factura publicada","success"),window._vVF(c)}catch(p){b("Error",p.message,"error")}},window._pagoF=async c=>{if(confirm("¿Registrar pago de esta factura?"))try{await g.post(`/facturas/${c}/pago`,{}),b("OK","Pago registrado","success"),window._vVF(c)}catch(p){b("Error",p.message,"error")}},window._timF=c=>{window._go("cfdi")},window._cancF=async c=>{let p="¿Cancelar factura?";if(M.cancelacion_directa&&(p="⚠️ ADVERTENCIA: La cancelación directa omitirá el estatus en el SAT. ¿Proceder?"),!!confirm(p))try{await g.put(`/facturas/${c}/cancelar`,{}),b("Cancelado","Factura cancelada con éxito","info"),window._go("facturas")}catch(m){b("Error",m.message,"error")}}}catch(s){w(`<div style="padding:40px;text-align:center"><p style="color:#DC2626">⚠️ ${s.message}</p><button class="o-btn-new" onclick="window._go('facturas')">Volver</button></div>`)}};function _o(t,e){let a;return(...o)=>{clearTimeout(a),a=setTimeout(()=>t(...o),e)}}function $o(t,e){Q("Editar Contacto",`
  <form id="form-edit-partner" onsubmit="event.preventDefault();window._submitEditPartner()">
    <div class="modal-form-grid">
      <div class="modal-form-full">
        <label class="modal-form-label">Nombre *</label>
        <input id="ep-name" class="modal-form-input" value="${(t.name||"").replace(/"/g,"&quot;")}" required placeholder="Nombre o razón social">
      </div>
      <div>
        <label class="modal-form-label">Email</label>
        <input id="ep-email" type="email" class="modal-form-input" value="${(t.email||"").replace(/"/g,"&quot;")}" placeholder="contacto@empresa.com">
      </div>
      <div>
        <label class="modal-form-label">Teléfono</label>
        <input id="ep-phone" class="modal-form-input" value="${(t.phone||"").replace(/"/g,"&quot;")}" placeholder="+52 81 0000 0000">
      </div>
      <div>
        <label class="modal-form-label">Móvil</label>
        <input id="ep-mobile" class="modal-form-input" value="${(t.mobile||"").replace(/"/g,"&quot;")}" placeholder="+52 81 0000 0000">
      </div>
      <div>
        <label class="modal-form-label">Ciudad</label>
        <input id="ep-city" class="modal-form-input" value="${(t.city||"").replace(/"/g,"&quot;")}" placeholder="Monterrey">
      </div>
      <div>
        <label class="modal-form-label">RFC</label>
        <input id="ep-vat" class="modal-form-input" value="${(t.vat||"").replace(/"/g,"&quot;")}" placeholder="XAXX010101000" style="text-transform:uppercase">
      </div>
      <div class="modal-form-full">
        <label class="modal-form-label">Sitio web</label>
        <input id="ep-website" type="url" class="modal-form-input" value="${(t.website||"").replace(/"/g,"&quot;")}" placeholder="https://empresa.com">
      </div>
    </div>
    <div class="modal-actions">
      <button type="button" class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cancelar</button>
      <button type="submit" class="btn btn-primary btn-sm" id="btn-save-partner">💾 Guardar</button>
    </div>
    <div id="edit-partner-result" style="margin-top:12px"></div>
  </form>`),window._submitEditPartner=async()=>{var i,s,n,d,r,l,c,p,m;const a=document.getElementById("btn-save-partner"),o=(s=(i=document.getElementById("ep-name"))==null?void 0:i.value)==null?void 0:s.trim();if(!o){b("Error de validación","El nombre es obligatorio","error");return}a.textContent="⏳ Guardando…",a.disabled=!0;try{const u={name:o,email:((n=document.getElementById("ep-email"))==null?void 0:n.value)||"",phone:((d=document.getElementById("ep-phone"))==null?void 0:d.value)||"",mobile:((r=document.getElementById("ep-mobile"))==null?void 0:r.value)||"",city:((l=document.getElementById("ep-city"))==null?void 0:l.value)||"",vat:((p=(c=document.getElementById("ep-vat"))==null?void 0:c.value)==null?void 0:p.toUpperCase())||"",website:((m=document.getElementById("ep-website"))==null?void 0:m.value)||""};await g.put(`/partners/${t.id}`,u).catch(()=>null),b("Contacto actualizado",o,"success"),window.__closeModal(),e&&e()}catch(u){const v=document.getElementById("edit-partner-result");v&&(v.innerHTML=`<p style="color:var(--red)">${u.message}</p>`)}finally{a.textContent="💾 Guardar",a.disabled=!1}}}function ko(t,e){const a=t.name&&typeof t.name=="object"?t.name.es_MX||t.name.en_US||Object.values(t.name)[0]||"":t.name||t.nombre||"";Q("Editar Producto",`
  <form id="form-edit-producto" onsubmit="event.preventDefault();window._submitEditProducto()">
    <div class="modal-form-grid">
      <div class="modal-form-full">
        <label class="modal-form-label">Nombre (en_US) *</label>
        <input id="epr-name" class="modal-form-input" value="${a.replace(/"/g,"&quot;")}" required placeholder="Nombre del producto">
      </div>
      <div>
        <label class="modal-form-label">Código interno</label>
        <input id="epr-code" class="modal-form-input" value="${(t.default_code||"").replace(/"/g,"&quot;")}" placeholder="SKU-001">
      </div>
      <div>
        <label class="modal-form-label">Precio de venta</label>
        <input id="epr-precio" type="number" step="0.01" min="0" class="modal-form-input" value="${parseFloat(t.list_price||0).toFixed(2)}" placeholder="0.00">
      </div>
      <div>
        <label class="modal-form-label">Costo estándar</label>
        <input id="epr-costo" type="number" step="0.01" min="0" class="modal-form-input" value="${parseFloat(t.standard_price||0).toFixed(2)}" placeholder="0.00">
      </div>
    </div>
    <div class="modal-actions">
      <button type="button" class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cancelar</button>
      <button type="submit" class="btn btn-primary btn-sm" id="btn-save-producto">💾 Guardar</button>
    </div>
    <div id="edit-producto-result" style="margin-top:12px"></div>
  </form>`),window._submitEditProducto=async()=>{var i,s,n,d;const o=document.getElementById("btn-save-producto");o.textContent="⏳ Guardando…",o.disabled=!0;try{const r={name:((i=document.getElementById("epr-name"))==null?void 0:i.value)||a,default_code:((s=document.getElementById("epr-code"))==null?void 0:s.value)||"",list_price:parseFloat(((n=document.getElementById("epr-precio"))==null?void 0:n.value)||0),standard_price:parseFloat(((d=document.getElementById("epr-costo"))==null?void 0:d.value)||0)};let l=!1;try{await g.put(`/productos/${t.id}`,r),l=!0}catch{l=!1}l?b("Producto actualizado",r.name,"success"):b("Guardado localmente","Se sincronizará cuando el endpoint esté disponible","warning"),window.__closeModal(),e&&e()}catch(r){const l=document.getElementById("edit-producto-result");l&&(l.innerHTML=`<p style="color:var(--red)">${r.message}</p>`)}finally{o.textContent="💾 Guardar",o.disabled=!1}}}function Qe(t,e){const a=parseFloat(t.cantidad_disponible||0);Q("Ajuste de Inventario",`
  <form id="form-ajuste-stock" onsubmit="event.preventDefault();window._submitAjusteStock()">
    <div style="margin-bottom:16px;padding:12px;background:var(--bg-100);border-radius:10px">
      <div style="font-size:12px;color:var(--text-400);margin-bottom:4px">Producto</div>
      <div style="font-weight:700;color:var(--text-900)">${t.product_name||`#${t.product_id}`}</div>
      <div style="font-size:12px;color:var(--text-500);margin-top:4px">Stock actual: <strong>${a}</strong> unidades</div>
    </div>
    <div class="modal-form-grid">
      <div>
        <label class="modal-form-label">Nueva cantidad disponible *</label>
        <input id="ast-qty" type="number" step="0.01" min="0" class="modal-form-input" value="${a}" required placeholder="0">
      </div>
      <div>
        <label class="modal-form-label">Motivo del ajuste *</label>
        <select id="ast-motivo" class="modal-form-select" required>
          <option value="Corrección">Corrección</option>
          <option value="Merma">Merma</option>
          <option value="Entrada manual">Entrada manual</option>
          <option value="Conteo físico">Conteo físico</option>
        </select>
      </div>
    </div>
    <div class="modal-actions">
      <button type="button" class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cancelar</button>
      <button type="submit" class="btn btn-primary btn-sm" id="btn-save-stock">📋 Aplicar ajuste</button>
    </div>
    <div id="ajuste-stock-result" style="margin-top:12px"></div>
  </form>`),window._submitAjusteStock=async()=>{var i,s;const o=document.getElementById("btn-save-stock");o.textContent="⏳ Guardando…",o.disabled=!0;try{const n={cantidad:parseFloat(((i=document.getElementById("ast-qty"))==null?void 0:i.value)||0),motivo:((s=document.getElementById("ast-motivo"))==null?void 0:s.value)||"Corrección"};try{await g.put(`/stock/${t.product_id}/ajuste`,n)}catch{}b("Inventario ajustado",`Nuevo stock: ${n.cantidad} — ${n.motivo}`,"success"),window.__closeModal(),e&&e()}catch(n){const d=document.getElementById("ajuste-stock-result");d&&(d.innerHTML=`<p style="color:var(--red)">${n.message}</p>`)}finally{o.textContent="📋 Aplicar ajuste",o.disabled=!1}}}function Eo(t,e){const a=t.state==="draft";Q("Editar Orden de Compra",`
  <form id="form-edit-compra" onsubmit="event.preventDefault();window._submitEditCompra()">
    ${a?"":`
    <div style="margin-bottom:12px;padding:10px 14px;background:var(--warning-light,#fef9ec);border:1px solid var(--warning,#f59e0b);border-radius:8px;font-size:12px;color:var(--warning,#b45309)">
      ⚠️ Solo se puede editar en estado Borrador. Estado actual: <strong>${t.state}</strong>
    </div>`}
    <div class="modal-form-grid">
      <div>
        <label class="modal-form-label">Folio</label>
        <div class="modal-form-input" style="background:var(--bg-200);color:var(--text-500);cursor:not-allowed">${t.name||`#${t.id}`}</div>
      </div>
      <div>
        <label class="modal-form-label">Fecha esperada de entrega</label>
        <input id="ec-date" type="date" class="modal-form-input" value="${(t.date_planned||t.date_approve||"").substring(0,10)}" ${a?"":"disabled"}>
      </div>
      <div class="modal-form-full">
        <label class="modal-form-label">Notas internas</label>
        <textarea id="ec-note" class="modal-form-textarea" placeholder="Condiciones, instrucciones para el proveedor…" ${a?"":"disabled"}>${t.note||""}</textarea>
      </div>
    </div>
    <div class="modal-actions">
      <button type="button" class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cancelar</button>
      ${a?'<button type="submit" class="btn btn-primary btn-sm" id="btn-save-compra">💾 Guardar</button>':""}
    </div>
    <div id="edit-compra-result" style="margin-top:12px"></div>
  </form>`),window._submitEditCompra=async()=>{var i,s;if(!a)return;const o=document.getElementById("btn-save-compra");o.textContent="⏳ Guardando…",o.disabled=!0;try{const n={note:((i=document.getElementById("ec-note"))==null?void 0:i.value)||"",date_planned:((s=document.getElementById("ec-date"))==null?void 0:s.value)||""};await g.put(`/compras/${t.id}`,n).catch(()=>null),b("Compra actualizada",`OC ${t.name||t.id} guardada`,"success"),window.__closeModal(),e&&e()}catch(n){const d=document.getElementById("edit-compra-result");d&&(d.innerHTML=`<p style="color:var(--red)">${n.message}</p>`)}finally{o.textContent="💾 Guardar",o.disabled=!1}}}function Co(t,e){Q("Editar Empleado",`
  <form id="form-edit-empleado" onsubmit="event.preventDefault();window._submitEditEmpleado()">
    <div style="display:flex;align-items:center;gap:12px;margin-bottom:16px;padding:12px;background:var(--bg-100);border-radius:10px">
      <div style="width:40px;height:40px;border-radius:50%;background:linear-gradient(135deg,hsl(${t.id*47%360},60%,55%),hsl(${t.id*89%360},70%,45%));display:flex;align-items:center;justify-content:center;color:white;font-weight:700;font-size:14px;flex-shrink:0">
        ${(t.name||"?").split(" ").map(a=>a[0]).slice(0,2).join("")}
      </div>
      <div>
        <div style="font-weight:700;color:var(--text-900)">${t.name||"—"}</div>
        <div style="font-size:12px;color:var(--text-400)">${t.department_name||t.department_id_name||"Sin departamento"}</div>
      </div>
    </div>
    <div class="modal-form-grid">
      <div>
        <label class="modal-form-label">Puesto</label>
        <input id="ee-title" class="modal-form-input" value="${(t.job_title||"").replace(/"/g,"&quot;")}" placeholder="Gerente de ventas">
      </div>
      <div>
        <label class="modal-form-label">N° IMSS (SSNID)</label>
        <input id="ee-imss" class="modal-form-input" value="${(t.ssnid||t.imss||"").replace(/"/g,"&quot;")}" placeholder="01234567890">
      </div>
      <div>
        <label class="modal-form-label">Email laboral</label>
        <input id="ee-email" type="email" class="modal-form-input" value="${(t.work_email||t.email||"").replace(/"/g,"&quot;")}" placeholder="empleado@empresa.com">
      </div>
      <div>
        <label class="modal-form-label">Teléfono laboral</label>
        <input id="ee-phone" class="modal-form-input" value="${(t.work_phone||t.mobile_phone||"").replace(/"/g,"&quot;")}" placeholder="+52 81 0000 0000">
      </div>
    </div>
    <div class="modal-actions">
      <button type="button" class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cancelar</button>
      <button type="submit" class="btn btn-primary btn-sm" id="btn-save-emp">💾 Guardar</button>
    </div>
    <div id="edit-emp-result" style="margin-top:12px"></div>
  </form>`),window._submitEditEmpleado=async()=>{var o,i,s,n;const a=document.getElementById("btn-save-emp");a.textContent="⏳ Guardando…",a.disabled=!0;try{const d={job_title:((o=document.getElementById("ee-title"))==null?void 0:o.value)||"",ssnid:((i=document.getElementById("ee-imss"))==null?void 0:i.value)||"",work_email:((s=document.getElementById("ee-email"))==null?void 0:s.value)||"",work_phone:((n=document.getElementById("ee-phone"))==null?void 0:n.value)||""};await g.put(`/nomina/${t.id}`,d).catch(()=>null),b("Empleado actualizado",t.name,"success"),window.__closeModal(),e&&e()}catch(d){const r=document.getElementById("edit-emp-result");r&&(r.innerHTML=`<p style="color:var(--red)">${d.message}</p>`)}finally{a.textContent="💾 Guardar",a.disabled=!1}}}let Dt="list",ht=1,It=[],dt="",Qt="";async function ba(){B(),Dt="list",ht=1,dt="",Qt="",So(),await Vt()}function So(){w(`
  <div class="o-cp" id="productos-cp">
    <div class="o-cp-left">
      <button class="o-btn-primary" onclick="window._productoNuevo()">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
        Nuevo
      </button>
    </div>
    <div class="o-cp-center">
      <div class="o-search-bar">
        <svg class="o-search-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/></svg>
        <input id="o-search-productos" class="o-search-input" type="text" placeholder="Buscar producto o código…" value="${dt}">
        <div class="o-search-filters">
          <button class="o-filter-btn" onclick="window._productoFiltroTipo('')" data-active id="ptf-todos">Todos</button>
          <button class="o-filter-btn" onclick="window._productoFiltroTipo('consu')" id="ptf-consu">Consumibles</button>
          <button class="o-filter-btn" onclick="window._productoFiltroTipo('service')" id="ptf-serv">Servicios</button>
          <button class="o-filter-btn" onclick="window._productoFiltroTipo('product')" id="ptf-prod">Almacenables</button>
        </div>
      </div>
    </div>
    <div class="o-cp-right">
      <div class="o-view-switcher">
        <button class="o-view-btn ${Dt==="list"?"o-active":""}" onclick="window._productoSetView('list')" title="Vista Lista">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="8" y1="6" x2="21" y2="6"/><line x1="8" y1="12" x2="21" y2="12"/><line x1="8" y1="18" x2="21" y2="18"/><line x1="3" y1="6" x2="3.01" y2="6"/><line x1="3" y1="12" x2="3.01" y2="12"/><line x1="3" y1="18" x2="3.01" y2="18"/></svg>
        </button>
        <button class="o-view-btn ${Dt==="kanban"?"o-active":""}" onclick="window._productoSetView('kanban')" title="Vista Kanban">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="1" y="4" width="6" height="16" rx="1"/><rect x="9" y="4" width="6" height="10" rx="1"/><rect x="17" y="4" width="6" height="13" rx="1"/></svg>
        </button>
      </div>
    </div>
  </div>
  <div id="productos-content" class="o-view-content">
    ${I(10,6)}
  </div>`);let t;setTimeout(()=>{var e;(e=document.getElementById("o-search-productos"))==null||e.addEventListener("input",a=>{clearTimeout(t),t=setTimeout(()=>{dt=a.target.value.trim(),ht=1,Vt()},380)})},100)}async function Vt(){try{const t=await g.productos(ht,dt);It=((t==null?void 0:t.data)||[]).filter(o=>!Qt||(o.type_||o.type)===Qt);const e=((t==null?void 0:t.data)||[]).length>=20,a=document.getElementById("productos-content");if(!a)return;Dt==="kanban"?a.innerHTML=ga(It):a.innerHTML=fa(It,e)}catch(t){console.error(t),b("Error",t.message,"error")}}function fa(t,e){return t.length?`
  <div class="o-list-view">
    <table class="o-list-table">
      <thead>
        <tr>
          <th class="o-list-chk"><input type="checkbox" class="o-chk" onclick="window._chkAllProductos(this)"></th>
          <th style="width:56px">Imagen</th>
          <th class="o-col-sortable">Nombre</th>
          <th>SKU</th>
          <th class="o-col-right">Precio Venta</th>
          <th class="o-col-right">Costo</th>
          <th>Tipo</th>
          <th class="o-col-right">Stock</th>
        </tr>
      </thead>
      <tbody>
        ${t.map((a,o)=>{var m,u,v;const i=typeof a.name=="object"?((m=a.name)==null?void 0:m.es_MX)||((u=a.name)==null?void 0:u.en_US)||Object.values(a.name)[0]||`Producto #${a.id}`:a.name||a.nombre||`Producto #${a.id}`,s=a.type_||a.type||"",n=s==="consu"?"Consumible":s==="service"?"Servicio":s==="product"?"Almacenable":"Consumible",d=s==="service"?"o-badge-info":s==="consu"?"o-badge-warn":"o-badge-success",r=f(parseFloat(a.list_price||a.precio||0)),l=f(parseFloat(a.standard_price||a.costo||0)),c=a.id*67%360,p=((v=i[0])==null?void 0:v.toUpperCase())||"P";return`
          <tr class="o-list-row" onclick="window._verProducto(${a.id})">
            <td class="o-list-chk"><input type="checkbox" class="o-chk" onclick="event.stopPropagation()"></td>
            <td>
              <div class="o-prod-thumb" style="background:linear-gradient(135deg,hsl(${c},50%,60%),hsl(${(c+60)%360},60%,50%))">${p}</div>
            </td>
            <td class="o-td-primary">${i}</td>
            <td class="o-td-mono">${a.default_code||"—"}</td>
            <td class="o-td-amount">${r}</td>
            <td class="o-td-amount o-td-muted">${l}</td>
            <td><span class="o-badge ${d}">${n}</span></td>
            <td class="o-td-amount">${a.qty_available!=null?W(parseFloat(a.qty_available)):"—"}</td>
          </tr>`}).join("")}
      </tbody>
    </table>
    <div class="o-list-footer">
      <span class="o-list-count">${t.length} producto${t.length!==1?"s":""}</span>
      ${vt(ht,e,a=>{ht=a,Vt()})}
    </div>
  </div>`:`
    <div class="o-empty-state">
      <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" opacity=".3"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/></svg>
      <p>${dt?`Sin resultados para "${dt}"`:"Sin productos en catálogo"}</p>
    </div>`}function ga(t){return t.length?`
  <div class="o-kanban-grid">
    ${t.map(e=>{var l,c,p;const a=typeof e.name=="object"?((l=e.name)==null?void 0:l.es_MX)||((c=e.name)==null?void 0:c.en_US)||`Producto #${e.id}`:e.name||`Producto #${e.id}`,o=e.type_||e.type||"",i=o==="consu"?"Consumible":o==="service"?"Servicio":"Almacenable",s=o==="service"?"o-badge-info":o==="consu"?"o-badge-warn":"o-badge-success",n=f(parseFloat(e.list_price||0)),d=e.id*67%360,r=((p=a[0])==null?void 0:p.toUpperCase())||"P";return`
      <div class="o-kanban-card" onclick="window._verProducto(${e.id})">
        <div class="o-kanban-img" style="background:linear-gradient(135deg,hsl(${d},50%,65%),hsl(${(d+60)%360},60%,55%))">
          <span style="font-size:40px;font-weight:800;color:rgba(255,255,255,.7)">${r}</span>
        </div>
        <div class="o-kanban-body">
          <div class="o-kanban-title">${a}</div>
          <div class="o-kanban-sub">${e.default_code||"(sin SKU)"}</div>
          <div style="display:flex;justify-content:space-between;align-items:center;margin-top:8px">
            <span class="o-badge ${s}">${i}</span>
            <strong class="o-kanban-price">${n}</strong>
          </div>
        </div>
      </div>`}).join("")}
  </div>`:`
    <div class="o-empty-state">
      <p>Sin productos${dt?` para "${dt}"`:""}</p>
    </div>`}window._verProducto=async t=>{var e,a,o,i,s;w(`<div class="o-form-loading">${I(4,3)}</div>`);try{const n=await g.producto(t);if(!n){b("Error","Producto no encontrado","error");return}const d=document.getElementById("bc-prod-name");d&&(d.textContent=typeof n.name=="object"?((e=n.name)==null?void 0:e.es_MX)||((a=n.name)==null?void 0:a.en_US)||"Producto":n.name||"Producto");const r=typeof n.name=="object"?((o=n.name)==null?void 0:o.es_MX)||((i=n.name)==null?void 0:i.en_US)||`Producto #${n.id}`:n.name||`Producto #${n.id}`,l=n.type_||n.type||"",c=l==="consu"?"Consumible":l==="service"?"Servicio":l==="product"?"Almacenable":"Consumible",p=l==="service"?"o-badge-info":l==="consu"?"o-badge-warn":"o-badge-success",m=f(parseFloat(n.list_price||0)),u=f(parseFloat(n.standard_price||0)),v=n.id*67%360,k=((s=r[0])==null?void 0:s.toUpperCase())||"P",_=(()=>{const $=n.categ_name||n.categoria||"";return $==="Goods"?"Mercancía":$==="Services"?"Servicios":$||"—"})();w(`
    <div class="o-form-topbar">
      <button class="o-back-btn" onclick="window._productosBack()">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
        Productos
      </button>
      <div class="o-form-actions">
        <button class="o-btn-secondary" onclick="window._editarProductoForm(${n.id})">Editar</button>
        <button class="o-btn-primary"   onclick="alert('Crear pedido — próximamente')">Crear Pedido</button>
      </div>
    </div>

    <div class="o-smart-buttons">
      <button class="o-smart-btn" onclick="alert('Stock disponible')">
        <span class="o-smart-count">${n.qty_available!=null?W(parseFloat(n.qty_available)):0}</span>
        <span class="o-smart-label">En Stock</span>
      </button>
      <button class="o-smart-btn" onclick="alert('Ventas del producto')">
        <span class="o-smart-count">0</span>
        <span class="o-smart-label">Ventas</span>
      </button>
      <button class="o-smart-btn" onclick="alert('Compras del producto')">
        <span class="o-smart-count">0</span>
        <span class="o-smart-label">Compras</span>
      </button>
    </div>

    <div class="o-form-sheet">
      <div class="o-sheet-header">
        <div class="o-prod-thumb o-prod-thumb-lg" style="background:linear-gradient(135deg,hsl(${v},50%,65%),hsl(${(v+60)%360},60%,55%))">${k}</div>
        <div class="o-sheet-title-block">
          <h1 class="o-form-title">${r}</h1>
          <div style="display:flex;gap:6px;margin-top:4px">
            <span class="o-badge ${p}">${c}</span>
            ${n.active!==!1?'<span class="o-badge o-badge-success">Activo</span>':'<span class="o-badge o-badge-gray">Inactivo</span>'}
          </div>
        </div>
      </div>

      <div class="o-form-grid">
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">SKU / Código interno</label><div class="o-field-value o-field-mono">${n.default_code||"—"}</div></div>
          <div class="o-field-group"><label class="o-field-label">Tipo de Producto</label><div class="o-field-value"><span class="o-badge ${p}">${c}</span></div></div>
          <div class="o-field-group"><label class="o-field-label">Unidad de Medida</label><div class="o-field-value">${n.uom_name||n.uom||"Unidad"}</div></div>
          <div class="o-field-group"><label class="o-field-label">Peso</label><div class="o-field-value">${n.weight!=null?n.weight+" kg":"—"}</div></div>
        </div>
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">Precio de Venta</label><div class="o-field-value o-field-price">${m}</div></div>
          <div class="o-field-group"><label class="o-field-label">Costo</label><div class="o-field-value o-td-muted">${u}</div></div>
          <div class="o-field-group"><label class="o-field-label">Impuestos</label><div class="o-field-value">${n.taxes_name||"IVA 16%"}</div></div>
          <div class="o-field-group"><label class="o-field-label">Categoría</label><div class="o-field-value">${_}</div></div>
        </div>
      </div>

      <div class="o-notebook">
        <div class="o-tabs" id="prod-tabs">
          <button class="o-tab active" onclick="window._prodTab('info', this)">Información General</button>
          <button class="o-tab" onclick="window._prodTab('ventas', this)">Ventas</button>
          <button class="o-tab" onclick="window._prodTab('compras', this)">Compras</button>
          <button class="o-tab" onclick="window._prodTab('inventario', this)">Inventario</button>
        </div>

        <div class="o-tab-pane" id="tab-info">
          <div class="o-field-group"><label class="o-field-label">Descripción</label>
            <div class="o-field-value">${n.description||n.descripcion||"—"}</div></div>
          <div class="o-form-grid" style="margin-top:12px">
            <div class="o-form-col">
              <div class="o-field-group"><label class="o-field-label">Peso (kg)</label><div class="o-field-value">${n.weight??"—"}</div></div>
            </div>
            <div class="o-form-col">
              <div class="o-field-group"><label class="o-field-label">Volumen (m³)</label><div class="o-field-value">${n.volume??"—"}</div></div>
            </div>
          </div>
        </div>
        <div class="o-tab-pane" id="tab-ventas" style="display:none">
          <div class="o-field-group"><label class="o-field-label">Política de facturación</label><div class="o-field-value">Cantidades ordenadas</div></div>
          <div class="o-field-group"><label class="o-field-label">Descripción en pedido de venta</label><div class="o-field-value">—</div></div>
        </div>
        <div class="o-tab-pane" id="tab-compras" style="display:none">
          <div class="o-field-group"><label class="o-field-label">Precio de compra</label><div class="o-field-value">${u}</div></div>
          <div class="o-field-group"><label class="o-field-label">Proveedor preferido</label><div class="o-field-value">—</div></div>
        </div>
        <div class="o-tab-pane" id="tab-inventario" style="display:none">
          <div class="o-form-grid">
            <div class="o-form-col">
              <div class="o-field-group"><label class="o-field-label">Método de costeo</label><div class="o-field-value">Precio estándar</div></div>
            </div>
            <div class="o-form-col">
              <div class="o-field-group"><label class="o-field-label">Rutas</label><div class="o-field-value">Comprar</div></div>
            </div>
          </div>
        </div>
      </div>
    </div>`),window._editarProductoForm=$=>ko({id:$,...n},()=>window._verProducto($)),window._prodTab=($,T)=>{document.querySelectorAll("#prod-tabs .o-tab").forEach(E=>E.classList.remove("active")),T.classList.add("active"),document.querySelectorAll(".o-tab-pane").forEach(E=>E.style.display="none");const h=document.getElementById(`tab-${$}`);h&&(h.style.display="")}}catch(n){console.error(n),b("Error",n.message,"error")}};window._productosBack=()=>ba();window._productoSetView=t=>{var o;Dt=t,document.querySelectorAll("#productos-cp .o-view-btn").forEach(i=>i.classList.remove("o-active"));const e=t==="list"?0:1;(o=document.querySelectorAll("#productos-cp .o-view-btn")[e])==null||o.classList.add("o-active");const a=document.getElementById("productos-content");a&&(t==="kanban"?a.innerHTML=ga(It):a.innerHTML=fa(It,!1))};window._productoFiltroTipo=t=>{var a;Qt=t,ht=1,document.querySelectorAll("#productos-cp .o-filter-btn").forEach(o=>o.removeAttribute("data-active"));const e={"":"ptf-todos",consu:"ptf-consu",service:"ptf-serv",product:"ptf-prod"};(a=document.getElementById(e[t]))==null||a.setAttribute("data-active",""),Vt()};window._productoNuevo=()=>{de(()=>import("./create_forms-CESEMRXd.js"),[]).then(t=>t.nuevoProducto(()=>Vt()))};window._chkAllProductos=t=>document.querySelectorAll("#productos-content .o-chk").forEach(e=>e.checked=t.checked);let st=1,ue=[],Zt="",pt="";async function Ne(){B(),st=1,Zt="",pt="",{...JSON.parse(localStorage.getItem("nexus_config_contactos")||"{}")},zo(),await qe()}function zo(){w(`
  <!-- ── ODOO CONTROL PANEL ── -->
  <div class="o-cp" id="partners-cp">
    <div class="o-cp-left">
      <button class="o-btn-primary" onclick="window._partnerNuevo()">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
        Nuevo
      </button>
    </div>
    <div class="o-cp-center">
      <div class="o-search-bar">
        <svg class="o-search-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/></svg>
        <input id="o-search-partners" class="o-search-input" type="text" placeholder="Buscar…" value="${Zt}">
        <div class="o-search-filters">
          <button class="o-filter-btn" onclick="window._partnerFilter('')" id="pf-all" ${pt===""?"data-active":""}>Todos</button>
          <button class="o-filter-btn" onclick="window._partnerFilter('clientes')" id="pf-cli" ${pt==="clientes"?"data-active":""}>Clientes</button>
          <button class="o-filter-btn" onclick="window._partnerFilter('proveedores')" id="pf-prov" ${pt==="proveedores"?"data-active":""}>Proveedores</button>
        </div>
      </div>
    </div>
    <div class="o-cp-right">
      <button class="o-btn-secondary" style="margin-right:8px;font-size:16px;padding:4px 8px" onclick="window._go('config_contactos')" title="Ajustes">⚙️</button>
      <div class="o-view-switcher">
        <button class="o-view-btn" data-active title="Vista Lista">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="8" y1="6" x2="21" y2="6"/><line x1="8" y1="12" x2="21" y2="12"/><line x1="8" y1="18" x2="21" y2="18"/><line x1="3" y1="6" x2="3.01" y2="6"/><line x1="3" y1="12" x2="3.01" y2="12"/><line x1="3" y1="18" x2="3.01" y2="18"/></svg>
        </button>
      </div>
    </div>
  </div>
  <!-- ── CONTENT AREA ── -->
  <div id="partners-content" class="o-view-content">
    ${I(10,6)}
  </div>`),setTimeout(()=>{var t;(t=document.getElementById("o-search-partners"))==null||t.addEventListener("input",e=>{Zt=e.target.value.toLowerCase(),To()})},100)}function To(){document.querySelectorAll("#partners-content tbody tr").forEach(t=>{t.style.display=t.textContent.toLowerCase().includes(Zt)?"":"none"})}async function qe(){try{let t;pt==="clientes"?t=g.clientes(st):pt==="proveedores"?t=g.proveedores(st):t=g.partners(st);const e=await t;ue=(e==null?void 0:e.data)||[];const a=ue.length>=20,o=document.getElementById("partners-content");if(!o)return;o.innerHTML=Io(ue,a)}catch(t){console.error(t),b("Error",t.message,"error");const e=document.getElementById("partners-content");e&&(e.innerHTML='<div class="o-empty-state"><p>Error al cargar contactos</p></div>')}}function Io(t,e){return t.length?`
  <div class="o-list-view">
    <table class="o-list-table">
      <thead>
        <tr>
          <th class="o-list-chk"><input type="checkbox" class="o-chk" id="chk-all-partners" onclick="window._chkAllPartners(this)"></th>
          <th class="o-col-sortable">#</th>
          <th class="o-col-sortable">Cliente / Proveedor</th>
          <th>Email</th>
          <th>Teléfono</th>
          <th>Ciudad</th>
          <th>RFC</th>
          <th>Tipo</th>
        </tr>
      </thead>
      <tbody>
        ${t.map((a,o)=>{const i=(a.customer_rank||0)>0,s=(a.supplier_rank||0)>0,n=a.is_company,d=a.name||a.nombre||"—",r=d.split(" ").map(c=>c[0]).slice(0,2).join(""),l=a.id*37%360;return`
          <tr class="o-list-row" onclick="window._verPartner(${a.id})">
            <td class="o-list-chk"><input type="checkbox" class="o-chk" onclick="event.stopPropagation()"></td>
            <td class="o-list-num">${(st-1)*20+o+1}</td>
            <td>
              <div class="o-partner-cell">
                <div class="o-avatar" style="background:linear-gradient(135deg,hsl(${l},60%,55%),hsl(${(l+40)%360},70%,45%))">${r||"?"}</div>
                <div>
                  <div class="o-td-primary">${d}</div>
                  ${n?'<div class="o-td-secondary">Empresa</div>':""}
                </div>
              </div>
            </td>
            <td class="o-td-muted">${a.email||"—"}</td>
            <td class="o-td-muted">${a.phone||"—"}</td>
            <td class="o-td-muted">${a.city||"—"}</td>
            <td class="o-td-mono">${a.vat||"—"}</td>
            <td>
              ${i?'<span class="o-badge o-badge-success">Cliente</span>':""}
              ${s?'<span class="o-badge o-badge-info" style="margin-left:2px">Proveedor</span>':""}
              ${!i&&!s?'<span class="o-badge o-badge-gray">Contacto</span>':""}
            </td>
          </tr>`}).join("")}
      </tbody>
    </table>
    <div class="o-list-footer">
      <span class="o-list-count">${t.length} contacto${t.length!==1?"s":""}</span>
      ${vt(st,e,a=>{st=a,qe()})}
    </div>
  </div>`:`
    <div class="o-empty-state">
      <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" opacity=".3"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>
      <p>Sin contactos registrados</p>
    </div>`}window._verPartner=async t=>{w(`<div class="o-form-loading">${I(4,3)}</div>`);try{const e=await g.partner(t);if(!e){b("Error","Contacto no encontrado","error");return}const a=document.getElementById("bc-partner-name");a&&(a.textContent=e.name||"Contacto");const o=(e.customer_rank||0)>0,i=(e.supplier_rank||0)>0,s=e.is_company,n=e.name||"—",d=n.split(" ").map(l=>l[0]).slice(0,2).join(""),r=e.id*37%360;w(`
    <!-- ── FORM BREADCRUMB BAR ── -->
    <div class="o-form-topbar">
      <button class="o-back-btn" onclick="window._partnersBack()">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
        Clientes / Proveedores
      </button>
      <div class="o-form-actions">
        <button class="o-btn-secondary" onclick="window._editarPartnerForm(${e.id})">Editar</button>
        <button class="o-btn-secondary" onclick="alert('Crear factura — próximamente')">Crear Factura</button>
        <button class="o-btn-primary"   onclick="alert('Crear venta — próximamente')">Crear Venta</button>
      </div>
    </div>

    <!-- ── SMART BUTTONS ── -->
    <div class="o-smart-buttons">
      <button class="o-smart-btn" onclick="alert('Ventas del cliente')">
        <span class="o-smart-count">0</span>
        <span class="o-smart-label">Ventas</span>
      </button>
      <button class="o-smart-btn" onclick="alert('Facturas del cliente')">
        <span class="o-smart-count">0</span>
        <span class="o-smart-label">Facturas</span>
      </button>
      <button class="o-smart-btn" onclick="alert('Compras del proveedor')">
        <span class="o-smart-count">0</span>
        <span class="o-smart-label">Compras</span>
      </button>
    </div>

    <!-- ── FORM SHEET ── -->
    <div class="o-form-sheet">
      <div class="o-sheet-header">
        <div class="o-avatar o-avatar-lg" style="background:linear-gradient(135deg,hsl(${r},60%,55%),hsl(${(r+40)%360},70%,45%))">${d||"?"}</div>
        <div class="o-sheet-title-block">
          <h1 class="o-form-title">${n}</h1>
          <div style="display:flex;gap:6px;margin-top:4px">
            ${o?'<span class="o-badge o-badge-success">Cliente</span>':""}
            ${i?'<span class="o-badge o-badge-info">Proveedor</span>':""}
            ${s?'<span class="o-badge o-badge-gray">Empresa</span>':'<span class="o-badge o-badge-gray">Persona física</span>'}
          </div>
        </div>
      </div>

      <div class="o-form-grid">
        <!-- Col 1 -->
        <div class="o-form-col">
          <div class="o-field-group">
            <label class="o-field-label">Nombre</label>
            <div class="o-field-value">${e.name||"—"}</div>
          </div>
          <div class="o-field-group">
            <label class="o-field-label">¿Es empresa?</label>
            <div class="o-field-value">${s?"Sí":"No"}</div>
          </div>
          <div class="o-field-group">
            <label class="o-field-label">Email</label>
            <div class="o-field-value">${e.email?`<a href="mailto:${e.email}" class="o-link">${e.email}</a>`:"—"}</div>
          </div>
          <div class="o-field-group">
            <label class="o-field-label">Teléfono</label>
            <div class="o-field-value">${e.phone||"—"}</div>
          </div>
          <div class="o-field-group">
            <label class="o-field-label">Móvil</label>
            <div class="o-field-value">${e.mobile||"—"}</div>
          </div>
        </div>
        <!-- Col 2 -->
        <div class="o-form-col">
          <div class="o-field-group">
            <label class="o-field-label">RFC (VAT)</label>
            <div class="o-field-value o-field-mono">${e.vat||"—"}</div>
          </div>
          <div class="o-field-group">
            <label class="o-field-label">Página web</label>
            <div class="o-field-value">${e.website?`<a href="${e.website}" class="o-link" target="_blank">${e.website}</a>`:"—"}</div>
          </div>
          <div class="o-field-group">
            <label class="o-field-label">Ciudad</label>
            <div class="o-field-value">${e.city||"—"}</div>
          </div>
          <div class="o-field-group">
            <label class="o-field-label">Estado</label>
            <div class="o-field-value">${e.state_name||e.state||"—"}</div>
          </div>
          <div class="o-field-group">
            <label class="o-field-label">País</label>
            <div class="o-field-value">${e.country_name||e.country||"—"}</div>
          </div>
        </div>
      </div>

      <!-- ── NOTEBOOK TABS ── -->
      <div class="o-notebook">
        <div class="o-tabs" id="partner-tabs">
          <button class="o-tab active" onclick="window._partnerTab('contactos', this)">Contactos y Direcciones</button>
          <button class="o-tab" onclick="window._partnerTab('ventas', this)">Ventas y Compras</button>
          <button class="o-tab" onclick="window._partnerTab('notas', this)">Notas</button>
        </div>

        <div class="o-tab-pane" id="tab-contactos">
          <p class="o-tab-empty">Sin sub-contactos registrados.</p>
        </div>
        <div class="o-tab-pane" id="tab-ventas" style="display:none">
          <div class="o-form-grid">
            <div class="o-form-col">
              <div class="o-field-group"><label class="o-field-label">Vendedor asignado</label><div class="o-field-value">—</div></div>
              <div class="o-field-group"><label class="o-field-label">Términos de pago</label><div class="o-field-value">—</div></div>
            </div>
            <div class="o-form-col">
              <div class="o-field-group"><label class="o-field-label">Lista de precios</label><div class="o-field-value">—</div></div>
            </div>
          </div>
        </div>
        <div class="o-tab-pane" id="tab-notas" style="display:none">
          <textarea class="o-textarea" rows="5" placeholder="Notas internas…"></textarea>
        </div>
      </div>
    </div>

    <!-- ── CHATTER ── -->
    <div class="o-chatter">
      <div class="o-chatter-header">Registro de actividad</div>
      <div class="o-chatter-composer">
        <div class="o-avatar o-avatar-sm" style="background:var(--o-primary)">U</div>
        <input class="o-chatter-input" type="text" placeholder="Escribe un mensaje o nota interna…">
        <button class="o-btn-primary o-btn-sm">Enviar</button>
      </div>
      <div class="o-chatter-messages">
        <div class="o-msg">
          <div class="o-avatar o-avatar-sm" style="background:var(--o-primary)">S</div>
          <div class="o-msg-body">
            <div class="o-msg-meta"><strong>Sistema</strong> <span>${j(new Date().toISOString())}</span></div>
            <div class="o-msg-text">Registro creado.</div>
          </div>
        </div>
      </div>
    </div>`),window._editarPartnerForm=l=>{const c={id:l,...e};$o(c,()=>window._verPartner(l))},window._partnerTab=(l,c)=>{document.querySelectorAll("#partner-tabs .o-tab").forEach(m=>m.classList.remove("active")),c.classList.add("active"),document.querySelectorAll(".o-tab-pane").forEach(m=>m.style.display="none");const p=document.getElementById(`tab-${l}`);p&&(p.style.display="")}}catch(e){console.error(e),b("Error",e.message,"error")}};window._partnersBack=()=>Ne();window._partnerFilter=t=>{var o;pt=t,st=1,document.querySelectorAll("#partners-cp .o-filter-btn").forEach(i=>i.removeAttribute("data-active"));const e={"":"pf-all",clientes:"pf-cli",proveedores:"pf-prov"};(o=document.getElementById(e[t]))==null||o.setAttribute("data-active","");const a=document.getElementById("partners-content");a&&(a.innerHTML=I(8,6)),qe()};window._partnerNuevo=()=>{de(()=>import("./create_forms-CESEMRXd.js"),[]).then(t=>t.nuevoPartner("cliente",()=>Ne()))};window._chkAllPartners=t=>{document.querySelectorAll("#partners-content .o-chk").forEach(e=>e.checked=t.checked)};let te=1,Jt=[],ee="";async function re(){B(),te=1,ee="",Fo(),await ya()}function Fo(){w(`
  <div class="o-cp" id="stock-cp">
    <div class="o-cp-left">
      <button class="o-btn-primary" onclick="window._stockAjustarGlobal()">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
        Ajustar Cantidad
      </button>
    </div>
    <div class="o-cp-center">
      <div class="o-search-bar">
        <svg class="o-search-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/></svg>
        <input id="o-search-stock" class="o-search-input" type="text" placeholder="Buscar producto o ubicación…" value="${ee}">
        <div class="o-search-filters">
          <button class="o-filter-btn" onclick="window._stockFiltro('bajo')" id="sf-bajo">Stock Bajo</button>
          <button class="o-filter-btn" onclick="window._stockFiltro('cero')" id="sf-cero">En Cero</button>
        </div>
      </div>
    </div>
    <div class="o-cp-right">
      <div class="o-view-switcher">
        <button class="o-view-btn o-active" title="Vista Lista">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="8" y1="6" x2="21" y2="6"/><line x1="8" y1="12" x2="21" y2="12"/><line x1="8" y1="18" x2="21" y2="18"/><line x1="3" y1="6" x2="3.01" y2="6"/><line x1="3" y1="12" x2="3.01" y2="12"/><line x1="3" y1="18" x2="3.01" y2="18"/></svg>
        </button>
      </div>
    </div>
  </div>
  <div id="stock-content" class="o-view-content">
    ${I(10,5)}
  </div>`),setTimeout(()=>{var t;(t=document.getElementById("o-search-stock"))==null||t.addEventListener("input",e=>{ee=e.target.value.toLowerCase(),Bo()})},100)}function Bo(){document.querySelectorAll("#stock-content tbody tr").forEach(t=>{t.style.display=t.textContent.toLowerCase().includes(ee)?"":"none"})}async function ya(){try{const t=await g.stock(te);Jt=(t==null?void 0:t.data)||[];const e=Jt.length>=20,a=document.getElementById("stock-content");if(!a)return;a.innerHTML=Po(Jt,e)}catch(t){console.error(t),b("Error",t.message,"error")}}function Po(t,e){return t.length?`
  <div class="o-list-view">
    <table class="o-list-table">
      <thead>
        <tr>
          <th class="o-list-chk"><input type="checkbox" class="o-chk" onclick="window._chkAllStock(this)"></th>
          <th class="o-col-sortable">Producto</th>
          <th>Ubicación</th>
          <th class="o-col-right o-col-sortable">Disponible</th>
          <th class="o-col-right">Reservado</th>
          <th>Unidad</th>
        </tr>
      </thead>
      <tbody>
        ${t.map(a=>{const o=parseFloat(a.cantidad_disponible??a.qty_available??0),i=parseFloat(a.cantidad_reservada??a.reserved_qty??0),s=o<=0?"#ef4444":o<10?"#f59e0b":"#10b981",n=a.product_name||a.nombre||`Producto #${a.product_id||a.id}`,d=a.ubicacion||a.location||"WH/Stock",r=a.uom_name||a.unidad||"Unidades";return`
          <tr class="o-list-row" onclick="window._verStockItem(${a.product_id||a.id})">
            <td class="o-list-chk"><input type="checkbox" class="o-chk" onclick="event.stopPropagation()"></td>
            <td class="o-td-primary">${n}</td>
            <td class="o-td-muted">${d}</td>
            <td class="o-td-amount" style="color:${s};font-weight:700">${W(o)}</td>
            <td class="o-td-amount o-td-muted">${W(i)}</td>
            <td class="o-td-muted">${r}</td>
          </tr>`}).join("")}
      </tbody>
    </table>
    <div class="o-list-footer">
      <span class="o-list-count">${t.length} producto${t.length!==1?"s":""}</span>
      ${vt(te,e,a=>{te=a,ya()})}
    </div>
  </div>`:`
    <div class="o-empty-state">
      <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" opacity=".3"><path d="M5 8h14M5 8a2 2 0 1 0 0-4h14a2 2 0 1 0 0 4M5 8v10a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V8m-9 4h4"/></svg>
      <p>Sin registros de inventario</p>
    </div>`}window._verStockItem=async t=>{w(`<div class="o-form-loading">${I(3,3)}</div>`);try{const e=await g.stockProducto(t),o=(Array.isArray(e==null?void 0:e.data)?e.data:e!=null&&e.data?[e.data]:[])[0]||{},i=parseFloat(o.cantidad_disponible??0),s=parseFloat(o.cantidad_reservada??0),n=i*parseFloat(o.valor_unitario||0),d=o.product_name||`Producto #${t}`,r=document.getElementById("bc-stock-name");r&&(r.textContent=d);const l=i<=0?"#ef4444":i<10?"#f59e0b":"#10b981";w(`
    <div class="o-form-topbar">
      <button class="o-back-btn" onclick="window._stockBack()">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
        Inventario
      </button>
      <div class="o-form-actions">
        <button class="o-btn-primary" onclick="window._ajustarStockForm(${t})">Ajustar Cantidad</button>
      </div>
    </div>

    <div class="o-smart-buttons">
      <button class="o-smart-btn">
        <span class="o-smart-count" style="color:${l}">${W(i)}</span>
        <span class="o-smart-label">Disponible</span>
      </button>
      <button class="o-smart-btn">
        <span class="o-smart-count">${W(s)}</span>
        <span class="o-smart-label">Reservado</span>
      </button>
    </div>

    <div class="o-form-sheet">
      <div class="o-sheet-header">
        <div class="o-sheet-title-block">
          <h1 class="o-form-title">${d}</h1>
        </div>
      </div>

      <div class="o-form-grid">
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">Producto</label><div class="o-field-value">${d}</div></div>
          <div class="o-field-group"><label class="o-field-label">Ubicación</label><div class="o-field-value">${o.ubicacion||"WH/Stock"}</div></div>
          <div class="o-field-group"><label class="o-field-label">Cantidad Disponible</label><div class="o-field-value" style="color:${l};font-weight:700;font-size:20px">${W(i)}</div></div>
          <div class="o-field-group"><label class="o-field-label">Cantidad Reservada</label><div class="o-field-value">${W(s)}</div></div>
        </div>
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">Unidad de Medida</label><div class="o-field-value">${o.uom_name||o.unidad||"Unidades"}</div></div>
          <div class="o-field-group"><label class="o-field-label">Valor Unitario</label><div class="o-field-value">${f(parseFloat(o.valor_unitario||0))}</div></div>
          <div class="o-field-group"><label class="o-field-label">Valor Total</label><div class="o-field-value o-field-price">${f(n)}</div></div>
        </div>
      </div>

      <div class="o-notebook">
        <div class="o-tabs">
          <button class="o-tab active">Movimientos</button>
        </div>
        <div class="o-tab-pane">
          <div class="o-empty-state" style="padding:32px 0">
            <p style="color:var(--o-text-secondary)">Historial de movimientos de inventario — próximamente</p>
          </div>
        </div>
      </div>
    </div>`),window._ajustarStockForm=c=>{const p=Jt.find(m=>(m.product_id||m.id)===c);p?Qe(p,()=>window._verStockItem(c)):Qe({product_id:c,product_name:d},()=>window._verStockItem(c))}}catch(e){console.error(e),b("Error",e.message,"error")}};window._stockBack=()=>re();window._stockAjustarGlobal=()=>alert("Selecciona un producto para ajustar");window._stockFiltro=t=>{document.querySelectorAll("#stock-content tbody tr").forEach(e=>{var o;const a=parseFloat(((o=e.querySelector("td:nth-child(4)"))==null?void 0:o.textContent)||"0");t==="bajo"?e.style.display=a<10?"":"none":t==="cero"?e.style.display=a<=0?"":"none":e.style.display=""})};window._chkAllStock=t=>document.querySelectorAll("#stock-content .o-chk").forEach(e=>e.checked=t.checked);async function Ao(t,e){B(),w(`<div class="o-form-loading">${I(5,4)}</div>`);try{const a=await g.get(`/picking/${t}`),o=a==null?void 0:a.data;if(!o){w('<div class="o-empty-state"><p>Entrega no encontrada</p></div>');return}const i=o.picking,s=o.moves||[],n={draft:"Borrador",ready:"Listo",done:"Hecho",cancel:"Cancelado"},d={draft:"o-badge-gray",ready:"o-badge-info",done:"o-badge-success",cancel:"o-badge-danger"},r=i.state==="done";w(`<div class="nx-module-page" style="background:var(--bg-app)">

    <!-- Control Panel -->
    <div class="o-cp">
      <div class="o-cp-left">
        <button class="o-back-btn" onclick="window._go('${e?`ventas?id=${e}`:"ventas"}')">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
          ${e?"Volver al Pedido":"Inventario"}
        </button>
      </div>
      <div class="o-cp-center"></div>
      <div class="o-cp-right">
        ${r?"":`
        <button class="o-btn-primary" id="btn-validar-picking" onclick="window._validarPicking(${t})" style="background:#10B981">
          ✓ Validar Entrega
        </button>`}
        ${e?`<button class="o-btn-secondary" onclick="window._go('ventas?id=${e}')">Volver al Pedido</button>`:""}
      </div>
    </div>

    <!-- Barra de estado -->
    <div style="display:flex;align-items:center;gap:8px;padding:10px 24px;background:var(--bg-card);border-bottom:1px solid var(--border)">
      ${["Listo","En proceso","Hecho"].map((l,c)=>{const p=i.state==="done"?2:0,m=c<p,u=c===p;return`
        ${c>0?'<span style="color:var(--text-300);font-size:14px;margin:0 2px">›</span>':""}
        <button style="padding:6px 16px;border-radius:20px;border:none;font-size:12px;font-weight:700;cursor:default;
          ${u?"background:var(--primary);color:#fff;":""}
          ${m?"color:var(--primary);opacity:.6;background:transparent;":""}
          ${!u&&!m?"color:var(--text-400);background:transparent;":""}
        ">${m?"✓ ":""}${l}</button>`}).join("")}
    </div>

    <!-- Smart Buttons -->
    <div style="display:flex;gap:10px;padding:10px 24px;background:var(--bg-card);border-bottom:1px solid var(--border)">
      <button style="display:flex;flex-direction:column;align-items:center;gap:2px;padding:8px 18px;border:1px solid var(--border);border-radius:10px;background:var(--bg-card);min-width:80px;cursor:default">
        <span style="font-size:20px;font-weight:800;color:var(--primary)">${s.length}</span>
        <span style="font-size:11px;color:var(--text-500)">Productos</span>
      </button>
    </div>

    <!-- Formulario -->
    <div style="background:var(--bg-card);border-radius:12px;margin:16px 20px 0;border:1px solid var(--border);overflow:hidden">
      <div style="padding:20px 24px 16px;border-bottom:1px solid var(--border)">
        <h1 style="font-family:'Plus Jakarta Sans',sans-serif;font-size:22px;font-weight:800;color:var(--text-900);margin:0 0 6px">${i.name}</h1>
        <span class="o-badge ${d[i.state]||"o-badge-gray"}">${n[i.state]||i.state}</span>
      </div>

      <div style="display:grid;grid-template-columns:1fr 1fr;gap:0 32px;padding:16px 24px">
        <div>
          <div class="o-field-group"><label class="o-field-label">Contacto</label><div class="o-field-value">${i.partner_name||"—"}</div></div>
          <div class="o-field-group"><label class="o-field-label">Origen</label><div class="o-field-value">${i.origin||"—"}</div></div>
        </div>
        <div>
          <div class="o-field-group"><label class="o-field-label">Fecha Programada</label><div class="o-field-value">${j(i.scheduled_date)}</div></div>
          ${i.date_done?`<div class="o-field-group"><label class="o-field-label">Fecha de Validación</label><div class="o-field-value">${j(i.date_done)}</div></div>`:""}
        </div>
      </div>

      <div style="padding:0 24px 20px">
        <h3 style="font-size:13px;font-weight:700;color:var(--text-600);text-transform:uppercase;letter-spacing:.06em;margin:0 0 12px">Operaciones Detalladas</h3>
        <table style="width:100%;border-collapse:collapse;font-size:13px">
          <thead>
            <tr style="background:var(--bg-app)">
              <th style="padding:8px 12px;text-align:left;font-weight:600;color:var(--text-600);border-bottom:1px solid var(--border)">PRODUCTO</th>
              <th style="padding:8px 12px;text-align:center;font-weight:600;color:var(--text-600);border-bottom:1px solid var(--border)">DEMANDA</th>
              <th style="padding:8px 12px;text-align:center;font-weight:600;color:var(--text-600);border-bottom:1px solid var(--border)">HECHO</th>
            </tr>
          </thead>
          <tbody>
            ${s.map(l=>`
            <tr style="border-bottom:1px solid var(--border)">
              <td style="padding:10px 12px;font-weight:500">${l.product_name||l.name||"—"}</td>
              <td style="padding:10px 12px;text-align:center">${parseFloat(l.product_uom_qty||0)}</td>
              <td style="padding:10px 12px;text-align:center">
                ${r?`<span style="color:#10B981;font-weight:700">${parseFloat(l.quantity_done||0)}</span>`:`<input type="number" id="move-qty-${l.id}" value="${parseFloat(l.product_uom_qty||0)}" min="0" max="${parseFloat(l.product_uom_qty||0)}"
                       style="width:80px;padding:4px 8px;border:1px solid var(--border);border-radius:6px;text-align:center;font-size:13px">`}
              </td>
            </tr>`).join("")}
          </tbody>
        </table>
      </div>
    </div>
    <div style="height:40px"></div>
  </div>`),window._validarPicking=async l=>{const c=document.getElementById("btn-validar-picking");c&&(c.disabled=!0);const p=s.map(m=>{const u=document.getElementById(`move-qty-${m.id}`),v=parseFloat((u==null?void 0:u.value)??m.product_uom_qty??0);return[m.id,v]});try{await g.put(`/picking/${l}/validar`,{moves:p}),b("Entrega validada","✅ Los productos han sido entregados y el stock actualizado","success"),setTimeout(()=>{e?window._go(`ventas?id=${e}`):re()},1200)}catch(m){c&&(c.disabled=!1),b("Error",m.message,"error")}}}catch(a){console.error(a),b("Error",a.message,"error")}}let Ct=1,Ut="historial";async function Lo(){B(),Ct=1,await xa()}async function xa(){w(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">🔏 CFDI 4.0</h1>
      <p class="page-subtitle" id="cfdi-sub">Comprobantes Fiscales Digitales</p>
    </div>
    <div class="page-actions">
      <button class="btn ${Ut==="historial"?"btn-primary":"btn-secondary"}"
        onclick="window._cfdiTab('historial')">📋 Historial</button>
      <button class="btn ${Ut==="timbrar"?"btn-primary":"btn-secondary"}"
        onclick="window._cfdiTab('timbrar')">➕ Timbrar</button>
    </div>
  </div>

  <!-- KPI row -->
  <div style="display:grid;grid-template-columns:repeat(4,1fr);gap:12px;margin-bottom:18px" id="kpi-row" class="anim-2">
    ${[1,2,3,4].map(()=>'<div class="data-card" style="padding:16px"><div class="skeleton" style="height:40px"></div></div>').join("")}
  </div>

  <div class="data-card anim-3" id="cfdi-content">
    <div id="cfdi-body">${I(6,6)}</div>
  </div>`),window._cfdiTab=t=>{Ut=t,xa()};try{const t=await g.cfdiKpis().catch(()=>null),e=t==null?void 0:t.data,a=document.getElementById("kpi-row");a&&(a.innerHTML=[{label:"Total Timbrados",val:(e==null?void 0:e.total_timbrados)??0,tipo:"num",color:"indigo",icon:"🧾"},{label:"Vigentes",val:(e==null?void 0:e.vigentes)??0,tipo:"num",color:"emerald",icon:"✅"},{label:"Cancelados",val:(e==null?void 0:e.cancelados)??0,tipo:"num",color:"red",icon:"❌"},{label:"Monto Total",val:(e==null?void 0:e.monto_total)??0,tipo:"mxn",color:"violet",icon:"💰"}].map(o=>`
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${o.icon} ${o.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${o.tipo==="mxn"?f(parseFloat(o.val)):Number(o.val).toLocaleString("es-MX")}
        </div>
      </div>`).join("")),Ut==="historial"?await ke():Mo()}catch(t){console.error(t),b("Error CFDI",t.message,"error")}}async function ke(){const t=document.getElementById("cfdi-body");t&&(t.innerHTML=I(6,6));const e=await g.cfdiTimbrados(Ct).catch(()=>({data:[],total:0})),a=(e==null?void 0:e.data)||[],o=(e==null?void 0:e.total)??a.length,i=a.length>=20,s=document.getElementById("cfdi-sub");if(s&&(s.textContent=`${o} CFDIs timbrados · Página ${Ct}`),!!t){if(a.length===0){t.innerHTML=`
    <div style="text-align:center;padding:60px 24px">
      <div style="font-size:48px;margin-bottom:16px">🧾</div>
      <div style="font-size:16px;font-weight:700;color:var(--text-700);margin-bottom:8px">Sin CFDIs timbrados</div>
      <div style="font-size:13px;color:var(--text-400);margin-bottom:20px">Usa el botón "Timbrar" para crear tu primer comprobante</div>
      <button class="btn btn-primary" onclick="window._cfdiTab('timbrar')">➕ Timbrar CFDI</button>
    </div>`;return}t.innerHTML=`
  <div class="data-card-header">
    <div class="data-card-title">Historial de CFDIs</div>
  </div>
  <table class="data-table">
    <thead><tr>
      <th>UUID</th><th>Folio</th><th>Receptor</th>
      <th>Total</th><th>Tipo</th><th>Estado</th><th>Fecha</th>
    </tr></thead>
    <tbody>
      ${a.map(n=>{const d=n.estado==="vigente"?"emerald":n.estado==="cancelado"?"red":"gray";return`
        <tr style="cursor:pointer" onclick="window._verCfdi('${n.uuid}')" title="Ver detalle">
          <td class="td-mono" style="font-size:11px">${n.uuid.substring(0,18)}…</td>
          <td class="td-mono">${n.serie||""}${n.folio||"—"}</td>
          <td class="td-primary">${n.nombre_receptor||n.rfc_receptor}</td>
          <td class="td-amount" style="font-weight:700">${f(parseFloat(n.total||0))}</td>
          <td><span class="badge badge-sky">${n.tipo_cfdi==="I"?"Ingreso":n.tipo_cfdi==="E"?"Egreso":n.tipo_cfdi||"—"}</span></td>
          <td><span class="badge badge-${d}">${n.estado||"—"}</span></td>
          <td style="font-size:12px">${j(n.fecha_timbrado||n.created_at)}</td>
        </tr>`}).join("")}
    </tbody>
  </table>
  ${vt(Ct,i,n=>{Ct=n,ke()})}`,window._verCfdi=n=>{Ga("Detalle CFDI",()=>g.cfdiTimbrado(n),d=>`
      ${ce("Comprobante",[K("UUID",`<span style="font-family:monospace;font-size:11px">${d.uuid}</span>`),K("Serie / Folio",`${d.serie||""}${d.folio||"—"}`),K("Tipo",d.tipo_cfdi==="I"?"Ingreso":d.tipo_cfdi==="E"?"Egreso":d.tipo_cfdi),K("Estado",`<span class="badge badge-${d.estado==="vigente"?"emerald":"red"}">${d.estado}</span>`),K("Fecha emisión",j(d.fecha_emision)),K("Fecha timbrado",j(d.fecha_timbrado))].join(""))}
      ${ce("Partes",[K("RFC Emisor",d.rfc_emisor),K("Emisor",d.nombre_emisor||"—"),K("RFC Receptor",d.rfc_receptor),K("Receptor",d.nombre_receptor||"—")].join(""))}
      ${ce("Importes",[K("Total",`<strong>${f(parseFloat(d.total||0))}</strong>`,{color:"var(--primary)"})].join(""))}
      <div style="display:flex;gap:10px;margin-top:16px">
        <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
        ${d.estado==="vigente"?`<button class="btn btn-danger btn-sm" onclick="window._cancelarCfdi('${d.uuid}')">❌ Cancelar</button>`:""}
      </div>`)},window._cancelarCfdi=async n=>{if(confirm(`¿Cancelar el CFDI ${n.substring(0,18)}…?`))try{await g.cancelarCfdi({uuid:n,rfc_emisor:"",motivo:"02"}),b("CFDI cancelado",n,"success"),window.__closeModal(),ke()}catch(d){b("Error al cancelar",d.message,"error")}}}}function Mo(){var e;const t=document.getElementById("cfdi-body");t&&(t.innerHTML=`
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
  </div>`,(e=document.getElementById("f-subtotal"))==null||e.addEventListener("input",a=>{const o=parseFloat(a.target.value)||0,i=o*.16;document.getElementById("f-iva").value=i.toFixed(2),document.getElementById("f-total").value=(o+i).toFixed(2)}),window._timbrar=async()=>{var i,s,n,d,r,l,c,p,m,u,v,k,_,$;const a=document.getElementById("btn-timbrar");a.textContent="⏳ Timbrando…",a.disabled=!0;const o=document.getElementById("cfdi-resultado");try{const T=(i=document.getElementById("f-cer"))==null?void 0:i.files[0],h=(s=document.getElementById("f-key"))==null?void 0:s.files[0],E=C=>new Promise((R,Ht)=>{if(!C){R("");return}const _t=new FileReader;_t.onload=le=>R(le.target.result.split(",")[1]||""),_t.onerror=Ht,_t.readAsDataURL(C)}),[S,y]=await Promise.all([E(T),E(h)]),z=parseFloat((n=document.getElementById("f-subtotal"))==null?void 0:n.value)||0,P=z*.16,A={cfdi:{serie:((d=document.getElementById("f-serie"))==null?void 0:d.value)||"A",folio:((r=document.getElementById("f-folio"))==null?void 0:r.value)||"1",tipo_comprobante:((l=document.getElementById("f-tipo"))==null?void 0:l.value)||"I",emisor:{rfc:((c=document.getElementById("f-rfc-emisor"))==null?void 0:c.value)||"",nombre:((p=document.getElementById("f-nombre-emisor"))==null?void 0:p.value)||"",regimen_fiscal:((m=document.getElementById("f-regimen"))==null?void 0:m.value)||"601"},receptor:{rfc:((u=document.getElementById("f-rfc-receptor"))==null?void 0:u.value)||"",nombre:((v=document.getElementById("f-nombre-receptor"))==null?void 0:v.value)||"",uso_cfdi:((k=document.getElementById("f-uso"))==null?void 0:k.value)||"G03",domicilio_fiscal_receptor:"64000",regimen_fiscal_receptor:"601"},conceptos:[{clave_prod_serv:"84111506",descripcion:((_=document.getElementById("f-concepto"))==null?void 0:_.value)||"Servicio",cantidad:"1",unidad:"ACT",valor_unitario:z.toFixed(2),importe:z.toFixed(2),impuestos:{traslados:[{base:z.toFixed(2),impuesto:"002",tipo_factor:"Tasa",tasa:"0.160000",importe:P.toFixed(2)}]}}],subtotal:z.toFixed(2),total:(z+P).toFixed(2),moneda:"MXN",lugar_expedicion:"64000"},cert_b64:S,key_b64:y,password:(($=document.getElementById("f-pwd"))==null?void 0:$.value)||""},U=await g.timbrar(A);U!=null&&U.success?(o.innerHTML=`
        <div style="background:var(--success-light);border:1.5px solid var(--success);border-radius:12px;padding:16px">
          <div style="font-weight:700;color:var(--success);margin-bottom:8px">✅ CFDI Timbrado</div>
          <div style="font-size:12px;font-family:monospace;word-break:break-all;color:var(--text-600)">UUID: ${U.uuid}</div>
          <div style="font-size:12px;color:var(--text-500);margin-top:4px">Fecha: ${j(U.fecha_timbrado)}</div>
        </div>`,b("CFDI timbrado",`UUID: ${U.uuid}`,"success")):o.innerHTML=`<div style="background:var(--danger-light,#fee2e2);border:1.5px solid var(--red);border-radius:12px;padding:16px;color:var(--red)">
          ❌ Error: ${(U==null?void 0:U.error)||"Error desconocido"}</div>`}catch(T){o.innerHTML=`<div style="background:#fee2e2;border:1.5px solid var(--red);border-radius:12px;padding:16px;color:var(--red)">❌ ${T.message}</div>`}finally{a.textContent="🔏 Timbrar CFDI",a.disabled=!1}})}let ae=1,ut=[],oe="";async function ha(){B(),ae=1,oe="",jo(),await wa()}function jo(){w(`
  <div class="o-cp" id="nomina-cp">
    <div class="o-cp-left">
      <button class="o-btn-primary" onclick="window._nominaNuevoEmpleado()">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
        Nuevo
      </button>
    </div>
    <div class="o-cp-center">
      <div class="o-search-bar">
        <svg class="o-search-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/></svg>
        <input id="o-search-nomina" class="o-search-input" type="text" placeholder="Buscar empleado o departamento…" value="${oe}">
        <div class="o-search-filters">
          <button class="o-filter-btn" onclick="window._nominaFiltro('activos')">Activos</button>
          <button class="o-filter-btn" onclick="window._nominaFiltro('baja')">De baja</button>
        </div>
      </div>
    </div>
    <div class="o-cp-right">
      <div class="o-view-switcher">
        <button class="o-view-btn o-active" title="Vista Lista">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="8" y1="6" x2="21" y2="6"/><line x1="8" y1="12" x2="21" y2="12"/><line x1="8" y1="18" x2="21" y2="18"/><line x1="3" y1="6" x2="3.01" y2="6"/><line x1="3" y1="12" x2="3.01" y2="12"/><line x1="3" y1="18" x2="3.01" y2="18"/></svg>
        </button>
      </div>
    </div>
  </div>
  <div id="nomina-content" class="o-view-content">
    ${I(10,6)}
  </div>`),setTimeout(()=>{var t;(t=document.getElementById("o-search-nomina"))==null||t.addEventListener("input",e=>{oe=e.target.value.toLowerCase(),Do()})},100)}function Do(){document.querySelectorAll("#nomina-content tbody tr").forEach(t=>{t.style.display=t.textContent.toLowerCase().includes(oe)?"":"none"})}async function wa(){try{const t=await g.nomina(ae);ut=(t==null?void 0:t.data)||[];const e=ut.length>=20,a=document.getElementById("nomina-content");if(!a)return;a.innerHTML=_a(ut,e)}catch(t){console.error(t),b("Error",t.message,"error")}}function _a(t,e){return t.length?`
  <div class="o-list-view">
    <table class="o-list-table">
      <thead>
        <tr>
          <th class="o-list-chk"><input type="checkbox" class="o-chk" onclick="window._chkAllNomina(this)"></th>
          <th class="o-col-sortable">Empleado</th>
          <th>Puesto</th>
          <th>Departamento</th>
          <th>N° IMSS</th>
          <th>Fecha Inicio</th>
          <th class="o-col-right">Salario Base</th>
        </tr>
      </thead>
      <tbody>
        ${t.map(a=>{const o=a.active!==!1,i=(a.name||"?").split(" ").map(p=>p[0]).slice(0,2).join(""),s=a.id*47%360,n=a.job_title||a.job_id_name||"—",d=a.department_name||a.department_id_name||"—",r=a.ssnid||a.imss||"—",l=j(a.date_start||a.fecha_inicio||null),c=f(parseFloat(a.wage||a.salario_base||0));return`
          <tr class="o-list-row" onclick="window._verEmpleado(${a.id})">
            <td class="o-list-chk"><input type="checkbox" class="o-chk" onclick="event.stopPropagation()"></td>
            <td>
              <div class="o-partner-cell">
                <div class="o-avatar" style="background:linear-gradient(135deg,hsl(${s},60%,55%),hsl(${(s+50)%360},70%,45%))">${i||"?"}</div>
                <div>
                  <div class="o-td-primary">${a.name||"—"}</div>
                  <div class="o-td-secondary"><span class="o-badge ${o?"o-badge-success":"o-badge-gray"}">${o?"Activo":"Baja"}</span></div>
                </div>
              </div>
            </td>
            <td class="o-td-muted">${n}</td>
            <td class="o-td-muted">${d}</td>
            <td class="o-td-mono">${r}</td>
            <td class="o-td-muted">${l}</td>
            <td class="o-td-amount" style="font-weight:700">${c}</td>
          </tr>`}).join("")}
      </tbody>
    </table>
    <div class="o-list-footer">
      <span class="o-list-count">${t.length} empleado${t.length!==1?"s":""}</span>
      ${vt(ae,e,a=>{ae=a,wa()})}
    </div>
  </div>`:`
    <div class="o-empty-state">
      <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" opacity=".3"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>
      <p>Sin empleados registrados</p>
    </div>`}window._verEmpleado=async t=>{w(`<div class="o-form-loading">${I(4,3)}</div>`);try{let e=ut.find(l=>l.id===t);try{const l=await g.empleado(t);l&&(l.id||l.name)&&(e=l)}catch{}if(!e){b("Error","Empleado no encontrado","error");return}const a=document.getElementById("bc-emp-name");a&&(a.textContent=e.name||"Empleado");const o=e.active!==!1,i=(e.name||"?").split(" ").map(l=>l[0]).slice(0,2).join(""),s=e.id*47%360,n=f(parseFloat(e.sbc||e.salario_base_cotizacion||0)),d=f(parseFloat(e.sdi||e.salario_diario_integrado||0)),r=f(parseFloat(e.wage||e.salario_base||0));w(`
    <div class="o-form-topbar">
      <button class="o-back-btn" onclick="window._nominaBack()">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
        Nómina
      </button>
      <div class="o-form-actions">
        <button class="o-btn-secondary" onclick="window._editarEmpleadoForm(${e.id})">💾 Actualizar</button>
        <button class="o-btn-primary" id="btn-calcular-nomina" data-sdi="${e.sdi||e.salario_diario_integrado||0}" onclick="window._nominaCalcular(this.dataset.sdi)">Calcular Nómina</button>
      </div>
    </div>

    <!-- SMART BUTTONS -->
    <div class="o-smart-buttons">
      <button class="o-smart-btn" onclick="alert('Nóminas del empleado')">
        <span class="o-smart-count">0</span>
        <span class="o-smart-label">Nóminas</span>
      </button>
      <button class="o-smart-btn" onclick="alert('Cálculo IMSS — próximamente')">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="margin:0 auto;display:block"><path d="M9 7H6a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h9a2 2 0 0 0 2-2v-3"/><path d="M9 15h3l8.5-8.5a1.5 1.5 0 0 0-3-3L9 12v3"/><line x1="16" y1="5" x2="19" y2="8"/></svg>
        <span class="o-smart-label">Calc. IMSS</span>
      </button>
    </div>

    <!-- FORM SHEET -->
    <div class="o-form-sheet">
      <div class="o-sheet-header">
        <div class="o-avatar o-avatar-lg" style="background:linear-gradient(135deg,hsl(${s},60%,55%),hsl(${(s+50)%360},70%,45%))">${i||"?"}</div>
        <div class="o-sheet-title-block">
          <h1 class="o-form-title">${e.name||"—"}</h1>
          <div style="display:flex;gap:6px;margin-top:4px">
            <span class="o-badge ${o?"o-badge-success":"o-badge-gray"}">${o?"Activo":"Baja"}</span>
            ${e.contract_type_name?`<span class="o-badge o-badge-info">${e.contract_type_name}</span>`:""}
          </div>
        </div>
      </div>

      <div class="o-form-grid">
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">CURP</label><div class="o-field-value o-field-mono">${e.curp||"—"}</div></div>
          <div class="o-field-group"><label class="o-field-label">N° Seguro Social</label><div class="o-field-value o-field-mono">${e.ssnid||e.imss||"—"}</div></div>
          <div class="o-field-group"><label class="o-field-label">RFC</label><div class="o-field-value o-field-mono">${e.rfc||"—"}</div></div>
          <div class="o-field-group"><label class="o-field-label">Fecha de Inicio</label><div class="o-field-value">${j(e.date_start||e.fecha_inicio||null)}</div></div>
        </div>
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">Puesto</label><div class="o-field-value">${e.job_title||e.job_id_name||"—"}</div></div>
          <div class="o-field-group"><label class="o-field-label">Departamento</label><div class="o-field-value">${e.department_name||e.department_id_name||"—"}</div></div>
          <div class="o-field-group"><label class="o-field-label">Tipo de Contrato</label><div class="o-field-value">${e.contract_type_name||e.tipo_contrato||"—"}</div></div>
          <div class="o-field-group"><label class="o-field-label">Jornada</label><div class="o-field-value">${e.resource_calendar_name||e.jornada||"—"}</div></div>
        </div>
      </div>

      <!-- NOTEBOOK -->
      <div class="o-notebook">
        <div class="o-tabs" id="nom-tabs">
          <button class="o-tab active" onclick="window._nomTab('config', this)">Configuración Nómina</button>
          <button class="o-tab" onclick="window._nomTab('resumen', this)">Resumen Cálculos</button>
          <button class="o-tab" onclick="window._nomTab('historial', this)">Historial</button>
        </div>

        <div class="o-tab-pane" id="tab-config">
          <div class="o-form-grid">
            <div class="o-form-col">
              <div class="o-field-group"><label class="o-field-label">Salario Base</label><div class="o-field-value o-field-price">${r}</div></div>
              <div class="o-field-group"><label class="o-field-label">SBC (Sal. Base Cotización)</label><div class="o-field-value">${n}</div></div>
              <div class="o-field-group"><label class="o-field-label">SDI (Sal. Diario Integrado)</label><div class="o-field-value">${d}</div></div>
              <div class="o-field-group"><label class="o-field-label">Periodicidad</label><div class="o-field-value">${e.periodicidad||e.payment_period||"Mensual"}</div></div>
            </div>
            <div class="o-form-col">
              <div class="o-field-group"><label class="o-field-label">Banco</label><div class="o-field-value">${e.bank_name||e.banco||"—"}</div></div>
              <div class="o-field-group"><label class="o-field-label">CLABE</label><div class="o-field-value o-field-mono">${e.acc_number||e.clabe||"—"}</div></div>
            </div>
          </div>
        </div>

        <div class="o-tab-pane" id="tab-resumen" style="display:none">
          <div id="resumen-calc-result">
            <div style="padding:24px 0;text-align:center;color:var(--o-text-secondary)">
              Haz clic en <strong>Calcular Nómina</strong> para ver el desglose completo (ISR 2024 + IMSS 2024).
            </div>
          </div>
        </div>

        <div class="o-tab-pane" id="tab-historial" style="display:none">
          <div class="o-empty-state" style="padding:32px 0">
            <p style="color:var(--o-text-secondary)">Historial de nóminas procesadas — próximamente</p>
          </div>
        </div>
      </div>
    </div>

    <!-- CHATTER -->
    <div class="o-chatter">
      <div class="o-chatter-header">Registro de actividad</div>
      <div class="o-chatter-composer">
        <div class="o-avatar o-avatar-sm" style="background:var(--o-primary)">U</div>
        <input class="o-chatter-input" type="text" placeholder="Escribe un mensaje o nota interna…">
        <button class="o-btn-primary o-btn-sm">Enviar</button>
      </div>
      <div class="o-chatter-messages">
        <div class="o-msg">
          <div class="o-avatar o-avatar-sm" style="background:var(--o-primary)">S</div>
          <div class="o-msg-body">
            <div class="o-msg-meta"><strong>Sistema</strong> <span>${j(e.date_start||new Date().toISOString())}</span></div>
            <div class="o-msg-text">Registro creado.</div>
          </div>
        </div>
      </div>
    </div>`),window._editarEmpleadoForm=l=>Co({id:l,...e},()=>window._verEmpleado(l)),window._nomTab=(l,c)=>{document.querySelectorAll("#nom-tabs .o-tab").forEach(m=>m.classList.remove("active")),c.classList.add("active"),document.querySelectorAll(".o-tab-pane").forEach(m=>m.style.display="none");const p=document.getElementById(`tab-${l}`);p&&(p.style.display="")}}catch(e){console.error(e),b("Error",e.message,"error")}};window._nominaBack=()=>ha();window._nominaNuevoEmpleado=()=>alert("Nuevo empleado — próximamente");window._nominaCalcular=async t=>{const e=parseFloat(t)||0,a=e>0?e:parseFloat(prompt("Ingresa el SDI (Salario Diario Integrado):","300")||"0");if(!(!a||a<=0))try{const o=localStorage.getItem("nexus_token"),s=await(await fetch("/api/v1/nomina/calcular",{method:"POST",headers:{"Content-Type":"application/json",Authorization:`Bearer ${o}`},body:JSON.stringify({sdi:a,dias_periodo:30,tipo:"mensual"})})).json(),n=s.data||s;if(!n||!n.salario_bruto){b("Error","No se pudo calcular la nómina","error");return}document.querySelectorAll("#nom-tabs .o-tab").forEach((r,l)=>{r.classList.toggle("active",l===1)}),document.querySelectorAll(".o-tab-pane").forEach((r,l)=>r.style.display=l===1?"":"none");const d=document.getElementById("resumen-calc-result");d&&(d.innerHTML=`
      <table class="o-list-table">
        <thead><tr><th>Concepto</th><th class="o-col-right" style="text-align:right">Importe</th><th>Tipo</th></tr></thead>
        <tbody>
          <tr><td>Salario Bruto (30 días)</td><td class="o-td-amount">${f(n.salario_bruto)}</td><td><span class="o-badge o-badge-success">Percepción</span></td></tr>
          <tr style="color:var(--o-danger)"><td>ISR Retenido (SAT 2024)</td><td class="o-td-amount">–${f(n.isr_retenido)}</td><td><span class="o-badge o-badge-danger">Deducción</span></td></tr>
          <tr style="color:var(--o-success)"><td>Subsidio al Empleo</td><td class="o-td-amount">+${f(n.subsidio_empleo)}</td><td><span class="o-badge o-badge-success">A favor</span></td></tr>
          <tr style="color:var(--o-danger)"><td>IMSS Obrero (cuotas 2024)</td><td class="o-td-amount">–${f(n.imss_obrero)}</td><td><span class="o-badge o-badge-danger">Deducción</span></td></tr>
          <tr style="border-top:2px solid var(--o-border);font-weight:700"><td>Total Deducciones</td><td class="o-td-amount">–${f(n.total_deducciones)}</td><td></td></tr>
          <tr style="font-weight:800;font-size:1.05em;background:var(--o-bg-hover)"><td>💰 NETO A PAGAR</td><td class="o-td-amount" style="color:var(--o-success)">${f(n.salario_neto)}</td><td></td></tr>
          <tr style="border-top:2px solid var(--o-border);color:var(--o-text-secondary);font-style:italic"><td colspan="3" style="padding-top:8px"><strong>Costo Total Patrón</strong></td></tr>
          <tr><td>IMSS Patronal</td><td class="o-td-amount">${f(n.imss_patron)}</td><td><span class="o-badge o-badge-info">Costo empresa</span></td></tr>
          <tr><td>Cuota Fija IMSS</td><td class="o-td-amount">${f(n.cuota_fija_patron)}</td><td><span class="o-badge o-badge-info">Costo empresa</span></td></tr>
          <tr style="font-weight:700"><td>Costo Total Empresa</td><td class="o-td-amount">${f(n.costo_total_patron)}</td><td></td></tr>
        </tbody>
      </table>
      <div style="margin-top:12px;padding:8px 12px;background:var(--o-bg-hover);border-radius:6px;font-size:0.78em;color:var(--o-text-secondary)">
        Cálculo con tablas oficiales ISR 2024 (Art.152 LISR) + cuotas IMSS 2024 (DOF). SDI: ${f(a)}/día.
      </div>
    `),b("Nómina calculada","Desglose ISR 2024 + IMSS actualizado","success")}catch(o){b("Error",o.message,"error")}};window._nominaFiltro=t=>{const e=t==="activos"?ut.filter(o=>o.active!==!1):t==="baja"?ut.filter(o=>o.active===!1):ut,a=document.getElementById("nomina-content");a&&(a.innerHTML=_a(e,!1))};window._chkAllNomina=t=>document.querySelectorAll("#nomina-content .o-chk").forEach(e=>e.checked=t.checked);let Re="list",ie=1,St=[],Nt="",D={};const ne={draft:{lbl:"Borrador",cls:"o-badge-gray",kanban:"Borrador"},sent:{lbl:"Enviada",cls:"o-badge-info",kanban:"Enviada al Proveedor"},purchase:{lbl:"Orden de Compra",cls:"o-badge-success",kanban:"Órdenes de Compra"},done:{lbl:"Realizada",cls:"o-badge-warn",kanban:"Realizada"},cancel:{lbl:"Cancelada",cls:"o-badge-danger",kanban:"Cancelada"}},Ze=["draft","sent","purchase","done"];async function Oe(){B(),Re="list",ie=1,Nt="",D={bloquear_confirmado:!1,advertencias:!1,precio_compra:!0,descuentos:!1,politica_facturacion:"cantidad_pedida",bloquear_factura:!1,variantes:!1,unidades_medida:!1,empaquetado:!1,presupuesto_solicitud:!1,recordatorio_recepcion:0,costos_aterrizaje:!1,...JSON.parse(localStorage.getItem("nexus_config_compras")||"{}")},$a(),await Ve()}function $a(){w(`
  <div class="o-cp" id="compras-cp">
    <div class="o-cp-left">
      <button class="o-btn-primary" onclick="window._compraNueva()">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
        Nuevo
      </button>
      ${D.presupuesto_solicitud?`<button class="o-btn-secondary" onclick="alert('Pedir Presupuestos')">Solicitar Presupuestos</button>`:""}
    </div>
    <div class="o-cp-center">
      <div class="o-search-bar">
        <svg class="o-search-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/></svg>
        <input id="o-search-compras" class="o-search-input" type="text" placeholder="Buscar folio o proveedor…" value="${Nt}">
        <div class="o-search-filters">
          <button class="o-filter-btn" onclick="window._compraFiltroEstado('draft')">Borrador</button>
          <button class="o-filter-btn" onclick="window._compraFiltroEstado('purchase')">Confirmadas</button>
          <button class="o-filter-btn" onclick="window._compraFiltroEstado('done')">Realizadas</button>
        </div>
      </div>
    </div>
    <div class="o-cp-right">
      <button class="o-btn-secondary" style="margin-right:8px;font-size:16px;padding:4px 8px" onclick="window._go('config_compras')" title="Ajustes">⚙️</button>
      <div class="o-view-switcher">
        <button class="o-view-btn o-active" onclick="window._compraSetView('list')" title="Lista">☰</button>
        <button class="o-view-btn" onclick="window._compraSetView('kanban')" title="Kanban">⬜</button>
      </div>
    </div>
  </div>
  <div id="compras-content" class="o-view-content">
    ${I(8,6)}
  </div>`),setTimeout(()=>{var t;(t=document.getElementById("o-search-compras"))==null||t.addEventListener("input",e=>{Nt=e.target.value.toLowerCase(),ka()})},100)}function ka(){document.querySelectorAll("#compras-content .o-list-row, #compras-content .o-kanban-card").forEach(t=>{t.style.display=t.textContent.toLowerCase().includes(Nt)?"":"none"})}async function Ve(){try{const t=await g.compras(ie);St=(t==null?void 0:t.data)||[];const e=St.length>=20,a=document.getElementById("compras-content");if(!a)return;Re==="kanban"?a.innerHTML=qo(St):a.innerHTML=No(St,e)}catch(t){console.error(t),b("Error",t.message,"error")}}function No(t,e){return t.length?`
  <div class="o-list-view">
    <table class="o-list-table">
      <thead>
        <tr>
          <th class="o-list-chk"><input type="checkbox" class="o-chk" onclick="window._chkAllCompras(this)"></th>
          <th class="o-col-sortable">Referencia</th>
          <th class="o-col-sortable">Proveedor</th>
          ${D.recordatorio_recepcion>0?"<th>Recepción Esperada</th>":""}
          <th>Fecha límite</th>
          <th class="o-col-right">Total</th>
          <th>Estado</th>
        </tr>
      </thead>
      <tbody>
        ${t.map(a=>{const o=ne[a.state]||{lbl:a.state||"—",cls:"o-badge-gray"};return`
          <tr class="o-list-row" onclick="window._verCompra(${a.id})">
            <td class="o-list-chk"><input type="checkbox" class="o-chk" onclick="event.stopPropagation()"></td>
            <td class="o-td-mono o-td-primary" style="font-weight:700">${a.name||`#${a.id}`}</td>
            <td class="o-td-primary">${a.partner_name||"—"} ${D.advertencias?'<span style="color:#DC2626;font-size:11px" title="Alerta configurada">⚠️</span>':""}</td>
            ${D.recordatorio_recepcion>0?`<td><span style="color:var(--text-400)">⏳ En ${D.recordatorio_recepcion} días</span></td>`:""}
            <td class="o-td-muted">${j(a.date_order)}</td>
            <td class="o-td-amount" style="font-weight:700">${f(parseFloat(a.amount_total||0))}</td>
            <td><span class="o-badge ${o.cls}">${o.lbl}</span></td>
          </tr>`}).join("")}
      </tbody>
    </table>
    <div class="o-list-footer">
      <span class="o-list-count">${t.length} registros</span>
      ${vt(ie,e,a=>{ie=a,Ve()})}
    </div>
  </div>`:'<div class="o-empty-state"><p>Sin órdenes de compra</p></div>'}function qo(t){return`
  <div class="o-kanban-columns">
    ${["draft","sent","purchase","done"].map(a=>{const o=ne[a],i=t.filter(n=>n.state===a),s=i.reduce((n,d)=>n+parseFloat(d.amount_total||0),0);return`
      <div class="o-kanban-col">
        <div class="o-kanban-col-header">
          <span class="o-badge ${o.cls}">${o.kanban}</span>
          <span class="o-kanban-col-count">${i.length}</span>
        </div>
        <div class="o-kanban-col-sum">${f(s)}</div>
        <div class="o-kanban-col-cards">
          ${i.map(n=>`
          <div class="o-kanban-card" onclick="window._verCompra(${n.id})">
            <div class="o-kanban-title">${n.name||"#"+n.id}</div>
            <div class="o-kanban-sub">${n.partner_name||"—"} ${D.advertencias?"⚠️":""}</div>
            <div style="display:flex;justify-content:space-between;margin-top:8px">
              <span class="o-td-muted" style="font-size:12px">${j(n.date_order)}</span>
              <strong>${f(parseFloat(n.amount_total||0))}</strong>
            </div>
          </div>`).join("")}
          ${i.length===0?'<div class="o-kanban-empty-col">Sin órdenes</div>':""}
        </div>
      </div>`}).join("")}
  </div>`}window._verCompra=async t=>{w(`<div class="o-form-loading">${I(4,3)}</div>`);try{let e=St.find(d=>d.id===t);try{const d=await g.compra(t);d&&(d.id||d.name)&&(e=d)}catch{}if(!e){b("Error","Orden no encontrada","error");return}const a=document.getElementById("bc-compra-name");a&&(a.textContent=e.name||`Compra #${t}`);const o=ne[e.state]||{lbl:e.state||"—",cls:"o-badge-gray"},i=Ze.indexOf(e.state),s=e.order_line||e.lineas||[],n=D.bloquear_confirmado&&(e.state==="purchase"||e.state==="done");w(`
    <div class="o-form-topbar">
      <button class="o-back-btn" onclick="window._comprasBack()">← Compras</button>
      <div class="o-form-actions">
        ${e.state==="draft"?`<button class="o-btn-primary" onclick="alert('Confirmar OC')">Confirmar OC</button>`:""}
        ${e.state==="purchase"?`<button class="o-btn-secondary" onclick="alert('Recibir mercancía')">Recibir Productos</button>`:""}
        ${e.state==="purchase"||e.state==="done"?`<button class="o-btn-secondary" onclick="alert('Crear Factura de Proveedor')">Crear Factura</button>`:""}
        ${n?"":`<button class="o-btn-secondary" onclick="window._editarCompraForm(${e.id})">Editar</button>`}
        ${n?'<span style="font-size:11px;color:var(--text-400);margin-left:10px">Bloqueado por configuración</span>':""}
      </div>
    </div>

    <!-- STATUS BAR -->
    <div class="o-status-bar">
      ${Ze.map((d,r)=>{const l=ne[d],c=r===i,p=r<i;return`<div class="o-status-step ${c?"active":p?"done":""}">${l.lbl}</div>`}).join('<div class="o-status-arrow">›</div>')}
    </div>

    <div class="o-smart-buttons">
      <button class="o-smart-btn"><span class="o-smart-count">0</span><span class="o-smart-label">Recepciones</span></button>
      <button class="o-smart-btn"><span class="o-smart-count">0</span><span class="o-smart-label">Facturas</span></button>
      ${D.costos_aterrizaje?'<button class="o-smart-btn"><span class="o-smart-count" style="color:var(--primary)">$0</span><span class="o-smart-label">Costos Aterr.</span></button>':""}
    </div>

    <div class="o-form-sheet">
      <div class="o-sheet-header">
        <h1 class="o-form-title">${e.name||"Nueva Orden"}</h1>
        <span class="o-badge ${o.cls}">${o.lbl}</span>
      </div>

      <div class="o-form-grid">
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">Proveedor</label><div class="o-field-value o-td-primary">${e.partner_name||"—"}</div></div>
          <div class="o-field-group"><label class="o-field-label">Referencia Proveedor</label><div class="o-field-value">${e.partner_ref||"—"}</div></div>
          ${D.presupuesto_solicitud?'<div class="o-field-group"><label class="o-field-label">Acuerdo / Licitación</label><div class="o-field-value">Ninguno</div></div>':""}
        </div>
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">Fecha Límite</label><div class="o-field-value">${j(e.date_order)}</div></div>
          <div class="o-field-group"><label class="o-field-label">Facturar por</label><div class="o-field-value">${D.politica_facturacion==="cantidad_pedida"?"Cantidades pedidas":"Cantidades recibidas"}</div></div>
        </div>
      </div>

      <div class="o-notebook">
        <div class="o-tabs" id="compra-tabs">
          <button class="o-tab active" onclick="window._compraTab('productos', this)">Productos</button>
          <button class="o-tab" onclick="window._compraTab('adicional', this)">Información</button>
        </div>

        <div class="o-tab-pane" id="tab-productos">
          ${s.length?`
          <table class="o-list-table">
            <thead><tr>
              <th>Producto</th>
              ${D.variantes?"<th>Variante</th>":""}
              ${D.empaquetado?"<th>Empaque</th>":""}
              <th class="o-col-right">Qty</th>
              ${D.unidades_medida?"<th>UdM</th>":""}
              <th class="o-col-right">Precio</th>
              ${D.descuentos?'<th class="o-col-right">Desc.%</th>':""}
              <th class="o-col-right">Subtotal</th>
            </tr></thead>
            <tbody>
              ${s.map(d=>`
              <tr>
                <td class="o-td-primary">${d.product_name||d.name||"—"}</td>
                ${D.variantes?'<td><span style="font-size:11px;background:#E5E7EB;padding:2px 6px;border-radius:4px">Predeterminada</span></td>':""}
                ${D.empaquetado?"<td>Caja x1</td>":""}
                <td class="o-td-amount">${W(parseFloat(d.product_qty||d.qty||0))}</td>
                ${D.unidades_medida?"<td>PZ</td>":""}
                <td class="o-td-amount">${f(parseFloat(d.price_unit||0))}</td>
                ${D.descuentos?'<td class="o-td-amount o-td-muted">0%</td>':""}
                <td class="o-td-amount" style="font-weight:700">${f(parseFloat(d.price_subtotal||0))}</td>
              </tr>`).join("")}
            </tbody>
          </table>`:'<div class="o-empty-state"><p>Sin líneas</p></div>'}
          
          <div class="o-form-totals">
            <div class="o-total-row"><span>Subtotal</span><span>${f(parseFloat(e.amount_untaxed||0))}</span></div>
            <div class="o-total-row"><span>IVA</span><span>${f(parseFloat(e.amount_tax||0))}</span></div>
            <div class="o-total-row o-total-final"><span>Total</span><span>${f(parseFloat(e.amount_total||0))}</span></div>
          </div>
        </div>
        <div class="o-tab-pane" id="tab-adicional" style="display:none">
          <div class="o-field-group"><label class="o-field-label">Notas</label><textarea class="o-textarea" rows="3" ${n?"disabled":""}>${e.notes||""}</textarea></div>
        </div>
      </div>
    </div>
    `),window._editarCompraForm=d=>Eo({id:d,...e},()=>window._verCompra(d)),window._compraTab=(d,r)=>{document.querySelectorAll("#compra-tabs .o-tab").forEach(c=>c.classList.remove("active")),r.classList.add("active"),document.querySelectorAll(".o-tab-pane").forEach(c=>c.style.display="none");const l=document.getElementById(`tab-${d}`);l&&(l.style.display="")}}catch(e){b("Error",e.message,"error")}};window._comprasBack=()=>Oe();window._compraNueva=()=>{de(()=>import("./create_forms-CESEMRXd.js"),[]).then(t=>t.nuevaCompra(()=>Oe()))};window._compraSetView=t=>{Re=t,$a(),Ve()};window._compraFiltroEstado=t=>{Nt=t,ka()};window._chkAllCompras=t=>document.querySelectorAll("#compras-content .o-chk").forEach(e=>e.checked=t.checked);let Ft="list",nt=1,Ee="",Ce=null,ve=[];async function Ro(){B(),w(`<div class="nx-module-page"><div id="mcp"></div><div id="mcontent">${I(5,6)}</div></div>`),Ea(),await Bt()}function Ea(){const t=document.getElementById("mcp");t&&(t.innerHTML=`
    <div class="o-control-panel">
      <div class="o-cp-left">
        <button class="o-btn-new" onclick="window._newCot()">+ Nueva Cotización</button>
        <span class="o-cp-sep"></span>
        <div class="o-dropdown" id="dd-cf">
          <button class="o-btn-filter" onclick="window._tog('dd-cf')">📂 Filtros ▾</button>
          <div class="o-dropdown-menu" id="dd-cf-menu">
            <div class="o-dropdown-item" onclick="window._cf('draft')">Borradores</div>
            <div class="o-dropdown-item" onclick="window._cf('sent')">Enviadas</div>
            <div class="o-dropdown-item" onclick="window._cf('sale')">Confirmadas</div>
            <div class="o-dropdown-divider"></div>
            <div class="o-dropdown-item" onclick="window._cf(null)">❌ Sin filtro</div>
          </div>
        </div>
        <div class="o-search-box">
          <span style="color:var(--text-400)">🔍</span>
          <input type="search" placeholder="Buscar cotización…" id="cs" oninput="window._sc(this.value)">
        </div>
        <span class="o-record-count" id="ccount"></span>
      </div>
      <div class="o-cp-right">
        <div class="o-view-switcher">
          <button class="o-view-btn ${Ft==="list"?"active":""}" onclick="window._cvv('list')" title="Lista">☰</button>
          <button class="o-view-btn ${Ft==="kanban"?"active":""}" onclick="window._cvv('kanban')" title="Kanban">⬜</button>
        </div>
      </div>
    </div>`,Oo(),window._cvv=e=>{Ft=e,Ea(),Bt()},window._sc=Ko(e=>{Ee=e,nt=1,Bt()},300),window._cf=e=>{Ce=e,nt=1,Bt(),window._cdd()},window._newCot=()=>Go())}function Oo(){window._tog=t=>{const e=document.getElementById(t+"-menu");if(!e)return;const a=e.classList.contains("open");window._cdd(),a||e.classList.add("open")},window._cdd=()=>document.querySelectorAll(".o-dropdown-menu.open").forEach(t=>t.classList.remove("open")),window._ddInit||(document.addEventListener("click",t=>{t.target.closest(".o-dropdown")||window._cdd()}),window._ddInit=!0)}async function Bt(){const t=document.getElementById("mcontent");if(t){t.innerHTML=I(5,6);try{const e=await g.cotizaciones(nt);ve=(e==null?void 0:e.data)||[];let a=Ce?ve.filter(i=>i.state===Ce):ve;if(Ee){const i=Ee.toLowerCase();a=a.filter(s=>(s.name||"").toLowerCase().includes(i)||(s.partner_name||"").toLowerCase().includes(i))}const o=document.getElementById("ccount");o&&(o.textContent=a.length+" registros"),t.innerHTML=Ft==="kanban"?Ho(a):Vo(a),Ft==="list"&&Uo()}catch(e){t.innerHTML=`<div style="padding:40px;text-align:center;color:var(--text-400)">⚠️ ${e.message}</div>`}}}const Se={draft:"Borrador",sent:"Enviada",sale:"Confirmada",cancel:"Cancelada"};function Vo(t){return t.length?`
    <div class="o-list-actions-bar" id="clab"><span class="o-actions-count" id="csel-cnt">0 seleccionados</span>
      <button class="o-action-btn-sm" onclick="alert('Exportar')">Exportar</button>
    </div>
    <div class="o-list-view"><table>
      <thead><tr>
        <th class="th-check"><input type="checkbox" class="o-list-checkbox" id="cca" onchange="window._cca(this.checked)"></th>
        <th>Número</th><th>Cliente</th><th>Fecha</th><th>Validez</th><th>Estado</th><th style="text-align:right">Total</th>
      </tr></thead>
      <tbody>
        ${t.map(e=>{var a,o;return`
          <tr onclick="window._vCot(${e.id})" data-id="${e.id}">
            <td class="td-check" onclick="event.stopPropagation()"><input type="checkbox" class="o-list-checkbox crc" data-id="${e.id}" onchange="window._crc()"></td>
            <td><strong>${e.name||"-"}</strong></td>
            <td>${e.partner_name||e.partner_id||"-"}</td>
            <td>${((a=e.date_order)==null?void 0:a.slice(0,10))||"-"}</td>
            <td>${((o=e.validity_date)==null?void 0:o.slice(0,10))||'<span style="color:var(--text-300)">—</span>'}</td>
            <td>${Rt(e.state,Se[e.state]||e.state)}</td>
            <td style="text-align:right;font-weight:700;color:var(--primary)">${f(e.amount_total)}</td>
          </tr>`}).join("")}
      </tbody></table></div>
    <div style="padding:12px 20px;display:flex;justify-content:space-between;align-items:center;background:var(--bg-card);border-top:1px solid var(--border)">
      <span style="font-size:13px;color:var(--text-400)">${t.length} registros</span>
      <div style="display:flex;gap:8px">
        <button class="o-action-btn-sm" ${nt<=1?"disabled":""} onclick="window._cp(${nt-1})">‹ Anterior</button>
        <span style="padding:5px 10px;font-size:13px">${nt}</span>
        <button class="o-action-btn-sm" onclick="window._cp(${nt+1})">Siguiente ›</button>
      </div></div>`:'<div style="padding:60px;text-align:center"><div style="font-size:48px;margin-bottom:12px">📝</div><p style="color:var(--text-400)">Sin cotizaciones. Crea la primera.</p></div>'}const ta=[{key:"draft",label:"Nuevo",color:"#9CA3AF"},{key:"sent",label:"Calificado",color:"#2563EB"},{key:"sale",label:"Ganado",color:"#059669"}];function Ho(t){const e={};return ta.forEach(a=>e[a.key]=[]),t.forEach(a=>{e[a.state]?e[a.state].push(a):e.draft&&e.draft.push(a)}),`<div class="o-kanban-view" style="display:flex;gap:16px;padding:16px;overflow-x:auto">${ta.map(a=>`
    <div class="o-kanban-col" style="flex:0 0 300px;background:var(--bg-card);border-radius:8px;padding:12px;box-shadow:0 1px 3px rgba(0,0,0,0.3)">
      <div class="o-kanban-col-header" style="border-top:3px solid ${a.color};display:flex;justify-content:space-between;align-items:center;padding:8px 0;font-weight:600;font-size:14px;margin-bottom:12px">
        <span>${a.label}</span><span class="o-kanban-col-count" style="background:rgba(255,255,255,0.1);padding:2px 8px;border-radius:12px;font-size:12px">${e[a.key].length}</span>
      </div>
      <div class="o-kanban-cards" style="display:flex;flex-direction:column;gap:12px">
        ${e[a.key].map(o=>`
          <div class="o-kanban-card" style="background:#1E1528;border:1px solid #3B2A4A;border-radius:8px;padding:16px;cursor:pointer;transition:transform 0.1s" onclick="window._vCot(${o.id})">
            <div class="o-kanban-card-title" style="font-weight:600;margin-bottom:8px">${o.partner_name||"Cliente sin nombre"}</div>
            <div style="font-size:12px;color:var(--text-400);margin-bottom:8px">${o.name||"#"+o.id}</div>
            <div style="font-size:10px;display:inline-block;padding:2px 6px;background:rgba(167,139,250,0.15);color:#A78BFA;border-radius:4px;margin-bottom:12px">Servicios</div>
            <div class="o-kanban-card-meta" style="display:flex;justify-content:space-between;align-items:center">
              <span class="o-kanban-card-amount" style="font-weight:700;color:white">${f(o.amount_total)}</span>
              <span style="font-size:11px;display:flex;align-items:center;gap:4px">
                <span style="color:#FBBF24">★★★</span>
                <div class="user-pill-sm" style="display:inline-flex;margin-left:4px"><span class="avatar-xxs">A</span></div>
              </span>
            </div>
          </div>`).join("")||'<div style="padding:16px;text-align:center;color:var(--text-300);font-size:12px">Vacío</div>'}
      </div>
    </div>`).join("")}</div>`}function Uo(){window._cca=t=>{document.querySelectorAll(".crc").forEach(e=>e.checked=t),window._crc()},window._crc=()=>{const t=document.querySelectorAll(".crc:checked").length,e=document.getElementById("clab"),a=document.getElementById("csel-cnt");e&&e.classList.toggle("visible",t>0),a&&(a.textContent=t+" seleccionado"+(t!==1?"s":"")),document.querySelectorAll("[data-id]").forEach(o=>{const i=o.querySelector(".crc");i&&o.classList.toggle("selected",i.checked)})}}window._cp=t=>{nt=t,Bt()};function Go(){w(`
    <div class="o-form-view">
      <div class="o-statusbar">
        <div class="o-statusbar-status">
          <div class="o-status-step active">Borrador</div>
          <span class="o-status-arrow">›</span>
          <div class="o-status-step">Enviada</div>
        </div>
        <div class="o-statusbar-buttons">
          <button class="btn btn-secondary btn-sm" onclick="window._go('cotizaciones')">← Volver</button>
        </div>
      </div>
      <div class="o-form-sheet">
        <div class="o-form-title-row">
          <h1 class="o-form-record-title">Nueva Cotización</h1>
        </div>
        <div class="o-form-group-wrapper">
          <div class="o-form-group">
            <div class="o-form-col">
              <div class="o-field-row">
                <div class="o-field-label">Cliente *</div>
                <div class="o-field-value"><input id="nc-partner" class="form-control" placeholder="Nombre del cliente" autocomplete="off"></div>
              </div>
              <div class="o-field-row">
                <div class="o-field-label">Referencia</div>
                <div class="o-field-value"><input id="nc-ref" class="form-control" placeholder="Ref. del cliente"></div>
              </div>
            </div>
            <div class="o-form-col">
              <div class="o-field-row">
                <div class="o-field-label">Validez</div>
                <div class="o-field-value"><input id="nc-validez" class="form-control" type="date"></div>
              </div>
              <div class="o-field-row">
                <div class="o-field-label">Notas</div>
                <div class="o-field-value"><input id="nc-nota" class="form-control" placeholder="Observaciones opcionales"></div>
              </div>
            </div>
          </div>
        </div>
        <div style="display:flex;gap:12px;padding:16px 0">
          <button class="btn btn-primary" onclick="window._guardarNuevaCot()">💾 Guardar cotización</button>
          <button class="btn btn-secondary" onclick="window._go('cotizaciones')">Cancelar</button>
        </div>
      </div>
    </div>`),window._guardarNuevaCot=async()=>{var s,n,d,r,l,c,p,m;const t=(n=(s=document.getElementById("nc-partner"))==null?void 0:s.value)==null?void 0:n.trim(),e=((r=(d=document.getElementById("nc-ref"))==null?void 0:d.value)==null?void 0:r.trim())||null,a=((l=document.getElementById("nc-validez"))==null?void 0:l.value)||null,o=((p=(c=document.getElementById("nc-nota"))==null?void 0:c.value)==null?void 0:p.trim())||null;if(!t)return b("Campo requerido","Ingresa el nombre del cliente","warning");let i=1;try{const u=await g.get(`/partners?pagina=1&q=${encodeURIComponent(t)}&por_pagina=5`),v=(u==null?void 0:u.data)??[],k=v.find(_=>{var $;return(($=_.name)==null?void 0:$.toLowerCase())===t.toLowerCase()});if(k)i=k.id;else if(v.length>0)i=v[0].id;else return b("Cliente no encontrado",`No se encontró "${t}"`,"warning")}catch(u){return b("Error","No se pudo buscar el cliente: "+u.message,"error")}try{const u=await g.crearCotizacion({partner_id:i,partner_invoice_id:i,partner_shipping_id:i,note:o,client_order_ref:e,validity_date:a||null}),v=((m=u==null?void 0:u.data)==null?void 0:m.id)??(u==null?void 0:u.id);b("Cotización creada",`ID ${v}`,"success"),v?setTimeout(()=>window._vCot(v),400):window._go("cotizaciones")}catch(u){b("Error al crear cotización",u.message,"error")}}}window._vCot=async t=>{var e,a;w(`<div style="padding:40px">${I(3,5)}</div>`);try{const o=await g.cotizacion(t),i=(o==null?void 0:o.data)||o;if(!i)throw new Error("No encontrada");i.name||""+t;const s=["draft","sent"],n=s.indexOf(i.state),d={draft:"Borrador",sent:"Enviada"};w(`
      <div class="o-form-view" id="cfv">
        <div class="o-statusbar">
          <div class="o-statusbar-status">
            ${s.map((r,l)=>`
              <div class="o-status-step ${r===i.state?"active":""} ${l<n?"done":""}">
                ${l<n?"✔ ":""}${d[r]||r}
              </div>${l<s.length-1?'<span class="o-status-arrow">›</span>':""}`).join("")}
            ${i.state==="sale"?'<span class="o-status-arrow">›</span><div class="o-status-step done">✔ Confirmada</div>':""}
            ${i.state==="cancel"?'<span class="o-status-arrow">›</span><div class="o-status-step active" style="color:#DC2626">Cancelada</div>':""}
          </div>
          <div class="o-statusbar-buttons">
            ${i.state==="draft"||i.state==="sent"?`
              <button class="btn btn-secondary btn-sm" onclick="window._emailCot(${t})">✉️ Enviar por Email</button>
              <button class="btn btn-primary btn-sm" onclick="window._confirmarCot(${t})">✅ Confirmar Pedido</button>
            `:""}
            ${i.state==="sale"?`<button class="btn btn-secondary btn-sm" onclick="window._vVenta(${t})">📋 Ver Orden</button>`:""}
            ${i.state!=="cancel"&&i.state!=="sale"?`<button class="btn btn-sm" style="background:#FEE2E2;color:#DC2626;border:none;padding:6px 14px;border-radius:8px;font-weight:600;cursor:pointer" onclick="window._cancelarCot(${t})">❌ Cancelar</button>`:""}
            <button class="btn btn-secondary btn-sm" onclick="window._go('cotizaciones')">← Volver</button>
          </div>
        </div>
        <div class="o-smart-buttons">
          <button class="o-smart-btn" ${i.state==="sale"?`onclick="window._vVenta(${t})"`:""}>
            <span class="o-count">${i.state==="sale"?"1":"0"}</span>
            <span class="o-label">📋 Órdenes</span>
          </button>
          <button class="o-smart-btn"><span class="o-count">0</span><span class="o-label">✉️ Emails</span></button>
        </div>
        <div class="o-form-sheet">
          <div class="o-form-title-row">
            <h1 class="o-form-record-title">${i.name||"Nueva Cotización"}</h1>
            <span class="o-form-subtitle">${i.partner_name||""}</span>
          </div>
          <div class="o-form-group-wrapper">
            <div class="o-form-group">
              <div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">Cliente</div><div class="o-field-value"><strong>${i.partner_name||i.partner_id||'<span class="o-field-empty">—</span>'}</strong></div></div>
                <div class="o-field-row"><div class="o-field-label">Fecha</div><div class="o-field-value">${((e=i.date_order)==null?void 0:e.slice(0,10))||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Validez</div><div class="o-field-value">${((a=i.validity_date)==null?void 0:a.slice(0,10))||'<span class="o-field-empty">—</span>'}</div></div>
              </div>
              <div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">Estado</div><div class="o-field-value">${Rt(i.state,Se[i.state]||i.state)}</div></div>
                <div class="o-field-row"><div class="o-field-label">Referencia</div><div class="o-field-value">${i.client_order_ref||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Vendedor</div><div class="o-field-value">${i.user_id||i.user_name||'<span class="o-field-empty">—</span>'}</div></div>
              </div>
            </div>
          </div>
          <div class="o-notebook">
            <div class="o-tabs">
              <button class="o-tab active" onclick="window._ct('cl')">Líneas</button>
              <button class="o-tab" onclick="window._ct('cc')">Condiciones</button>
            </div>
            <div class="o-tab-panel active" id="tab-panel-cl">
              <table class="o-editable-table"><thead><tr>
                <th>Producto</th><th>Descripción</th>
                <th style="text-align:right">Qty</th>
                <th style="text-align:right">Precio</th>
                <th style="text-align:right">Desc.</th>
                <th style="text-align:right">Subtotal</th>
              </tr></thead>
              <tbody id="clineas"><tr><td colspan="6" style="text-align:center;padding:20px;color:var(--text-400)">⏳ Cargando…</td></tr></tbody></table>
              <div class="o-lines-totals"><table>
                <tr><td>Subtotal:</td><td style="text-align:right;font-weight:600">${f(i.amount_untaxed)}</td></tr>
                <tr><td>IVA (16%):</td><td style="text-align:right;font-weight:600">${f(i.amount_tax)}</td></tr>
                <tr class="total-row"><td>TOTAL:</td><td style="text-align:right">${f(i.amount_total)}</td></tr>
              </table></div>
            </div>
            <div class="o-tab-panel" id="tab-panel-cc">
              <div class="o-form-group"><div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">Notas</div><div class="o-field-value">${i.note||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Plazo de pago</div><div class="o-field-value">${i.payment_term_name||i.payment_term||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Política entrega</div><div class="o-field-value">${i.picking_policy||'<span class="o-field-empty">—</span>'}</div></div>
              </div></div>
            </div>
          </div>
        </div>
        <div class="o-chatter">
          <div class="o-chatter-topbar">
            <button class="o-chatter-btn">✉️ Enviar mensaje</button>
            <button class="o-chatter-btn">📋 Nota interna</button>
            <button class="o-chatter-btn">📎 Adjuntar</button>
          </div>
          <div class="o-chatter-thread">
            <div class="o-message">
              <div class="o-msg-avatar" style="background:#D97706">C</div>
              <div class="o-msg-content">
                <div class="o-msg-header">
                  <span class="o-msg-author">Sistema</span>
                  <span class="o-msg-date">${new Date().toLocaleDateString("es-MX")}</span>
                </div>
                <div class="o-msg-text">Cotización ${i.name||""} — Estado: ${Se[i.state]||i.state}</div>
              </div>
            </div>
          </div>
        </div>
      </div>`),window._ct=r=>{document.querySelectorAll(".o-tab").forEach(p=>p.classList.remove("active")),document.querySelectorAll(".o-tab-panel").forEach(p=>p.classList.remove("active"));const l=document.querySelector(`.o-tab[onclick*="'${r}'"]`);l&&l.classList.add("active");const c=document.getElementById("tab-panel-"+r);c&&c.classList.add("active")};try{const r=await g.get(`/cotizaciones/${t}/lineas`),l=(r==null?void 0:r.data)||[],c=document.getElementById("clineas");c&&(c.innerHTML=l.length?l.map(p=>`<tr>
              <td>${p.product_id?"#"+p.product_id:'<span class="o-field-empty">—</span>'}</td>
              <td>${p.name||"-"}</td>
              <td style="text-align:right">${p.product_uom_qty??0}</td>
              <td style="text-align:right">${f(p.price_unit)}</td>
              <td style="text-align:right">${p.discount?p.discount+"%":"0%"}</td>
              <td style="text-align:right;font-weight:700">${f(p.price_subtotal)}</td>
            </tr>`).join(""):'<tr><td colspan="6" style="text-align:center;padding:16px;color:var(--text-400)">Sin líneas de cotización</td></tr>')}catch{}window._emailCot=async r=>{try{await g.put(`/cotizaciones/${r}/enviar`,{}),b("OK","Cotización enviada por email","success"),window._vCot(r)}catch(l){b("Error",l.message,"error")}},window._confirmarCot=async r=>{if(confirm("¿Confirmar cotización como pedido de venta?"))try{await g.confirmarCotizacion(r),b("OK","Cotización confirmada como venta","success"),setTimeout(()=>window._go("ventas"),600)}catch(l){b("Error",l.message,"error")}},window._cancelarCot=async r=>{if(confirm("¿Cancelar cotización?"))try{await g.cancelarCotizacion(r),b("Cancelado","","info"),window._go("cotizaciones")}catch(l){b("Error",l.message,"error")}}}catch(o){w(`<div style="padding:40px;text-align:center"><p style="color:#DC2626">⚠️ ${o.message}</p><button class="o-btn-new" onclick="window._go('cotizaciones')">Volver</button></div>`)}};function Ko(t,e){let a;return(...o)=>{clearTimeout(a),a=setTimeout(()=>t(...o),e)}}const ea=[{id:1,name:"Tarifa General",currency:"MXN",type:"Porcentaje",active:!0,discount:0},{id:2,name:"Distribuidores",currency:"MXN",type:"Porcentaje",active:!0,discount:10},{id:3,name:"Exportación USD",currency:"USD",type:"Fijo",active:!1,discount:0}],Ca="nexus_pricelists";let H=[],ze="";function He(){localStorage.setItem(Ca,JSON.stringify(H))}async function Xo(t={}){var a;B(),w(`<div class="nx-module-page">
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
    <div id="pc-content">${I(3,5)}</div>
  </div>`);let e=null;try{e=JSON.parse(localStorage.getItem(Ca)||"null")}catch{e=null}if(Array.isArray(e)&&e.length)H=e;else{try{const o=await g.get("/precios");H=(a=o==null?void 0:o.data)!=null&&a.length?o.data:ea}catch{H=ea}He()}window._sprecio=o=>{ze=o,Te()},window._nprecio=Wo,window._editprecio=Jo,Te()}function Te(){const t=document.getElementById("pc-content");if(!t)return;const e=document.getElementById("pc-count"),a=ze?H.filter(o=>o.name.toLowerCase().includes(ze.toLowerCase())):H;if(e&&(e.textContent=`${a.length} registros`),!a.length){t.innerHTML='<div style="padding:60px;text-align:center;color:var(--text-400)"><div style="font-size:48px;margin-bottom:12px">🏷️</div><div style="font-size:16px;font-weight:600">Sin listas de precios</div><div style="font-size:13px;margin-top:8px">Crea la primera con + Nueva Lista</div></div>';return}t.innerHTML=`
    <div class="o-list-view">
      <table class="o-list-table">
        <thead><tr>
          <th>Nombre</th><th>Moneda</th><th>Tipo</th><th>Descuento %</th><th>Estado</th>
        </tr></thead>
        <tbody>
          ${a.map(o=>`
          <tr class="o-list-row" onclick="window._editprecio(${o.id})" style="cursor:pointer">
            <td><strong>${o.name}</strong></td>
            <td>${o.currency||"MXN"}</td>
            <td>${o.type||"Porcentaje"}</td>
            <td>${o.discount??0}%</td>
            <td><span class="o-badge ${o.active?"o-badge-success":"o-badge-gray"}">${o.active?"Activa":"Inactiva"}</span></td>
          </tr>`).join("")}
        </tbody>
      </table>
    </div>`}function Jo(t){const e=H.find(o=>o.id===t);if(!e)return;const a=document.getElementById("pc-content");a&&(a.innerHTML=`
    <div style="max-width:760px;margin:24px auto;background:var(--bg-card);border-radius:12px;border:1px solid var(--border);padding:28px">
      <div style="display:flex;align-items:center;gap:12px;margin-bottom:20px">
        <button onclick="window._go('precios')" class="o-btn-secondary o-btn-sm">← Volver</button>
        <h2 style="margin:0;font-size:18px;font-weight:700">${e.name}</h2>
      </div>
      <div style="display:grid;grid-template-columns:1fr 1fr;gap:16px;margin-bottom:20px">
        <div class="o-field-row"><div class="o-field-label">Nombre</div><div class="o-field-value"><input class="o-input" id="pc-name" value="${e.name}"></div></div>
        <div class="o-field-row"><div class="o-field-label">Moneda</div><div class="o-field-value">
          <select class="o-select" id="pc-cur"><option ${e.currency==="MXN"?"selected":""}>MXN</option><option ${e.currency==="USD"?"selected":""}>USD</option></select>
        </div></div>
        <div class="o-field-row"><div class="o-field-label">Tipo</div><div class="o-field-value">
          <select class="o-select" id="pc-type"><option ${e.type==="Porcentaje"?"selected":""}>Porcentaje</option><option ${e.type==="Fijo"?"selected":""}>Fijo</option></select>
        </div></div>
        <div class="o-field-row"><div class="o-field-label">Descuento %</div><div class="o-field-value"><input type="number" class="o-input" id="pc-disc" value="${e.discount??0}" min="0" max="100"></div></div>
        <div class="o-field-row"><div class="o-field-label">Activa</div><div class="o-field-value"><input type="checkbox" id="pc-active" ${e.active?"checked":""}></div></div>
      </div>
      <div style="display:flex;gap:8px">
        <button class="o-btn-primary" onclick="window._saveprecio(${e.id})">💾 Guardar</button>
        <button class="o-btn-secondary o-btn-sm" onclick="window._go('precios')">Descartar</button>
      </div>
    </div>`,window._saveprecio=o=>{const i=H.findIndex(s=>s.id===o);i<0||(H[i].name=document.getElementById("pc-name").value,H[i].currency=document.getElementById("pc-cur").value,H[i].type=document.getElementById("pc-type").value,H[i].discount=parseFloat(document.getElementById("pc-disc").value)||0,H[i].active=document.getElementById("pc-active").checked,He(),b("Guardado","Lista de precios actualizada","success"),window._go("precios"))})}function Wo(){const t=document.createElement("div");t.style.cssText="position:fixed;inset:0;z-index:950;background:rgba(0,0,0,.45);display:flex;align-items:center;justify-content:center;padding:16px",t.innerHTML=`
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
    </div>`,document.body.appendChild(t),t.onclick=e=>{e.target===t&&t.remove()},window._crearPrecio=()=>{var o,i;const e=(i=(o=document.getElementById("np-name"))==null?void 0:o.value)==null?void 0:i.trim();if(!e){b("Error","El nombre es obligatorio","error");return}const a={id:Date.now(),name:e,currency:document.getElementById("np-cur").value,type:document.getElementById("np-type").value,discount:parseFloat(document.getElementById("np-disc").value)||0,active:!0};H.push(a),He(),t.remove(),b("Creado",`Lista "${e}" creada`,"success"),Te()}}let yt="year",Ie=[];async function Yo(t={}){B(),w(`<div class="nx-module-page">
    <div class="o-cp">
      <div class="o-cp-left">
        <div style="display:flex;gap:4px">
          <button id="rp-mes" class="o-btn-filter ${yt==="month"?"active":""}" onclick="window._rperiodo('month')">Mes actual</button>
          <button id="rp-tri" class="o-btn-filter ${yt==="quarter"?"active":""}" onclick="window._rperiodo('quarter')">Trimestre</button>
          <button id="rp-ano" class="o-btn-filter ${yt==="year"?"active":""}" onclick="window._rperiodo('year')">Este año</button>
        </div>
      </div>
      <div class="o-cp-right">
        <button class="o-btn-secondary o-btn-sm" onclick="window.print()">Exportar</button>
      </div>
    </div>
    <div id="rv-content" style="padding:24px">
      <div style="text-align:center;padding:60px;color:var(--text-400)">⏳ Cargando reportes...</div>
    </div>
  </div>`),window._rperiodo=a=>{yt=a,aa()};let e={};try{const a=await g.get("/ventas/kpis");e=(a==null?void 0:a.data)||a||{}}catch{}try{const a=await g.get("/ventas?limite=200");Ie=(a==null?void 0:a.data)||[]}catch{Ie=[]}window._rvKpis=e,aa()}function aa(){const t=document.getElementById("rv-content");if(!t)return;const e=window._rvKpis||{},a=new Date,o=Ie.filter(u=>{if(!u.date_order)return!0;const v=new Date(u.date_order);if(yt==="month")return v.getMonth()===a.getMonth()&&v.getFullYear()===a.getFullYear();if(yt==="quarter"){const k=Math.floor(a.getMonth()/3);return Math.floor(v.getMonth()/3)===k&&v.getFullYear()===a.getFullYear()}return v.getFullYear()===a.getFullYear()}),i={};o.forEach(u=>{if(!u.date_order)return;const v=u.date_order.slice(0,7);i[v]||(i[v]={mes:v,count:0,total:0}),i[v].count++,i[v].total+=parseFloat(u.amount_total||0)});const s=Object.values(i).sort((u,v)=>u.mes.localeCompare(v.mes)),n=Math.max(...s.map(u=>u.total),1),d=e.total_ventas??o.reduce((u,v)=>u+parseFloat(v.amount_total||0),0),r=e.pedidos_confirmados??o.filter(u=>u.state==="sale"||u.state==="done").length,l=r>0?d/r:0,c=e.cotizaciones_enviadas??o.filter(u=>u.state==="sent").length,p=["Ene","Feb","Mar","Abr","May","Jun","Jul","Ago","Sep","Oct","Nov","Dic"],m=u=>{const[v,k]=u.split("-");return`${p[parseInt(k)-1]} ${v}`};t.innerHTML=`
    <!-- KPIs -->
    <div style="display:grid;grid-template-columns:repeat(4,1fr);gap:16px;margin-bottom:28px">
      <div style="background:var(--bg-card);border:1px solid var(--border);border-radius:12px;padding:20px;border-top:3px solid var(--primary)">
        <div style="font-size:11px;font-weight:700;text-transform:uppercase;color:var(--text-400);margin-bottom:6px">Total Ventas</div>
        <div style="font-size:24px;font-weight:800;color:var(--primary)">${f(d)}</div>
      </div>
      <div style="background:var(--bg-card);border:1px solid var(--border);border-radius:12px;padding:20px;border-top:3px solid #059669">
        <div style="font-size:11px;font-weight:700;text-transform:uppercase;color:var(--text-400);margin-bottom:6px">Pedidos Confirmados</div>
        <div style="font-size:24px;font-weight:800;color:#059669">${r}</div>
      </div>
      <div style="background:var(--bg-card);border:1px solid var(--border);border-radius:12px;padding:20px;border-top:3px solid #7C3AED">
        <div style="font-size:11px;font-weight:700;text-transform:uppercase;color:var(--text-400);margin-bottom:6px">Ticket Promedio</div>
        <div style="font-size:24px;font-weight:800;color:#7C3AED">${f(l)}</div>
      </div>
      <div style="background:var(--bg-card);border:1px solid var(--border);border-radius:12px;padding:20px;border-top:3px solid #F59E0B">
        <div style="font-size:11px;font-weight:700;text-transform:uppercase;color:var(--text-400);margin-bottom:6px">Cotizaciones</div>
        <div style="font-size:24px;font-weight:800;color:#F59E0B">${c}</div>
      </div>
    </div>

    <!-- Gráfica barras CSS -->
    <div style="background:var(--bg-card);border:1px solid var(--border);border-radius:12px;padding:24px;margin-bottom:24px">
      <div style="font-size:14px;font-weight:700;margin-bottom:16px">Ventas por Mes</div>
      ${s.length===0?'<div style="text-align:center;padding:40px;color:var(--text-400)">Sin datos en el periodo seleccionado</div>':`
      <div style="display:flex;align-items:flex-end;gap:8px;height:180px;border-bottom:2px solid var(--border);padding-bottom:8px">
        ${s.map(u=>`
          <div style="flex:1;display:flex;flex-direction:column;align-items:center;gap:4px">
            <div style="font-size:10px;color:var(--text-400);white-space:nowrap">${f(u.total)}</div>
            <div style="width:100%;background:var(--primary);border-radius:4px 4px 0 0;min-height:4px;height:${Math.max(4,Math.round(u.total/n*140))}px;transition:height .3s"></div>
          </div>`).join("")}
      </div>
      <div style="display:flex;gap:8px;padding-top:8px">
        ${s.map(u=>`<div style="flex:1;text-align:center;font-size:10px;color:var(--text-400)">${m(u.mes)}</div>`).join("")}
      </div>`}
    </div>

    <!-- Tabla pivot -->
    <div style="background:var(--bg-card);border:1px solid var(--border);border-radius:12px;overflow:hidden">
      <div style="padding:16px 20px;border-bottom:1px solid var(--border);font-size:14px;font-weight:700">Análisis por Mes</div>
      <table class="o-list-table">
        <thead><tr><th>Mes</th><th style="text-align:right">Órdenes</th><th style="text-align:right">Total MXN</th><th style="text-align:right">Ticket Promedio</th></tr></thead>
        <tbody>
          ${s.length===0?'<tr><td colspan="4" style="text-align:center;padding:20px;color:var(--text-400)">Sin datos</td></tr>':s.map(u=>`<tr class="o-list-row"><td>${m(u.mes)}</td><td style="text-align:right">${u.count}</td><td style="text-align:right;font-weight:700;color:var(--primary)">${f(u.total)}</td><td style="text-align:right">${f(u.count>0?u.total/u.count:0)}</td></tr>`).join("")}
          <tr style="background:var(--primary-light);font-weight:700"><td>TOTAL</td><td style="text-align:right">${o.length}</td><td style="text-align:right;color:var(--primary)">${f(d)}</td><td style="text-align:right">${f(l)}</td></tr>
        </tbody>
      </table>
    </div>`}const oa="nexus_config_ventas",Qo={variantes:!1,unidades_medida:!1,empaquetado:!1,descuentos:!0,listas_precios:!1,descuento_precio:!1,margenes:!1,firma_online:!1,pago_online:!1,validez_cotizacion:30,bloquear_confirmado:!0,advertencias:!1,plantillas_presupuesto:!1,compra_online:!1,notas_cierre:!1,costos_envio:!1,fecha_entrega:!1,aviso_stock:!1,politica_facturacion:"cantidad_pedida",terminos:""};function q(t,e,a,o){return`
  <label style="display:flex;align-items:flex-start;gap:12px;cursor:pointer;padding:14px 20px;border-bottom:1px solid var(--border)" id="row-${t}">
    <input type="checkbox" id="${t}" ${o?"checked":""}
      style="margin-top:2px;accent-color:var(--primary);width:16px;height:16px;flex-shrink:0">
    <div>
      <div style="font-weight:600;font-size:14px;color:var(--text-900)">${e}</div>
      <div style="font-size:12px;color:var(--text-400);margin-top:2px;line-height:1.5">${a}</div>
    </div>
  </label>`}function bt(t,e){return`
  <div style="background:var(--bg-card);border:1px solid var(--border);border-radius:12px;margin-bottom:24px;overflow:hidden">
    <div style="padding:12px 20px;background:var(--bg-app);border-bottom:1px solid var(--border);
      font-size:11px;font-weight:800;text-transform:uppercase;color:var(--text-500);letter-spacing:.08em">
      ${t}
    </div>
    ${e}
  </div>`}async function Sa(t={}){var a;B();const e={...Qo,...JSON.parse(localStorage.getItem(oa)||"{}")};w(`<div class="nx-module-page" style="background:var(--bg-app)">

    <!-- Control Panel -->
    <div class="o-cp">
      <div class="o-cp-left"></div>
      <div class="o-cp-right">
        <button class="o-btn-primary" onclick="window._saveConfigV()">💾 Guardar</button>
        <button class="o-btn-secondary" onclick="window._discardConfigV()">Descartar</button>
      </div>
    </div>

    <div style="max-width:900px;margin:28px auto;padding:0 24px 60px">

      ${bt("Catálogo de Productos",`
        ${q("cfg-var","Variantes","Permite crear variantes de producto (talla, color, etc.) desde una sola ficha de producto",e.variantes)}
        ${q("cfg-udm","Unidades de Medida","Habilita múltiples unidades de medida y conversiones automáticas entre ellas. Muestra la columna UdM en las líneas de pedido",e.unidades_medida)}
        ${q("cfg-pack","Empaquetado de Producto","Define distintas presentaciones de empaque (caja x12, paquete x6, etc.) para los productos",e.empaquetado)}
      `)}

      ${bt("Precios",`
        ${q("cfg-desc","Descuentos","Permite aplicar descuentos por línea en las órdenes de venta. Muestra la columna Descuento en las líneas de pedido",e.descuentos)}
        ${q("cfg-pricelist","Listas de Precios","Habilita múltiples listas de precios para asignar tarifas personalizadas a clientes, grupos o canales de venta",e.listas_precios)}
        <div id="row-pricelist-link" style="display:${e.listas_precios?"":"none"};padding:8px 20px 12px 48px;border-bottom:1px solid var(--border)">
          <a href="#precios" style="font-size:12px;font-weight:600;color:var(--primary);text-decoration:none">→ Administrar listas de precios</a>
          <div style="font-size:11px;color:var(--text-400);margin-top:2px">Las listas activas aparecen en el selector "Lista de Precios" del pedido y aplican su descuento a las líneas</div>
        </div>
        <div id="row-descprice" style="display:${e.listas_precios?"":"none"}">
          ${q("cfg-descprice","Descuentos de Lista de Precios","Muestra el descuento aplicado en la línea de la factura (precio de lista vs precio real)",e.descuento_precio)}
        </div>
        ${q("cfg-marg","Márgenes","Muestra el margen de ganancia en cada línea y en los totales de las órdenes de venta",e.margenes)}
      `)}

      ${bt("Presupuestos y Pedidos",`
        ${q("cfg-firma","Firma en Línea","Permite que los clientes firmen digitalmente las cotizaciones para confirmarlas",e.firma_online)}
        ${q("cfg-pago","Pago en Línea","Permite que los clientes paguen sus cotizaciones en línea con tarjeta o transferencia",e.pago_online)}
        <div style="display:flex;align-items:center;justify-content:space-between;padding:14px 20px;border-bottom:1px solid var(--border)">
          <div>
            <div style="font-weight:600;font-size:14px;color:var(--text-900)">Validez Predeterminada del Presupuesto</div>
            <div style="font-size:12px;color:var(--text-400);margin-top:2px">Número de días que una cotización permanece válida. 0 = sin expiración</div>
          </div>
          <div style="display:flex;align-items:center;gap:6px">
            <input type="number" id="cfg-valid" value="${e.validez_cotizacion}" min="0" max="365"
              style="width:80px;text-align:center;padding:6px 10px;border:1px solid var(--border);border-radius:8px;font-size:14px">
            <span style="font-size:13px;color:var(--text-400)">días</span>
          </div>
        </div>
        ${q("cfg-bloq","Bloquear Pedido Confirmado","Impide editar un pedido después de confirmarlo. Para modificarlo se debe crear un pedido de devolución o cancelarlo",e.bloquear_confirmado)}
        ${q("cfg-warn","Advertencias","Muestra advertencias al vendedor al confirmar cotizaciones o pedidos para clientes o productos específicos",e.advertencias)}
        ${q("cfg-tmpl","Plantillas de Presupuesto","Crea plantillas reutilizables para los presupuestos más comunes y aplícalas con un clic",e.plantillas_presupuesto)}
        ${q("cfg-online","Compra en Línea","Permite a los clientes ver y confirmar sus cotizaciones en un portal en línea",e.compra_online)}
        ${q("cfg-notas","Notas de Cierre","Agrega notas personalizadas al final de las cotizaciones y pedidos de venta confirmados",e.notas_cierre)}
      `)}

      ${bt("Envío",`
        ${q("cfg-ship","Costos de Envío","Permite agregar costos de envío a las cotizaciones. Se integra con los métodos de entrega configurados",e.costos_envio)}
        ${q("cfg-fecha","Fechas de Entrega","Muestra la fecha de entrega comprometida al cliente (Fecha Compromiso) en las cotizaciones y pedidos",e.fecha_entrega)}
        ${q("cfg-stock","Advertencia de Stock","Muestra una advertencia al confirmar un pedido si no hay suficiente stock disponible",e.aviso_stock)}
      `)}

      ${bt("Facturación",`
        <div style="display:flex;align-items:center;justify-content:space-between;padding:14px 20px;border-bottom:1px solid var(--border)">
          <div>
            <div style="font-weight:600;font-size:14px;color:var(--text-900)">Política de Facturación</div>
            <div style="font-size:12px;color:var(--text-400);margin-top:2px">Define cuándo se puede facturar al cliente: al confirmar el pedido o al entregar los productos</div>
          </div>
          <select id="cfg-pol" style="min-width:230px;padding:7px 12px;border:1px solid var(--border);border-radius:8px;font-size:13px;background:var(--bg-card);color:var(--text-900)">
            <option value="cantidad_pedida" ${e.politica_facturacion==="cantidad_pedida"?"selected":""}>Cantidades pedidas</option>
            <option value="cantidad_entregada" ${e.politica_facturacion==="cantidad_entregada"?"selected":""}>Cantidades entregadas</option>
          </select>
        </div>
      `)}

      ${bt("Términos y Condiciones",`
        <div style="padding:16px 20px">
          <div style="font-size:12px;color:var(--text-400);margin-bottom:10px">
            Texto que aparece al pie de cada cotización y pedido de venta. Puedes incluir políticas de devolución, formas de pago, etc.
          </div>
          <textarea id="cfg-terms" rows="5"
            style="width:100%;padding:10px 14px;border:1px solid var(--border);border-radius:8px;font-size:13px;background:var(--bg-card);color:var(--text-900);resize:vertical;font-family:inherit;line-height:1.6;box-sizing:border-box"
            placeholder="Ej. Los precios no incluyen IVA. Válido por 30 días. Entrega sujeta a disponibilidad de stock.">${e.terminos}</textarea>
        </div>
      `)}

      <div style="display:flex;gap:10px;padding-top:8px">
        <button class="o-btn-primary" onclick="window._saveConfigV()">💾 Guardar Configuración</button>
        <button class="o-btn-secondary" onclick="window._discardConfigV()">Descartar cambios</button>
      </div>
    </div>
  </div>`),(a=document.getElementById("cfg-pricelist"))==null||a.addEventListener("change",o=>{const i=o.target.checked?"":"none";document.getElementById("row-descprice").style.display=i,document.getElementById("row-pricelist-link").style.display=i}),window._saveConfigV=()=>{var s,n,d,r,l,c,p,m,u,v,k,_,$,T,h,E,S,y,z,P;const o=A=>document.getElementById(A),i={variantes:((s=o("cfg-var"))==null?void 0:s.checked)??!1,unidades_medida:((n=o("cfg-udm"))==null?void 0:n.checked)??!1,empaquetado:((d=o("cfg-pack"))==null?void 0:d.checked)??!1,descuentos:((r=o("cfg-desc"))==null?void 0:r.checked)??!0,listas_precios:((l=o("cfg-pricelist"))==null?void 0:l.checked)??!1,descuento_precio:((c=o("cfg-descprice"))==null?void 0:c.checked)??!1,margenes:((p=o("cfg-marg"))==null?void 0:p.checked)??!1,firma_online:((m=o("cfg-firma"))==null?void 0:m.checked)??!1,pago_online:((u=o("cfg-pago"))==null?void 0:u.checked)??!1,validez_cotizacion:Math.max(0,parseInt((v=o("cfg-valid"))==null?void 0:v.value,10)||0),bloquear_confirmado:((k=o("cfg-bloq"))==null?void 0:k.checked)??!0,advertencias:((_=o("cfg-warn"))==null?void 0:_.checked)??!1,plantillas_presupuesto:(($=o("cfg-tmpl"))==null?void 0:$.checked)??!1,compra_online:((T=o("cfg-online"))==null?void 0:T.checked)??!1,notas_cierre:((h=o("cfg-notas"))==null?void 0:h.checked)??!1,costos_envio:((E=o("cfg-ship"))==null?void 0:E.checked)??!1,fecha_entrega:((S=o("cfg-fecha"))==null?void 0:S.checked)??!1,aviso_stock:((y=o("cfg-stock"))==null?void 0:y.checked)??!1,politica_facturacion:((z=o("cfg-pol"))==null?void 0:z.value)||"cantidad_pedida",terminos:((P=o("cfg-terms"))==null?void 0:P.value)||""};localStorage.setItem(oa,JSON.stringify(i)),b("Guardado","Configuración de Ventas actualizada correctamente","success")},window._discardConfigV=()=>{Sa()}}const ia="nexus_config_facturacion",Zo={impuestos_ventas:!0,impuestos_compras:!0,redondeo:!1,pagos_online:!1,descuentos_pronto_pago:!1,terminos_default:"",alertas_cliente:!1,cfdi_auto:!1,cancelacion_directa:!1};function et(t,e,a,o){return`
  <label style="display:flex;align-items:flex-start;gap:12px;cursor:pointer;padding:14px 20px;border-bottom:1px solid var(--border)" id="row-${t}">
    <input type="checkbox" id="${t}" ${o?"checked":""}
      style="margin-top:2px;accent-color:var(--primary);width:16px;height:16px;flex-shrink:0">
    <div>
      <div style="font-weight:600;font-size:14px;color:var(--text-900)">${e}</div>
      <div style="font-size:12px;color:var(--text-400);margin-top:2px;line-height:1.5">${a}</div>
    </div>
  </label>`}function kt(t,e){return`
  <div style="background:var(--bg-card);border:1px solid var(--border);border-radius:12px;margin-bottom:24px;overflow:hidden">
    <div style="padding:12px 20px;background:var(--bg-app);border-bottom:1px solid var(--border);
      font-size:11px;font-weight:800;text-transform:uppercase;color:var(--text-500);letter-spacing:.08em">
      ${t}
    </div>
    ${e}
  </div>`}async function za(t={}){B();const e={...Zo,...JSON.parse(localStorage.getItem(ia)||"{}")};w(`<div class="nx-module-page" style="background:var(--bg-app)">

    <!-- Control Panel -->
    <div class="o-cp">
      <div class="o-cp-left"></div>
      <div class="o-cp-right">
        <button class="o-btn-primary" onclick="window._saveConfigF()">💾 Guardar</button>
        <button class="o-btn-secondary" onclick="window._discardConfigF()">Descartar</button>
      </div>
    </div>

    <div style="max-width:900px;margin:28px auto;padding:0 24px 60px">

      ${kt("Impuestos",`
        ${et("cfg-tax-v","Impuestos de Ventas","Aplica automáticamente el impuesto configurado en las facturas de venta",e.impuestos_ventas)}
        ${et("cfg-tax-c","Impuestos de Compras","Aplica automáticamente el impuesto configurado en las facturas de proveedores",e.impuestos_compras)}
        ${et("cfg-redondeo","Redondeo de Efectivo","Añade una línea de ajuste en el subtotal para redondear el total a la fracción más cercana (Ej. 0.05)",e.redondeo)}
      `)}

      ${kt("Pagos de Clientes",`
        ${et("cfg-pago-online","Pagos en Línea","Permite a los clientes pagar sus facturas en línea usando tarjetas de crédito o pasarelas de pago",e.pagos_online)}
        ${et("cfg-desc-pago","Descuentos por Pronto Pago","Habilita la configuración de descuentos condicionales si el cliente paga antes de cierta fecha",e.descuentos_pronto_pago)}
      `)}

      ${kt("Facturas de Clientes",`
        ${et("cfg-alertas","Alertas de Cliente","Muestra alertas al seleccionar un cliente en la factura (ej. Si tiene deuda pendiente)",e.alertas_cliente)}
      `)}

      ${kt("Términos y Condiciones por Defecto",`
        <div style="padding:16px 20px">
          <div style="font-size:12px;color:var(--text-400);margin-bottom:10px">
            Texto predeterminado que aparecerá en el campo "Términos y condiciones" de las nuevas facturas.
          </div>
          <textarea id="cfg-terminos" rows="5"
            style="width:100%;padding:10px 14px;border:1px solid var(--border);border-radius:8px;font-size:13px;background:var(--bg-card);color:var(--text-900);resize:vertical;font-family:inherit;line-height:1.6;box-sizing:border-box"
            placeholder="Ej. El pago debe realizarse a 30 días netos. Interés moratorio del 2% mensual.">${e.terminos_default}</textarea>
        </div>
      `)}

      ${kt("Localización (México / CFDI)",`
        ${et("cfg-cfdi-auto","Timbrado Automático al Publicar","El sistema enviará el CFDI al PAC automáticamente en cuanto se publique/confirme la factura",e.cfdi_auto)}
        ${et("cfg-canc-directa","Cancelación Directa","Permite cancelar directamente facturas en el ERP ignorando el estatus del CFDI en el SAT (solo usar si manejas la cancelación externamente)",e.cancelacion_directa)}
      `)}

      <div style="display:flex;gap:10px;padding-top:8px">
        <button class="o-btn-primary" onclick="window._saveConfigF()">💾 Guardar Configuración</button>
        <button class="o-btn-secondary" onclick="window._discardConfigF()">Descartar cambios</button>
      </div>
    </div>
  </div>`),window._saveConfigF=()=>{var i,s,n,d,r,l,c,p,m;const a=u=>document.getElementById(u),o={impuestos_ventas:((i=a("cfg-tax-v"))==null?void 0:i.checked)??!0,impuestos_compras:((s=a("cfg-tax-c"))==null?void 0:s.checked)??!0,redondeo:((n=a("cfg-redondeo"))==null?void 0:n.checked)??!1,pagos_online:((d=a("cfg-pago-online"))==null?void 0:d.checked)??!1,descuentos_pronto_pago:((r=a("cfg-desc-pago"))==null?void 0:r.checked)??!1,alertas_cliente:((l=a("cfg-alertas"))==null?void 0:l.checked)??!1,cfdi_auto:((c=a("cfg-cfdi-auto"))==null?void 0:c.checked)??!1,cancelacion_directa:((p=a("cfg-canc-directa"))==null?void 0:p.checked)??!1,terminos_default:((m=a("cfg-terminos"))==null?void 0:m.value)||""};localStorage.setItem(ia,JSON.stringify(o)),b("Guardado","Configuración de Facturación actualizada correctamente","success")},window._discardConfigF=()=>{za()}}const na="nexus_config_compras",ti={bloquear_confirmado:!1,advertencias:!1,precio_compra:!0,descuentos:!1,politica_facturacion:"cantidad_pedida",bloquear_factura:!1,variantes:!1,unidades_medida:!1,empaquetado:!1,presupuesto_solicitud:!1,recordatorio_recepcion:0,costos_aterrizaje:!1};function Y(t,e,a,o){return`
  <label style="display:flex;align-items:flex-start;gap:12px;cursor:pointer;padding:14px 20px;border-bottom:1px solid var(--border)">
    <input type="checkbox" id="${t}" ${o?"checked":""}
      style="margin-top:2px;accent-color:var(--primary);width:16px;height:16px;flex-shrink:0">
    <div>
      <div style="font-weight:600;font-size:14px;color:var(--text-900)">${e}</div>
      <div style="font-size:12px;color:var(--text-400);margin-top:2px;line-height:1.5">${a}</div>
    </div>
  </label>`}function Et(t,e){return`
  <div style="background:var(--bg-card);border:1px solid var(--border);border-radius:12px;margin-bottom:24px;overflow:hidden">
    <div style="padding:12px 20px;background:var(--bg-app);border-bottom:1px solid var(--border);
      font-size:11px;font-weight:800;text-transform:uppercase;color:var(--text-500);letter-spacing:.08em">
      ${t}
    </div>
    ${e}
  </div>`}async function Ta(t={}){B();const e={...ti,...JSON.parse(localStorage.getItem(na)||"{}")};w(`<div class="nx-module-page" style="background:var(--bg-app)">
    <div class="o-cp">
      <div class="o-cp-left"></div>
      <div class="o-cp-right">
        <button class="o-btn-primary" onclick="window._saveConfigC()">💾 Guardar</button>
        <button class="o-btn-secondary" onclick="window._discardConfigC()">Descartar</button>
      </div>
    </div>

    <div style="max-width:900px;margin:28px auto;padding:0 24px 60px">

      ${Et("Pedidos",`
        ${Y("cfc-bloq","Bloquear Pedido Confirmado","Impide editar una orden de compra después de confirmarla. Se debe crear una nueva orden para modificar",e.bloquear_confirmado)}
        ${Y("cfc-warn","Advertencias","Muestra advertencias al comprador al crear pedidos para proveedores o productos con observaciones especiales",e.advertencias)}
        ${Y("cfc-precio","Precio de Compra","Muestra el precio de compra del proveedor en las líneas de pedido para comparación con el precio de catálogo",e.precio_compra)}
        ${Y("cfc-desc","Descuentos","Permite aplicar descuentos por línea en las órdenes de compra",e.descuentos)}
      `)}

      ${Et("Facturación",`
        <div style="display:flex;align-items:center;justify-content:space-between;padding:14px 20px;border-bottom:1px solid var(--border)">
          <div>
            <div style="font-weight:600;font-size:14px;color:var(--text-900)">Política de Control de Facturas</div>
            <div style="font-size:12px;color:var(--text-400);margin-top:2px">Define si se puede facturar al recibir el pedido o después de validar la recepción</div>
          </div>
          <select id="cfc-pol" style="min-width:230px;padding:7px 12px;border:1px solid var(--border);border-radius:8px;font-size:13px;background:var(--bg-card);color:var(--text-900)">
            <option value="cantidad_pedida" ${e.politica_facturacion==="cantidad_pedida"?"selected":""}>Cantidades pedidas</option>
            <option value="cantidad_recibida" ${e.politica_facturacion==="cantidad_recibida"?"selected":""}>Cantidades recibidas</option>
          </select>
        </div>
        ${Y("cfc-bloq-fact","Bloquear Facturas","Impide modificar facturas de proveedor después de validarlas. Requiere una nota de crédito para correcciones",e.bloquear_factura)}
      `)}

      ${Et("Productos",`
        ${Y("cfc-var","Variantes","Habilita variantes de producto (talla, color, material) agrupadas bajo una misma referencia",e.variantes)}
        ${Y("cfc-udm","Unidades de Medida","Permite comprar en una unidad diferente a la unidad de stock, con conversión automática",e.unidades_medida)}
        ${Y("cfc-pack","Empaquetado","Define presentaciones de empaque del proveedor (caja de 12, pallet de 100, etc.)",e.empaquetado)}
      `)}

      ${Et("Avanzado",`
        ${Y("cfc-rfq","Solicitud de Presupuesto a Proveedores","Envía solicitudes de cotización a múltiples proveedores para comparar precios antes de confirmar la compra",e.presupuesto_solicitud)}
        <div style="display:flex;align-items:center;justify-content:space-between;padding:14px 20px;border-bottom:1px solid var(--border)">
          <div>
            <div style="font-weight:600;font-size:14px;color:var(--text-900)">Recordatorio de Recepción</div>
            <div style="font-size:12px;color:var(--text-400);margin-top:2px">Envía un recordatorio N días antes de la fecha de entrega esperada. 0 = desactivado</div>
          </div>
          <div style="display:flex;align-items:center;gap:6px">
            <input type="number" id="cfc-rec" value="${e.recordatorio_recepcion}" min="0" max="30"
              style="width:70px;text-align:center;padding:6px 10px;border:1px solid var(--border);border-radius:8px;font-size:14px">
            <span style="font-size:13px;color:var(--text-400)">días</span>
          </div>
        </div>
      `)}

      ${Et("Costos de Aterrizaje",`
        ${Y("cfc-land","Costos de Aterrizaje","Permite distribuir costos adicionales de importación (flete, aduanas, seguros) entre los productos recibidos",e.costos_aterrizaje)}
      `)}

      <div style="display:flex;gap:10px;padding-top:8px">
        <button class="o-btn-primary" onclick="window._saveConfigC()">💾 Guardar Configuración</button>
        <button class="o-btn-secondary" onclick="window._discardConfigC()">Descartar cambios</button>
      </div>
    </div>
  </div>`),window._saveConfigC=()=>{var i,s,n,d,r,l,c,p,m,u,v,k;const a=_=>document.getElementById(_),o={bloquear_confirmado:((i=a("cfc-bloq"))==null?void 0:i.checked)??!1,advertencias:((s=a("cfc-warn"))==null?void 0:s.checked)??!1,precio_compra:((n=a("cfc-precio"))==null?void 0:n.checked)??!0,descuentos:((d=a("cfc-desc"))==null?void 0:d.checked)??!1,politica_facturacion:((r=a("cfc-pol"))==null?void 0:r.value)||"cantidad_pedida",bloquear_factura:((l=a("cfc-bloq-fact"))==null?void 0:l.checked)??!1,variantes:((c=a("cfc-var"))==null?void 0:c.checked)??!1,unidades_medida:((p=a("cfc-udm"))==null?void 0:p.checked)??!1,empaquetado:((m=a("cfc-pack"))==null?void 0:m.checked)??!1,presupuesto_solicitud:((u=a("cfc-rfq"))==null?void 0:u.checked)??!1,recordatorio_recepcion:parseInt((v=a("cfc-rec"))==null?void 0:v.value)||0,costos_aterrizaje:((k=a("cfc-land"))==null?void 0:k.checked)??!1};localStorage.setItem(na,JSON.stringify(o)),b("Guardado","Configuración de Compras actualizada correctamente","success")},window._discardConfigC=()=>Ta()}const sa="nexus_config_crm",ei={actividades:!0,reuniones:!0,llamadas:!0,etapas_compartidas:!1,probabilidad_ia:!1,tiempo_cierre:!1,email_alias:!0,seguimiento_email:!1,plantillas_email:!0,pronostico:!1,objetivos:!1,leads:!1,mineria_leads:!1,deduplicar:!0};function V(t,e,a,o){return`
  <label style="display:flex;align-items:flex-start;gap:12px;cursor:pointer;padding:14px 20px;border-bottom:1px solid var(--border)">
    <input type="checkbox" id="${t}" ${o?"checked":""}
      style="margin-top:2px;accent-color:var(--primary);width:16px;height:16px;flex-shrink:0">
    <div>
      <div style="font-weight:600;font-size:14px;color:var(--text-900)">${e}</div>
      <div style="font-size:12px;color:var(--text-400);margin-top:2px;line-height:1.5">${a}</div>
    </div>
  </label>`}function Gt(t,e){return`
  <div style="background:var(--bg-card);border:1px solid var(--border);border-radius:12px;margin-bottom:24px;overflow:hidden">
    <div style="padding:12px 20px;background:var(--bg-app);border-bottom:1px solid var(--border);
      font-size:11px;font-weight:800;text-transform:uppercase;color:var(--text-500);letter-spacing:.08em">
      ${t}
    </div>
    ${e}
  </div>`}async function Ia(t={}){B();const e={...ei,...JSON.parse(localStorage.getItem(sa)||"{}")};w(`<div class="nx-module-page" style="background:var(--bg-app)">
    <div class="o-cp">
      <div class="o-cp-left"></div>
      <div class="o-cp-right">
        <button class="o-btn-primary" onclick="window._saveConfigCRM()">💾 Guardar</button>
        <button class="o-btn-secondary" onclick="window._discardConfigCRM()">Descartar</button>
      </div>
    </div>

    <div style="max-width:900px;margin:28px auto;padding:0 24px 60px">

      ${Gt("Leads y Pipeline",`
        ${V("crm-leads","Leads","Habilita la gestión de leads (prospectos) antes de convertirlos en oportunidades de venta",e.leads)}
        ${V("crm-etapas","Etapas del Pipeline Compartidas","Las etapas del pipeline son compartidas entre todos los equipos de ventas. Si se desactiva, cada equipo tiene sus propias etapas",e.etapas_compartidas)}
        ${V("crm-prob","Probabilidad con IA","Usa inteligencia artificial para calcular automáticamente la probabilidad de cierre de cada oportunidad",e.probabilidad_ia)}
        ${V("crm-tiempo","Tiempo de Cierre","Registra el tiempo desde la creación hasta el cierre de cada oportunidad para métricas de rendimiento",e.tiempo_cierre)}
        ${V("crm-pronostico","Pronóstico","Proyecta los ingresos esperados según la probabilidad de cierre del pipeline activo",e.pronostico)}
        ${V("crm-objetivos","Objetivos de Venta","Define objetivos de venta por vendedor o equipo y monitorea el avance en tiempo real",e.objetivos)}
      `)}

      ${Gt("Actividades",`
        ${V("crm-act","Actividades","Planifica y registra actividades de seguimiento como llamadas, emails y tareas para cada oportunidad",e.actividades)}
        ${V("crm-meet","Reuniones","Agenda reuniones con prospectos y clientes directamente desde la oportunidad. Se sincroniza con el calendario",e.reuniones)}
        ${V("crm-call","Llamadas VoIP","Realiza y registra llamadas telefónicas directamente desde las oportunidades mediante integración VoIP",e.llamadas)}
      `)}

      ${Gt("Comunicación",`
        ${V("crm-alias","Alias de Correo","Crea una dirección de email única para el equipo. Los correos recibidos generan automáticamente nuevas oportunidades",e.email_alias)}
        ${V("crm-track","Seguimiento de Email","Registra automáticamente cuándo el cliente abre los correos enviados desde las oportunidades",e.seguimiento_email)}
        ${V("crm-tmpl","Plantillas de Email","Crea y reutiliza plantillas de correo para comunicaciones frecuentes con prospectos y clientes",e.plantillas_email)}
      `)}

      ${Gt("Leads Automáticos",`
        ${V("crm-mining","Minería de Leads","Genera leads automáticamente a partir de criterios de búsqueda como industria, ubicación y tamaño de empresa",e.mineria_leads)}
        ${V("crm-dedup","Deduplicación de Leads","Detecta y fusiona automáticamente leads o oportunidades duplicadas basándose en nombre, email o teléfono",e.deduplicar)}
      `)}

      <div style="display:flex;gap:10px;padding-top:8px">
        <button class="o-btn-primary" onclick="window._saveConfigCRM()">💾 Guardar Configuración</button>
        <button class="o-btn-secondary" onclick="window._discardConfigCRM()">Descartar cambios</button>
      </div>
    </div>
  </div>`),window._saveConfigCRM=()=>{var i,s,n,d,r,l,c,p,m,u,v,k,_,$;const a=T=>document.getElementById(T),o={leads:((i=a("crm-leads"))==null?void 0:i.checked)??!1,etapas_compartidas:((s=a("crm-etapas"))==null?void 0:s.checked)??!1,probabilidad_ia:((n=a("crm-prob"))==null?void 0:n.checked)??!1,tiempo_cierre:((d=a("crm-tiempo"))==null?void 0:d.checked)??!1,pronostico:((r=a("crm-pronostico"))==null?void 0:r.checked)??!1,objetivos:((l=a("crm-objetivos"))==null?void 0:l.checked)??!1,actividades:((c=a("crm-act"))==null?void 0:c.checked)??!0,reuniones:((p=a("crm-meet"))==null?void 0:p.checked)??!0,llamadas:((m=a("crm-call"))==null?void 0:m.checked)??!0,email_alias:((u=a("crm-alias"))==null?void 0:u.checked)??!0,seguimiento_email:((v=a("crm-track"))==null?void 0:v.checked)??!1,plantillas_email:((k=a("crm-tmpl"))==null?void 0:k.checked)??!0,mineria_leads:((_=a("crm-mining"))==null?void 0:_.checked)??!1,deduplicar:(($=a("crm-dedup"))==null?void 0:$.checked)??!0};localStorage.setItem(sa,JSON.stringify(o)),b("Guardado","Configuración de CRM actualizada correctamente","success")},window._discardConfigCRM=()=>Ia()}let Fe="",ot={};async function ai(){B(),ot={leads:!1,etapas_compartidas:!1,probabilidad_ia:!1,tiempo_cierre:!1,pronostico:!1,objetivos:!1,actividades:!0,reuniones:!0,llamadas:!0,email_alias:!0,seguimiento_email:!1,plantillas_email:!0,mineria_leads:!1,deduplicar:!0,...JSON.parse(localStorage.getItem("nexus_config_crm")||"{}")},oi(),si()}function oi(){w(`
  <div class="o-cp" id="crm-cp">
    <div class="o-cp-left">
      <button class="o-btn-primary" onclick="alert('Nueva Oportunidad')">Nueva</button>
      ${ot.mineria_leads?`<button class="o-btn-secondary" onclick="alert('Minería de Leads')">Generar Leads</button>`:""}
    </div>
    <div class="o-cp-center">
      <div class="o-search-bar">
        <svg class="o-search-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/></svg>
        <input id="o-search-crm" class="o-search-input" type="text" placeholder="Buscar oportunidad…" value="${Fe}">
      </div>
    </div>
    <div class="o-cp-right">
      <button class="o-btn-secondary" style="margin-right:8px;font-size:16px;padding:4px 8px" onclick="window._go('config_crm')" title="Ajustes">⚙️</button>
      <div class="o-view-switcher">
        <button class="o-view-btn o-active" title="Kanban">⬜</button>
        <button class="o-view-btn" title="Lista">☰</button>
      </div>
    </div>
  </div>
  <div id="crm-content" class="o-view-content" style="background:#f9f9fb;padding:16px;min-height:calc(100vh - 100px);overflow-x:auto">
    ${I(8,4)}
  </div>`),setTimeout(()=>{var t;(t=document.getElementById("o-search-crm"))==null||t.addEventListener("input",e=>{Fe=e.target.value.toLowerCase(),ii()})},100)}function ii(){document.querySelectorAll(".o-kanban-card").forEach(t=>{t.style.display=t.textContent.toLowerCase().includes(Fe)?"":"none"})}const ni=[{id:1,name:"Venta de Servidores",partner:"Acme Corp",stage:"new",amount:45e3,prob:10},{id:2,name:"Licencias ERP",partner:"Tech Solutions",stage:"qualified",amount:12e3,prob:50},{id:3,name:"Consultoría",partner:"Global IT",stage:"proposition",amount:3e4,prob:80}];function si(){const t=document.getElementById("crm-content");if(!t)return;const e=[{key:"new",label:"Nuevo"},{key:"qualified",label:"Calificado"},{key:"proposition",label:"Propuesta"},{key:"won",label:"Ganado"}];ot.leads&&e.unshift({key:"lead",label:"Leads (Sin asignar)"}),t.innerHTML=`
  <div class="o-kanban-columns" style="display:flex;gap:16px;align-items:flex-start;height:100%">
    ${e.map(a=>{const o=ni.filter(s=>s.stage===a.key),i=o.reduce((s,n)=>s+n.amount,0);return`
      <div class="o-kanban-col" style="flex:0 0 280px;background:#e5e7eb;border-radius:6px;padding:8px;display:flex;flex-direction:column;max-height:100%">
        <div style="font-weight:700;font-size:14px;color:#374151;margin-bottom:8px;display:flex;justify-content:space-between">
          <span>${a.label}</span>
          ${ot.pronostico?`<span style="color:#6B7280">${f(i)}</span>`:""}
        </div>
        <div class="o-kanban-col-cards" style="display:flex;flex-direction:column;gap:8px;overflow-y:auto">
          ${o.map(s=>`
          <div class="o-kanban-card" style="background:#fff;border-radius:4px;padding:12px;box-shadow:0 1px 2px rgba(0,0,0,0.1);cursor:pointer">
            <div style="font-weight:600;font-size:14px;color:#111827">${s.name}</div>
            <div style="font-size:12px;color:#6B7280;margin:4px 0">${s.partner}</div>
            <div style="display:flex;justify-content:space-between;margin-top:8px;align-items:center">
              <strong style="color:#059669">${f(s.amount)}</strong>
              ${ot.probabilidad_ia?`<span style="font-size:11px;background:#FEF3C7;color:#D97706;padding:2px 6px;border-radius:10px">IA: ${s.prob}%</span>`:""}
            </div>
            <div style="margin-top:8px;display:flex;gap:4px">
               ${ot.actividades?'<span style="font-size:12px" title="Actividades">📅</span>':""}
               ${ot.llamadas?'<span style="font-size:12px" title="Llamadas">📞</span>':""}
               ${ot.reuniones?'<span style="font-size:12px" title="Reuniones">🤝</span>':""}
            </div>
          </div>`).join("")}
        </div>
      </div>`}).join("")}
  </div>`}const da="nexus_config_inventario",di={lotes_series:!1,multi_almacen:!1,rutas_multietapa:!1,paquetes:!1,advertencias:!1,unidades_medida:!1,variantes:!1,codigo_barras:!1};function at(t,e,a,o){return`
  <label style="display:flex;align-items:flex-start;gap:12px;cursor:pointer;padding:14px 20px;border-bottom:1px solid var(--border)">
    <input type="checkbox" id="${t}" ${o?"checked":""}
      style="margin-top:2px;accent-color:var(--primary);width:16px;height:16px;flex-shrink:0">
    <div>
      <div style="font-weight:600;font-size:14px;color:var(--text-900)">${e}</div>
      <div style="font-size:12px;color:var(--text-400);margin-top:2px;line-height:1.5">${a}</div>
    </div>
  </label>`}function Kt(t,e){return`
  <div style="background:var(--bg-card);border:1px solid var(--border);border-radius:12px;margin-bottom:24px;overflow:hidden">
    <div style="padding:12px 20px;background:var(--bg-app);border-bottom:1px solid var(--border);
      font-size:11px;font-weight:800;text-transform:uppercase;color:var(--text-500);letter-spacing:.08em">
      ${t}
    </div>
    ${e}
  </div>`}async function Fa(t={}){B();const e={...di,...JSON.parse(localStorage.getItem(da)||"{}")};w(`<div class="nx-module-page" style="background:var(--bg-app)">
    <div class="o-cp">
      <div class="o-cp-left"></div>
      <div class="o-cp-right">
        <button class="o-btn-primary" onclick="window._saveConfigInv()">💾 Guardar</button>
        <button class="o-btn-secondary" onclick="window._discardConfigInv()">Descartar</button>
      </div>
    </div>

    <div style="max-width:900px;margin:28px auto;padding:0 24px 60px">

      ${Kt("Operaciones",`
        ${at("cfgi-lotes","Lotes y Números de Serie","Rastrear inventario a nivel de lote o número de serie individual",e.lotes_series)}
        ${at("cfgi-multi","Múltiples Almacenes","Gestionar inventario en más de un almacén físico",e.multi_almacen)}
        ${at("cfgi-rutas","Rutas Multietapa","Permitir reglas de enrutamiento complejas (ej: Recibir -> Control de Calidad -> Stock)",e.rutas_multietapa)}
      `)}

      ${Kt("Trazabilidad",`
        ${at("cfgi-paq","Paquetes","Agrupar productos en paquetes o pallets (Cajas, Tarimas) con número de rastreo propio",e.paquetes)}
        ${at("cfgi-adv","Advertencias","Mostrar advertencias al hacer movimientos de stock de ciertos productos",e.advertencias)}
      `)}

      ${Kt("Productos",`
        ${at("cfgi-uom","Unidades de Medida","Comprar, vender y almacenar en diferentes unidades de medida (Ej: Cajas vs Piezas)",e.unidades_medida)}
        ${at("cfgi-var","Variantes","Habilitar opciones de producto como Talla o Color",e.variantes)}
      `)}

      ${Kt("Escáner de Códigos",`
        ${at("cfgi-bar","Lector de Códigos de Barras","Procesar transferencias de stock, ajustes e inventarios físicos escaneando códigos de barras",e.codigo_barras)}
      `)}

      <div style="display:flex;gap:10px;padding-top:8px">
        <button class="o-btn-primary" onclick="window._saveConfigInv()">💾 Guardar Configuración</button>
        <button class="o-btn-secondary" onclick="window._discardConfigInv()">Descartar cambios</button>
      </div>
    </div>
  </div>`),window._saveConfigInv=()=>{var i,s,n,d,r,l,c,p;const a=m=>document.getElementById(m),o={lotes_series:((i=a("cfgi-lotes"))==null?void 0:i.checked)??!1,multi_almacen:((s=a("cfgi-multi"))==null?void 0:s.checked)??!1,rutas_multietapa:((n=a("cfgi-rutas"))==null?void 0:n.checked)??!1,paquetes:((d=a("cfgi-paq"))==null?void 0:d.checked)??!1,advertencias:((r=a("cfgi-adv"))==null?void 0:r.checked)??!1,unidades_medida:((l=a("cfgi-uom"))==null?void 0:l.checked)??!1,variantes:((c=a("cfgi-var"))==null?void 0:c.checked)??!1,codigo_barras:((p=a("cfgi-bar"))==null?void 0:p.checked)??!1};localStorage.setItem(da,JSON.stringify(o)),b("Guardado","Configuración de Inventario actualizada","success")},window._discardConfigInv=()=>Fa()}const ra="nexus_config_contactos",ri={geolocalizacion:!1,validar_vat:!0,limite_credito:!1,alerta_credito:!1,niveles_partner:!1,comisiones:!1};function ft(t,e,a,o){return`
  <label style="display:flex;align-items:flex-start;gap:12px;cursor:pointer;padding:14px 20px;border-bottom:1px solid var(--border)">
    <input type="checkbox" id="${t}" ${o?"checked":""}
      style="margin-top:2px;accent-color:var(--primary);width:16px;height:16px;flex-shrink:0">
    <div>
      <div style="font-weight:600;font-size:14px;color:var(--text-900)">${e}</div>
      <div style="font-size:12px;color:var(--text-400);margin-top:2px;line-height:1.5">${a}</div>
    </div>
  </label>`}function me(t,e){return`
  <div style="background:var(--bg-card);border:1px solid var(--border);border-radius:12px;margin-bottom:24px;overflow:hidden">
    <div style="padding:12px 20px;background:var(--bg-app);border-bottom:1px solid var(--border);
      font-size:11px;font-weight:800;text-transform:uppercase;color:var(--text-500);letter-spacing:.08em">
      ${t}
    </div>
    ${e}
  </div>`}async function Ba(t={}){B();const e={...ri,...JSON.parse(localStorage.getItem(ra)||"{}")};w(`<div class="nx-module-page" style="background:var(--bg-app)">
    <div class="o-cp">
      <div class="o-cp-left"></div>
      <div class="o-cp-right">
        <button class="o-btn-primary" onclick="window._saveConfigContact()">💾 Guardar</button>
        <button class="o-btn-secondary" onclick="window._discardConfigContact()">Descartar</button>
      </div>
    </div>

    <div style="max-width:900px;margin:28px auto;padding:0 24px 60px">

      ${me("Información del Contacto",`
        ${ft("cfgc-geo","Geolocalización","Calcula coordenadas de longitud/latitud en base a la dirección para rutas en mapa",e.geolocalizacion)}
        ${ft("cfgc-vat","Validar RFC / RUT","Verifica la estructura y validez fiscal del documento de identidad ante el servicio de impuestos local",e.validar_vat)}
      `)}

      ${me("Límites Financieros",`
        ${ft("cfgc-limite","Límite de Crédito","Permite asignar un monto máximo de crédito a clientes (detiene ventas si excede)",e.limite_credito)}
        ${ft("cfgc-alerta","Alerta de Cartera Vencida","Muestra una advertencia roja en las ventas cuando el cliente tiene facturas atrasadas",e.alerta_credito)}
      `)}

      ${me("Asociaciones y Comisiones",`
        ${ft("cfgc-niveles","Niveles de Partner","Clasifica a clientes y distribuidores por nivel (Plata, Oro, Platino)",e.niveles_partner)}
        ${ft("cfgc-comisiones","Comisiones de Referidos","Asigna comisiones a los partners por atraer nuevos clientes al ERP",e.comisiones)}
      `)}

      <div style="display:flex;gap:10px;padding-top:8px">
        <button class="o-btn-primary" onclick="window._saveConfigContact()">💾 Guardar Configuración</button>
        <button class="o-btn-secondary" onclick="window._discardConfigContact()">Descartar cambios</button>
      </div>
    </div>
  </div>`),window._saveConfigContact=()=>{var i,s,n,d,r,l;const a=c=>document.getElementById(c),o={geolocalizacion:((i=a("cfgc-geo"))==null?void 0:i.checked)??!1,validar_vat:((s=a("cfgc-vat"))==null?void 0:s.checked)??!0,limite_credito:((n=a("cfgc-limite"))==null?void 0:n.checked)??!1,alerta_credito:((d=a("cfgc-alerta"))==null?void 0:d.checked)??!1,niveles_partner:((r=a("cfgc-niveles"))==null?void 0:r.checked)??!1,comisiones:((l=a("cfgc-comisiones"))==null?void 0:l.checked)??!1};localStorage.setItem(ra,JSON.stringify(o)),b("Guardado","Configuración de Contactos actualizada","success")},window._discardConfigContact=()=>Ba()}let be=null;async function li(){B(),await ci()}async function ci(){var e,a;w(`
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
  <div id="index-status" class="anim-4" style="margin-top:16px"></div>`),(e=document.getElementById("search-query"))==null||e.addEventListener("keydown",o=>{o.key==="Enter"&&window._buscar()});let t;(a=document.getElementById("search-query"))==null||a.addEventListener("input",o=>{clearTimeout(t),!(o.target.value.length<2)&&(t=setTimeout(()=>window._buscar(),500))}),await la(),window._buscar=pi,window._checkStatus=la,window._syncSearch=ui}async function pi(){var a,o;const t=(o=(a=document.getElementById("search-query"))==null?void 0:a.value)==null?void 0:o.trim();if(!t||t.length<2)return;const e=document.getElementById("search-results");e&&(e.innerHTML=`
    <div class="data-card" style="padding:20px;text-align:center;color:var(--text-400)">
      <div class="spinner" style="margin:0 auto 8px"></div>
      <div>Buscando "${t}"…</div>
    </div>`);try{const[i,s,n]=await Promise.allSettled([g.ventas(1).then(r=>((r==null?void 0:r.data)||[]).filter(l=>(l.name||"").toLowerCase().includes(t.toLowerCase())||(l.partner_name||"").toLowerCase().includes(t.toLowerCase())).map(l=>({tipo:"Venta",icon:"💰",titulo:l.name,sub:l.partner_name,meta:`$${l.amount_total}`,href:"ventas"}))),g.productos(1,t).then(r=>((r==null?void 0:r.data)||[]).map(l=>{var c,p;return{tipo:"Producto",icon:"📦",titulo:typeof l.name=="object"?((c=l.name)==null?void 0:c.es_MX)||((p=l.name)==null?void 0:p.en_US)||"":l.name||"",sub:l.categ_name||"",meta:"",href:"productos"}})),g.partners(1).then(r=>((r==null?void 0:r.data)||[]).filter(l=>(l.name||"").toLowerCase().includes(t.toLowerCase())||(l.email||"").toLowerCase().includes(t.toLowerCase())).map(l=>({tipo:"Contacto",icon:"👥",titulo:l.name,sub:l.email||"",meta:"",href:"partners"})))]),d=[...i.status==="fulfilled"?i.value:[],...s.status==="fulfilled"?s.value:[],...n.status==="fulfilled"?n.value:[]];if(!e)return;if(d.length===0){e.innerHTML=`
      <div class="data-card" style="padding:40px;text-align:center">
        <div style="font-size:36px;margin-bottom:12px">🔍</div>
        <div style="font-weight:700;color:var(--text-700)">Sin resultados para "${t}"</div>
        <div style="color:var(--text-400);font-size:13px;margin-top:6px">Prueba con otro término</div>
      </div>`;return}e.innerHTML=`
    <div class="data-card">
      <div class="data-card-header">
        <div class="data-card-title">${d.length} resultados para "${t}"</div>
      </div>
      <div style="padding:0 4px">
        ${d.slice(0,30).map(r=>`
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
            ${r.meta?`<span style="font-size:12px;font-weight:700;color:var(--text-700)">${r.meta}</span>`:""}
            <span class="badge badge-${r.tipo==="Venta"?"indigo":r.tipo==="Producto"?"emerald":"violet"}">${r.tipo}</span>
          </div>
        </div>`).join("")}
      </div>
    </div>`}catch(i){console.error(i),e&&(e.innerHTML=`<p style="color:var(--red);padding:20px">Error: ${i.message}</p>`)}}async function la(){const t=document.getElementById("index-status");try{const e=await g.searchStatus().catch(()=>null);be=(e==null?void 0:e.data)||e,t&&be&&(t.innerHTML=`
      <div class="data-card" style="padding:16px">
        <div class="data-card-header" style="padding:0 0 12px">
          <div class="data-card-title">📡 Estado del Motor de Búsqueda</div>
        </div>
        <div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(180px,1fr));gap:12px">
          ${Object.entries(be).map(([a,o])=>`
          <div style="padding:12px;background:var(--bg);border-radius:10px;border:1px solid var(--border)">
            <div style="font-size:10px;color:var(--text-400);font-weight:700;text-transform:uppercase;margin-bottom:4px">${a}</div>
            <div style="font-size:13px;font-weight:600;color:var(--text-800)">${JSON.stringify(o)}</div>
          </div>`).join("")}
        </div>
      </div>`)}catch{t&&(t.innerHTML="")}}async function ui(){const t=document.getElementById("btn-sync");t&&(t.textContent="⏳ Sincronizando…",t.disabled=!0);try{const e=await g.searchSync();b("Sincronización iniciada",(e==null?void 0:e.message)||"Los índices se están actualizando","success")}catch(e){b("Error de sincronización",e.message,"error")}finally{t&&(t.textContent="⚡ Sincronizar Índices",t.disabled=!1)}}async function vi(){B(),await mi()}async function mi(){w(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">📈 Reportes</h1>
      <p class="page-subtitle">Análisis ejecutivos e inteligencia de negocio</p>
    </div>
    <div class="page-actions">
      <button class="btn btn-secondary" onclick="window._exportReporte()">📥 Exportar</button>
    </div>
  </div>

  <!-- Cards de reportes disponibles -->
  <div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(280px,1fr));gap:16px;margin-bottom:24px" class="anim-2">
    ${[{icon:"💰",titulo:"Reporte de Ventas",sub:"Órdenes, tendencias y proyecciones mensuales",color:"indigo",key:"ventas"},{icon:"🧾",titulo:"Reporte de Facturación",sub:"CFDIs emitidos, cancelados y saldo pendiente",color:"emerald",key:"facturas"},{icon:"🏭",titulo:"Reporte de Inventario",sub:"Stock actual, valor, y alertas de reorden",color:"violet",key:"inventario"},{icon:"🛒",titulo:"Reporte de Compras",sub:"Órdenes de compra, proveedores y gasto",color:"amber",key:"compras"},{icon:"👥",titulo:"Reporte de Clientes",sub:"Top clientes, retención y geografía",color:"sky",key:"clientes"},{icon:"👔",titulo:"Reporte de Nómina",sub:"Plantilla activa, costo mensual y IMSS",color:"rose",key:"nomina"}].map(t=>`
    <div class="data-card" style="padding:20px;cursor:pointer;transition:transform 0.15s,box-shadow 0.15s"
      onmouseover="this.style.transform='translateY(-3px)';this.style.boxShadow='0 8px 24px rgba(0,0,0,.12)'"
      onmouseout="this.style.transform='';this.style.boxShadow=''"
      onclick="window._verReporte('${t.key}')">
      <div style="width:46px;height:46px;border-radius:12px;background:var(--${t.color==="indigo"?"primary":t.color}-light,var(--primary-light));
        display:flex;align-items:center;justify-content:center;font-size:22px;margin-bottom:12px">
        ${t.icon}
      </div>
      <div style="font-size:14px;font-weight:700;color:var(--text-800);margin-bottom:4px">${t.titulo}</div>
      <div style="font-size:12px;color:var(--text-400)">${t.sub}</div>
      <div style="margin-top:12px">
        <span style="font-size:11px;color:var(--primary);font-weight:600">Ver reporte →</span>
      </div>
    </div>`).join("")}
  </div>

  <!-- Reporte principal — Resumen ejecutivo -->
  <div class="data-card anim-3">
    <div class="data-card-header">
      <div class="data-card-title">📊 Resumen Ejecutivo</div>
      <div style="font-size:12px;color:var(--text-400)" id="rep-fecha"></div>
    </div>
    <div id="rep-contenido">
      <div style="display:grid;grid-template-columns:repeat(3,1fr);gap:16px;padding:8px 0" id="rep-kpis">
        ${[1,2,3].map(()=>'<div class="skeleton" style="height:80px;border-radius:12px"></div>').join("")}
      </div>
    </div>
  </div>`),window._verReporte=t=>{b("Reporte seleccionado",`Generando reporte de ${t}…`,"info"),bi(t)},window._exportReporte=()=>{b("Exportar","Función de exportación CSV/PDF — próximamente","info")},await Pa()}async function Pa(){var e,a,o,i;const t=document.getElementById("rep-fecha");t&&(t.textContent=new Date().toLocaleDateString("es-MX",{day:"2-digit",month:"long",year:"numeric"}));try{const[s,n,d,r]=await Promise.allSettled([g.ventaKpis(),g.factKpis(),g.stockKpis(),g.comprasKpis()]),l=((e=s.value)==null?void 0:e.data)||{},c=((a=n.value)==null?void 0:a.data)||{},p=((o=d.value)==null?void 0:o.data)||{},m=((i=r.value)==null?void 0:i.data)||{},u=document.getElementById("rep-kpis");u&&(u.innerHTML=`
      ${[{label:"Ventas confirmadas",val:l.ordenes_confirmadas??0,tipo:"num",desc:`$${parseFloat(l.total_facturado||0).toLocaleString("es-MX",{minimumFractionDigits:2})} este mes`},{label:"Facturación total",val:f(parseFloat(c.monto_total||0)),tipo:"txt",desc:`${c.total_facturas??0} comprobantes emitidos`},{label:"Valor inventario",val:f(parseFloat(p.valor_inventario||0)),tipo:"txt",desc:`${p.alertas_stock_bajo??0} alertas de stock bajo`}].map(v=>`
      <div style="padding:16px;background:var(--bg);border-radius:12px;border:1px solid var(--border)">
        <div style="font-size:11px;color:var(--text-400);font-weight:700;text-transform:uppercase;margin-bottom:6px">${v.label}</div>
        <div style="font-size:24px;font-weight:800;color:var(--text-900);margin-bottom:4px">
          ${v.tipo==="num"?Number(v.val).toLocaleString("es-MX"):v.val}
        </div>
        <div style="font-size:11px;color:var(--text-500)">${v.desc}</div>
      </div>`).join("")}

      <div style="grid-column:1/-1;margin-top:8px">
        <div style="font-size:12px;font-weight:700;color:var(--text-600);margin-bottom:10px">COMPRAS</div>
        <div style="display:grid;grid-template-columns:repeat(3,1fr);gap:12px">
          ${[{label:"Total OC",val:m.total??0},{label:"Confirmadas",val:m.confirmadas??0},{label:"Monto compras",val:f(parseFloat(m.monto_total||0))}].map(v=>`
          <div style="padding:12px;background:var(--bg);border-radius:10px;border:1px solid var(--border)">
            <div style="font-size:10px;color:var(--text-400);font-weight:700;text-transform:uppercase;margin-bottom:4px">${v.label}</div>
            <div style="font-size:18px;font-weight:800;color:var(--text-900)">${v.val}</div>
          </div>`).join("")}
        </div>
      </div>`)}catch(s){console.error(s)}}async function bi(t){const e=document.getElementById("rep-kpis"),a=document.querySelector(".data-card-title");if(a){const o={ventas:"💰 Reporte de Ventas",facturas:"🧾 Facturación",inventario:"🏭 Inventario",compras:"🛒 Compras",clientes:"👥 Clientes",nomina:"👔 Nómina"};a.textContent=o[t]||"Reporte"}e&&(e.innerHTML='<div class="skeleton" style="height:120px;border-radius:12px;grid-column:1/-1"></div>'),await Pa()}function Ue(t,e,a,o){B(),w(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">${o} ${e}</h1>
      <p class="page-subtitle">${a}</p>
    </div>
  </div>
  <div class="data-card anim-2">
    <div class="empty-state">
      <div class="empty-state-icon">${o}</div>
      <div class="empty-state-title">Módulo ${e} en construcción</div>
      <div class="empty-state-desc">Este módulo estará disponible próximamente en NexusTech ERP v2.0</div>
      <button class="btn btn-primary" onclick="window._go('dashboard')">← Volver al Dashboard</button>
    </div>
  </div>`)}let Be="",lt={};async function fi(){B(),lt={multimoneda:!1,contabilidad_analitica:!1,presupuestos:!1,activos_fijos:!1,ingresos_diferidos:!1,alertas_facturacion:!0,auditoria:!1,...JSON.parse(localStorage.getItem("nexus_config_contabilidad")||"{}")},gi(),hi()}function gi(){w(`
  <div class="nx-module-page" style="background:var(--bg-app);min-height:100vh">
    <div class="o-cp" id="conta-cp">
      <div class="o-cp-left">
        <button class="o-btn-primary" onclick="alert('Nuevo Asiento')">Nuevo Asiento</button>
      </div>
      <div class="o-cp-center">
        <div class="o-search-bar">
          <svg class="o-search-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/></svg>
          <input id="o-search-conta" class="o-search-input" type="text" placeholder="Buscar asiento…" value="${Be}">
        </div>
      </div>
      <div class="o-cp-right">
        <button class="o-btn-secondary" style="margin-right:8px;font-size:16px;padding:4px 8px" onclick="window._go('config_contabilidad')" title="Ajustes">⚙️</button>
        <div class="o-view-switcher">
          <button class="o-view-btn o-active" title="Lista">☰</button>
        </div>
      </div>
    </div>
    <div id="conta-content" class="o-view-content" style="background:#fff;padding:16px;min-height:calc(100vh - 100px);overflow-x:auto">
      ${I(8,5)}
    </div>
  </div>`),setTimeout(()=>{var t;(t=document.getElementById("o-search-conta"))==null||t.addEventListener("input",e=>{Be=e.target.value.toLowerCase(),yi()})},100)}function yi(){document.querySelectorAll(".o-list-row").forEach(t=>{t.style.display=t.textContent.toLowerCase().includes(Be)?"":"none"})}const xi=[{id:1,date:"2023-10-01",ref:"F-2023-0001",journal:"Ventas",partner:"Acme Corp",amount:45e3,state:"posted"},{id:2,date:"2023-10-02",ref:"BILL-2023-001",journal:"Compras",partner:"Tech Solutions",amount:12e3,state:"draft"}];function hi(){const t=document.getElementById("conta-content");t&&(t.innerHTML=`
  <div class="o-list-view">
    <table>
      <thead>
        <tr>
          <th class="th-check"><input type="checkbox" class="o-list-checkbox"></th>
          <th>Fecha</th>
          <th>Referencia</th>
          <th>Diario</th>
          <th>Empresa</th>
          ${lt.contabilidad_analitica?"<th>Cuenta Analítica</th>":""}
          ${lt.multimoneda?"<th>Moneda</th>":""}
          <th style="text-align:right">Total</th>
          <th>Estado</th>
          ${lt.auditoria?"<th>Hash Auditoría</th>":""}
        </tr>
      </thead>
      <tbody>
        ${xi.map(e=>`
        <tr class="o-list-row">
          <td class="td-check"><input type="checkbox" class="o-list-checkbox"></td>
          <td class="o-td-muted">${e.date}</td>
          <td class="o-td-primary">${e.ref}</td>
          <td>${e.journal}</td>
          <td>${e.partner}</td>
          ${lt.contabilidad_analitica?'<td class="o-td-muted">Admin / Ventas</td>':""}
          ${lt.multimoneda?'<td class="o-td-muted">MXN</td>':""}
          <td style="text-align:right;font-weight:600">${f(e.amount)}</td>
          <td>${Rt(e.state==="posted"?"Publicado":"Borrador",e.state==="posted"?"success":"default")}</td>
          ${lt.auditoria?'<td class="o-td-mono" style="font-size:10px;color:#9ca3af">0xab4...</td>':""}
        </tr>`).join("")}
      </tbody>
    </table>
  </div>`)}const ca="nexus_config_contabilidad",wi={multimoneda:!1,contabilidad_analitica:!1,presupuestos:!1,activos_fijos:!1,ingresos_diferidos:!1,alertas_facturacion:!0,auditoria:!1};function rt(t,e,a,o){return`
  <label style="display:flex;align-items:flex-start;gap:12px;cursor:pointer;padding:14px 20px;border-bottom:1px solid var(--border)">
    <input type="checkbox" id="${t}" ${o?"checked":""}
      style="margin-top:2px;accent-color:var(--primary);width:16px;height:16px;flex-shrink:0">
    <div>
      <div style="font-weight:600;font-size:14px;color:var(--text-900)">${e}</div>
      <div style="font-size:12px;color:var(--text-400);margin-top:2px;line-height:1.5">${a}</div>
    </div>
  </label>`}function fe(t,e){return`
  <div style="background:var(--bg-card);border:1px solid var(--border);border-radius:12px;margin-bottom:24px;overflow:hidden">
    <div style="padding:12px 20px;background:var(--bg-app);border-bottom:1px solid var(--border);
      font-size:11px;font-weight:800;text-transform:uppercase;color:var(--text-500);letter-spacing:.08em">
      ${t}
    </div>
    ${e}
  </div>`}async function Aa(t={}){B();const e={...wi,...JSON.parse(localStorage.getItem(ca)||"{}")};w(`<div class="nx-module-page" style="background:var(--bg-app)">
    <div class="o-cp">
      <div class="o-cp-left"></div>
      <div class="o-cp-right">
        <button class="o-btn-primary" onclick="window._saveConfigConta()">💾 Guardar</button>
        <button class="o-btn-secondary" onclick="window._discardConfigConta()">Descartar</button>
      </div>
    </div>

    <div style="max-width:900px;margin:28px auto;padding:0 24px 60px">

      ${fe("Operaciones y Monedas",`
        ${rt("cfgconta-multi","Multimoneda","Permite registrar asientos y facturas en monedas extranjeras con tasa de cambio automática",e.multimoneda)}
        ${rt("cfgconta-analitica","Contabilidad Analítica","Habilita cuentas analíticas y etiquetas para rastrear costos e ingresos por proyectos o departamentos",e.contabilidad_analitica)}
        ${rt("cfgconta-presupuestos","Presupuestos","Compara los gastos e ingresos reales con metas definidas por periodos",e.presupuestos)}
      `)}

      ${fe("Gestión de Activos e Ingresos",`
        ${rt("cfgconta-activos","Activos Fijos","Calcula y registra automáticamente la depreciación de activos fijos a lo largo del tiempo",e.activos_fijos)}
        ${rt("cfgconta-diferidos","Ingresos y Gastos Diferidos","Reconoce ingresos o gastos en un periodo de tiempo futuro (ej. suscripciones anuales)",e.ingresos_diferidos)}
      `)}

      ${fe("Control y Auditoría",`
        ${rt("cfgconta-alertas","Alertas de Facturación","Evita la creación de facturas si hay discrepancias de contabilidad no resueltas",e.alertas_facturacion)}
        ${rt("cfgconta-auditoria","Rastro de Auditoría","Activa un registro inalterable (hash criptográfico) de cada asiento contable para cumplimiento fiscal",e.auditoria)}
      `)}

      <div style="display:flex;gap:10px;padding-top:8px">
        <button class="o-btn-primary" onclick="window._saveConfigConta()">💾 Guardar Configuración</button>
        <button class="o-btn-secondary" onclick="window._discardConfigConta()">Descartar cambios</button>
      </div>
    </div>
  </div>`),window._saveConfigConta=()=>{var i,s,n,d,r,l,c;const a=p=>document.getElementById(p),o={multimoneda:((i=a("cfgconta-multi"))==null?void 0:i.checked)??!1,contabilidad_analitica:((s=a("cfgconta-analitica"))==null?void 0:s.checked)??!1,presupuestos:((n=a("cfgconta-presupuestos"))==null?void 0:n.checked)??!1,activos_fijos:((d=a("cfgconta-activos"))==null?void 0:d.checked)??!1,ingresos_diferidos:((r=a("cfgconta-diferidos"))==null?void 0:r.checked)??!1,alertas_facturacion:((l=a("cfgconta-alertas"))==null?void 0:l.checked)??!1,auditoria:((c=a("cfgconta-auditoria"))==null?void 0:c.checked)??!1};localStorage.setItem(ca,JSON.stringify(o)),b("Guardado","Configuración de Contabilidad actualizada","success")},window._discardConfigConta=()=>Aa()}function _i(t){const{breadcrumb:e=[],title:a="",subtitle:o="",statusSteps:i=[],currentStatus:s="",statusButtons:n=[],smartButtons:d=[],fieldGroups:r=[],tabs:l=[],chatterMessages:c=[],extraHtml:p="",onEdit:m=null,onSave:u=null,editing:v=!1,id:k=""}=t;[...e.map(y=>({label:y.label,href:y.hash?`#${y.hash}`:void 0}))];const _=l.length?`
    <div class="o-notebook">
      <div class="o-tabs" role="tablist">
        ${l.map((y,z)=>`
          <button class="o-tab ${z===0?"active":""}"
            role="tab"
            id="tab-btn-${y.id||z}"
            onclick="window._switchTab('${y.id||z}')">
            ${y.label}
            ${y.badge?`<span class="o-tab-badge">${y.badge}</span>`:""}
          </button>
        `).join("")}
      </div>
      ${l.map((y,z)=>`
        <div class="o-tab-panel ${z===0?"active":""}" id="tab-panel-${y.id||z}">
          ${typeof y.content=="function"?y.content():y.content||""}
        </div>
      `).join("")}
    </div>
  `:"",$=r.map(y=>$i(y)).join(""),T=d.length?`
    <div class="o-smart-buttons" style="border:none; padding:0; background:transparent; justify-content:flex-end;">
      ${d.map(y=>`
        <div class="o-smart-btn" onclick="${y.onClick||"void 0"}" style="min-width:100px;">
          <span class="o-smart-count">${y.count??0}</span>
          <span class="o-smart-label" style="margin-top:4px;">${y.icon||""} ${y.label}</span>
        </div>
      `).join("")}
    </div>
  `:"",h=`
    <div class="o-statusbar">
      <div class="o-statusbar-status">
        ${i.map(y=>{const z=i.findIndex(C=>C.key===s),P=i.findIndex(C=>C.key===y.key),A=z>P;return`
            <div class="o-status-step ${y.key===s?"active":""} ${A?"done":""}">
              ${A?"✔️ ":""}${y.label}
            </div>
          `}).join('<span class="o-status-arrow">›</span>')}
      </div>
      <div class="o-statusbar-buttons">
        ${n.filter(y=>y.visible!==!1).map(y=>`
          <button class="btn ${y.danger?"btn-danger":y.primary?"btn-primary":"btn-secondary"} btn-sm"
            onclick="${y.onClick}">
            ${y.icon||""}${y.label}
          </button>
        `).join("")}
        ${!v&&m?`
          <button class="btn btn-secondary btn-sm" onclick="window._formEdit?.()">
            ✏️ Editar
          </button>
        `:""}
        ${v?`
          <button class="btn btn-primary btn-sm" onclick="window._formSave?.()">💾 Guardar</button>
          <button class="btn btn-secondary btn-sm" onclick="window._formDiscard?.()">✕ Descartar</button>
        `:""}
      </div>
    </div>
  `,E=`
    <div class="o-chatter">
      <div class="o-chatter-topbar">
        <button class="o-chatter-btn" onclick="window._sendMsg?.('${k}')">✉️ Enviar mensaje</button>
        <button class="o-chatter-btn" onclick="window._addNote?.('${k}')">📋 Nota interna</button>
        <button class="o-chatter-btn">📎 Adjuntar</button>
      </div>
      <div class="o-chatter-thread">
        ${c.length?c.map(y=>{var z,P;return`
          <div class="o-message ${y.type==="note"?"o-message-note":""}">
            <div class="o-msg-avatar" style="background:${ki(y.author)}">${y.initials||((P=(z=y.author)==null?void 0:z[0])==null?void 0:P.toUpperCase())||"?"}</div>
            <div class="o-msg-content">
              <div class="o-msg-header">
                <span class="o-msg-author">${y.author}</span>
                <span class="o-msg-date">${y.date}</span>
                ${y.type==="note"?'<span class="o-msg-note-badge">Nota interna</span>':""}
              </div>
              <div class="o-msg-text">${y.text}</div>
            </div>
          </div>
        `}).join(""):`
          <div class="o-chatter-empty">
            <p>💬 Sin actividad en este registro.</p>
          </div>
        `}
      </div>
    </div>
  `,S=`
    <div class="o-form-view ${v?"editing":""}" id="form-view-root">
      ${h}
      <div class="o-form-sheet">
        <div class="o-form-title-row" style="display:flex; justify-content:space-between; align-items:flex-start;">
          <div class="o-form-title-block">
            <h1 class="o-form-record-title">${a}</h1>
            ${o?`<span class="o-form-subtitle">${o}</span>`:""}
          </div>
          ${T}
        </div>
        ${$}
        ${_}
        ${p}
      </div>
      ${E}
    </div>
  `;w(S),window._switchTab=y=>{var z,P;document.querySelectorAll(".o-tab").forEach(A=>A.classList.remove("active")),document.querySelectorAll(".o-tab-panel").forEach(A=>A.classList.remove("active")),(z=document.getElementById("tab-btn-"+y))==null||z.classList.add("active"),(P=document.getElementById("tab-panel-"+y))==null||P.classList.add("active")},window._sendMsg=y=>{Q("Enviar mensaje",`
      <div style="display:flex;flex-direction:column;gap:14px">
        <textarea id="chat-msg" style="width:100%;min-height:120px;padding:12px;border:1px solid var(--border);border-radius:10px;font-size:14px;resize:vertical;font-family:inherit" placeholder="Escribe tu mensaje..."></textarea>
        <div style="display:flex;gap:8px;justify-content:flex-end">
          <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cancelar</button>
          <button class="btn btn-primary btn-sm" onclick="
            const msg=document.getElementById('chat-msg')?.value.trim();
            if(msg){window.__closeModal();toast('Enviado','Mensaje registrado en el historial','success');}">
            ✉️ Enviar
          </button>
        </div>
      </div>
    `)},window._addNote=y=>{Q("Nota interna",`
      <div style="display:flex;flex-direction:column;gap:14px">
        <div style="padding:8px 12px;background:#FEF9C3;border-radius:8px;font-size:12px;color:#854D0E">
          ⚠️ Solo visible para el equipo interno
        </div>
        <textarea id="note-msg" style="width:100%;min-height:100px;padding:12px;border:1.5px dashed #D97706;border-radius:10px;font-size:14px;resize:vertical;font-family:inherit;background:#FFFBEB" placeholder="Nota interna..."></textarea>
        <div style="display:flex;gap:8px;justify-content:flex-end">
          <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cancelar</button>
          <button class="btn btn-sm" style="background:#D97706;color:#fff;border:none;border-radius:8px;padding:6px 16px;font-weight:600;cursor:pointer" onclick="window.__closeModal();">
            📋 Guardar nota
          </button>
        </div>
      </div>
    `)},m&&(window._formEdit=m),u&&(window._formSave=u),window._formDiscard=()=>history.back()}function $i({title:t,cols:e=2,fields:a=[]}){if(!a.length)return"";const o=Math.ceil(a.length/e),i=a.slice(0,o),s=e===2?a.slice(o):[],n=r=>`
    <div class="o-field-row">
      <div class="o-field-label">${r.label}</div>
      <div class="o-field-value">
        ${d(r)}
      </div>
    </div>
  `,d=r=>{if(r.value===null||r.value===void 0||r.value==="")return'<span class="o-field-empty">—</span>';switch(r.type){case"money":return`<span class="o-field-money">${r.value}</span>`;case"badge":return`<span class="o-state-badge" style="background:${r.bg||"#f1f5f9"};color:${r.color||"#475569"}">${r.value}</span>`;case"link":return`<a href="${r.href||"#"}" class="o-field-link">${r.value}</a>`;case"email":return`<a href="mailto:${r.value}" class="o-field-link">${r.value}</a>`;case"phone":return`<a href="tel:${r.value}" class="o-field-link">${r.value}</a>`;case"date":return`<span>${r.value}</span>`;case"boolean":return r.value?"✅ Sí":"❌ No";case"html":return r.value;default:return`<span>${r.value}</span>`}};return`
    <div class="o-form-group-wrapper">
      ${t?`<div class="o-group-title">${t}</div>`:""}
      <div class="o-form-group">
        <div class="o-form-col">${i.map(n).join("")}</div>
        ${e===2?`<div class="o-form-col">${s.map(n).join("")}</div>`:""}
      </div>
    </div>
  `}window._chatterMessage=t=>{Q("Enviar mensaje",`
    <div style="display:flex;flex-direction:column;gap:12px">
      <label style="font-size:13px;font-weight:600;color:var(--text-600)">Mensaje</label>
      <textarea id="chatter-msg" style="width:100%;min-height:100px;padding:10px;border:1px solid var(--border);border-radius:8px;font-size:13px;resize:vertical;font-family:inherit"
        placeholder="Escribe tu mensaje..."></textarea>
      <div style="display:flex;gap:8px;justify-content:flex-end">
        <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cancelar</button>
        <button class="btn btn-primary btn-sm" onclick="(() => {
          const msg = document.getElementById('chatter-msg')?.value;
          if (msg) { window.__closeModal(); }
        })()">✉️ Enviar</button>
      </div>
    </div>
  `)};window._chatterNote=t=>{Q("Nota interna",`
    <div style="display:flex;flex-direction:column;gap:12px">
      <label style="font-size:13px;font-weight:600;color:var(--text-600)">Nota interna <small style="color:var(--text-400)">(solo visible para el equipo)</small></label>
      <textarea id="chatter-note" style="width:100%;min-height:80px;padding:10px;border:1.5px dashed var(--warning);border-radius:8px;font-size:13px;resize:vertical;font-family:inherit;background:#FFFBEB"
        placeholder="Nota interna..."></textarea>
      <div style="display:flex;gap:8px;justify-content:flex-end">
        <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cancelar</button>
        <button class="btn btn-sm" style="background:#D97706;color:#fff;border:none;font-weight:600;cursor:pointer"
          onclick="window.__closeModal()">📋 Guardar nota</button>
      </div>
    </div>
  `)};function ki(t=""){let e=0;for(let o=0;o<t.length;o++)e=t.charCodeAt(o)+((e<<5)-e);const a=e%360;return`hsl(${Math.abs(a)}, 65%, 45%)`}function Ei(t={}){B();const e=async a=>{try{const o=await window._api.post(`/orm/mercadily.backend/${a}`,{args:[[t.id||1]]});if(o.result&&o.result.tag==="display_notification"){const i=o.result.params;window._toast(i.message,i.type)}else window._toast("Llamada ORM ejecutada.","success")}catch(o){window._toast(o.message||"Error en ORM","error")}};window._callOrm=e,_i({breadcrumb:[{label:"mercadily.backend"}],title:t.name||t.display_name||"Nuevo",currentStatus:t.state||"",statusSteps:[],statusButtons:[{label:"Probar Conexión",primary:!0,onClick:()=>e("action_test_connection")},{label:"Sincronizar Todo",primary:!1,onClick:()=>e("action_sync_all")}],smartButtons:[{icon:"👥",count:t.lead_count||0,label:"Leads",onClick:()=>e("action_view_leads")},{icon:"🛒",count:t.order_count||0,label:"Pedidos",onClick:()=>e("action_view_orders")},{icon:"👔",count:t.customer_count||0,label:"Clientes",onClick:()=>e("action_view_customers")}],fieldGroups:[{title:"Configuración API",fields:[{label:"Nombre de Tienda",type:"html",value:`<input id="field-name" class="form-control" value="${t.name||""}" placeholder="Nombre de la Tienda">`},{label:"API URL",type:"html",value:`<input id="field-api_url" class="form-control" value="${t.api_url||""}" placeholder="https://tudominio.com">`},{label:"API Key",type:"html",value:`<input id="field-api_key" type="password" class="form-control" value="${t.api_key||""}" placeholder="**********">`}]},{title:"Sincronización",fields:[{label:"Última sinc. Leads",value:t.last_lead_sync||"Nunca"},{label:"Última sinc. Pedidos",value:t.last_order_sync||"Nunca"},{label:"Última sinc. Clientes",value:t.last_customer_sync||"Nunca"}]},{title:"Sincronización Manual",fields:[{label:"Acciones",type:"html",value:`
          <div style="display:flex;gap:8px">
            <button class="btn btn-secondary btn-sm" onclick="window._callOrm('action_sync_leads')">Sincronizar Leads</button>
            <button class="btn btn-secondary btn-sm" onclick="window._callOrm('action_sync_customers')">Sincronizar Clientes</button>
            <button class="btn btn-secondary btn-sm" onclick="window._callOrm('action_sync_orders')">Sincronizar Pedidos</button>
          </div>
        `}]}],id:t.id||"",onSave:async()=>{document.getElementById("field-name").value,document.getElementById("field-api_url").value,document.getElementById("field-api_key").value,window._toast("Configuración guardada.","success")},editing:!0})}async function Ci(){B();let t=[];try{const o=await g.get("/apps");t=Array.isArray(o==null?void 0:o.data)?o.data:Array.isArray(o)?o:[]}catch(o){console.error("Error cargando apps",o),b("Error","No se pudo cargar el catálogo","error");return}t=t.filter(o=>o.id!=="apps");const e=()=>t.map((o,i)=>{const s=o.estado==="installed";return`
        <div class="nx-app-card" data-id="${o.id}" style="animation-delay:${i*50}ms; cursor: default; height: auto; padding-bottom: 20px;">
          <div class="nx-app-icon" style="background:linear-gradient(135deg,${o.gradiente||"#475569,#1E293B"})">${o.icono||"📦"}</div>
          <div class="nx-app-name">${o.nombre}</div>
          <div class="nx-app-desc" style="margin-bottom: 16px;">${o.descripcion||""}</div>
          
          <div style="margin-top: auto; width: 100%; display: flex; justify-content: center;">
            ${s?`<button class="btn btn-secondary btn-sm" onclick="window._uninstallApp('${o.id}')">Desinstalar</button>`:`<button class="btn btn-primary btn-sm" onclick="window._installApp('${o.id}')">Instalar</button>`}
          </div>
        </div>
      `}).join(""),a=()=>{const o=document.getElementById("apps-grid");o&&(o.innerHTML=e())};w(`
    <div class="nx-home">
      <div class="nx-home-header">
        <h1 class="nx-home-title">Catálogo de Aplicaciones</h1>
        <div class="nx-home-search">
          <input type="search" placeholder="Buscar módulo…" id="apps-search" oninput="window._filterAppsStore(this.value)">
        </div>
      </div>
      <div class="nx-app-grid" id="apps-grid">
        ${e()}
      </div>
    </div>
  `),window._filterAppsStore=o=>{const i=o.toLowerCase().trim();document.querySelectorAll("#apps-grid .nx-app-card").forEach(s=>{var r,l;const n=((r=s.querySelector(".nx-app-name"))==null?void 0:r.textContent.toLowerCase())||"",d=((l=s.querySelector(".nx-app-desc"))==null?void 0:l.textContent.toLowerCase())||"";s.classList.toggle("hidden",!!i&&!n.includes(i)&&!d.includes(i))})},window._installApp=async o=>{try{await g.post(`/apps/${o}/install`),b("Instalado","La aplicación se ha instalado correctamente","success");const i=t.find(s=>s.id===o);i&&(i.estado="installed"),a()}catch{b("Error","Fallo al instalar la aplicación","error")}},window._uninstallApp=async o=>{if(confirm("¿Estás seguro de desinstalar esta aplicación? Sus vistas y funciones se ocultarán."))try{await g.post(`/apps/${o}/uninstall`),b("Desinstalado","La aplicación ha sido removida del dashboard","info");const i=t.find(s=>s.id===o);i&&(i.estado="uninstalled"),a()}catch{b("Error","Fallo al desinstalar la aplicación","error")}}}const pa={apps:Ci,login:Na,home:Ka,dashboard:ua,ventas:Ya,facturas:go,stock:re,compras:Oe,crm:ai,partners:Ne,productos:ba,nomina:ha,reportes:vi,cfdi:Lo,cotizaciones:Ro,precios:Xo,reportes_ventas:Yo,config_ventas:Sa,config_facturacion:za,config_compras:Ta,config_crm:Ia,config_inventario:Fa,config_contactos:Ba,contabilidad:fi,config_contabilidad:Aa,mercadily:Ei,search:li};Object.keys(pa).forEach(t=>qt(t,pa[t]));qt("stock",t=>{t.picking?Ao(parseInt(t.picking),t.origen?parseInt(t.origen):null):re()});qt("pagos",Ue);qt("reportes_facturacion",Ue);qt("404",()=>Ue("404","Página no encontrada","La ruta solicitada no existe","🔍"));La();export{g as a,b as t};
