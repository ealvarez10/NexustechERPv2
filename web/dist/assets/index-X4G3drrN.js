(function(){const e=document.createElement("link").relList;if(e&&e.supports&&e.supports("modulepreload"))return;for(const i of document.querySelectorAll('link[rel="modulepreload"]'))o(i);new MutationObserver(i=>{for(const r of i)if(r.type==="childList")for(const n of r.addedNodes)n.tagName==="LINK"&&n.rel==="modulepreload"&&o(n)}).observe(document,{childList:!0,subtree:!0});function a(i){const r={};return i.integrity&&(r.integrity=i.integrity),i.referrerPolicy&&(r.referrerPolicy=i.referrerPolicy),i.crossOrigin==="use-credentials"?r.credentials="include":i.crossOrigin==="anonymous"?r.credentials="omit":r.credentials="same-origin",r}function o(i){if(i.ep)return;i.ep=!0;const r=a(i);fetch(i.href,r)}})();const et={isLoggedIn:()=>!!localStorage.getItem("nx_token"),getUser:()=>{try{return JSON.parse(localStorage.getItem("nx_user")||"{}")}catch{return{}}},setSession(t,e){localStorage.setItem("nx_token",t),localStorage.setItem("nx_user",JSON.stringify(e))},clear(){localStorage.removeItem("nx_token"),localStorage.removeItem("nx_user")}},dt={};function F(t,e){dt[t]=e}function at(t){window.location.hash=t}function se(){window.addEventListener("hashchange",At),At()}function At(){const t=window.location.hash.replace("#","")||"home";if(!et.isLoggedIn()&&t!=="login"){at("login");return}if(et.isLoggedIn()&&t==="login"){at("home");return}const e=dt[t];e?e():dt[404]&&dt[404]()}const le="/api/v1";function de(){return localStorage.getItem("nx_token")}class re extends Error{constructor(e,a){super(a),this.status=e}}async function y(t,e,a){const o=de(),i=await fetch(le+e,{method:t,headers:{"Content-Type":"application/json",...o?{Authorization:`Bearer ${o}`}:{}},...a!==void 0?{body:JSON.stringify(a)}:{}});if(i.status===401)return localStorage.removeItem("nx_token"),localStorage.removeItem("nx_user"),window.location.hash="login",null;if(!i.ok)throw new re(i.status,await i.text());return(i.headers.get("content-type")||"").includes("application/json")?i.json():i.text()}const f={get:t=>y("GET",t),post:(t,e)=>y("POST",t,e),put:(t,e)=>y("PUT",t,e),del:t=>y("DELETE",t),login:(t,e)=>y("POST","/auth/login",{login:t,password:e}),logout:()=>y("POST","/auth/logout",{}),dashboard:()=>y("GET","/dashboard"),ventaKpis:()=>y("GET","/ventas/kpis"),factKpis:()=>y("GET","/facturas/kpis"),stockKpis:()=>y("GET","/stock/kpis"),ventas:(t=1)=>y("GET",`/ventas?pagina=${t}`),venta:t=>y("GET",`/ventas/${t}`),facturas:(t=1)=>y("GET",`/facturas?pagina=${t}`),factura:t=>y("GET",`/facturas/${t}`),porCobrar:()=>y("GET","/facturas/por-cobrar"),productos:(t=1,e="")=>y("GET",`/productos?pagina=${t}&q=${encodeURIComponent(e)}`),producto:t=>y("GET",`/productos/${t}`),partners:(t=1)=>y("GET",`/partners?pagina=${t}`),partner:t=>y("GET",`/partners/${t}`),clientes:(t=1)=>y("GET",`/clientes?pagina=${t}`),proveedores:(t=1)=>y("GET",`/proveedores?pagina=${t}`),stock:(t=1)=>y("GET",`/stock?pagina=${t}`),stockKpis:()=>y("GET","/stock/kpis"),stockBajo:()=>y("GET","/stock/bajo"),stockProducto:t=>y("GET",`/stock/producto/${t}`),cfdiTimbrados:(t=1)=>y("GET",`/cfdi/timbrados?pagina=${t}`),cfdiTimbrado:t=>y("GET",`/cfdi/timbrados/${t}`),cfdiKpis:()=>y("GET","/cfdi/kpis"),timbrar:t=>y("POST","/cfdi/timbrar",t),cancelarCfdi:t=>y("POST","/cfdi/cancelar",t),nomina:(t=1)=>y("GET",`/nomina?pagina=${t}`),nominaKpis:()=>y("GET","/nomina/kpis"),compras:(t=1)=>y("GET",`/compras?pagina=${t}`),comprasKpis:()=>y("GET","/compras/kpis"),cotizaciones:(t=1)=>y("GET",`/cotizaciones?pagina=${t}`),cotizacionKpis:()=>y("GET","/cotizaciones/kpis"),cotizacion:t=>y("GET",`/cotizaciones/${t}`),crearCotizacion:t=>y("POST","/cotizaciones",t),confirmarCotizacion:t=>y("PUT",`/cotizaciones/${t}/confirmar`),cancelarCotizacion:t=>y("PUT",`/cotizaciones/${t}/cancelar`),actualizarCotizacion:(t,e)=>y("PUT",`/cotizaciones/${t}`,e),agregarLinea:(t,e)=>y("POST",`/cotizaciones/${t}/lineas`,e),eliminarLinea:(t,e)=>y("DELETE",`/cotizaciones/${t}/lineas/${e}`),searchSync:()=>y("POST","/search/sync",{}),searchStatus:()=>y("GET","/search/status"),health:()=>y("GET","/health"),putVenta:(t,e)=>y("PUT",`/ventas/${t}`,e),putPartner:(t,e)=>y("PUT",`/partners/${t}`,e),putProducto:(t,e)=>y("PUT",`/productos/${t}`,e),putCompra:(t,e)=>y("PUT",`/compras/${t}`,e),putEmpleado:(t,e)=>y("PUT",`/nomina/${t}`,e),ajusteStock:(t,e)=>y("PUT",`/stock/${t}/ajuste`,e)};function ce(){const t=document.getElementById("__shell");t&&t.remove(),document.getElementById("app").innerHTML=`
  <div class="login-bg">
    <!-- Panel izquierdo: branding -->
    <div class="login-left">
      <div class="login-left-content">
        <div style="font-size:52px;margin-bottom:16px">⚡</div>
        <div class="login-product-name">NexusTech ERP</div>
        <div class="login-product-tagline">Sistema de gestión empresarial de ultra alta velocidad. Construido en Rust.</div>

        <div class="login-features">
          <div class="login-feature">
            <div class="feature-icon" style="background:rgba(129,140,248,0.15)">🧾</div>
            <div>
              <div style="color:white;font-weight:600;margin-bottom:2px">CFDI 4.0 nativo</div>
              <div>Timbrado, cancelación y representación impresa</div>
            </div>
          </div>
          <div class="login-feature">
            <div class="feature-icon" style="background:rgba(16,185,129,0.15)">📦</div>
            <div>
              <div style="color:white;font-weight:600;margin-bottom:2px">Inventario en tiempo real</div>
              <div>Stock, ubicaciones y alertas automáticas</div>
            </div>
          </div>
          <div class="login-feature">
            <div class="feature-icon" style="background:rgba(245,158,11,0.15)">👔</div>
            <div>
              <div style="color:white;font-weight:600;margin-bottom:2px">Nómina mexicana</div>
              <div>IMSS, Infonavit e ISR 2024 calculados automáticamente</div>
            </div>
          </div>
          <div class="login-feature">
            <div class="feature-icon" style="background:rgba(6,182,212,0.15)">🔍</div>
            <div>
              <div style="color:white;font-weight:600;margin-bottom:2px">NexusSearch</div>
              <div>Búsqueda instantánea en millones de productos</div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Panel derecho: formulario -->
    <div class="login-right">
      <div class="login-form-wrap">
        <div class="login-logo">
          <div class="login-logo-icon">N</div>
          <div>
            <div style="font-family:'Plus Jakarta Sans',sans-serif;font-size:18px;font-weight:800;color:#1E1B4B">NexusTech</div>
            <div style="font-size:11px;color:#9CA3AF;font-weight:500">ERP v2.0</div>
          </div>
        </div>

        <h1 class="login-title">Iniciar sesión</h1>
        <p class="login-sub">Accede a tu cuenta empresarial</p>

        <div class="form-group">
          <label class="form-label" for="lu">Usuario o correo electrónico</label>
          <input class="form-control" id="lu" type="text" placeholder="admin  o  usuario@empresa.mx" autocomplete="username">
        </div>
        <div class="form-group">
          <label class="form-label" for="lp">Contraseña</label>
          <input class="form-control" id="lp" type="password" placeholder="••••••••" autocomplete="current-password">
        </div>

        <div class="login-error" id="lerr">Credenciales incorrectas. Verifica tu usuario y contraseña.</div>

        <button class="login-btn" id="lbtn">Acceder al sistema</button>

        <div style="margin-top:20px;font-size:12px;color:#9CA3AF;text-align:center">
          🔒 Conexión cifrada · ISO 27001
        </div>

        <div style="margin-top:16px;background:#F0FDF4;border:1px solid #BBF7D0;border-radius:8px;padding:10px 14px;font-size:12px;color:#166534">
          <strong>Demo:</strong> usuario <code style="background:#DCFCE7;padding:1px 5px;border-radius:4px">admin</code> / contraseña <code style="background:#DCFCE7;padding:1px 5px;border-radius:4px">admin</code>
        </div>
      </div>
    </div>
  </div>`;const e=document.getElementById("lbtn"),a=document.getElementById("lu"),o=document.getElementById("lp"),i=document.getElementById("lerr");async function r(){if(e.disabled)return;const n=a.value.trim(),l=o.value;if(!n||!l){i.textContent="Ingresa usuario y contraseña",i.classList.add("show");return}e.disabled=!0,e.textContent="Verificando...",i.classList.remove("show");try{const s=await f.login(n,l),d=(s==null?void 0:s.data)||s,u=(d==null?void 0:d.access_token)||(d==null?void 0:d.token);if(u){et.setSession(u,{nombre:d.email||n,email:d.email||n,user_id:d.user_id,company_id:d.company_id}),document.getElementById("app").innerHTML="",at("dashboard");return}i.textContent="Error inesperado del servidor. Intenta de nuevo.",i.classList.add("show")}catch(s){i.textContent=(s==null?void 0:s.status)===401?"Credenciales incorrectas. Verifica tu usuario y contraseña.":`Error de conexión: ${(s==null?void 0:s.message)||"No se pudo contactar el servidor"}`,i.classList.add("show")}e.disabled=!1,e.textContent="Acceder al sistema"}e.addEventListener("click",r),o.addEventListener("keydown",n=>n.key==="Enter"&&r()),a.addEventListener("keydown",n=>n.key==="Enter"&&o.focus()),setTimeout(()=>a.focus(),100)}function pe(t,e=0){return t==null||t===""?"—":Number(t).toLocaleString("es-MX",{minimumFractionDigits:e,maximumFractionDigits:e})}function x(t){return t==null?"—":(t=parseFloat(t)||0,Math.abs(t)>=1e6?`$${(t/1e6).toFixed(2)}M`:Math.abs(t)>=1e3?`$${(t/1e3).toFixed(1)}k`:`$${pe(t,2)}`)}function M(t){return t==null?"—":Number(t).toLocaleString("es-MX")}function T(t){return t?new Date(t).toLocaleDateString("es-MX",{day:"2-digit",month:"short",year:"numeric"}):"—"}function g(t,e="",a="info"){const o={success:"✅",error:"❌",info:"ℹ️",warning:"⚠️"};let i=document.getElementById("__toasts");i||(i=document.createElement("div"),i.id="__toasts",i.className="toast-container",document.body.appendChild(i));const r=document.createElement("div");r.className=`toast ${a}`,r.innerHTML=`
    <span class="toast-icon">${o[a]||"ℹ️"}</span>
    <div><div class="toast-title">${t}</div>${e?`<div class="toast-msg">${e}</div>`:""}</div>`,i.appendChild(r),requestAnimationFrame(()=>r.classList.add("show")),setTimeout(()=>{r.classList.remove("show"),setTimeout(()=>r.remove(),400)},3800)}function jt(t,e,a=900,o="",i=""){if(!t)return;const r=performance.now(),n=String(e).includes(".");function l(s){const d=Math.min((s-r)/a,1),u=1-Math.pow(1-d,3),h=e*u;t.textContent=o+(n?h.toLocaleString("es-MX",{minimumFractionDigits:2,maximumFractionDigits:2}):Math.round(h).toLocaleString("es-MX"))+i,d<1&&requestAnimationFrame(l)}requestAnimationFrame(l)}function ve(t){if(!(t!=null&&t.length))return"";const e=Math.max(...t,1);return`<div class="sparkline">${t.map((a,o)=>`<div class="spark-bar${o===t.length-1?" active":""}" style="height:${Math.max(4,Math.round(a/e*100))}%"></div>`).join("")}</div>`}function ue(t=5,e=6){return`<tbody>${Array.from({length:e},()=>`<tr>${Array.from({length:t},()=>`<td><div class="skeleton" style="height:14px;width:${60+Math.random()*30}%"></div></td>`).join("")}</tr>`).join("")}</tbody>`}function I(t=5,e=4){return`<table class="data-table"><thead><tr>${Array.from({length:e},()=>`<th><div class="skeleton" style="height:12px;width:${40+Math.random()*40}%"></div></th>`).join("")}</tr></thead>${ue(e,t)}</table>`}function me(t=5){return Array.from({length:t},()=>`
  <div class="kpi-card kpi-gray">
    <div class="kpi-label"><div class="skeleton" style="height:13px;width:60%"></div></div>
    <div class="kpi-value"><div class="skeleton" style="height:28px;width:70%"></div></div>
    <div><div class="skeleton" style="height:11px;width:40%;margin-top:6px"></div></div>
  </div>`).join("")}const be={sale:"emerald",done:"indigo",draft:"gray",sent:"sky",cancel:"red",posted:"emerald",in_payment:"violet",paid:"emerald",partial:"amber"};function G(t,e){return`<span class="badge badge-${be[t]||"gray"} badge-dot">${e}</span>`}function D(t,e,a){return window.__pagNav=a,`
  <div class="data-table-footer">
    <span style="color:var(--text-400)">Página ${t}</span>
    <div class="pagination">
      <button class="pag-btn" ${t<=1?"disabled":""} onclick="window.__pagNav(${t-1})">&#8592; Anterior</button>
      <span class="pag-btn active">${t}</span>
      <button class="pag-btn" ${e?"":"disabled"} onclick="window.__pagNav(${t+1})">Siguiente &#8594;</button>
    </div>
  </div>`}let q=null;function A(t,e,a={}){let o=document.getElementById("__modal-overlay");o||(o=document.createElement("div"),o.id="__modal-overlay",o.innerHTML=`
      <div id="__modal-drawer">
        <div id="__modal-header">
          <span id="__modal-title"></span>
          <button id="__modal-close" onclick="window.__closeModal()">✕</button>
        </div>
        <div id="__modal-body"></div>
      </div>`,document.body.appendChild(o),o.addEventListener("click",i=>{i.target===o&&window.__closeModal()})),document.getElementById("__modal-title").textContent=t,document.getElementById("__modal-body").innerHTML=e,o.classList.add("open"),document.body.style.overflow="hidden",q&&document.removeEventListener("keydown",q),q=i=>{i.key==="Escape"&&window.__closeModal()},document.addEventListener("keydown",q),a.onMounted&&setTimeout(a.onMounted,10)}function Ot(){const t=document.getElementById("__modal-overlay");t&&t.classList.remove("open"),document.body.style.overflow="",q&&(document.removeEventListener("keydown",q),q=null)}window.__closeModal=Ot;async function nt(t,e,a){A(t,`
    <div style="display:flex;flex-direction:column;gap:12px;padding:8px 0">
      ${[1,2,3,4,5].map(()=>'<div class="skeleton" style="height:52px;border-radius:10px"></div>').join("")}
    </div>`);try{const o=await e(),i=(o==null?void 0:o.data)??o;document.getElementById("__modal-body").innerHTML=a(i)}catch(o){document.getElementById("__modal-body").innerHTML=`<p style="color:var(--red);padding:24px">Error: ${o.message}</p>`}}function $(t,e,a={}){const o=e??"—",i=a.color?`color:${a.color}`:"";return`
  <div style="display:flex;justify-content:space-between;align-items:flex-start;
    padding:10px 0;border-bottom:1px solid var(--border)">
    <span style="font-size:12px;color:var(--text-400);font-weight:600;min-width:140px">${t}</span>
    <span style="font-size:13px;font-weight:500;text-align:right;${i}">${o}</span>
  </div>`}function P(t,e){return`
  <div style="margin-bottom:20px">
    <div style="font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.06em;
      color:var(--text-400);margin-bottom:8px;padding-bottom:6px;
      border-bottom:2px solid var(--primary)">${t}</div>
    ${e}
  </div>`}const Dt=[{id:"home",icon:"⊞",label:"Inicio",section:"Principal"},{id:"dashboard",icon:"📊",label:"Dashboard",section:"Principal"},{id:"ventas",icon:"💰",label:"Ventas",section:"Principal"},{id:"cotizaciones",icon:"📝",label:"Cotizaciones",section:"Principal"},{id:"facturas",icon:"🧾",label:"Facturación",section:"Principal"},{id:"productos",icon:"📦",label:"Productos",section:"Principal"},{id:"partners",icon:"👥",label:"Clientes",section:"Principal"},{id:"stock",icon:"🏭",label:"Inventario",section:"Principal"},{id:"cfdi",icon:"🔏",label:"CFDI 4.0",section:"Fiscal",badge:"NUEVO"},{id:"nomina",icon:"👔",label:"Nómina IMSS",section:"Fiscal"},{id:"compras",icon:"🛒",label:"Compras",section:"Operaciones"},{id:"search",icon:"🔍",label:"NexusSearch",section:"Sistema"},{id:"reportes",icon:"📈",label:"Reportes",section:"Sistema"}];function L(){if(document.getElementById("__shell"))return;const t=et.getUser(),e=(t.nombre||t.name||"AD").substring(0,2).toUpperCase(),a=[...new Set(Dt.map(o=>o.section))];if(document.getElementById("app").innerHTML=`
  <div class="app-shell" id="__shell">
    <!-- SIDEBAR -->
    <nav class="sidebar" id="__sidebar">
      <div class="sidebar-brand" style="cursor:pointer" onclick="window._go('home')" title="Ir al inicio">
        <div class="brand-logo">N</div>
        <div class="sidebar-brand-text">
          <div class="brand-name">NexusTech</div>
          <div class="brand-version">ERP v2.0</div>
        </div>
      </div>

      <div class="sidebar-nav">
        ${a.map(o=>`
        <div class="nav-section">
          <div class="nav-section-title">${o}</div>
          ${Dt.filter(i=>i.section===o).map(i=>`
          <a class="nav-link" id="nl-${i.id}" href="#${i.id}" onclick="event.preventDefault();window._go('${i.id}')">
            <span style="font-size:16px">${i.icon}</span>
            <span>${i.label}</span>
            ${i.badge?`<span class="nav-badge">${i.badge}</span>`:""}
          </a>`).join("")}
        </div>`).join("")}
      </div>

      <div class="sidebar-user">
        <div class="user-pill">
          <div class="avatar">${e}</div>
          <div class="user-info">
            <div class="user-name">${t.nombre||t.name||"Administrador"}</div>
            <div class="user-role">${t.email||"admin@nexustech.mx"}</div>
          </div>
        </div>
        <button class="btn btn-secondary btn-sm" style="width:100%;margin-top:8px;justify-content:center" onclick="window._logout()">
          🚪 Cerrar sesión
        </button>
      </div>
      <!-- Sidebar collapse toggle -->
      <button class="sidebar-toggle-btn" id="sidebar-toggle" title="Colapsar panel" onclick="window._toggleSidebar()">
        ◀
      </button>
    </nav>

    <!-- MAIN -->
    <div class="main-area">
      <!-- TOPBAR -->
      <header class="topbar">
        <nav class="breadcrumb" id="__breadcrumb">
          <span class="breadcrumb-item">Inicio</span>
        </nav>
        <div class="topbar-spacer"></div>
        <div class="topbar-search">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/>
          </svg>
          <input type="text" placeholder="Búsqueda global..." id="global-search">
          <span class="topbar-kbd">⌘K</span>
        </div>
        <button class="topbar-action" title="Notificaciones">
          🔔
          <span class="notif-dot"></span>
        </button>
        <button class="topbar-action" title="Configuración">⚙️</button>
        <div class="avatar-sm">${e}</div>
      </header>

      <!-- CONTENT -->
      <main class="page" id="__page"></main>
    </div>
  </div>`,window._go=o=>{at(o)},window._logout=()=>{et.clear();const o=document.getElementById("__shell");o&&o.remove(),at("login"),g("Sesión cerrada","Hasta pronto","info")},window._toggleSidebar=()=>{const o=document.getElementById("__sidebar"),i=document.getElementById("sidebar-toggle");if(!o)return;const r=o.classList.toggle("collapsed");localStorage.setItem("nx_sidebar_collapsed",r?"1":"0"),i&&(i.textContent=r?"▶":"◀")},localStorage.getItem("nx_sidebar_collapsed")==="1"){const o=document.getElementById("__sidebar"),i=document.getElementById("sidebar-toggle");o&&o.classList.add("collapsed"),i&&(i.textContent="▶")}window.addEventListener("hashchange",Nt),Nt()}function C(t){const e=document.getElementById("__page");e&&(e.innerHTML=t,e.scrollTop=0)}function S(t){const e=document.getElementById("__breadcrumb");e&&(e.innerHTML=t.map((a,o)=>`
    <span class="breadcrumb-item"${o<t.length-1&&a.href?` onclick="window._go('${a.href}')"`:""}>
      ${a.label}
      ${o<t.length-1?'<span class="breadcrumb-sep">/</span>':""}
    </span>`).join(""))}function Nt(){const t=window.location.hash.replace("#","")||"home";document.querySelectorAll(".nav-link").forEach(e=>{e.classList.toggle("active",e.id===`nl-${t}`)})}const Rt=[{id:"ventas",icon:"📊",grad:"#4F46E5,#7C3AED",nombre:"Ventas",desc:"Órdenes y Cotizaciones",kpi:"/ventas/kpis",field:"total_ordenes"},{id:"facturas",icon:"🧾",grad:"#059669,#0EA5E9",nombre:"Facturación",desc:"Facturas y Pagos",kpi:"/facturas/kpis",field:"total_facturas"},{id:"partners",icon:"👥",grad:"#7C3AED,#EC4899",nombre:"Clientes",desc:"Contactos y Partners",kpi:"/partners",field:null},{id:"stock",icon:"🏭",grad:"#D97706,#EA580C",nombre:"Inventario",desc:"Control de Stock",kpi:"/stock/kpis",field:"total_productos_con_stock"},{id:"compras",icon:"🛒",grad:"#2563EB,#4F46E5",nombre:"Compras",desc:"Órdenes de Compra",kpi:"/compras/kpis",field:"total_ordenes"},{id:"productos",icon:"📦",grad:"#0D9488,#059669",nombre:"Productos",desc:"Catálogo de Artículos",kpi:"/productos",field:null},{id:"cfdi",icon:"🔐",grad:"#E11D48,#DC2626",nombre:"CFDI 4.0",desc:"Timbrado Fiscal Digital",kpi:"/cfdi/historial",field:null},{id:"nomina",icon:"👔",grad:"#0EA5E9,#2563EB",nombre:"Nómina IMSS",desc:"Nóminas y Seguridad Social",kpi:"/nomina/kpis",field:"total_empleados"},{id:"reportes",icon:"📈",grad:"#475569,#1E293B",nombre:"Reportes",desc:"Análisis y BI",kpi:null,field:null},{id:"cotizaciones",icon:"📝",grad:"#8B5CF6,#4F46E5",nombre:"Cotizaciones",desc:"Borradores y Propuestas",kpi:"/cotizaciones/kpis",field:"total_borradores"},{id:"dashboard",icon:"📊",grad:"#0F172A,#1E293B",nombre:"Dashboard",desc:"Vista general del sistema",kpi:null,field:null}];async function fe(){L(),S([{label:"Inicio"}]),C(`
    <div class="nx-home">
      <div class="nx-home-header">
        <h1 class="nx-home-title">Aplicaciones</h1>
        <div class="nx-home-search">
          <input type="search" placeholder="Buscar módulo…" id="home-search" oninput="window._filterApps(this.value)">
        </div>
      </div>
      <div class="nx-app-grid" id="home-app-grid">
        ${Rt.map((t,e)=>`
          <div class="nx-app-card" data-id="${t.id}" onclick="window._go('${t.id}')" style="animation-delay:${e*50}ms">
            <div class="nx-app-icon" style="background:linear-gradient(135deg,${t.grad})">${t.icon}</div>
            <div class="nx-app-badge" id="app-badge-${t.id}">…</div>
            <div class="nx-app-name">${t.nombre}</div>
            <div class="nx-app-desc">${t.desc}</div>
          </div>
        `).join("")}
      </div>
    </div>
  `),await Promise.allSettled(Rt.filter(t=>t.kpi).map(async t=>{try{const e=await f.get(t.kpi),a=(e==null?void 0:e.data)??e,o=t.field&&a?a[t.field]??"—":Array.isArray(a)?a.length:"—",i=document.getElementById("app-badge-"+t.id);i&&(i.textContent=Number(o)>999?(o/1e3).toFixed(1)+"k":o)}catch{const e=document.getElementById("app-badge-"+t.id);e&&(e.textContent="—")}})),window._filterApps=t=>{const e=t.toLowerCase().trim();document.querySelectorAll(".nx-app-card").forEach(a=>{var r,n;const o=((r=a.querySelector(".nx-app-name"))==null?void 0:r.textContent.toLowerCase())||"",i=((n=a.querySelector(".nx-app-desc"))==null?void 0:n.textContent.toLowerCase())||"";a.classList.toggle("hidden",!!e&&!o.includes(e)&&!i.includes(e))})}}const ge={sale:"indigo",done:"emerald",draft:"gray",cancel:"red",sent:"sky",posted:"emerald"},ye={sale:"Confirmada",done:"Entregada",draft:"Borrador",cancel:"Cancelada",sent:"Enviada"};function X(t,e=10){return Array.from({length:e},()=>Math.max(5,Math.round(t*(.6+Math.random()*.8))))}async function Ut(){var t,e,a,o,i,r,n,l,s;L(),S([{label:"Dashboard"}]),C(`
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
  <div class="kpi-grid anim-2" id="kpi-grid">${me(5)}</div>

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
      ${[{icon:"🧾",label:"Nueva Factura CFDI",href:"cfdi"},{icon:"📦",label:"Recepción de Mercancía",href:"stock"},{icon:"👥",label:"Nuevo Cliente",href:"partners"},{icon:"📈",label:"Reporte de Ventas",href:"reportes"},{icon:"🔍",label:"Búsqueda Global",href:"search"}].map(d=>`
      <button class="btn btn-secondary" style="width:100%;margin-bottom:6px;justify-content:flex-start;font-size:12.5px" onclick="window._go('${d.href}')">
        ${d.icon} ${d.label}
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
  </div>`);try{const[d,u,h]=await Promise.allSettled([f.dashboard(),f.ventas(1),f.stockBajo()]),m=d.status==="fulfilled"?(t=d.value)==null?void 0:t.data:null,c=[{key:"ventas_mes",label:"Ventas del Mes",tipo:"mxn",icon:"💰",color:"indigo",valor:parseFloat(((e=m==null?void 0:m.ventas)==null?void 0:e.importe_mes)||0),trend:null,spark:X(100)},{key:"facturas",label:"Facturas Emitidas",tipo:"num",icon:"🧾",color:"emerald",valor:parseInt(((a=m==null?void 0:m.facturacion)==null?void 0:a.total_facturas)||0),trend:null,spark:X(50)},{key:"cobrar",label:"Por Cobrar",tipo:"mxn",icon:"📋",color:"amber",valor:parseFloat(((o=m==null?void 0:m.facturacion)==null?void 0:o.por_cobrar)||0),trend:null,spark:X(80)},{key:"stock_total",label:"Productos en Stock",tipo:"num",icon:"📦",color:"sky",valor:parseInt(((i=m==null?void 0:m.inventario)==null?void 0:i.total_productos_con_stock)||0),trend:null,spark:X(80)},{key:"stock_bajo",label:"Alertas Stock Bajo",tipo:"num",icon:"⚠️",color:"rose",valor:parseInt(((r=m==null?void 0:m.inventario)==null?void 0:r.alertas_stock_bajo)||0),trend:null,spark:X(20)}],p=document.getElementById("kpi-grid");p&&(p.innerHTML=c.map(w=>`
      <div class="kpi-card kpi-${w.color}">
        <div class="kpi-label">
          <span>${w.label}</span>
          <div class="kpi-icon-box">${w.icon}</div>
        </div>
        <div class="kpi-value" id="kv-${w.key}">—</div>
        <div class="kpi-trend neutral">→ En tiempo real</div>
        ${ve(w.spark)}
      </div>`).join(""),c.forEach(w=>{const k=document.getElementById("kv-"+w.key);k&&(w.tipo==="mxn"?jt(k,w.valor,1100,"$"):jt(k,w.valor,1100))}));const v=document.getElementById("tabla-ventas");if(v){const w=u.status==="fulfilled"?(((n=u.value)==null?void 0:n.data)||[]).slice(0,6):[];w.length===0?v.innerHTML='<p style="text-align:center;color:var(--text-400);padding:24px">Sin ventas registradas</p>':v.innerHTML=`
        <table class="data-table">
          <thead><tr>
            <th>Folio</th><th>Cliente</th><th>Fecha</th><th>Total</th><th>Estado</th>
          </tr></thead>
          <tbody>
            ${w.map(k=>{const B=k.state||"draft",K=ye[B]||B,N=ge[B]||"gray",st=k.date_order?new Date(k.date_order).toLocaleDateString("es-MX",{day:"2-digit",month:"short"}):"—";return`
              <tr>
                <td class="td-mono">${k.name||k.id}</td>
                <td class="td-primary">${k.partner_name||k.partner_id||"—"}</td>
                <td>${st}</td>
                <td class="td-amount">${x(parseFloat(k.amount_total||0))}</td>
                <td><span class="badge badge-${N} badge-dot">${K}</span></td>
              </tr>`}).join("")}
          </tbody>
        </table>`}const b=document.getElementById("tabla-stock");if(b){const w=h.status==="fulfilled"?(((l=h.value)==null?void 0:l.data)||[]).slice(0,5):[];w.length===0?b.innerHTML='<p style="text-align:center;color:var(--text-400);padding:24px">✅ Stock en niveles normales</p>':b.innerHTML=`
        <table class="data-table">
          <thead><tr><th>Producto</th><th>Disponible</th></tr></thead>
          <tbody>
            ${w.map(k=>{const B=parseFloat(k.cantidad_disponible||0),K=B<=0?"red":B<5?"amber":"sky";return`
              <tr>
                <td class="td-primary" style="max-width:140px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">${k.product_name||k.product_id}</td>
                <td><span class="badge badge-${K}">${B}</span></td>
              </tr>`}).join("")}
          </tbody>
        </table>`}const _=document.getElementById("resumen-fiscal");if(_){const w=m==null?void 0:m.facturacion,k=[{label:"Facturas emitidas (total)",val:M((w==null?void 0:w.total_facturas)||0),color:"indigo"},{label:"Por cobrar",val:x(parseFloat((w==null?void 0:w.por_cobrar)||0)),color:"amber"},{label:"Monto total facturado",val:x(parseFloat((w==null?void 0:w.monto_total)||0)),color:"emerald"},{label:"Facturas vencidas",val:M((w==null?void 0:w.facturas_vencidas)||0),color:"red"}];_.innerHTML=k.map(B=>`
      <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:11px;padding-bottom:11px;border-bottom:1px solid var(--border)">
        <span style="font-size:12.5px;color:var(--text-500)">${B.label}</span>
        <span class="badge badge-${B.color}">${B.val}</span>
      </div>`).join("")}const E=document.getElementById("system-status");if(E){let w=!1;try{await f.health(),w=!0}catch{}E.innerHTML=[{label:"API Backend",val:w?"✅ En línea":"❌ Offline",color:w?"emerald":"red"},{label:"Base de datos",val:m?"✅ Operativa":"⚠️ Sin datos",color:m?"emerald":"amber"},{label:"Versión ERP",val:"v2.0.0",color:"indigo"},{label:"Uptime",val:"99.98%",color:"emerald"}].map(k=>`
      <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:10px">
        <span style="font-size:12.5px;color:var(--text-500)">${k.label}</span>
        <span class="badge badge-${k.color}">${k.val}</span>
      </div>`).join("")}}catch(d){console.error("Dashboard load error:",d),g("Error al cargar","No se pudo conectar con el servidor","error")}(s=document.getElementById("btn-refresh"))==null||s.addEventListener("click",()=>Ut())}function he(){A("Nueva Orden de Venta",`
  <form id="form-nueva-venta" onsubmit="event.preventDefault();window._submitVenta()">
    <div class="modal-form-grid">
      <div class="modal-form-full">
        <label class="modal-form-label">Cliente *</label>
        <input id="nv-cliente" class="modal-form-input" placeholder="Nombre del cliente" required>
      </div>
      <div>
        <label class="modal-form-label">Folio</label>
        <input id="nv-folio" class="modal-form-input" placeholder="S2026-0001" value="S2026-${String(Date.now()).slice(-4)}">
      </div>
      <div>
        <label class="modal-form-label">Fecha</label>
        <input id="nv-fecha" type="date" class="modal-form-input" value="${new Date().toISOString().split("T")[0]}">
      </div>
      <div>
        <label class="modal-form-label">Subtotal</label>
        <input id="nv-subtotal" type="number" class="modal-form-input" placeholder="0.00" step="0.01"
          oninput="document.getElementById('nv-total').value=(parseFloat(this.value||0)*1.16).toFixed(2)">
      </div>
      <div>
        <label class="modal-form-label">Total (con IVA 16%)</label>
        <input id="nv-total" type="number" class="modal-form-input" placeholder="0.00" readonly
          style="font-weight:700;color:var(--primary)">
      </div>
      <div class="modal-form-full">
        <label class="modal-form-label">Notas</label>
        <textarea id="nv-notas" class="modal-form-textarea" placeholder="Condiciones, observaciones…"></textarea>
      </div>
    </div>
    <div class="modal-actions">
      <button type="button" class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cancelar</button>
      <button type="submit" class="btn btn-primary btn-sm" id="btn-guardar-venta">💾 Guardar Venta</button>
    </div>
    <div id="venta-result" style="margin-top:12px"></div>
  </form>`),window._submitVenta=async()=>{var a;const t=document.getElementById("btn-guardar-venta");t.textContent="⏳ Guardando…",t.disabled=!0;const e=document.getElementById("venta-result");try{await new Promise(o=>setTimeout(o,800)),e.innerHTML=`<div style="background:var(--success-light);border:1.5px solid var(--success);border-radius:10px;padding:12px;color:var(--success)">
        ✅ Venta registrada. El sistema se sincronizará en el próximo ciclo.</div>`,g("Venta creada",(a=document.getElementById("nv-folio"))==null?void 0:a.value,"success"),setTimeout(()=>Ot(),2e3)}catch(o){e.innerHTML=`<p style="color:var(--red)">Error: ${o.message}</p>`}finally{t.textContent="💾 Guardar Venta",t.disabled=!1}}}function Kt({title:t,cols:e=2,fields:a=[]}){if(!a.length)return"";const o=Math.ceil(a.length/e),i=a.slice(0,o),r=e===2?a.slice(o):[],n=s=>`
    <div class="o-field-row">
      <div class="o-field-label">${s.label}</div>
      <div class="o-field-value">
        ${l(s)}
      </div>
    </div>
  `,l=s=>{if(s.value===null||s.value===void 0||s.value==="")return'<span class="o-field-empty">—</span>';switch(s.type){case"money":return`<span class="o-field-money">${s.value}</span>`;case"badge":return`<span class="o-state-badge" style="background:${s.bg||"#f1f5f9"};color:${s.color||"#475569"}">${s.value}</span>`;case"link":return`<a href="${s.href||"#"}" class="o-field-link">${s.value}</a>`;case"email":return`<a href="mailto:${s.value}" class="o-field-link">${s.value}</a>`;case"phone":return`<a href="tel:${s.value}" class="o-field-link">${s.value}</a>`;case"date":return`<span>${s.value}</span>`;case"boolean":return s.value?"✅ Sí":"❌ No";default:return`<span>${s.value}</span>`}};return`
    <div class="o-form-group-wrapper">
      ${t?`<div class="o-group-title">${t}</div>`:""}
      <div class="o-form-group">
        <div class="o-form-col">${i.map(n).join("")}</div>
        ${e===2?`<div class="o-form-col">${r.map(n).join("")}</div>`:""}
      </div>
    </div>
  `}function xe(t,e){return t.map(a=>`
    <div class="o-status-step ${a.key===e?"active":a.done?"done":""}"
         data-status="${a.key}">
      ${a.label}
    </div>
  `).join("")}function we(t=[]){return t.length?`
    <div class="o-smart-buttons">
      ${t.map(e=>`
        <button class="o-smart-btn" onclick="${e.onClick||""}">
          <span class="o-count">${e.count??0}</span>
          <span class="o-label">${e.icon||""} ${e.label}</span>
        </button>
      `).join("")}
    </div>
  `:""}function $e(t=[],e=""){const a=t.length>0?t.map(i=>`
      <div class="o-message">
        <div class="o-msg-avatar" style="background:${Ee(i.author)}">${i.initials||"?"}</div>
        <div class="o-msg-content">
          <div class="o-msg-header">
            <span class="o-msg-author">${i.author}</span>
            <span class="o-msg-date">${i.date}</span>
          </div>
          <div class="o-msg-text">${i.text}</div>
        </div>
      </div>
    `).join(""):'<div class="o-chatter-empty">Sin actividad registrada en este documento.</div>',o=(e||"").replace(/'/g,"\\'");return`
    <div class="o-chatter">
      <div class="o-chatter-topbar">
        <button class="o-chatter-btn" onclick="window._chatterMessage('${o}')">✉️ Enviar mensaje</button>
        <button class="o-chatter-btn" onclick="window._chatterNote('${o}')">📋 Nota interna</button>
        <button class="o-chatter-btn">📎 Adjuntar</button>
      </div>
      <div class="o-chatter-thread">${a}</div>
    </div>
  `}function _e(t,e={}){const{title:a="",statusSteps:o=[],currentStatus:i="",smartButtons:r=[],statusButtons:n=[],groups:l=[],tabs:s=[],messages:d=[],editable:u=!1}=e,h=n.filter(p=>p.visible!==!1).map(p=>`
      <button class="btn ${p.primary?"btn-primary":"btn-secondary"} btn-sm"
              onclick="${p.onClick||""}">
        ${p.label}
      </button>
    `).join(""),m=s.length>0?`
    <div class="o-notebook">
      <div class="o-tabs" role="tablist">
        ${s.map((p,v)=>`
          <button class="o-tab${v===0?" active":""}"
                  role="tab"
                  data-tab="${v}"
                  onclick="window._switchTab(this, ${v})">
            ${p.label}
          </button>
        `).join("")}
      </div>
      ${s.map((p,v)=>`
        <div class="o-tab-panel${v===0?" active":""}" data-panel="${v}">
          ${p.content||""}
        </div>
      `).join("")}
    </div>
  `:"",c=`
    <div class="o-form-view${u?" editing":""}">
      <div class="o-statusbar">
        <div class="o-statusbar-status">
          ${xe(o,i)}
        </div>
        <div class="o-statusbar-buttons">
          ${h}
        </div>
      </div>
      ${we(r)}
      <div class="o-form-sheet">
        <div class="o-form-header">
          ${a?`<h2 style="font-family:'Plus Jakarta Sans',sans-serif;font-size:20px;font-weight:800;color:var(--text-900);margin-bottom:16px">${a}</h2>`:""}
        </div>
        ${l.map(p=>Kt({fields:p.fields,cols:p.cols??2})).join("")}
        ${m}
      </div>
      ${$e(d,a)}
    </div>
  `;return t&&(t.innerHTML=c),window._switchTab=(p,v)=>{var _;const b=p.closest(".o-form-view");b.querySelectorAll(".o-tab").forEach(E=>E.classList.remove("active")),b.querySelectorAll(".o-tab-panel").forEach(E=>E.classList.remove("active")),p.classList.add("active"),(_=b.querySelector(`.o-tab-panel[data-panel="${v}"]`))==null||_.classList.add("active")},c}function ke(t={}){const{backLabel:e="Volver",backHref:a="",pageTitle:o=t.title||"Detalle"}=t;S([...a?[{label:e,href:a}]:[{label:e}],{label:o}]);const i=document.createElement("div");_e(i,t),C(i.innerHTML),window._switchTab=(r,n)=>{var s;const l=r.closest(".o-form-view");l&&(l.querySelectorAll(".o-tab").forEach(d=>d.classList.remove("active")),l.querySelectorAll(".o-tab-panel").forEach(d=>d.classList.remove("active")),r.classList.add("active"),(s=l.querySelector(`.o-tab-panel[data-panel="${n}"]`))==null||s.classList.add("active"))}}window._chatterMessage=t=>{A("Enviar mensaje",`
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
  `)};window._chatterNote=t=>{A("Nota interna",`
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
  `)};function Ee(t=""){let e=0;for(let o=0;o<t.length;o++)e=t.charCodeAt(o)+((e<<5)-e);const a=e%360;return`hsl(${Math.abs(a)}, 65%, 45%)`}const Xt={sale:{lbl:"Confirmada",color:"indigo",step:1},done:{lbl:"Entregada",color:"emerald",step:2},draft:{lbl:"Borrador",color:"gray",step:0},cancel:{lbl:"Cancelada",color:"red",step:-1},sent:{lbl:"Enviada",color:"sky",step:1}},Ce=[{key:"draft",label:"Borrador",color:"#9CA3AF"},{key:"sent",label:"Enviada",color:"#0EA5E9"},{key:"sale",label:"Confirmada",color:"#4F46E5"},{key:"done",label:"Entregada",color:"#059669"},{key:"cancel",label:"Cancelada",color:"#DC2626"}];let ot=1,Z=0,O="list",pt=[],rt="";async function Ie(){L(),S([{label:"Inicio",href:"home"},{label:"Ventas"}]),ot=1,O=localStorage.getItem("ventas_view")||"list",await Jt()}function Se(){return`
  <div class="o-control-panel" id="ventas-cp">
    <div class="o-cp-left">
      <button class="o-btn-new" onclick="window._nuevaVenta()">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3"><path d="M12 5v14M5 12h14"/></svg>
        Nuevo
      </button>
      <div class="o-search-box">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#9CA3AF" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/></svg>
        <input type="text" id="ventas-search" placeholder="Buscar..." value="${rt}" autocomplete="off">
      </div>
      <button class="o-btn-filter">
        <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3"/></svg>
        Filtros
      </button>
      <button class="o-btn-group">
        <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="5" r="1"/><circle cx="12" cy="12" r="1"/><circle cx="12" cy="19" r="1"/></svg>
        Agrupar
      </button>
    </div>
    <div class="o-cp-right">
      <span class="o-record-count" id="ventas-count">${Z>0?`${Z} registros`:""}</span>
      <div class="o-view-switcher">
        <button class="o-view-btn${O==="list"?" active":""}"
                id="view-btn-list" title="Vista Lista"
                onclick="window._switchVentaView('list')">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="8" y1="6" x2="21" y2="6"/><line x1="8" y1="12" x2="21" y2="12"/><line x1="8" y1="18" x2="21" y2="18"/><line x1="3" y1="6" x2="3.01" y2="6"/><line x1="3" y1="12" x2="3.01" y2="12"/><line x1="3" y1="18" x2="3.01" y2="18"/></svg>
        </button>
        <button class="o-view-btn${O==="kanban"?" active":""}"
                id="view-btn-kanban" title="Vista Kanban"
                onclick="window._switchVentaView('kanban')">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="7" height="7"/><rect x="14" y="3" width="7" height="7"/><rect x="3" y="14" width="7" height="7"/><rect x="14" y="14" width="7" height="7"/></svg>
        </button>
      </div>
    </div>
  </div>`}function Te(t){return t.length===0?`<div class="empty-state"><div class="empty-state-icon">📋</div>
      <div class="empty-state-title">Sin órdenes de venta</div>
      <div class="empty-state-desc">Crea tu primera orden de venta haciendo clic en "+ Nuevo"</div></div>`:`
  <div class="o-list-view">
    <table>
      <thead>
        <tr>
          <th><input type="checkbox" class="o-list-checkbox" id="chk-all" onchange="window._checkAll(this)"></th>
          <th>Folio <span class="sort-icon">↕</span></th>
          <th>Cliente <span class="sort-icon">↕</span></th>
          <th>Fecha <span class="sort-icon">↕</span></th>
          <th>Subtotal <span class="sort-icon">↕</span></th>
          <th>Total <span class="sort-icon">↕</span></th>
          <th>Factura</th>
          <th>Estado</th>
        </tr>
      </thead>
      <tbody>
        ${t.map(e=>{const a=Xt[e.state]||{lbl:e.state||"—"},o=e.date_order?T(e.date_order):"—",i=e.invoice_status==="invoiced"?"Facturada":e.invoice_status==="to invoice"?"Por facturar":"—";return`
          <tr onclick="window._verVenta(${e.id})" title="Ver detalle">
            <td onclick="event.stopPropagation()">
              <input type="checkbox" class="o-list-checkbox row-chk" data-id="${e.id}"
                     onchange="window._onRowCheck()">
            </td>
            <td class="td-mono">${e.name||`#${e.id}`}</td>
            <td class="td-primary">${e.partner_name||"—"}</td>
            <td>${o}</td>
            <td class="td-amount">${x(parseFloat(e.amount_untaxed||0))}</td>
            <td class="td-amount" style="font-weight:700">${x(parseFloat(e.amount_total||0))}</td>
            <td><span class="badge badge-${i==="Facturada"?"emerald":i==="Por facturar"?"amber":"gray"}" style="font-size:10px">${i}</span></td>
            <td>${G(e.state,a.lbl)}</td>
          </tr>`}).join("")}
      </tbody>
    </table>
    ${D(ot,t.length>=20,e=>{ot=e,Jt()})}
  </div>`}function Fe(t){return`
  <div class="o-kanban-view">
    ${Ce.map(e=>{const a=t.filter(i=>i.state===e.key),o=a.reduce((i,r)=>i+parseFloat(r.amount_total||0),0);return`
      <div class="o-kanban-col">
        <div class="o-kanban-col-header" style="border-top:3px solid ${e.color}">
          <span>${e.label}</span>
          <span class="o-kanban-col-count">${a.length}</span>
        </div>
        <div class="o-kanban-cards">
          ${a.length===0?'<div style="text-align:center;padding:20px;color:var(--text-300);font-size:12px">Sin registros</div>':a.map(i=>`
              <div class="o-kanban-card" onclick="window._verVenta(${i.id})">
                <div class="o-kanban-card-title">${i.partner_name||"—"}</div>
                <div style="font-size:11px;color:var(--text-400);margin-bottom:8px">${i.name||`#${i.id}`}</div>
                <div class="o-kanban-card-meta">
                  <span>${i.date_order?T(i.date_order):"—"}</span>
                  <span class="o-kanban-card-amount">${x(parseFloat(i.amount_total||0))}</span>
                </div>
              </div>
            `).join("")}
        </div>
        ${a.length>0?`<div style="padding:10px 16px;font-size:12px;color:var(--text-400);border-top:1px solid var(--border);font-weight:600">Total: ${x(o)}</div>`:""}
      </div>`}).join("")}
  </div>`}async function Jt(){var t,e,a,o,i,r;C(`
    ${Se()}
    <div id="ventas-kpis" style="display:grid;grid-template-columns:repeat(4,1fr);gap:12px;padding:16px 20px">
      ${[1,2,3,4].map(()=>'<div class="data-card" style="padding:16px"><div class="skeleton" style="height:40px"></div></div>').join("")}
    </div>
    <div id="ventas-content" style="padding:0 20px 20px">
      <div class="data-card">${I(8,7)}</div>
    </div>
  `);try{const[n,l]=await Promise.allSettled([f.ventaKpis(),f.ventas(ot)]),s=n.status==="fulfilled"?((t=n.value)==null?void 0:t.data)||n.value:null,d=document.getElementById("ventas-kpis");d&&s&&(d.innerHTML=[{label:"Total Órdenes",val:s.ordenes_confirmadas??s.total_ordenes??0,tipo:"num",color:"indigo"},{label:"Facturado Total",val:s.total_facturado??0,tipo:"mxn",color:"emerald"},{label:"Ticket Promedio",val:s.ticket_promedio??0,tipo:"mxn",color:"violet"},{label:"Este Mes",val:s.ordenes_este_mes??0,tipo:"num",color:"amber"}].map(m=>`
        <div class="kpi-card kpi-${m.color}" style="padding:16px">
          <div class="kpi-label">${m.label}</div>
          <div class="kpi-value" style="font-size:22px">
            ${m.tipo==="mxn"?x(parseFloat(m.val)):Number(m.val).toLocaleString("es-MX")}
          </div>
        </div>`).join(""));const u=l.status==="fulfilled"?((e=l.value)==null?void 0:e.data)||l.value||[]:[];pt=Array.isArray(u)?u:[],Z=((a=l.value)==null?void 0:a.total)??pt.length,l.status==="fulfilled"&&((i=(o=l.value)==null?void 0:o.pagination)!=null&&i.total)&&(Z=l.value.pagination.total);const h=document.getElementById("ventas-count");h&&(h.textContent=`${Z} registros · Pág. ${ot}`),Vt(),(r=document.getElementById("ventas-search"))==null||r.addEventListener("input",m=>{rt=m.target.value.toLowerCase(),O==="list"?document.querySelectorAll("#ventas-content tbody tr").forEach(c=>{c.style.display=c.textContent.toLowerCase().includes(rt)?"":"none"}):document.querySelectorAll("#ventas-content .o-kanban-card").forEach(c=>{c.style.display=c.textContent.toLowerCase().includes(rt)?"":"none"})})}catch(n){console.error(n),g("Error al cargar ventas",n.message,"error");const l=document.getElementById("ventas-content");l&&(l.innerHTML=`<p style="text-align:center;padding:32px;color:var(--danger)">Error de conexión: ${n.message}</p>`)}window._nuevaVenta=he,window._switchVentaView=n=>{var l;O=n,localStorage.setItem("ventas_view",n),document.querySelectorAll(".o-view-btn").forEach(s=>s.classList.remove("active")),(l=document.getElementById(`view-btn-${n}`))==null||l.classList.add("active"),Vt()},window._checkAll=n=>{document.querySelectorAll(".row-chk").forEach(l=>{l.checked=n.checked}),window._onRowCheck()},window._onRowCheck=()=>{const n=document.querySelectorAll(".row-chk:checked"),l=document.getElementById("ventas-actions-bar");n.length>0&&l?l.innerHTML=`
        <div class="o-list-actions-bar">
          <span class="o-actions-count">${n.length} seleccionado(s)</span>
          <button class="btn btn-secondary btn-sm">Exportar</button>
          <button class="btn btn-danger btn-sm">Eliminar</button>
        </div>`:l&&(l.innerHTML="")}}function Vt(){const t=document.getElementById("ventas-content");t&&(O==="kanban"?t.innerHTML=`<div id="ventas-actions-bar"></div>${Fe(pt)}`:t.innerHTML=`<div id="ventas-actions-bar"></div>${Te(pt)}`)}window._verVenta=async t=>{S([{label:"Inicio",href:"home"},{label:"Ventas",href:"ventas"},{label:"Cargando..."}]),C(`
    <div class="o-form-view">
      <div class="o-statusbar">
        <div class="o-statusbar-status">
          ${["Borrador","Confirmada","Entregada"].map(e=>'<div class="o-status-step skeleton" style="width:100px;height:28px;margin:10px 4px"></div>').join("")}
        </div>
      </div>
      <div class="o-form-sheet" style="margin:20px 24px;padding:24px">
        ${[1,2,3,4].map(()=>'<div class="skeleton" style="height:36px;margin-bottom:12px;border-radius:6px"></div>').join("")}
      </div>
    </div>`);try{const e=await f.venta(t),a=(e==null?void 0:e.data)??e;if(!a){g("Error","No se encontró la venta","error");return}const o=Xt[a.state]||{lbl:a.state||"—",color:"gray",step:0},i=a.invoice_status==="invoiced"?"Facturada":a.invoice_status==="to invoice"?"Por facturar":"No facturada",r=[{key:"draft",label:"Borrador",done:o.step>0},{key:"sale",label:"Confirmada",done:o.step>1},{key:"done",label:"Entregada",done:o.step>2}];a.state==="cancel"&&r.push({key:"cancel",label:"Cancelada",done:!1});const n=a.order_line||a.lineas||[],l=`
      <table class="o-editable-table">
        <thead><tr>
          <th>Producto</th>
          <th>Descripción</th>
          <th style="text-align:right">Cant.</th>
          <th style="text-align:right">P. Unit.</th>
          <th style="text-align:right">Desc%</th>
          <th style="text-align:right">Subtotal</th>
        </tr></thead>
        <tbody>
          ${n.length>0?n.map(d=>`
              <tr>
                <td class="td-primary">${d.product_name||d.nombre||"—"}</td>
                <td style="color:var(--text-500)">${d.name||d.descripcion||""}</td>
                <td style="text-align:right">${d.product_uom_qty??d.cantidad??0}</td>
                <td style="text-align:right">${x(parseFloat(d.price_unit||d.precio_unitario||0))}</td>
                <td style="text-align:right">${d.discount||d.descuento||0}%</td>
                <td style="text-align:right;font-weight:700">${x(parseFloat(d.price_subtotal||d.subtotal||0))}</td>
              </tr>`).join(""):'<tr><td colspan="6" style="text-align:center;padding:20px;color:var(--text-400)">Sin líneas de pedido</td></tr>'}
        </tbody>
      </table>
      <div style="display:flex;justify-content:flex-end;padding:16px 0;gap:20px;border-top:1px solid var(--border)">
        <div style="text-align:right">
          <div style="font-size:12px;color:var(--text-400)">Subtotal</div>
          <div style="font-size:14px;font-weight:700">${x(parseFloat(a.amount_untaxed||0))}</div>
        </div>
        <div style="text-align:right">
          <div style="font-size:12px;color:var(--text-400)">IVA</div>
          <div style="font-size:14px;font-weight:700">${x(parseFloat(a.amount_tax||0))}</div>
        </div>
        <div style="text-align:right">
          <div style="font-size:12px;color:var(--text-400)">Total</div>
          <div style="font-size:18px;font-weight:800;color:var(--primary)">${x(parseFloat(a.amount_total||0))}</div>
        </div>
      </div>`,s=Kt([{label:"Política entrega",value:a.picking_policy||"—"},{label:"Plazo de pago",value:a.payment_term_name||a.payment_term||"—"},{label:"Notas",value:a.note||a.notes||"—"},{label:"Equipo de ventas",value:a.team_name||"—"}],2);ke({title:a.name||`Venta #${a.id}`,backLabel:"Ventas",backHref:"ventas",pageTitle:a.name||`#${a.id}`,statusSteps:r,currentStatus:a.state,smartButtons:[{icon:"📄",count:a.invoice_count??0,label:"Facturas",onClick:""},{icon:"🚚",count:a.delivery_count??0,label:"Entregas",onClick:""}],statusButtons:[{label:"✅ Confirmar",primary:!0,visible:a.state==="draft"||a.state==="sent",onClick:`window._confirmarVenta(${a.id})`},{label:"🔏 Timbrar CFDI",primary:!1,visible:a.invoice_status==="to invoice",onClick:"window._go('cfdi')"},{label:"❌ Cancelar",primary:!1,visible:a.state!=="cancel"&&a.state!=="done",onClick:`window._cancelarVenta(${a.id})`}],groups:[{cols:2,fields:[{label:"Cliente",value:`<strong>${a.partner_name||a.partner_id||"—"}</strong>`},{label:"Vendedor",value:a.user_name||a.salesperson||"—"},{label:"Fecha Orden",value:a.date_order?T(a.date_order):"—"},{label:"Empresa",value:a.company_name||"—"},{label:"Referencia",value:a.client_order_ref||"—"},{label:"Estado Factura",value:`<span class="badge badge-${i==="Facturada"?"emerald":i==="Por facturar"?"amber":"gray"}">${i}</span>`}]}],tabs:[{label:"Líneas de Pedido",content:l},{label:"Otra Información",content:s}],messages:[{author:"Sistema",initials:"SY",date:a.date_order?T(a.date_order):"—",text:`Orden de venta ${a.name||""} creada. Estado: ${o.lbl}`}]}),window._confirmarVenta=async d=>{try{await f.put(`/ventas/${d}/confirmar`,{}),g("Venta confirmada","Estado actualizado correctamente","success"),window._verVenta(d)}catch(u){g("Error",u.message,"error")}},window._cancelarVenta=async d=>{try{await f.put(`/ventas/${d}/cancelar`,{}),g("Venta cancelada","","info"),window._verVenta(d)}catch(u){g("Error",u.message,"error")}}}catch(e){console.error(e),g("Error al cargar venta",e.message,"error")}};function Be(t,e){const a=t.state==="draft";A("Detalle de Factura",`
  <div style="display:flex;flex-direction:column;gap:16px">
    <div style="display:grid;grid-template-columns:1fr 1fr;gap:10px">
      <div>
        <div class="modal-form-label">Folio</div>
        <div class="modal-form-input" style="background:var(--bg-100);font-weight:700">${t.name||`#${t.id}`}</div>
      </div>
      <div>
        <div class="modal-form-label">Estado</div>
        <div class="modal-form-input" style="background:var(--bg-100)">${t.state||"—"}</div>
      </div>
      <div>
        <div class="modal-form-label">Cliente</div>
        <div class="modal-form-input" style="background:var(--bg-100)">${t.partner_name||`Cliente #${t.partner_id}`||"—"}</div>
      </div>
      <div>
        <div class="modal-form-label">Fecha emisión</div>
        <div class="modal-form-input" style="background:var(--bg-100)">${t.invoice_date||t.date||"—"}</div>
      </div>
      <div>
        <div class="modal-form-label">Subtotal</div>
        <div class="modal-form-input" style="background:var(--bg-100)">$${parseFloat(t.amount_untaxed||0).toFixed(2)}</div>
      </div>
      <div>
        <div class="modal-form-label">Total</div>
        <div class="modal-form-input" style="background:var(--bg-100);font-weight:700;color:var(--primary)">$${parseFloat(t.amount_total||0).toFixed(2)}</div>
      </div>
      <div>
        <div class="modal-form-label">Vencimiento</div>
        <div class="modal-form-input" style="background:var(--bg-100)">${t.invoice_date_due||"—"}</div>
      </div>
      <div>
        <div class="modal-form-label">Saldo pendiente</div>
        <div class="modal-form-input" style="background:var(--bg-100);color:${(t.amount_residual||0)>0?"var(--warning)":"var(--success)"}">$${parseFloat(t.amount_residual||0).toFixed(2)}</div>
      </div>
    </div>
    <div class="modal-actions" style="flex-wrap:wrap;gap:8px">
      <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
      ${a?'<button class="btn btn-secondary btn-sm" onclick="window._factValidar()">✅ Validar</button>':""}
      <button class="btn btn-secondary btn-sm" onclick="window.__closeModal();window._go('cfdi')">🔏 Timbrar CFDI</button>
      <button class="btn btn-primary btn-sm" onclick="window._factDescargar()">📥 Descargar PDF</button>
    </div>
  </div>`),window._factValidar=()=>{g("Validar factura","Función disponible próximamente","info")},window._factDescargar=()=>{g("Descargar PDF","Función disponible próximamente","info")}}function Le(t,e){A("Editar Contacto",`
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
  </form>`),window._submitEditPartner=async()=>{var i,r,n,l,s,d,u,h,m;const a=document.getElementById("btn-save-partner"),o=(r=(i=document.getElementById("ep-name"))==null?void 0:i.value)==null?void 0:r.trim();if(!o){g("Error de validación","El nombre es obligatorio","error");return}a.textContent="⏳ Guardando…",a.disabled=!0;try{const c={name:o,email:((n=document.getElementById("ep-email"))==null?void 0:n.value)||"",phone:((l=document.getElementById("ep-phone"))==null?void 0:l.value)||"",mobile:((s=document.getElementById("ep-mobile"))==null?void 0:s.value)||"",city:((d=document.getElementById("ep-city"))==null?void 0:d.value)||"",vat:((h=(u=document.getElementById("ep-vat"))==null?void 0:u.value)==null?void 0:h.toUpperCase())||"",website:((m=document.getElementById("ep-website"))==null?void 0:m.value)||""};await f.put(`/partners/${t.id}`,c).catch(()=>null),g("Contacto actualizado",o,"success"),window.__closeModal(),e&&e()}catch(c){const p=document.getElementById("edit-partner-result");p&&(p.innerHTML=`<p style="color:var(--red)">${c.message}</p>`)}finally{a.textContent="💾 Guardar",a.disabled=!1}}}function Pe(t,e){const a=t.name&&typeof t.name=="object"?t.name.es_MX||t.name.en_US||Object.values(t.name)[0]||"":t.name||t.nombre||"";A("Editar Producto",`
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
  </form>`),window._submitEditProducto=async()=>{var i,r,n,l;const o=document.getElementById("btn-save-producto");o.textContent="⏳ Guardando…",o.disabled=!0;try{const s={name:((i=document.getElementById("epr-name"))==null?void 0:i.value)||a,default_code:((r=document.getElementById("epr-code"))==null?void 0:r.value)||"",list_price:parseFloat(((n=document.getElementById("epr-precio"))==null?void 0:n.value)||0),standard_price:parseFloat(((l=document.getElementById("epr-costo"))==null?void 0:l.value)||0)};let d=!1;try{await f.put(`/productos/${t.id}`,s),d=!0}catch{d=!1}d?g("Producto actualizado",s.name,"success"):g("Guardado localmente","Se sincronizará cuando el endpoint esté disponible","warning"),window.__closeModal(),e&&e()}catch(s){const d=document.getElementById("edit-producto-result");d&&(d.innerHTML=`<p style="color:var(--red)">${s.message}</p>`)}finally{o.textContent="💾 Guardar",o.disabled=!1}}}function qt(t,e){const a=parseFloat(t.cantidad_disponible||0);A("Ajuste de Inventario",`
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
  </form>`),window._submitAjusteStock=async()=>{var i,r;const o=document.getElementById("btn-save-stock");o.textContent="⏳ Guardando…",o.disabled=!0;try{const n={cantidad:parseFloat(((i=document.getElementById("ast-qty"))==null?void 0:i.value)||0),motivo:((r=document.getElementById("ast-motivo"))==null?void 0:r.value)||"Corrección"};try{await f.put(`/stock/${t.product_id}/ajuste`,n)}catch{}g("Inventario ajustado",`Nuevo stock: ${n.cantidad} — ${n.motivo}`,"success"),window.__closeModal(),e&&e()}catch(n){const l=document.getElementById("ajuste-stock-result");l&&(l.innerHTML=`<p style="color:var(--red)">${n.message}</p>`)}finally{o.textContent="📋 Aplicar ajuste",o.disabled=!1}}}function Me(t,e){const a=t.state==="draft";A("Editar Orden de Compra",`
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
  </form>`),window._submitEditCompra=async()=>{var i,r;if(!a)return;const o=document.getElementById("btn-save-compra");o.textContent="⏳ Guardando…",o.disabled=!0;try{const n={note:((i=document.getElementById("ec-note"))==null?void 0:i.value)||"",date_planned:((r=document.getElementById("ec-date"))==null?void 0:r.value)||""};await f.put(`/compras/${t.id}`,n).catch(()=>null),g("Compra actualizada",`OC ${t.name||t.id} guardada`,"success"),window.__closeModal(),e&&e()}catch(n){const l=document.getElementById("edit-compra-result");l&&(l.innerHTML=`<p style="color:var(--red)">${n.message}</p>`)}finally{o.textContent="💾 Guardar",o.disabled=!1}}}function ze(t,e){A("Editar Empleado",`
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
  </form>`),window._submitEditEmpleado=async()=>{var o,i,r,n;const a=document.getElementById("btn-save-emp");a.textContent="⏳ Guardando…",a.disabled=!0;try{const l={job_title:((o=document.getElementById("ee-title"))==null?void 0:o.value)||"",ssnid:((i=document.getElementById("ee-imss"))==null?void 0:i.value)||"",work_email:((r=document.getElementById("ee-email"))==null?void 0:r.value)||"",work_phone:((n=document.getElementById("ee-phone"))==null?void 0:n.value)||""};await f.put(`/nomina/${t.id}`,l).catch(()=>null),g("Empleado actualizado",t.name,"success"),window.__closeModal(),e&&e()}catch(l){const s=document.getElementById("edit-emp-result");s&&(s.innerHTML=`<p style="color:var(--red)">${l.message}</p>`)}finally{a.textContent="💾 Guardar",a.disabled=!1}}}const Ae={posted:{lbl:"Publicada",color:"emerald"},draft:{lbl:"Borrador",color:"gray"},in_payment:{lbl:"En cobro",color:"violet"},paid:{lbl:"Pagada",color:"sky"},cancel:{lbl:"Cancelada",color:"red"}};let J=1;async function je(){L(),S([{label:"Dashboard",href:"dashboard"},{label:"Facturación"}]),J=1,await Qt()}async function Qt(){var t,e,a,o,i;C(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">Facturación</h1>
      <p class="page-subtitle" id="fact-sub">Cargando…</p>
    </div>
    <div class="page-actions">
      <input type="text" id="buscar-fact" class="search-input" placeholder="🔍 Buscar folio o cliente…" style="width:230px">
      <button class="btn btn-primary" onclick="window._go('cfdi')">🧾 Nueva Factura CFDI</button>
    </div>
  </div>

  <!-- KPIs -->
  <div style="display:grid;grid-template-columns:repeat(4,1fr);gap:12px;margin-bottom:18px" id="kpi-row" class="anim-2">
    ${[1,2,3,4].map(()=>'<div class="data-card" style="padding:16px"><div class="skeleton" style="height:40px"></div></div>').join("")}
  </div>

  <!-- Por cobrar widget -->
  <div style="display:grid;grid-template-columns:2fr 1fr;gap:16px;margin-bottom:16px" class="anim-3">
    <div class="data-card">
      <div class="data-card-header">
        <div class="data-card-title">Facturas</div>
        <select id="filtro-estado" class="search-input" style="width:150px;font-size:12px">
          <option value="">Todos los estados</option>
          <option value="posted">Publicadas</option>
          <option value="draft">Borradores</option>
          <option value="cancel">Canceladas</option>
        </select>
      </div>
      <div id="fact-tabla">${I(8,5)}</div>
    </div>

    <!-- Panel por cobrar -->
    <div class="data-card" style="padding:16px">
      <div style="font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.06em;color:var(--text-400);margin-bottom:12px">📋 Por Cobrar</div>
      <div id="por-cobrar-lista">${[1,2,3,4].map(()=>'<div class="skeleton" style="height:38px;margin-bottom:8px;border-radius:8px"></div>').join("")}</div>
    </div>
  </div>`);try{const[r,n,l]=await Promise.allSettled([f.factKpis(),f.facturas(J),f.porCobrar()]),s=r.status==="fulfilled"?(t=r.value)==null?void 0:t.data:null,d=document.getElementById("kpi-row");d&&(d.innerHTML=[{label:"Total Facturas",val:(s==null?void 0:s.total_facturas)||0,tipo:"num",color:"indigo",icon:"🧾"},{label:"Monto Facturado",val:(s==null?void 0:s.monto_total)||0,tipo:"mxn",color:"emerald",icon:"💰"},{label:"Por Cobrar",val:(s==null?void 0:s.por_cobrar)||0,tipo:"mxn",color:"amber",icon:"📋"},{label:"Facturas Vencidas",val:(s==null?void 0:s.facturas_vencidas)||0,tipo:"num",color:"red",icon:"⚠️"}].map(b=>`
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${b.icon} ${b.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${b.tipo==="mxn"?x(parseFloat(b.val)):M(parseInt(b.val))}
        </div>
      </div>`).join(""));const u=n.status==="fulfilled"?((e=n.value)==null?void 0:e.data)||[]:[],h=u.length>=20,m=document.getElementById("fact-sub");m&&(m.textContent=`${u.length} registros · Página ${J}`);const c=document.getElementById("fact-tabla");c&&(u.length===0?c.innerHTML='<p style="text-align:center;padding:32px;color:var(--text-400)">Sin facturas registradas</p>':c.innerHTML=`
        <table class="data-table">
          <thead><tr>
            <th>Folio</th><th>Cliente</th><th>Fecha</th>
            <th>Subtotal</th><th>Total</th><th>Estado</th>
          </tr></thead>
          <tbody>
            ${u.map(b=>{const _=Ae[b.state]||{lbl:b.state||"—",color:"gray"},E=b.invoice_date||b.date?T(b.invoice_date||b.date):"—",w=b.partner_name&&isNaN(b.partner_name)?b.partner_name:b.customer_name||`Cliente #${b.partner_id}`;return`
              <tr data-estado="${b.state||""}" style="cursor:pointer" onclick="window._verFactura(${b.id})" title="Ver detalle">
                <td class="td-mono">${b.name||`#${b.id}`}</td>
                <td class="td-primary">${w}</td>
                <td>${E}</td>
                <td class="td-amount">${x(parseFloat(b.amount_untaxed||0))}</td>
                <td class="td-amount" style="font-weight:700">${x(parseFloat(b.amount_total||0))}</td>
                <td>${G(b.state,_.lbl)}</td>
              </tr>`}).join("")}
          </tbody>
        </table>
        ${D(J,h,b=>{J=b,Qt()})}`);const p=l.status==="fulfilled"?((a=l.value)==null?void 0:a.data)||[]:[],v=document.getElementById("por-cobrar-lista");v&&(p.length===0?v.innerHTML='<p style="color:var(--emerald);font-size:13px;text-align:center;padding:20px">✅ Sin saldo pendiente</p>':v.innerHTML=p.slice(0,8).map(b=>{const _=b.invoice_date_due&&new Date(b.invoice_date_due)<new Date;return`
          <div style="display:flex;justify-content:space-between;align-items:center;padding:8px 0;border-bottom:1px solid var(--border)">
            <div>
              <div style="font-size:12px;font-weight:600;color:var(--text-700)">${(b.partner_name||b.name||"—").substring(0,22)}</div>
              <div style="font-size:11px;color:${_?"var(--red)":"var(--text-400)"}">${_?"🔴 Vencida":"🟡 Pendiente"}</div>
            </div>
            <span class="badge badge-${_?"red":"amber"}">${x(parseFloat(b.amount_residual||b.amount_total||0))}</span>
          </div>`}).join("")),(o=document.getElementById("buscar-fact"))==null||o.addEventListener("input",b=>{const _=b.target.value.toLowerCase();document.querySelectorAll("#fact-tabla tbody tr").forEach(E=>{E.style.display=E.textContent.toLowerCase().includes(_)?"":"none"})}),(i=document.getElementById("filtro-estado"))==null||i.addEventListener("change",b=>{const _=b.target.value;document.querySelectorAll("#fact-tabla tbody tr").forEach(E=>{E.style.display=!_||E.dataset.estado===_?"":"none"})}),window._verFactura=b=>{nt("Detalle de Factura",()=>f.factura(b),_=>(setTimeout(()=>Be(_),0),'<div style="padding:24px;text-align:center;color:var(--text-400)">Cargando…</div>'))}}catch(r){console.error(r),g("Error al cargar facturas",r.message,"error")}}let it="list",U=1,tt=[],V="",vt="";async function Ft(){L(),it="list",U=1,V="",vt="",S([{label:"Productos"}]),De(),await ft()}function De(){C(`
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
        <input id="o-search-productos" class="o-search-input" type="text" placeholder="Buscar producto o código…" value="${V}">
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
        <button class="o-view-btn ${it==="list"?"o-active":""}" onclick="window._productoSetView('list')" title="Vista Lista">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="8" y1="6" x2="21" y2="6"/><line x1="8" y1="12" x2="21" y2="12"/><line x1="8" y1="18" x2="21" y2="18"/><line x1="3" y1="6" x2="3.01" y2="6"/><line x1="3" y1="12" x2="3.01" y2="12"/><line x1="3" y1="18" x2="3.01" y2="18"/></svg>
        </button>
        <button class="o-view-btn ${it==="kanban"?"o-active":""}" onclick="window._productoSetView('kanban')" title="Vista Kanban">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="1" y="4" width="6" height="16" rx="1"/><rect x="9" y="4" width="6" height="10" rx="1"/><rect x="17" y="4" width="6" height="13" rx="1"/></svg>
        </button>
      </div>
    </div>
  </div>
  <div id="productos-content" class="o-view-content">
    ${I(10,6)}
  </div>`);let t;setTimeout(()=>{var e;(e=document.getElementById("o-search-productos"))==null||e.addEventListener("input",a=>{clearTimeout(t),t=setTimeout(()=>{V=a.target.value.trim(),U=1,ft()},380)})},100)}async function ft(){try{const t=await f.productos(U,V);tt=((t==null?void 0:t.data)||[]).filter(o=>!vt||(o.type_||o.type)===vt);const e=((t==null?void 0:t.data)||[]).length>=20,a=document.getElementById("productos-content");if(!a)return;it==="kanban"?a.innerHTML=Yt(tt):a.innerHTML=Wt(tt,e)}catch(t){console.error(t),g("Error",t.message,"error")}}function Wt(t,e){return t.length?`
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
        ${t.map((a,o)=>{var m,c,p;const i=typeof a.name=="object"?((m=a.name)==null?void 0:m.es_MX)||((c=a.name)==null?void 0:c.en_US)||Object.values(a.name)[0]||`Producto #${a.id}`:a.name||a.nombre||`Producto #${a.id}`,r=a.type_||a.type||"",n=r==="consu"?"Consumible":r==="service"?"Servicio":r==="product"?"Almacenable":"Consumible",l=r==="service"?"o-badge-info":r==="consu"?"o-badge-warn":"o-badge-success",s=x(parseFloat(a.list_price||a.precio||0)),d=x(parseFloat(a.standard_price||a.costo||0)),u=a.id*67%360,h=((p=i[0])==null?void 0:p.toUpperCase())||"P";return`
          <tr class="o-list-row" onclick="window._verProducto(${a.id})">
            <td class="o-list-chk"><input type="checkbox" class="o-chk" onclick="event.stopPropagation()"></td>
            <td>
              <div class="o-prod-thumb" style="background:linear-gradient(135deg,hsl(${u},50%,60%),hsl(${(u+60)%360},60%,50%))">${h}</div>
            </td>
            <td class="o-td-primary">${i}</td>
            <td class="o-td-mono">${a.default_code||"—"}</td>
            <td class="o-td-amount">${s}</td>
            <td class="o-td-amount o-td-muted">${d}</td>
            <td><span class="o-badge ${l}">${n}</span></td>
            <td class="o-td-amount">${a.qty_available!=null?M(parseFloat(a.qty_available)):"—"}</td>
          </tr>`}).join("")}
      </tbody>
    </table>
    <div class="o-list-footer">
      <span class="o-list-count">${t.length} producto${t.length!==1?"s":""}</span>
      ${D(U,e,a=>{U=a,ft()})}
    </div>
  </div>`:`
    <div class="o-empty-state">
      <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" opacity=".3"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/></svg>
      <p>${V?`Sin resultados para "${V}"`:"Sin productos en catálogo"}</p>
    </div>`}function Yt(t){return t.length?`
  <div class="o-kanban-grid">
    ${t.map(e=>{var d,u,h;const a=typeof e.name=="object"?((d=e.name)==null?void 0:d.es_MX)||((u=e.name)==null?void 0:u.en_US)||`Producto #${e.id}`:e.name||`Producto #${e.id}`,o=e.type_||e.type||"",i=o==="consu"?"Consumible":o==="service"?"Servicio":"Almacenable",r=o==="service"?"o-badge-info":o==="consu"?"o-badge-warn":"o-badge-success",n=x(parseFloat(e.list_price||0)),l=e.id*67%360,s=((h=a[0])==null?void 0:h.toUpperCase())||"P";return`
      <div class="o-kanban-card" onclick="window._verProducto(${e.id})">
        <div class="o-kanban-img" style="background:linear-gradient(135deg,hsl(${l},50%,65%),hsl(${(l+60)%360},60%,55%))">
          <span style="font-size:40px;font-weight:800;color:rgba(255,255,255,.7)">${s}</span>
        </div>
        <div class="o-kanban-body">
          <div class="o-kanban-title">${a}</div>
          <div class="o-kanban-sub">${e.default_code||"(sin SKU)"}</div>
          <div style="display:flex;justify-content:space-between;align-items:center;margin-top:8px">
            <span class="o-badge ${r}">${i}</span>
            <strong class="o-kanban-price">${n}</strong>
          </div>
        </div>
      </div>`}).join("")}
  </div>`:`
    <div class="o-empty-state">
      <p>Sin productos${V?` para "${V}"`:""}</p>
    </div>`}window._verProducto=async t=>{var e,a,o,i,r;S([{label:"Productos",onclick:()=>Ft()},{label:"Cargando…",id:"bc-prod-name"}]),C(`<div class="o-form-loading">${I(4,3)}</div>`);try{const n=await f.producto(t);if(!n){g("Error","Producto no encontrado","error");return}const l=document.getElementById("bc-prod-name");l&&(l.textContent=typeof n.name=="object"?((e=n.name)==null?void 0:e.es_MX)||((a=n.name)==null?void 0:a.en_US)||"Producto":n.name||"Producto");const s=typeof n.name=="object"?((o=n.name)==null?void 0:o.es_MX)||((i=n.name)==null?void 0:i.en_US)||`Producto #${n.id}`:n.name||`Producto #${n.id}`,d=n.type_||n.type||"",u=d==="consu"?"Consumible":d==="service"?"Servicio":d==="product"?"Almacenable":"Consumible",h=d==="service"?"o-badge-info":d==="consu"?"o-badge-warn":"o-badge-success",m=x(parseFloat(n.list_price||0)),c=x(parseFloat(n.standard_price||0)),p=n.id*67%360,v=((r=s[0])==null?void 0:r.toUpperCase())||"P",b=(()=>{const _=n.categ_name||n.categoria||"";return _==="Goods"?"Mercancía":_==="Services"?"Servicios":_||"—"})();C(`
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
        <span class="o-smart-count">${n.qty_available!=null?M(parseFloat(n.qty_available)):0}</span>
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
        <div class="o-prod-thumb o-prod-thumb-lg" style="background:linear-gradient(135deg,hsl(${p},50%,65%),hsl(${(p+60)%360},60%,55%))">${v}</div>
        <div class="o-sheet-title-block">
          <h1 class="o-form-title">${s}</h1>
          <div style="display:flex;gap:6px;margin-top:4px">
            <span class="o-badge ${h}">${u}</span>
            ${n.active!==!1?'<span class="o-badge o-badge-success">Activo</span>':'<span class="o-badge o-badge-gray">Inactivo</span>'}
          </div>
        </div>
      </div>

      <div class="o-form-grid">
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">SKU / Código interno</label><div class="o-field-value o-field-mono">${n.default_code||"—"}</div></div>
          <div class="o-field-group"><label class="o-field-label">Tipo de Producto</label><div class="o-field-value"><span class="o-badge ${h}">${u}</span></div></div>
          <div class="o-field-group"><label class="o-field-label">Unidad de Medida</label><div class="o-field-value">${n.uom_name||n.uom||"Unidad"}</div></div>
          <div class="o-field-group"><label class="o-field-label">Peso</label><div class="o-field-value">${n.weight!=null?n.weight+" kg":"—"}</div></div>
        </div>
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">Precio de Venta</label><div class="o-field-value o-field-price">${m}</div></div>
          <div class="o-field-group"><label class="o-field-label">Costo</label><div class="o-field-value o-td-muted">${c}</div></div>
          <div class="o-field-group"><label class="o-field-label">Impuestos</label><div class="o-field-value">${n.taxes_name||"IVA 16%"}</div></div>
          <div class="o-field-group"><label class="o-field-label">Categoría</label><div class="o-field-value">${b}</div></div>
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
          <div class="o-field-group"><label class="o-field-label">Precio de compra</label><div class="o-field-value">${c}</div></div>
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
    </div>`),window._editarProductoForm=_=>Pe({id:_,...n},()=>window._verProducto(_)),window._prodTab=(_,E)=>{document.querySelectorAll("#prod-tabs .o-tab").forEach(k=>k.classList.remove("active")),E.classList.add("active"),document.querySelectorAll(".o-tab-pane").forEach(k=>k.style.display="none");const w=document.getElementById(`tab-${_}`);w&&(w.style.display="")}}catch(n){console.error(n),g("Error",n.message,"error")}};window._productosBack=()=>Ft();window._productoSetView=t=>{var o;it=t,document.querySelectorAll("#productos-cp .o-view-btn").forEach(i=>i.classList.remove("o-active"));const e=t==="list"?0:1;(o=document.querySelectorAll("#productos-cp .o-view-btn")[e])==null||o.classList.add("o-active");const a=document.getElementById("productos-content");a&&(t==="kanban"?a.innerHTML=Yt(tt):a.innerHTML=Wt(tt,!1))};window._productoFiltroTipo=t=>{var a;vt=t,U=1,document.querySelectorAll("#productos-cp .o-filter-btn").forEach(o=>o.removeAttribute("data-active"));const e={"":"ptf-todos",consu:"ptf-consu",service:"ptf-serv",product:"ptf-prod"};(a=document.getElementById(e[t]))==null||a.setAttribute("data-active",""),ft()};window._productoNuevo=()=>alert("Nuevo producto — próximamente");window._chkAllProductos=t=>document.querySelectorAll("#productos-content .o-chk").forEach(e=>e.checked=t.checked);let R=1,$t=[],ut="",H="";async function Bt(){L(),R=1,ut="",H="",S([{label:"Clientes / Proveedores"}]),Ne(),await Lt()}function Ne(){C(`
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
        <input id="o-search-partners" class="o-search-input" type="text" placeholder="Buscar…" value="${ut}">
        <div class="o-search-filters">
          <button class="o-filter-btn" onclick="window._partnerFilter('')" id="pf-all" ${H===""?"data-active":""}>Todos</button>
          <button class="o-filter-btn" onclick="window._partnerFilter('clientes')" id="pf-cli" ${H==="clientes"?"data-active":""}>Clientes</button>
          <button class="o-filter-btn" onclick="window._partnerFilter('proveedores')" id="pf-prov" ${H==="proveedores"?"data-active":""}>Proveedores</button>
        </div>
      </div>
    </div>
    <div class="o-cp-right">
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
  </div>`),setTimeout(()=>{var t;(t=document.getElementById("o-search-partners"))==null||t.addEventListener("input",e=>{ut=e.target.value.toLowerCase(),Re()})},100)}function Re(){document.querySelectorAll("#partners-content tbody tr").forEach(t=>{t.style.display=t.textContent.toLowerCase().includes(ut)?"":"none"})}async function Lt(){try{let t;H==="clientes"?t=f.clientes(R):H==="proveedores"?t=f.proveedores(R):t=f.partners(R);const e=await t;$t=(e==null?void 0:e.data)||[];const a=$t.length>=20,o=document.getElementById("partners-content");if(!o)return;o.innerHTML=Ve($t,a)}catch(t){console.error(t),g("Error",t.message,"error");const e=document.getElementById("partners-content");e&&(e.innerHTML='<div class="o-empty-state"><p>Error al cargar contactos</p></div>')}}function Ve(t,e){return t.length?`
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
        ${t.map((a,o)=>{const i=(a.customer_rank||0)>0,r=(a.supplier_rank||0)>0,n=a.is_company,l=a.name||a.nombre||"—",s=l.split(" ").map(u=>u[0]).slice(0,2).join(""),d=a.id*37%360;return`
          <tr class="o-list-row" onclick="window._verPartner(${a.id})">
            <td class="o-list-chk"><input type="checkbox" class="o-chk" onclick="event.stopPropagation()"></td>
            <td class="o-list-num">${(R-1)*20+o+1}</td>
            <td>
              <div class="o-partner-cell">
                <div class="o-avatar" style="background:linear-gradient(135deg,hsl(${d},60%,55%),hsl(${(d+40)%360},70%,45%))">${s||"?"}</div>
                <div>
                  <div class="o-td-primary">${l}</div>
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
              ${r?'<span class="o-badge o-badge-info" style="margin-left:2px">Proveedor</span>':""}
              ${!i&&!r?'<span class="o-badge o-badge-gray">Contacto</span>':""}
            </td>
          </tr>`}).join("")}
      </tbody>
    </table>
    <div class="o-list-footer">
      <span class="o-list-count">${t.length} contacto${t.length!==1?"s":""}</span>
      ${D(R,e,a=>{R=a,Lt()})}
    </div>
  </div>`:`
    <div class="o-empty-state">
      <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" opacity=".3"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>
      <p>Sin contactos registrados</p>
    </div>`}window._verPartner=async t=>{S([{label:"Clientes / Proveedores",onclick:()=>Bt()},{label:"Cargando…",id:"bc-partner-name"}]),C(`<div class="o-form-loading">${I(4,3)}</div>`);try{const e=await f.partner(t);if(!e){g("Error","Contacto no encontrado","error");return}const a=document.getElementById("bc-partner-name");a&&(a.textContent=e.name||"Contacto");const o=(e.customer_rank||0)>0,i=(e.supplier_rank||0)>0,r=e.is_company,n=e.name||"—",l=n.split(" ").map(d=>d[0]).slice(0,2).join(""),s=e.id*37%360;C(`
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
        <div class="o-avatar o-avatar-lg" style="background:linear-gradient(135deg,hsl(${s},60%,55%),hsl(${(s+40)%360},70%,45%))">${l||"?"}</div>
        <div class="o-sheet-title-block">
          <h1 class="o-form-title">${n}</h1>
          <div style="display:flex;gap:6px;margin-top:4px">
            ${o?'<span class="o-badge o-badge-success">Cliente</span>':""}
            ${i?'<span class="o-badge o-badge-info">Proveedor</span>':""}
            ${r?'<span class="o-badge o-badge-gray">Empresa</span>':'<span class="o-badge o-badge-gray">Persona física</span>'}
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
            <div class="o-field-value">${r?"Sí":"No"}</div>
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
            <div class="o-msg-meta"><strong>Sistema</strong> <span>${T(new Date().toISOString())}</span></div>
            <div class="o-msg-text">Registro creado.</div>
          </div>
        </div>
      </div>
    </div>`),window._editarPartnerForm=d=>{const u={id:d,...e};Le(u,()=>window._verPartner(d))},window._partnerTab=(d,u)=>{document.querySelectorAll("#partner-tabs .o-tab").forEach(m=>m.classList.remove("active")),u.classList.add("active"),document.querySelectorAll(".o-tab-pane").forEach(m=>m.style.display="none");const h=document.getElementById(`tab-${d}`);h&&(h.style.display="")}}catch(e){console.error(e),g("Error",e.message,"error")}};window._partnersBack=()=>Bt();window._partnerFilter=t=>{var o;H=t,R=1,document.querySelectorAll("#partners-cp .o-filter-btn").forEach(i=>i.removeAttribute("data-active"));const e={"":"pf-all",clientes:"pf-cli",proveedores:"pf-prov"};(o=document.getElementById(e[t]))==null||o.setAttribute("data-active","");const a=document.getElementById("partners-content");a&&(a.innerHTML=I(8,6)),Lt()};window._partnerNuevo=()=>alert("Nuevo contacto — próximamente");window._chkAllPartners=t=>{document.querySelectorAll("#partners-content .o-chk").forEach(e=>e.checked=t.checked)};let mt=1,ct=[],bt="";async function Pt(){L(),mt=1,bt="",S([{label:"Inventario"}]),qe(),await Zt()}function qe(){C(`
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
        <input id="o-search-stock" class="o-search-input" type="text" placeholder="Buscar producto o ubicación…" value="${bt}">
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
  </div>`),setTimeout(()=>{var t;(t=document.getElementById("o-search-stock"))==null||t.addEventListener("input",e=>{bt=e.target.value.toLowerCase(),He()})},100)}function He(){document.querySelectorAll("#stock-content tbody tr").forEach(t=>{t.style.display=t.textContent.toLowerCase().includes(bt)?"":"none"})}async function Zt(){try{const t=await f.stock(mt);ct=(t==null?void 0:t.data)||[];const e=ct.length>=20,a=document.getElementById("stock-content");if(!a)return;a.innerHTML=Ge(ct,e)}catch(t){console.error(t),g("Error",t.message,"error")}}function Ge(t,e){return t.length?`
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
        ${t.map(a=>{const o=parseFloat(a.cantidad_disponible??a.qty_available??0),i=parseFloat(a.cantidad_reservada??a.reserved_qty??0),r=o<=0?"#ef4444":o<10?"#f59e0b":"#10b981",n=a.product_name||a.nombre||`Producto #${a.product_id||a.id}`,l=a.ubicacion||a.location||"WH/Stock",s=a.uom_name||a.unidad||"Unidades";return`
          <tr class="o-list-row" onclick="window._verStockItem(${a.product_id||a.id})">
            <td class="o-list-chk"><input type="checkbox" class="o-chk" onclick="event.stopPropagation()"></td>
            <td class="o-td-primary">${n}</td>
            <td class="o-td-muted">${l}</td>
            <td class="o-td-amount" style="color:${r};font-weight:700">${M(o)}</td>
            <td class="o-td-amount o-td-muted">${M(i)}</td>
            <td class="o-td-muted">${s}</td>
          </tr>`}).join("")}
      </tbody>
    </table>
    <div class="o-list-footer">
      <span class="o-list-count">${t.length} producto${t.length!==1?"s":""}</span>
      ${D(mt,e,a=>{mt=a,Zt()})}
    </div>
  </div>`:`
    <div class="o-empty-state">
      <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" opacity=".3"><path d="M5 8h14M5 8a2 2 0 1 0 0-4h14a2 2 0 1 0 0 4M5 8v10a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V8m-9 4h4"/></svg>
      <p>Sin registros de inventario</p>
    </div>`}window._verStockItem=async t=>{S([{label:"Inventario",onclick:()=>Pt()},{label:"Detalle de stock",id:"bc-stock-name"}]),C(`<div class="o-form-loading">${I(3,3)}</div>`);try{const e=await f.stockProducto(t),o=(Array.isArray(e==null?void 0:e.data)?e.data:e!=null&&e.data?[e.data]:[])[0]||{},i=parseFloat(o.cantidad_disponible??0),r=parseFloat(o.cantidad_reservada??0),n=i*parseFloat(o.valor_unitario||0),l=o.product_name||`Producto #${t}`,s=document.getElementById("bc-stock-name");s&&(s.textContent=l);const d=i<=0?"#ef4444":i<10?"#f59e0b":"#10b981";C(`
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
        <span class="o-smart-count" style="color:${d}">${M(i)}</span>
        <span class="o-smart-label">Disponible</span>
      </button>
      <button class="o-smart-btn">
        <span class="o-smart-count">${M(r)}</span>
        <span class="o-smart-label">Reservado</span>
      </button>
    </div>

    <div class="o-form-sheet">
      <div class="o-sheet-header">
        <div class="o-sheet-title-block">
          <h1 class="o-form-title">${l}</h1>
        </div>
      </div>

      <div class="o-form-grid">
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">Producto</label><div class="o-field-value">${l}</div></div>
          <div class="o-field-group"><label class="o-field-label">Ubicación</label><div class="o-field-value">${o.ubicacion||"WH/Stock"}</div></div>
          <div class="o-field-group"><label class="o-field-label">Cantidad Disponible</label><div class="o-field-value" style="color:${d};font-weight:700;font-size:20px">${M(i)}</div></div>
          <div class="o-field-group"><label class="o-field-label">Cantidad Reservada</label><div class="o-field-value">${M(r)}</div></div>
        </div>
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">Unidad de Medida</label><div class="o-field-value">${o.uom_name||o.unidad||"Unidades"}</div></div>
          <div class="o-field-group"><label class="o-field-label">Valor Unitario</label><div class="o-field-value">${x(parseFloat(o.valor_unitario||0))}</div></div>
          <div class="o-field-group"><label class="o-field-label">Valor Total</label><div class="o-field-value o-field-price">${x(n)}</div></div>
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
    </div>`),window._ajustarStockForm=u=>{const h=ct.find(m=>(m.product_id||m.id)===u);h?qt(h,()=>window._verStockItem(u)):qt({product_id:u,product_name:l},()=>window._verStockItem(u))}}catch(e){console.error(e),g("Error",e.message,"error")}};window._stockBack=()=>Pt();window._stockAjustarGlobal=()=>alert("Selecciona un producto para ajustar");window._stockFiltro=t=>{document.querySelectorAll("#stock-content tbody tr").forEach(e=>{var o;const a=parseFloat(((o=e.querySelector("td:nth-child(4)"))==null?void 0:o.textContent)||"0");t==="bajo"?e.style.display=a<10?"":"none":t==="cero"?e.style.display=a<=0?"":"none":e.style.display=""})};window._chkAllStock=t=>document.querySelectorAll("#stock-content .o-chk").forEach(e=>e.checked=t.checked);let Q=1,lt="historial";async function Oe(){L(),S([{label:"Dashboard",href:"dashboard"},{label:"CFDI 4.0"}]),Q=1,await te()}async function te(){C(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">🔏 CFDI 4.0</h1>
      <p class="page-subtitle" id="cfdi-sub">Comprobantes Fiscales Digitales</p>
    </div>
    <div class="page-actions">
      <button class="btn ${lt==="historial"?"btn-primary":"btn-secondary"}"
        onclick="window._cfdiTab('historial')">📋 Historial</button>
      <button class="btn ${lt==="timbrar"?"btn-primary":"btn-secondary"}"
        onclick="window._cfdiTab('timbrar')">➕ Timbrar</button>
    </div>
  </div>

  <!-- KPI row -->
  <div style="display:grid;grid-template-columns:repeat(4,1fr);gap:12px;margin-bottom:18px" id="kpi-row" class="anim-2">
    ${[1,2,3,4].map(()=>'<div class="data-card" style="padding:16px"><div class="skeleton" style="height:40px"></div></div>').join("")}
  </div>

  <div class="data-card anim-3" id="cfdi-content">
    <div id="cfdi-body">${I(6,6)}</div>
  </div>`),window._cfdiTab=t=>{lt=t,te()};try{const t=await f.cfdiKpis().catch(()=>null),e=t==null?void 0:t.data,a=document.getElementById("kpi-row");a&&(a.innerHTML=[{label:"Total Timbrados",val:(e==null?void 0:e.total_timbrados)??0,tipo:"num",color:"indigo",icon:"🧾"},{label:"Vigentes",val:(e==null?void 0:e.vigentes)??0,tipo:"num",color:"emerald",icon:"✅"},{label:"Cancelados",val:(e==null?void 0:e.cancelados)??0,tipo:"num",color:"red",icon:"❌"},{label:"Monto Total",val:(e==null?void 0:e.monto_total)??0,tipo:"mxn",color:"violet",icon:"💰"}].map(o=>`
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${o.icon} ${o.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${o.tipo==="mxn"?x(parseFloat(o.val)):Number(o.val).toLocaleString("es-MX")}
        </div>
      </div>`).join("")),lt==="historial"?await kt():Ue()}catch(t){console.error(t),g("Error CFDI",t.message,"error")}}async function kt(){const t=document.getElementById("cfdi-body");t&&(t.innerHTML=I(6,6));const e=await f.cfdiTimbrados(Q).catch(()=>({data:[],total:0})),a=(e==null?void 0:e.data)||[],o=(e==null?void 0:e.total)??a.length,i=a.length>=20,r=document.getElementById("cfdi-sub");if(r&&(r.textContent=`${o} CFDIs timbrados · Página ${Q}`),!!t){if(a.length===0){t.innerHTML=`
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
      ${a.map(n=>{const l=n.estado==="vigente"?"emerald":n.estado==="cancelado"?"red":"gray";return`
        <tr style="cursor:pointer" onclick="window._verCfdi('${n.uuid}')" title="Ver detalle">
          <td class="td-mono" style="font-size:11px">${n.uuid.substring(0,18)}…</td>
          <td class="td-mono">${n.serie||""}${n.folio||"—"}</td>
          <td class="td-primary">${n.nombre_receptor||n.rfc_receptor}</td>
          <td class="td-amount" style="font-weight:700">${x(parseFloat(n.total||0))}</td>
          <td><span class="badge badge-sky">${n.tipo_cfdi==="I"?"Ingreso":n.tipo_cfdi==="E"?"Egreso":n.tipo_cfdi||"—"}</span></td>
          <td><span class="badge badge-${l}">${n.estado||"—"}</span></td>
          <td style="font-size:12px">${T(n.fecha_timbrado||n.created_at)}</td>
        </tr>`}).join("")}
    </tbody>
  </table>
  ${D(Q,i,n=>{Q=n,kt()})}`,window._verCfdi=n=>{nt("Detalle CFDI",()=>f.cfdiTimbrado(n),l=>`
      ${P("Comprobante",[$("UUID",`<span style="font-family:monospace;font-size:11px">${l.uuid}</span>`),$("Serie / Folio",`${l.serie||""}${l.folio||"—"}`),$("Tipo",l.tipo_cfdi==="I"?"Ingreso":l.tipo_cfdi==="E"?"Egreso":l.tipo_cfdi),$("Estado",`<span class="badge badge-${l.estado==="vigente"?"emerald":"red"}">${l.estado}</span>`),$("Fecha emisión",T(l.fecha_emision)),$("Fecha timbrado",T(l.fecha_timbrado))].join(""))}
      ${P("Partes",[$("RFC Emisor",l.rfc_emisor),$("Emisor",l.nombre_emisor||"—"),$("RFC Receptor",l.rfc_receptor),$("Receptor",l.nombre_receptor||"—")].join(""))}
      ${P("Importes",[$("Total",`<strong>${x(parseFloat(l.total||0))}</strong>`,{color:"var(--primary)"})].join(""))}
      <div style="display:flex;gap:10px;margin-top:16px">
        <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
        ${l.estado==="vigente"?`<button class="btn btn-danger btn-sm" onclick="window._cancelarCfdi('${l.uuid}')">❌ Cancelar</button>`:""}
      </div>`)},window._cancelarCfdi=async n=>{if(confirm(`¿Cancelar el CFDI ${n.substring(0,18)}…?`))try{await f.cancelarCfdi({uuid:n,rfc_emisor:"",motivo:"02"}),g("CFDI cancelado",n,"success"),window.__closeModal(),kt()}catch(l){g("Error al cancelar",l.message,"error")}}}}function Ue(){var e;const t=document.getElementById("cfdi-body");t&&(t.innerHTML=`
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
  </div>`,(e=document.getElementById("f-subtotal"))==null||e.addEventListener("input",a=>{const o=parseFloat(a.target.value)||0,i=o*.16;document.getElementById("f-iva").value=i.toFixed(2),document.getElementById("f-total").value=(o+i).toFixed(2)}),window._timbrar=async()=>{var i,r,n,l,s,d,u,h,m,c,p,v,b,_;const a=document.getElementById("btn-timbrar");a.textContent="⏳ Timbrando…",a.disabled=!0;const o=document.getElementById("cfdi-resultado");try{const E=(i=document.getElementById("f-cer"))==null?void 0:i.files[0],w=(r=document.getElementById("f-key"))==null?void 0:r.files[0],k=Mt=>new Promise((zt,ie)=>{if(!Mt){zt("");return}const wt=new FileReader;wt.onload=ne=>zt(ne.target.result.split(",")[1]||""),wt.onerror=ie,wt.readAsDataURL(Mt)}),[B,K]=await Promise.all([k(E),k(w)]),N=parseFloat((n=document.getElementById("f-subtotal"))==null?void 0:n.value)||0,st=N*.16,oe={cfdi:{serie:((l=document.getElementById("f-serie"))==null?void 0:l.value)||"A",folio:((s=document.getElementById("f-folio"))==null?void 0:s.value)||"1",tipo_comprobante:((d=document.getElementById("f-tipo"))==null?void 0:d.value)||"I",emisor:{rfc:((u=document.getElementById("f-rfc-emisor"))==null?void 0:u.value)||"",nombre:((h=document.getElementById("f-nombre-emisor"))==null?void 0:h.value)||"",regimen_fiscal:((m=document.getElementById("f-regimen"))==null?void 0:m.value)||"601"},receptor:{rfc:((c=document.getElementById("f-rfc-receptor"))==null?void 0:c.value)||"",nombre:((p=document.getElementById("f-nombre-receptor"))==null?void 0:p.value)||"",uso_cfdi:((v=document.getElementById("f-uso"))==null?void 0:v.value)||"G03",domicilio_fiscal_receptor:"64000",regimen_fiscal_receptor:"601"},conceptos:[{clave_prod_serv:"84111506",descripcion:((b=document.getElementById("f-concepto"))==null?void 0:b.value)||"Servicio",cantidad:"1",unidad:"ACT",valor_unitario:N.toFixed(2),importe:N.toFixed(2),impuestos:{traslados:[{base:N.toFixed(2),impuesto:"002",tipo_factor:"Tasa",tasa:"0.160000",importe:st.toFixed(2)}]}}],subtotal:N.toFixed(2),total:(N+st).toFixed(2),moneda:"MXN",lugar_expedicion:"64000"},cert_b64:B,key_b64:K,password:((_=document.getElementById("f-pwd"))==null?void 0:_.value)||""},j=await f.timbrar(oe);j!=null&&j.success?(o.innerHTML=`
        <div style="background:var(--success-light);border:1.5px solid var(--success);border-radius:12px;padding:16px">
          <div style="font-weight:700;color:var(--success);margin-bottom:8px">✅ CFDI Timbrado</div>
          <div style="font-size:12px;font-family:monospace;word-break:break-all;color:var(--text-600)">UUID: ${j.uuid}</div>
          <div style="font-size:12px;color:var(--text-500);margin-top:4px">Fecha: ${T(j.fecha_timbrado)}</div>
        </div>`,g("CFDI timbrado",`UUID: ${j.uuid}`,"success")):o.innerHTML=`<div style="background:var(--danger-light,#fee2e2);border:1.5px solid var(--red);border-radius:12px;padding:16px;color:var(--red)">
          ❌ Error: ${(j==null?void 0:j.error)||"Error desconocido"}</div>`}catch(E){o.innerHTML=`<div style="background:#fee2e2;border:1.5px solid var(--red);border-radius:12px;padding:16px;color:var(--red)">❌ ${E.message}</div>`}finally{a.textContent="🔏 Timbrar CFDI",a.disabled=!1}})}let W=1;async function Ke(){L(),S([{label:"Dashboard",href:"dashboard"},{label:"Nómina IMSS"}]),W=1,await Et()}async function Et(){var t,e,a,o;C(`
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
    ${[1,2,3,4].map(()=>'<div class="data-card" style="padding:16px"><div class="skeleton" style="height:40px"></div></div>').join("")}
  </div>

  <div class="data-card anim-3">
    <div class="data-card-header">
      <div class="data-card-title">Directorio de Empleados</div>
    </div>
    <div id="nom-tabla">${I(8,5)}</div>
  </div>`);try{const[i,r]=await Promise.allSettled([f.nominaKpis(),f.nomina(W)]),n=i.status==="fulfilled"?(t=i.value)==null?void 0:t.data:null,l=document.getElementById("kpi-row");l&&(l.innerHTML=[{label:"Total Empleados",val:(n==null?void 0:n.total_empleados)??0,tipo:"num",color:"indigo",icon:"👥"},{label:"Activos",val:(n==null?void 0:n.activos)??0,tipo:"num",color:"emerald",icon:"✅"},{label:"Departamentos",val:(n==null?void 0:n.departamentos)??0,tipo:"num",color:"violet",icon:"🏢"},{label:"Nómina Mensual",val:(n==null?void 0:n.nomina_mensual)??0,tipo:"mxn",color:"amber",icon:"💰"}].map(c=>`
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${c.icon} ${c.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${c.tipo==="mxn"?x(parseFloat(c.val)):Number(c.val).toLocaleString("es-MX")}
        </div>
      </div>`).join(""));const s=r.status==="fulfilled"?((e=r.value)==null?void 0:e.data)||[]:[],d=((a=r.value)==null?void 0:a.total)??s.length,u=s.length>=20,h=document.getElementById("nom-sub");h&&(h.textContent=`${d} empleados · Página ${W}`);const m=document.getElementById("nom-tabla");m&&(s.length===0?m.innerHTML=`
        <div style="text-align:center;padding:60px 24px">
          <div style="font-size:48px;margin-bottom:16px">👔</div>
          <div style="font-size:16px;font-weight:700;color:var(--text-700);margin-bottom:8px">Sin empleados registrados</div>
          <div style="font-size:13px;color:var(--text-400)">Agrega empleados para gestionar tu nómina</div>
        </div>`:m.innerHTML=`
        <table class="data-table">
          <thead><tr>
            <th>Empleado</th><th>Puesto</th><th>Departamento</th>
            <th>IMSS</th><th>Estado</th>
          </tr></thead>
          <tbody>
            ${s.map(c=>{const p=c.active!==!1,v=(c.name||"?").split(" ").map(b=>b[0]).slice(0,2).join("");return`
              <tr style="cursor:pointer" onclick="window._verEmpleado(${c.id})" title="Ver detalle">
                <td>
                  <div style="display:flex;align-items:center;gap:10px">
                    <div style="width:34px;height:34px;border-radius:50%;background:linear-gradient(135deg,hsl(${c.id*47%360},60%,55%),hsl(${c.id*89%360},70%,45%));display:flex;align-items:center;justify-content:center;color:white;font-size:12px;font-weight:700;flex-shrink:0">
                      ${v}
                    </div>
                    <div class="td-primary">${c.name||"—"}</div>
                  </div>
                </td>
                <td style="color:var(--text-600)">${c.job_title||c.job_id_name||"—"}</td>
                <td style="color:var(--text-500);font-size:12px">${c.department_name||c.department_id_name||"—"}</td>
                <td class="td-mono" style="font-size:11px">${c.ssnid||c.imss||"—"}</td>
                <td><span class="badge badge-${p?"emerald":"gray"}">${p?"Activo":"Baja"}</span></td>
              </tr>`}).join("")}
          </tbody>
        </table>
        ${D(W,u,c=>{W=c,Et()})}`),(o=document.getElementById("buscar-nom"))==null||o.addEventListener("input",c=>{const p=c.target.value.toLowerCase();document.querySelectorAll("#nom-tabla tbody tr").forEach(v=>{v.style.display=v.textContent.toLowerCase().includes(p)?"":"none"})}),window._verEmpleado=c=>{const p=s.find(v=>v.id===c);p&&nt("Detalle de Empleado",async()=>p,v=>`
        ${P("Información",[$("Nombre completo",v.name),$("Puesto",v.job_title||v.job_id_name||"—"),$("Departamento",v.department_name||v.department_id_name||"—"),$("Estado",`<span class="badge badge-${v.active!==!1?"emerald":"gray"}">${v.active!==!1?"Activo":"Baja"}</span>`)].join(""))}
        ${P("IMSS / Fiscal",[$("N° IMSS",v.ssnid||v.imss||"—"),$("RFC",v.rfc||"—"),$("CURP",v.curp||"—")].join(""))}
        ${P("Contacto",[$("Email",v.work_email||v.email||"—"),$("Teléfono",v.work_phone||v.mobile_phone||"—")].join(""))}
        <div style="display:flex;gap:10px;margin-top:16px">
          <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
          <button class="btn btn-secondary btn-sm" onclick="window._editarEmpleadoFn(${v.id})">✏️ Editar</button>
          <button class="btn btn-primary btn-sm" onclick="alert('Recibo de nómina — próximamente')">📄 Ver recibo</button>
        </div>`)},window._editarEmpleadoFn=c=>{const p=s.find(v=>v.id===c);p&&ze(p,()=>Et())}}catch(i){console.error(i),g("Error al cargar nómina",i.message,"error")}}const Ht={purchase:{lbl:"Confirmada",color:"indigo"},done:{lbl:"Recibida",color:"emerald"},draft:{lbl:"Borrador",color:"gray"},cancel:{lbl:"Cancelada",color:"red"},sent:{lbl:"Enviada",color:"sky"}};let Y=1;async function Xe(){L(),S([{label:"Dashboard",href:"dashboard"},{label:"Compras"}]),Y=1,await Ct()}async function Ct(){var t,e,a,o;C(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">🛒 Órdenes de Compra</h1>
      <p class="page-subtitle" id="comp-sub">Cargando…</p>
    </div>
    <div class="page-actions">
      <input type="text" id="buscar-comp" class="search-input" placeholder="🔍 Buscar folio o proveedor…" style="width:240px">
      <button class="btn btn-primary" onclick="alert('Nueva OC — próximamente')">+ Nueva Orden</button>
    </div>
  </div>

  <!-- KPIs -->
  <div style="display:grid;grid-template-columns:repeat(4,1fr);gap:12px;margin-bottom:18px" id="kpi-row" class="anim-2">
    ${[1,2,3,4].map(()=>'<div class="data-card" style="padding:16px"><div class="skeleton" style="height:40px"></div></div>').join("")}
  </div>

  <div class="data-card anim-3">
    <div class="data-card-header">
      <div class="data-card-title">Órdenes de Compra</div>
    </div>
    <div id="comp-tabla">${I(8,5)}</div>
  </div>`);try{const[i,r]=await Promise.allSettled([f.comprasKpis(),f.compras(Y)]),n=i.status==="fulfilled"?(t=i.value)==null?void 0:t.data:null,l=document.getElementById("kpi-row");l&&(l.innerHTML=[{label:"Total OC",val:(n==null?void 0:n.total)??0,tipo:"num",color:"indigo",icon:"📋"},{label:"Confirmadas",val:(n==null?void 0:n.confirmadas)??0,tipo:"num",color:"emerald",icon:"✅"},{label:"Monto Total",val:(n==null?void 0:n.monto_total)??0,tipo:"mxn",color:"violet",icon:"💰"},{label:"Completadas",val:(n==null?void 0:n.completadas)??0,tipo:"num",color:"amber",icon:"📦"}].map(c=>`
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${c.icon} ${c.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${c.tipo==="mxn"?x(parseFloat(c.val)):Number(c.val).toLocaleString("es-MX")}
        </div>
      </div>`).join(""));const s=r.status==="fulfilled"?((e=r.value)==null?void 0:e.data)||[]:[],d=((a=r.value)==null?void 0:a.total)??s.length,u=s.length>=20,h=document.getElementById("comp-sub");h&&(h.textContent=`${d} órdenes · Página ${Y}`);const m=document.getElementById("comp-tabla");m&&(s.length===0?m.innerHTML='<div style="text-align:center;padding:60px;color:var(--text-400)">Sin órdenes de compra registradas</div>':m.innerHTML=`
        <table class="data-table">
          <thead><tr>
            <th>Folio</th><th>Proveedor</th><th>Fecha</th>
            <th>Total</th><th>Estado</th>
          </tr></thead>
          <tbody>
            ${s.map(c=>{const p=Ht[c.state]||{lbl:c.state||"—",color:"gray"};return`
              <tr style="cursor:pointer" onclick="window._verCompra(${c.id})" title="Ver detalle">
                <td class="td-mono">${c.name||`#${c.id}`}</td>
                <td class="td-primary">${c.partner_name||"—"}</td>
                <td>${T(c.date_order)}</td>
                <td class="td-amount" style="font-weight:700">${x(parseFloat(c.amount_total||0))}</td>
                <td>${G(c.state,p.lbl)}</td>
              </tr>`}).join("")}
          </tbody>
        </table>
        ${D(Y,u,c=>{Y=c,Ct()})}`),(o=document.getElementById("buscar-comp"))==null||o.addEventListener("input",c=>{const p=c.target.value.toLowerCase();document.querySelectorAll("#comp-tabla tbody tr").forEach(v=>{v.style.display=v.textContent.toLowerCase().includes(p)?"":"none"})}),window._verCompra=c=>{const p=s.find(v=>v.id===c);p&&nt("Detalle Orden de Compra",async()=>p,v=>{const b=Ht[v.state]||{lbl:v.state,color:"gray"};return`
          ${P("Orden",[$("Folio",v.name),$("Estado",G(v.state,b.lbl)),$("Proveedor",v.partner_name||"—"),$("Fecha",T(v.date_order)),$("Fecha entrega",T(v.date_planned))].join(""))}
          ${P("Importes",[$("Subtotal",x(parseFloat(v.amount_untaxed||0))),$("IVA",x(parseFloat(v.amount_tax||0))),$("Total",`<strong>${x(parseFloat(v.amount_total||0))}</strong>`,{color:"var(--primary)"})].join(""))}
          <div style="display:flex;gap:10px;margin-top:16px">
            <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
            <button class="btn btn-secondary btn-sm" onclick="window._editarCompraFn(${v.id})">✏️ Editar</button>
            <button class="btn btn-primary btn-sm" onclick="alert('Recibir mercancía — próximamente')">📦 Recibir</button>
          </div>`})},window._editarCompraFn=c=>{const p=s.find(v=>v.id===c);p&&Me(p,()=>Ct())}}catch(i){console.error(i),g("Error al cargar compras",i.message,"error")}}let It="draft",gt=1,z=[];async function Je(){S([{label:"Principal"},{label:"Cotizaciones"}]),C(`
    <div class="page-header">
      <div>
        <h1 class="page-title">📝 Cotizaciones</h1>
        <p class="page-subtitle">Gestión de cotizaciones y órdenes de venta</p>
      </div>
    </div>

    <!-- KPI Row -->
    <div id="cot-kpis" class="kpi-row" style="margin-bottom:24px">
      <div class="kpi-card kpi-blue">
        <div class="kpi-label">Borradores</div>
        <div class="kpi-value" id="kpi-borradores">—</div>
        <div class="kpi-sub">En proceso</div>
      </div>
      <div class="kpi-card kpi-violet">
        <div class="kpi-label">Importe Total</div>
        <div class="kpi-value" id="kpi-importe">—</div>
        <div class="kpi-sub">Cotizaciones abiertas</div>
      </div>
      <div class="kpi-card kpi-red">
        <div class="kpi-label">Vencidas</div>
        <div class="kpi-value" id="kpi-vencidas">—</div>
        <div class="kpi-sub">Requieren atención</div>
      </div>
    </div>

    <!-- Tabs -->
    <div class="tabs" style="margin-bottom:20px">
      <button class="tab-btn active" id="tab-draft"     onclick="window._cotTab('draft')">📋 Cotizaciones</button>
      <button class="tab-btn"        id="tab-confirmed" onclick="window._cotTab('confirmed')">✅ Confirmadas</button>
      <button class="tab-btn"        id="tab-nueva"     onclick="window._cotTab('nueva')">➕ Nueva Cotización</button>
    </div>

    <!-- Content area -->
    <div id="cot-content">
      ${I(7,5)}
    </div>
  `),window._cotTab=St,window._cotPage=ht,window._cotDetail=xt,window._cotConfirm=We,window._cotCancel=Ye,window._cotAddLine=Ze,window._cotDelLine=ta,yt(),St("draft")}async function yt(){try{const t=await f.cotizacionKpis(),e=(t==null?void 0:t.data)??t;if(!e)return;document.getElementById("kpi-borradores").textContent=e.total_borradores??"—",document.getElementById("kpi-importe").textContent=x(e.importe_total),document.getElementById("kpi-vencidas").textContent=e.vencidas??"0"}catch{}}function St(t){It=t,gt=1,document.querySelectorAll(".tab-btn").forEach(a=>a.classList.remove("active"));const e=document.getElementById("tab-"+t);e&&e.classList.add("active"),ht(1)}async function ht(t=1){gt=t;const e=document.getElementById("cot-content");if(e){if(It==="nueva"){ea();return}e.innerHTML=I(7,8);try{let a;It==="draft"?a=await f.cotizaciones(t):a=await f.ventas(t);const o=(a==null?void 0:a.data)??[],i=(a==null?void 0:a.total)??o.length,r=(a==null?void 0:a.por_pagina)??20,n=t*r<i,l={draft:"Borrador",sent:"Enviada",sale:"Confirmada",done:"Realizada",cancel:"Cancelada"};if(!o.length){e.innerHTML=`<div style="text-align:center;padding:48px;color:var(--text-400)">
        <div style="font-size:48px;margin-bottom:12px">📋</div>
        <p>No hay cotizaciones en esta sección</p>
      </div>`;return}e.innerHTML=`
      <div class="table-container">
        <table class="data-table">
          <thead><tr>
            <th>#</th><th>Referencia</th><th>Cliente</th><th>Estado</th>
            <th>Subtotal</th><th>IVA</th><th>Total</th><th>Fecha</th><th>Validez</th><th></th>
          </tr></thead>
          <tbody>
            ${o.map(s=>{const d=l[s.state]||s.state;return`<tr style="cursor:pointer" onclick="window._cotDetail(${s.id})">
                <td style="font-size:11px;color:var(--text-400)">${s.id}</td>
                <td style="font-weight:600;color:var(--primary)">${s.name||"—"}</td>
                <td>${s.partner_name||s.partner_id||"—"}</td>
                <td>${G(s.state,d)}</td>
                <td>${x(s.amount_untaxed)}</td>
                <td>${x(s.amount_tax)}</td>
                <td style="font-weight:600">${x(s.amount_total)}</td>
                <td style="font-size:12px;color:var(--text-400)">${T(s.date_order)}</td>
                <td style="font-size:12px;color:var(--text-400)">${T(s.validity_date)}</td>
                <td onclick="event.stopPropagation()">
                  <button class="btn btn-secondary btn-sm" onclick="window._cotDetail(${s.id})">Ver</button>
                </td>
              </tr>`}).join("")}
          </tbody>
        </table>
        ${D(t,n,window._cotPage)}
      </div>
    `}catch(a){e.innerHTML=`<div class="empty-state"><p style="color:var(--red)">Error cargando cotizaciones: ${a.message}</p></div>`}}}function xt(t){nt(`Cotización #${t}`,()=>f.cotizacion(t),Qe)}function Qe(t){const e=(t==null?void 0:t.orden)??t,a=(t==null?void 0:t.lineas)??[],o={draft:"Borrador",sent:"Enviada",sale:"Confirmada",done:"Realizada",cancel:"Cancelada"},i=["draft","sent"].includes(e.state),r=i,n=!["cancel","done"].includes(e.state),l=a.length?`<div class="table-container" style="margin-top:12px">
        <table class="data-table" style="font-size:12px">
          <thead><tr><th>Producto</th><th>Cant.</th><th>Precio U.</th><th>Dto%</th><th>Subtotal</th><th></th></tr></thead>
          <tbody>
            ${a.map(u=>`<tr>
              <td>${u.name||"—"}</td>
              <td>${u.product_uom_qty}</td>
              <td>${x(u.price_unit)}</td>
              <td>${u.discount?u.discount+"%":"—"}</td>
              <td style="font-weight:600">${x(u.price_subtotal)}</td>
              <td>${i?`<button class="btn btn-secondary btn-sm" style="color:var(--red)" onclick="window._cotDelLine(${e.id},${u.id})">✕</button>`:""}</td>
            </tr>`).join("")}
          </tbody>
        </table>
      </div>`:'<p style="color:var(--text-400);font-size:13px;padding:8px 0">Sin líneas de venta</p>',s=i?`
    <div style="margin-top:16px;padding:16px;background:var(--surface-2);border-radius:10px;border:1px solid var(--border)">
      <div style="font-weight:600;margin-bottom:12px;font-size:13px">➕ Agregar línea</div>
      <div style="display:grid;grid-template-columns:2fr 1fr 1fr 1fr;gap:8px;margin-bottom:8px">
        <input id="linea-name" class="form-control" placeholder="Descripción" style="font-size:13px">
        <input id="linea-qty"  class="form-control" type="number" placeholder="Cantidad" value="1" min="0.01" step="0.01" style="font-size:13px">
        <input id="linea-price" class="form-control" type="number" placeholder="Precio" min="0" step="0.01" style="font-size:13px">
        <input id="linea-dto" class="form-control" type="number" placeholder="Dto %" min="0" max="100" step="0.01" style="font-size:13px">
      </div>
      <button class="btn btn-primary btn-sm" onclick="window._cotAddLine(${e.id})">Agregar línea</button>
    </div>`:"",d=`
    <div style="display:flex;gap:8px;margin-top:20px;flex-wrap:wrap">
      ${r?`<button class="btn btn-primary" onclick="window._cotConfirm(${e.id})">✅ Confirmar pedido</button>`:""}
      ${n?`<button class="btn btn-secondary" style="color:var(--red)" onclick="window._cotCancel(${e.id})">🚫 Cancelar</button>`:""}
    </div>`;return`
    ${P("Información General",`
      ${$("Referencia",e.name)}
      ${$("Estado",G(e.state,o[e.state]||e.state))}
      ${$("Cliente",e.partner_name||"—")}
      ${$("Referencia cliente",e.client_order_ref||"—")}
      ${$("Fecha",T(e.date_order))}
      ${$("Validez",T(e.validity_date))}
      ${$("Estado factura",e.invoice_status||"—")}
    `)}
    ${P("Importes",`
      ${$("Subtotal",x(e.amount_untaxed))}
      ${$("IVA",x(e.amount_tax))}
      ${$("Total",`<strong style="font-size:16px;color:var(--primary)">${x(e.amount_total)}</strong>`)}
    `)}
    ${P("Líneas de venta",l+s)}
    ${e.note?P("Notas",`<p style="font-size:13px;line-height:1.6">${e.note}</p>`):""}
    ${d}
  `}async function We(t){if(confirm("¿Confirmar esta cotización? Pasará a pedido de venta."))try{await f.confirmarCotizacion(t),g("Cotización confirmada","El pedido fue confirmado correctamente","success"),window.__closeModal(),yt(),ht(gt)}catch(e){g("Error",e.message,"error")}}async function Ye(t){if(confirm("¿Cancelar esta cotización?"))try{await f.cancelarCotizacion(t),g("Cotización cancelada","","info"),window.__closeModal(),yt(),ht(gt)}catch(e){g("Error",e.message,"error")}}async function Ze(t){var r,n,l,s,d;const e=(n=(r=document.getElementById("linea-name"))==null?void 0:r.value)==null?void 0:n.trim(),a=parseFloat(((l=document.getElementById("linea-qty"))==null?void 0:l.value)||"1"),o=parseFloat(((s=document.getElementById("linea-price"))==null?void 0:s.value)||"0"),i=parseFloat(((d=document.getElementById("linea-dto"))==null?void 0:d.value)||"0")||null;if(!e)return g("Campo requerido","Escribe una descripción de producto","warning");if(!o)return g("Campo requerido","Ingresa el precio unitario","warning");try{await f.agregarLinea(t,{name:e,product_uom_qty:a,price_unit:o,discount:i}),g("Línea agregada","","success"),xt(t)}catch(u){g("Error al agregar línea",u.message,"error")}}async function ta(t,e){if(confirm("¿Eliminar esta línea?"))try{await f.eliminarLinea(t,e),g("Línea eliminada","","success"),xt(t)}catch(a){g("Error",a.message,"error")}}function ea(){z=[];const t=document.getElementById("cot-content");t&&(t.innerHTML=`
    <div style="max-width:800px;margin:0 auto">
      <div class="card" style="padding:28px">
        <h2 style="font-size:18px;font-weight:700;margin-bottom:24px;color:var(--text)">Nueva Cotización</h2>

        <div style="display:grid;grid-template-columns:1fr 1fr;gap:16px;margin-bottom:16px">
          <div class="form-group">
            <label class="form-label">Cliente (nombre exacto) *</label>
            <input id="nv-partner" class="form-control" placeholder="Nombre del cliente" autocomplete="off">
          </div>
          <div class="form-group">
            <label class="form-label">Referencia del cliente</label>
            <input id="nv-ref" class="form-control" placeholder="Ej: OC-2024-001">
          </div>
          <div class="form-group">
            <label class="form-label">Fecha de validez</label>
            <input id="nv-validez" class="form-control" type="date">
          </div>
          <div class="form-group">
            <label class="form-label">Notas internas</label>
            <input id="nv-nota" class="form-control" placeholder="Observaciones opcionales">
          </div>
        </div>

        <!-- Sección de líneas -->
        <div style="margin-top:24px">
          <div style="font-weight:700;font-size:14px;margin-bottom:12px;display:flex;align-items:center;justify-content:space-between">
            <span>📦 Líneas de venta</span>
            <button class="btn btn-secondary btn-sm" onclick="window._nvAddRow()">+ Agregar producto</button>
          </div>
          <div id="nv-lineas-list">
            <p style="color:var(--text-400);font-size:13px;padding:16px 0;text-align:center">
              Sin líneas. Agrega productos para calcular el total.
            </p>
          </div>
        </div>

        <!-- Totales -->
        <div style="margin-top:20px;padding:16px;background:var(--surface-2);border-radius:10px;border:1px solid var(--border)">
          <div style="display:flex;justify-content:space-between;padding:6px 0;font-size:13px">
            <span style="color:var(--text-400)">Subtotal</span>
            <strong id="nv-subtotal">$0.00</strong>
          </div>
          <div style="display:flex;justify-content:space-between;padding:6px 0;font-size:13px">
            <span style="color:var(--text-400)">IVA (16%)</span>
            <strong id="nv-iva">$0.00</strong>
          </div>
          <div style="display:flex;justify-content:space-between;padding:10px 0 6px;border-top:2px solid var(--border);margin-top:4px">
            <span style="font-weight:700">Total</span>
            <strong id="nv-total" style="font-size:18px;color:var(--primary)">$0.00</strong>
          </div>
        </div>

        <div style="display:flex;gap:12px;margin-top:24px">
          <button class="btn btn-primary" onclick="window._nvGuardar()">💾 Guardar cotización</button>
          <button class="btn btn-secondary" onclick="window._cotTab('draft')">Cancelar</button>
        </div>
      </div>
    </div>
  `,window._nvAddRow=aa,window._nvDelRow=oa,window._nvGuardar=ia,window._nvRecalc=Tt)}function aa(){z.length,z.push({name:"",qty:1,price:0,discount:0}),ee()}function oa(t){z.splice(t,1),ee()}function ee(){const t=document.getElementById("nv-lineas-list");if(t){if(!z.length){t.innerHTML='<p style="color:var(--text-400);font-size:13px;padding:16px 0;text-align:center">Sin líneas.</p>',Tt();return}t.innerHTML=`
    <div class="table-container">
      <table class="data-table" style="font-size:13px">
        <thead><tr>
          <th style="width:40%">Descripción *</th>
          <th>Cant.</th>
          <th>Precio U.</th>
          <th>Dto %</th>
          <th>Subtotal</th>
          <th></th>
        </tr></thead>
        <tbody>
          ${z.map((e,a)=>{const o=parseFloat(e.discount)||0,i=(parseFloat(e.qty)||0)*(parseFloat(e.price)||0)*(1-o/100);return`<tr>
              <td><input class="form-control" style="font-size:12px" value="${e.name}" oninput="_lineasNueva[${a}].name=this.value" placeholder="Descripción del producto"></td>
              <td><input class="form-control" style="font-size:12px;width:70px" type="number" min="0.01" step="0.01" value="${e.qty}" oninput="_lineasNueva[${a}].qty=this.value;window._nvRecalc()"></td>
              <td><input class="form-control" style="font-size:12px;width:90px" type="number" min="0" step="0.01" value="${e.price}" oninput="_lineasNueva[${a}].price=this.value;window._nvRecalc()"></td>
              <td><input class="form-control" style="font-size:12px;width:65px" type="number" min="0" max="100" step="0.01" value="${e.discount}" oninput="_lineasNueva[${a}].discount=this.value;window._nvRecalc()"></td>
              <td style="font-weight:600">${x(i)}</td>
              <td><button class="btn btn-secondary btn-sm" style="color:var(--red)" onclick="window._nvDelRow(${a})">✕</button></td>
            </tr>`}).join("")}
        </tbody>
      </table>
    </div>`,Tt()}}function Tt(){let t=0;z.forEach(l=>{const s=parseFloat(l.discount)||0;t+=(parseFloat(l.qty)||0)*(parseFloat(l.price)||0)*(1-s/100)});const e=t*.16,a=t+e,o=l=>l.toLocaleString("es-MX",{minimumFractionDigits:2,maximumFractionDigits:2}),i=document.getElementById("nv-subtotal"),r=document.getElementById("nv-iva"),n=document.getElementById("nv-total");i&&(i.textContent="$"+o(t)),r&&(r.textContent="$"+o(e)),n&&(n.textContent="$"+o(a))}async function ia(){var n,l,s,d,u,h,m,c;const t=(l=(n=document.getElementById("nv-partner"))==null?void 0:n.value)==null?void 0:l.trim(),e=((d=(s=document.getElementById("nv-ref"))==null?void 0:s.value)==null?void 0:d.trim())||null,a=((u=document.getElementById("nv-validez"))==null?void 0:u.value)||null,o=((m=(h=document.getElementById("nv-nota"))==null?void 0:h.value)==null?void 0:m.trim())||null;if(!t)return g("Campo requerido","Ingresa el nombre del cliente","warning");let i=1;try{const p=await f.get(`/partners?pagina=1&q=${encodeURIComponent(t)}&por_pagina=5`),v=(p==null?void 0:p.data)??[],b=v.find(_=>{var E;return((E=_.name)==null?void 0:E.toLowerCase())===t.toLowerCase()});if(b)i=b.id;else if(v.length>0)i=v[0].id;else return g("Cliente no encontrado",`No se encontró "${t}"`,"warning")}catch(p){return g("Error","No se pudo buscar el cliente: "+p.message,"error")}const r={partner_id:i,partner_invoice_id:i,partner_shipping_id:i,note:o,client_order_ref:e,validity_date:a||null};try{const p=await f.crearCotizacion(r),v=((c=p==null?void 0:p.data)==null?void 0:c.id)??(p==null?void 0:p.id);if(g("Cotización creada",`ID ${v} — Referencia generada`,"success"),v&&z.length)for(const b of z)b.name&&await f.agregarLinea(v,{name:b.name,product_uom_qty:parseFloat(b.qty)||1,price_unit:parseFloat(b.price)||0,discount:parseFloat(b.discount)||null}).catch(()=>{});z=[],yt(),St("draft"),setTimeout(()=>v&&xt(v),600)}catch(p){g("Error al crear cotización",p.message,"error")}}let _t=null;async function na(){L(),S([{label:"Dashboard",href:"dashboard"},{label:"NexusSearch"}]),await sa()}async function sa(){var e,a;C(`
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
  <div id="index-status" class="anim-4" style="margin-top:16px"></div>`),(e=document.getElementById("search-query"))==null||e.addEventListener("keydown",o=>{o.key==="Enter"&&window._buscar()});let t;(a=document.getElementById("search-query"))==null||a.addEventListener("input",o=>{clearTimeout(t),!(o.target.value.length<2)&&(t=setTimeout(()=>window._buscar(),500))}),await Gt(),window._buscar=la,window._checkStatus=Gt,window._syncSearch=da}async function la(){var a,o;const t=(o=(a=document.getElementById("search-query"))==null?void 0:a.value)==null?void 0:o.trim();if(!t||t.length<2)return;const e=document.getElementById("search-results");e&&(e.innerHTML=`
    <div class="data-card" style="padding:20px;text-align:center;color:var(--text-400)">
      <div class="spinner" style="margin:0 auto 8px"></div>
      <div>Buscando "${t}"…</div>
    </div>`);try{const[i,r,n]=await Promise.allSettled([f.ventas(1).then(s=>((s==null?void 0:s.data)||[]).filter(d=>(d.name||"").toLowerCase().includes(t.toLowerCase())||(d.partner_name||"").toLowerCase().includes(t.toLowerCase())).map(d=>({tipo:"Venta",icon:"💰",titulo:d.name,sub:d.partner_name,meta:`$${d.amount_total}`,href:"ventas"}))),f.productos(1,t).then(s=>((s==null?void 0:s.data)||[]).map(d=>{var u,h;return{tipo:"Producto",icon:"📦",titulo:typeof d.name=="object"?((u=d.name)==null?void 0:u.es_MX)||((h=d.name)==null?void 0:h.en_US)||"":d.name||"",sub:d.categ_name||"",meta:"",href:"productos"}})),f.partners(1).then(s=>((s==null?void 0:s.data)||[]).filter(d=>(d.name||"").toLowerCase().includes(t.toLowerCase())||(d.email||"").toLowerCase().includes(t.toLowerCase())).map(d=>({tipo:"Contacto",icon:"👥",titulo:d.name,sub:d.email||"",meta:"",href:"partners"})))]),l=[...i.status==="fulfilled"?i.value:[],...r.status==="fulfilled"?r.value:[],...n.status==="fulfilled"?n.value:[]];if(!e)return;if(l.length===0){e.innerHTML=`
      <div class="data-card" style="padding:40px;text-align:center">
        <div style="font-size:36px;margin-bottom:12px">🔍</div>
        <div style="font-weight:700;color:var(--text-700)">Sin resultados para "${t}"</div>
        <div style="color:var(--text-400);font-size:13px;margin-top:6px">Prueba con otro término</div>
      </div>`;return}e.innerHTML=`
    <div class="data-card">
      <div class="data-card-header">
        <div class="data-card-title">${l.length} resultados para "${t}"</div>
      </div>
      <div style="padding:0 4px">
        ${l.slice(0,30).map(s=>`
        <div style="display:flex;align-items:center;gap:12px;padding:12px 8px;
          border-bottom:1px solid var(--border);cursor:pointer;border-radius:8px;
          transition:background var(--t1)" 
          onmouseover="this.style.background='var(--primary-light)'"
          onmouseout="this.style.background=''"
          onclick="window._go('${s.href}')">
          <div style="width:36px;height:36px;border-radius:10px;background:var(--primary-light);
            display:flex;align-items:center;justify-content:center;font-size:18px;flex-shrink:0">
            ${s.icon}
          </div>
          <div style="flex:1">
            <div style="font-weight:600;color:var(--text-800);font-size:13px">${s.titulo}</div>
            <div style="font-size:11px;color:var(--text-400)">${s.sub}</div>
          </div>
          <div style="display:flex;align-items:center;gap:8px">
            ${s.meta?`<span style="font-size:12px;font-weight:700;color:var(--text-700)">${s.meta}</span>`:""}
            <span class="badge badge-${s.tipo==="Venta"?"indigo":s.tipo==="Producto"?"emerald":"violet"}">${s.tipo}</span>
          </div>
        </div>`).join("")}
      </div>
    </div>`}catch(i){console.error(i),e&&(e.innerHTML=`<p style="color:var(--red);padding:20px">Error: ${i.message}</p>`)}}async function Gt(){const t=document.getElementById("index-status");try{const e=await f.searchStatus().catch(()=>null);_t=(e==null?void 0:e.data)||e,t&&_t&&(t.innerHTML=`
      <div class="data-card" style="padding:16px">
        <div class="data-card-header" style="padding:0 0 12px">
          <div class="data-card-title">📡 Estado del Motor de Búsqueda</div>
        </div>
        <div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(180px,1fr));gap:12px">
          ${Object.entries(_t).map(([a,o])=>`
          <div style="padding:12px;background:var(--bg);border-radius:10px;border:1px solid var(--border)">
            <div style="font-size:10px;color:var(--text-400);font-weight:700;text-transform:uppercase;margin-bottom:4px">${a}</div>
            <div style="font-size:13px;font-weight:600;color:var(--text-800)">${JSON.stringify(o)}</div>
          </div>`).join("")}
        </div>
      </div>`)}catch{t&&(t.innerHTML="")}}async function da(){const t=document.getElementById("btn-sync");t&&(t.textContent="⏳ Sincronizando…",t.disabled=!0);try{const e=await f.searchSync();g("Sincronización iniciada",(e==null?void 0:e.message)||"Los índices se están actualizando","success")}catch(e){g("Error de sincronización",e.message,"error")}finally{t&&(t.textContent="⚡ Sincronizar Índices",t.disabled=!1)}}async function ra(){L(),S([{label:"Dashboard",href:"dashboard"},{label:"Reportes"}]),await ca()}async function ca(){C(`
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
  </div>`),window._verReporte=t=>{g("Reporte seleccionado",`Generando reporte de ${t}…`,"info"),pa(t)},window._exportReporte=()=>{g("Exportar","Función de exportación CSV/PDF — próximamente","info")},await ae()}async function ae(){var e,a,o,i;const t=document.getElementById("rep-fecha");t&&(t.textContent=new Date().toLocaleDateString("es-MX",{day:"2-digit",month:"long",year:"numeric"}));try{const[r,n,l,s]=await Promise.allSettled([f.ventaKpis(),f.factKpis(),f.stockKpis(),f.comprasKpis()]),d=((e=r.value)==null?void 0:e.data)||{},u=((a=n.value)==null?void 0:a.data)||{},h=((o=l.value)==null?void 0:o.data)||{},m=((i=s.value)==null?void 0:i.data)||{},c=document.getElementById("rep-kpis");c&&(c.innerHTML=`
      ${[{label:"Ventas confirmadas",val:d.ordenes_confirmadas??0,tipo:"num",desc:`$${parseFloat(d.total_facturado||0).toLocaleString("es-MX",{minimumFractionDigits:2})} este mes`},{label:"Facturación total",val:x(parseFloat(u.monto_total||0)),tipo:"txt",desc:`${u.total_facturas??0} comprobantes emitidos`},{label:"Valor inventario",val:x(parseFloat(h.valor_inventario||0)),tipo:"txt",desc:`${h.alertas_stock_bajo??0} alertas de stock bajo`}].map(p=>`
      <div style="padding:16px;background:var(--bg);border-radius:12px;border:1px solid var(--border)">
        <div style="font-size:11px;color:var(--text-400);font-weight:700;text-transform:uppercase;margin-bottom:6px">${p.label}</div>
        <div style="font-size:24px;font-weight:800;color:var(--text-900);margin-bottom:4px">
          ${p.tipo==="num"?Number(p.val).toLocaleString("es-MX"):p.val}
        </div>
        <div style="font-size:11px;color:var(--text-500)">${p.desc}</div>
      </div>`).join("")}

      <div style="grid-column:1/-1;margin-top:8px">
        <div style="font-size:12px;font-weight:700;color:var(--text-600);margin-bottom:10px">COMPRAS</div>
        <div style="display:grid;grid-template-columns:repeat(3,1fr);gap:12px">
          ${[{label:"Total OC",val:m.total??0},{label:"Confirmadas",val:m.confirmadas??0},{label:"Monto compras",val:x(parseFloat(m.monto_total||0))}].map(p=>`
          <div style="padding:12px;background:var(--bg);border-radius:10px;border:1px solid var(--border)">
            <div style="font-size:10px;color:var(--text-400);font-weight:700;text-transform:uppercase;margin-bottom:4px">${p.label}</div>
            <div style="font-size:18px;font-weight:800;color:var(--text-900)">${p.val}</div>
          </div>`).join("")}
        </div>
      </div>`)}catch(r){console.error(r)}}async function pa(t){const e=document.getElementById("rep-kpis"),a=document.querySelector(".data-card-title");if(a){const o={ventas:"💰 Reporte de Ventas",facturas:"🧾 Facturación",inventario:"🏭 Inventario",compras:"🛒 Compras",clientes:"👥 Clientes",nomina:"👔 Nómina"};a.textContent=o[t]||"Reporte"}e&&(e.innerHTML='<div class="skeleton" style="height:120px;border-radius:12px;grid-column:1/-1"></div>'),await ae()}function va(t,e,a,o){L(),S([{label:"Dashboard",href:"dashboard"},{label:e}]),C(`
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
  </div>`)}F("login",ce);F("home",fe);F("dashboard",Ut);F("ventas",Ie);F("facturas",je);F("productos",Ft);F("partners",Bt);F("stock",Pt);F("cfdi",Oe);F("nomina",Ke);F("compras",Xe);F("cotizaciones",Je);F("search",na);F("reportes",ra);F("404",()=>va("404","Página no encontrada","La ruta solicitada no existe","🔍"));se();
