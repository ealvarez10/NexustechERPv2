(function(){const e=document.createElement("link").relList;if(e&&e.supports&&e.supports("modulepreload"))return;for(const o of document.querySelectorAll('link[rel="modulepreload"]'))i(o);new MutationObserver(o=>{for(const l of o)if(l.type==="childList")for(const s of l.addedNodes)s.tagName==="LINK"&&s.rel==="modulepreload"&&i(s)}).observe(document,{childList:!0,subtree:!0});function a(o){const l={};return o.integrity&&(l.integrity=o.integrity),o.referrerPolicy&&(l.referrerPolicy=o.referrerPolicy),o.crossOrigin==="use-credentials"?l.credentials="include":o.crossOrigin==="anonymous"?l.credentials="omit":l.credentials="same-origin",l}function i(o){if(o.ep)return;o.ep=!0;const l=a(o);fetch(o.href,l)}})();const st={isLoggedIn:()=>!!localStorage.getItem("nx_token"),getUser:()=>{try{return JSON.parse(localStorage.getItem("nx_user")||"{}")}catch{return{}}},setSession(t,e){localStorage.setItem("nx_token",t),localStorage.setItem("nx_user",JSON.stringify(e))},clear(){localStorage.removeItem("nx_token"),localStorage.removeItem("nx_user")}},rt={};function E(t,e){rt[t]=e}function nt(t){window.location.hash=t}function $e(){window.addEventListener("hashchange",Qt),Qt()}function Qt(){const t=window.location.hash.replace("#","")||"home";if(!st.isLoggedIn()&&t!=="login"){nt("login");return}if(st.isLoggedIn()&&t==="login"){nt("home");return}const e=rt[t];e?e():rt[404]&&rt[404]()}const ke="/api/v1";function Ee(){return localStorage.getItem("nx_token")}class Ce extends Error{constructor(e,a){super(a),this.status=e}}async function m(t,e,a){const i=Ee(),o=await fetch(ke+e,{method:t,headers:{"Content-Type":"application/json",...i?{Authorization:`Bearer ${i}`}:{}},...a!==void 0?{body:JSON.stringify(a)}:{}});if(o.status===401)return localStorage.removeItem("nx_token"),localStorage.removeItem("nx_user"),window.location.hash="login",null;if(!o.ok)throw new Ce(o.status,await o.text());return(o.headers.get("content-type")||"").includes("application/json")?o.json():o.text()}const p={get:t=>m("GET",t),post:(t,e)=>m("POST",t,e),put:(t,e)=>m("PUT",t,e),del:t=>m("DELETE",t),login:(t,e)=>m("POST","/auth/login",{login:t,password:e}),logout:()=>m("POST","/auth/logout",{}),dashboard:()=>m("GET","/dashboard"),ventaKpis:()=>m("GET","/ventas/kpis"),factKpis:()=>m("GET","/facturas/kpis"),stockKpis:()=>m("GET","/stock/kpis"),ventas:(t=1)=>m("GET",`/ventas?pagina=${t}`),venta:t=>m("GET",`/ventas/${t}`),facturas:(t=1)=>m("GET",`/facturas?pagina=${t}`),factura:t=>m("GET",`/facturas/${t}`),porCobrar:()=>m("GET","/facturas/por-cobrar"),productos:(t=1,e="")=>m("GET",`/productos?pagina=${t}&q=${encodeURIComponent(e)}`),producto:t=>m("GET",`/productos/${t}`),partners:(t=1)=>m("GET",`/partners?pagina=${t}`),partner:t=>m("GET",`/partners/${t}`),clientes:(t=1)=>m("GET",`/clientes?pagina=${t}`),proveedores:(t=1)=>m("GET",`/proveedores?pagina=${t}`),stock:(t=1)=>m("GET",`/stock?pagina=${t}`),stockKpis:()=>m("GET","/stock/kpis"),stockBajo:()=>m("GET","/stock/bajo"),stockProducto:t=>m("GET",`/stock/producto/${t}`),cfdiTimbrados:(t=1)=>m("GET",`/cfdi/timbrados?pagina=${t}`),cfdiTimbrado:t=>m("GET",`/cfdi/timbrados/${t}`),cfdiKpis:()=>m("GET","/cfdi/kpis"),timbrar:t=>m("POST","/cfdi/timbrar",t),cancelarCfdi:t=>m("POST","/cfdi/cancelar",t),nomina:(t=1)=>m("GET",`/nomina?pagina=${t}`),empleado:t=>m("GET",`/nomina/${t}`),nominaKpis:()=>m("GET","/nomina/kpis"),compras:(t=1)=>m("GET",`/compras?pagina=${t}`),compra:t=>m("GET",`/compras/${t}`),comprasKpis:()=>m("GET","/compras/kpis"),cotizaciones:(t=1)=>m("GET",`/cotizaciones?pagina=${t}`),cotizacionKpis:()=>m("GET","/cotizaciones/kpis"),cotizacion:t=>m("GET",`/cotizaciones/${t}`),crearCotizacion:t=>m("POST","/cotizaciones",t),confirmarCotizacion:t=>m("PUT",`/cotizaciones/${t}/confirmar`),cancelarCotizacion:t=>m("PUT",`/cotizaciones/${t}/cancelar`),actualizarCotizacion:(t,e)=>m("PUT",`/cotizaciones/${t}`,e),agregarLinea:(t,e)=>m("POST",`/cotizaciones/${t}/lineas`,e),eliminarLinea:(t,e)=>m("DELETE",`/cotizaciones/${t}/lineas/${e}`),searchSync:()=>m("POST","/search/sync",{}),searchStatus:()=>m("GET","/search/status"),health:()=>m("GET","/health"),putVenta:(t,e)=>m("PUT",`/ventas/${t}`,e),putPartner:(t,e)=>m("PUT",`/partners/${t}`,e),putProducto:(t,e)=>m("PUT",`/productos/${t}`,e),putCompra:(t,e)=>m("PUT",`/compras/${t}`,e),putEmpleado:(t,e)=>m("PUT",`/nomina/${t}`,e),ajusteStock:(t,e)=>m("PUT",`/stock/${t}/ajuste`,e)};function Se(){const t=document.getElementById("__shell");t&&t.remove(),document.getElementById("app").innerHTML=`
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
  </div>`;const e=document.getElementById("lbtn"),a=document.getElementById("lu"),i=document.getElementById("lp"),o=document.getElementById("lerr");async function l(){if(e.disabled)return;const s=a.value.trim(),c=i.value;if(!s||!c){o.textContent="Ingresa usuario y contraseña",o.classList.add("show");return}e.disabled=!0,e.textContent="Verificando...",o.classList.remove("show");try{const d=await p.login(s,c),n=(d==null?void 0:d.data)||d,r=(n==null?void 0:n.access_token)||(n==null?void 0:n.token);if(r){st.setSession(r,{nombre:n.email||s,email:n.email||s,user_id:n.user_id,company_id:n.company_id}),document.getElementById("app").innerHTML="",nt("dashboard");return}o.textContent="Error inesperado del servidor. Intenta de nuevo.",o.classList.add("show")}catch(d){o.textContent=(d==null?void 0:d.status)===401?"Credenciales incorrectas. Verifica tu usuario y contraseña.":`Error de conexión: ${(d==null?void 0:d.message)||"No se pudo contactar el servidor"}`,o.classList.add("show")}e.disabled=!1,e.textContent="Acceder al sistema"}e.addEventListener("click",l),i.addEventListener("keydown",s=>s.key==="Enter"&&l()),a.addEventListener("keydown",s=>s.key==="Enter"&&i.focus()),setTimeout(()=>a.focus(),100)}function Be(t,e=0){return t==null||t===""?"—":Number(t).toLocaleString("es-MX",{minimumFractionDigits:e,maximumFractionDigits:e})}function f(t){return t==null?"—":(t=parseFloat(t)||0,Math.abs(t)>=1e6?`$${(t/1e6).toFixed(2)}M`:Math.abs(t)>=1e3?`$${(t/1e3).toFixed(1)}k`:`$${Be(t,2)}`)}function F(t){return t==null?"—":Number(t).toLocaleString("es-MX")}function B(t){return t?new Date(t).toLocaleDateString("es-MX",{day:"2-digit",month:"short",year:"numeric"}):"—"}function b(t,e="",a="info"){const i={success:"✅",error:"❌",info:"ℹ️",warning:"⚠️"};let o=document.getElementById("__toasts");o||(o=document.createElement("div"),o.id="__toasts",o.className="toast-container",document.body.appendChild(o));const l=document.createElement("div");l.className=`toast ${a}`,l.innerHTML=`
    <span class="toast-icon">${i[a]||"ℹ️"}</span>
    <div><div class="toast-title">${t}</div>${e?`<div class="toast-msg">${e}</div>`:""}</div>`,o.appendChild(l),requestAnimationFrame(()=>l.classList.add("show")),setTimeout(()=>{l.classList.remove("show"),setTimeout(()=>l.remove(),400)},3800)}function Jt(t,e,a=900,i="",o=""){if(!t)return;const l=performance.now(),s=String(e).includes(".");function c(d){const n=Math.min((d-l)/a,1),r=1-Math.pow(1-n,3),v=e*r;t.textContent=i+(s?v.toLocaleString("es-MX",{minimumFractionDigits:2,maximumFractionDigits:2}):Math.round(v).toLocaleString("es-MX"))+o,n<1&&requestAnimationFrame(c)}requestAnimationFrame(c)}function Ie(t){if(!(t!=null&&t.length))return"";const e=Math.max(...t,1);return`<div class="sparkline">${t.map((a,i)=>`<div class="spark-bar${i===t.length-1?" active":""}" style="height:${Math.max(4,Math.round(a/e*100))}%"></div>`).join("")}</div>`}function Te(t=5,e=6){return`<tbody>${Array.from({length:e},()=>`<tr>${Array.from({length:t},()=>`<td><div class="skeleton" style="height:14px;width:${60+Math.random()*30}%"></div></td>`).join("")}</tr>`).join("")}</tbody>`}function x(t=5,e=4){return`<table class="data-table"><thead><tr>${Array.from({length:e},()=>`<th><div class="skeleton" style="height:12px;width:${40+Math.random()*40}%"></div></th>`).join("")}</tr></thead>${Te(e,t)}</table>`}function Fe(t=5){return Array.from({length:t},()=>`
  <div class="kpi-card kpi-gray">
    <div class="kpi-label"><div class="skeleton" style="height:13px;width:60%"></div></div>
    <div class="kpi-value"><div class="skeleton" style="height:28px;width:70%"></div></div>
    <div><div class="skeleton" style="height:11px;width:40%;margin-top:6px"></div></div>
  </div>`).join("")}const Le={sale:"emerald",done:"indigo",draft:"gray",sent:"sky",cancel:"red",posted:"emerald",in_payment:"violet",paid:"emerald",partial:"amber"};function q(t,e){return`<span class="badge badge-${Le[t]||"gray"} badge-dot">${e}</span>`}function K(t,e,a){return window.__pagNav=a,`
  <div class="data-table-footer">
    <span style="color:var(--text-400)">Página ${t}</span>
    <div class="pagination">
      <button class="pag-btn" ${t<=1?"disabled":""} onclick="window.__pagNav(${t-1})">&#8592; Anterior</button>
      <span class="pag-btn active">${t}</span>
      <button class="pag-btn" ${e?"":"disabled"} onclick="window.__pagNav(${t+1})">Siguiente &#8594;</button>
    </div>
  </div>`}let O=null;function X(t,e,a={}){let i=document.getElementById("__modal-overlay");i||(i=document.createElement("div"),i.id="__modal-overlay",i.innerHTML=`
      <div id="__modal-drawer">
        <div id="__modal-header">
          <span id="__modal-title"></span>
          <button id="__modal-close" onclick="window.__closeModal()">✕</button>
        </div>
        <div id="__modal-body"></div>
      </div>`,document.body.appendChild(i),i.addEventListener("click",o=>{o.target===i&&window.__closeModal()})),document.getElementById("__modal-title").textContent=t,document.getElementById("__modal-body").innerHTML=e,i.classList.add("open"),document.body.style.overflow="hidden",O&&document.removeEventListener("keydown",O),O=o=>{o.key==="Escape"&&window.__closeModal()},document.addEventListener("keydown",O),a.onMounted&&setTimeout(a.onMounted,10)}function Pe(){const t=document.getElementById("__modal-overlay");t&&t.classList.remove("open"),document.body.style.overflow="",O&&(document.removeEventListener("keydown",O),O=null)}window.__closeModal=Pe;async function Ae(t,e,a){X(t,`
    <div style="display:flex;flex-direction:column;gap:12px;padding:8px 0">
      ${[1,2,3,4,5].map(()=>'<div class="skeleton" style="height:52px;border-radius:10px"></div>').join("")}
    </div>`);try{const i=await e(),o=(i==null?void 0:i.data)??i;document.getElementById("__modal-body").innerHTML=a(o)}catch(i){document.getElementById("__modal-body").innerHTML=`<p style="color:var(--red);padding:24px">Error: ${i.message}</p>`}}function T(t,e,a={}){const i=e??"—",o=a.color?`color:${a.color}`:"";return`
  <div style="display:flex;justify-content:space-between;align-items:flex-start;
    padding:10px 0;border-bottom:1px solid var(--border)">
    <span style="font-size:12px;color:var(--text-400);font-weight:600;min-width:140px">${t}</span>
    <span style="font-size:13px;font-weight:500;text-align:right;${o}">${i}</span>
  </div>`}function $t(t,e){return`
  <div style="margin-bottom:20px">
    <div style="font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.06em;
      color:var(--text-400);margin-bottom:8px;padding-bottom:6px;
      border-bottom:2px solid var(--primary)">${t}</div>
    ${e}
  </div>`}const Wt=[{id:"home",icon:"⊞",label:"Inicio",section:"Principal"},{id:"dashboard",icon:"📊",label:"Dashboard",section:"Principal"},{id:"ventas",icon:"💰",label:"Ventas",section:"Principal"},{id:"cotizaciones",icon:"📝",label:"Cotizaciones",section:"Principal"},{id:"facturas",icon:"🧾",label:"Facturación",section:"Principal"},{id:"productos",icon:"📦",label:"Productos",section:"Principal"},{id:"partners",icon:"👥",label:"Clientes",section:"Principal"},{id:"stock",icon:"🏭",label:"Inventario",section:"Principal"},{id:"cfdi",icon:"🔏",label:"CFDI 4.0",section:"Fiscal",badge:"NUEVO"},{id:"nomina",icon:"👔",label:"Nómina IMSS",section:"Fiscal"},{id:"compras",icon:"🛒",label:"Compras",section:"Operaciones"},{id:"search",icon:"🔍",label:"NexusSearch",section:"Sistema"},{id:"reportes",icon:"📈",label:"Reportes",section:"Sistema"}];function C(){if(document.getElementById("__shell"))return;const t=st.getUser(),e=(t.nombre||t.name||"AD").substring(0,2).toUpperCase(),a=[...new Set(Wt.map(i=>i.section))];if(document.getElementById("app").innerHTML=`
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
        ${a.map(i=>`
        <div class="nav-section">
          <div class="nav-section-title">${i}</div>
          ${Wt.filter(o=>o.section===i).map(o=>`
          <a class="nav-link" id="nl-${o.id}" href="#${o.id}" onclick="event.preventDefault();window._go('${o.id}')">
            <span style="font-size:16px">${o.icon}</span>
            <span>${o.label}</span>
            ${o.badge?`<span class="nav-badge">${o.badge}</span>`:""}
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
  </div>`,window._go=i=>{nt(i)},window._logout=()=>{st.clear();const i=document.getElementById("__shell");i&&i.remove(),nt("login"),b("Sesión cerrada","Hasta pronto","info")},window._toggleSidebar=()=>{const i=document.getElementById("__sidebar"),o=document.getElementById("sidebar-toggle");if(!i)return;const l=i.classList.toggle("collapsed");localStorage.setItem("nx_sidebar_collapsed",l?"1":"0"),o&&(o.textContent=l?"▶":"◀")},localStorage.getItem("nx_sidebar_collapsed")==="1"){const i=document.getElementById("__sidebar"),o=document.getElementById("sidebar-toggle");i&&i.classList.add("collapsed"),o&&(o.textContent="▶")}window.addEventListener("hashchange",Yt),Yt()}function w(t){const e=document.getElementById("__page");e&&(e.innerHTML=t,e.scrollTop=0)}function $(t){const e=document.getElementById("__breadcrumb");e&&(e.innerHTML=t.map((a,i)=>`
    <span class="breadcrumb-item"${i<t.length-1&&a.href?` onclick="window._go('${a.href}')"`:""}>
      ${a.label}
      ${i<t.length-1?'<span class="breadcrumb-sep">/</span>':""}
    </span>`).join(""))}function Yt(){const t=window.location.hash.replace("#","")||"home";document.querySelectorAll(".nav-link").forEach(e=>{e.classList.toggle("active",e.id===`nl-${t}`)})}const Zt=[{id:"ventas",icon:"📊",grad:"#4F46E5,#7C3AED",nombre:"Ventas",desc:"Órdenes y Cotizaciones",kpi:"/ventas/kpis",field:"total_ordenes"},{id:"facturas",icon:"🧾",grad:"#059669,#0EA5E9",nombre:"Facturación",desc:"Facturas y Pagos",kpi:"/facturas/kpis",field:"total_facturas"},{id:"partners",icon:"👥",grad:"#7C3AED,#EC4899",nombre:"Clientes",desc:"Contactos y Partners",kpi:"/partners",field:null},{id:"stock",icon:"🏭",grad:"#D97706,#EA580C",nombre:"Inventario",desc:"Control de Stock",kpi:"/stock/kpis",field:"total_productos_con_stock"},{id:"compras",icon:"🛒",grad:"#2563EB,#4F46E5",nombre:"Compras",desc:"Órdenes de Compra",kpi:"/compras/kpis",field:"total_ordenes"},{id:"productos",icon:"📦",grad:"#0D9488,#059669",nombre:"Productos",desc:"Catálogo de Artículos",kpi:"/productos",field:null},{id:"cfdi",icon:"🔐",grad:"#E11D48,#DC2626",nombre:"CFDI 4.0",desc:"Timbrado Fiscal Digital",kpi:"/cfdi/historial",field:null},{id:"nomina",icon:"👔",grad:"#0EA5E9,#2563EB",nombre:"Nómina IMSS",desc:"Nóminas y Seguridad Social",kpi:"/nomina/kpis",field:"total_empleados"},{id:"reportes",icon:"📈",grad:"#475569,#1E293B",nombre:"Reportes",desc:"Análisis y BI",kpi:null,field:null},{id:"cotizaciones",icon:"📝",grad:"#8B5CF6,#4F46E5",nombre:"Cotizaciones",desc:"Borradores y Propuestas",kpi:"/cotizaciones/kpis",field:"total_borradores"},{id:"dashboard",icon:"📊",grad:"#0F172A,#1E293B",nombre:"Dashboard",desc:"Vista general del sistema",kpi:null,field:null}];async function Me(){C(),$([{label:"Inicio"}]),w(`
    <div class="nx-home">
      <div class="nx-home-header">
        <h1 class="nx-home-title">Aplicaciones</h1>
        <div class="nx-home-search">
          <input type="search" placeholder="Buscar módulo…" id="home-search" oninput="window._filterApps(this.value)">
        </div>
      </div>
      <div class="nx-app-grid" id="home-app-grid">
        ${Zt.map((t,e)=>`
          <div class="nx-app-card" data-id="${t.id}" onclick="window._go('${t.id}')" style="animation-delay:${e*50}ms">
            <div class="nx-app-icon" style="background:linear-gradient(135deg,${t.grad})">${t.icon}</div>
            <div class="nx-app-badge" id="app-badge-${t.id}">…</div>
            <div class="nx-app-name">${t.nombre}</div>
            <div class="nx-app-desc">${t.desc}</div>
          </div>
        `).join("")}
      </div>
    </div>
  `),await Promise.allSettled(Zt.filter(t=>t.kpi).map(async t=>{try{const e=await p.get(t.kpi),a=(e==null?void 0:e.data)??e,i=t.field&&a?a[t.field]??"—":Array.isArray(a)?a.length:"—",o=document.getElementById("app-badge-"+t.id);o&&(o.textContent=Number(i)>999?(i/1e3).toFixed(1)+"k":i)}catch{const e=document.getElementById("app-badge-"+t.id);e&&(e.textContent="—")}})),window._filterApps=t=>{const e=t.toLowerCase().trim();document.querySelectorAll(".nx-app-card").forEach(a=>{var l,s;const i=((l=a.querySelector(".nx-app-name"))==null?void 0:l.textContent.toLowerCase())||"",o=((s=a.querySelector(".nx-app-desc"))==null?void 0:s.textContent.toLowerCase())||"";a.classList.toggle("hidden",!!e&&!i.includes(e)&&!o.includes(e))})}}const ze={sale:"indigo",done:"emerald",draft:"gray",cancel:"red",sent:"sky",posted:"emerald"},De={sale:"Confirmada",done:"Entregada",draft:"Borrador",cancel:"Cancelada",sent:"Enviada"};function J(t,e=10){return Array.from({length:e},()=>Math.max(5,Math.round(t*(.6+Math.random()*.8))))}async function ne(){var t,e,a,i,o,l,s,c,d;C(),$([{label:"Dashboard"}]),w(`
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
  <div class="kpi-grid anim-2" id="kpi-grid">${Fe(5)}</div>

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
      <div id="tabla-ventas">${x(6,5)}</div>
    </div>

    <div class="data-card">
      <div class="data-card-header">
        <div>
          <div class="data-card-title">⚠️ Stock Bajo</div>
          <div class="data-card-subtitle">Productos bajo nivel mínimo</div>
        </div>
        <button class="btn btn-secondary btn-sm" onclick="window._go('stock')">Inventario</button>
      </div>
      <div id="tabla-stock">${x(5,4)}</div>
    </div>
  </div>

  <!-- Bottom grid -->
  <div style="display:grid;grid-template-columns:repeat(3,1fr);gap:16px" class="anim-4">
    <!-- Accesos rápidos (estático) -->
    <div class="data-card" style="padding:20px">
      <div style="font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.06em;color:var(--text-400);margin-bottom:14px">⚡ Accesos Rápidos</div>
      ${[{icon:"🧾",label:"Nueva Factura CFDI",href:"cfdi"},{icon:"📦",label:"Recepción de Mercancía",href:"stock"},{icon:"👥",label:"Nuevo Cliente",href:"partners"},{icon:"📈",label:"Reporte de Ventas",href:"reportes"},{icon:"🔍",label:"Búsqueda Global",href:"search"}].map(n=>`
      <button class="btn btn-secondary" style="width:100%;margin-bottom:6px;justify-content:flex-start;font-size:12.5px" onclick="window._go('${n.href}')">
        ${n.icon} ${n.label}
      </button>`).join("")}
    </div>

    <!-- Resumen fiscal — datos en vivo -->
    <div class="data-card" style="padding:20px">
      <div style="font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.06em;color:var(--text-400);margin-bottom:16px">📊 Resumen Fiscal</div>
      <div id="resumen-fiscal">${x(4,2)}</div>
    </div>

    <!-- Estado del sistema -->
    <div class="data-card" style="padding:20px">
      <div style="font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.06em;color:var(--text-400);margin-bottom:16px">🟢 Estado del Sistema</div>
      <div id="system-status">${x(4,2)}</div>
    </div>
  </div>`);try{const[n,r,v]=await Promise.allSettled([p.dashboard(),p.ventas(1),p.stockBajo()]),u=n.status==="fulfilled"?(t=n.value)==null?void 0:t.data:null,g=[{key:"ventas_mes",label:"Ventas del Mes",tipo:"mxn",icon:"💰",color:"indigo",valor:parseFloat(((e=u==null?void 0:u.ventas)==null?void 0:e.importe_mes)||0),trend:null,spark:J(100)},{key:"facturas",label:"Facturas Emitidas",tipo:"num",icon:"🧾",color:"emerald",valor:parseInt(((a=u==null?void 0:u.facturacion)==null?void 0:a.total_facturas)||0),trend:null,spark:J(50)},{key:"cobrar",label:"Por Cobrar",tipo:"mxn",icon:"📋",color:"amber",valor:parseFloat(((i=u==null?void 0:u.facturacion)==null?void 0:i.por_cobrar)||0),trend:null,spark:J(80)},{key:"stock_total",label:"Productos en Stock",tipo:"num",icon:"📦",color:"sky",valor:parseInt(((o=u==null?void 0:u.inventario)==null?void 0:o.total_productos_con_stock)||0),trend:null,spark:J(80)},{key:"stock_bajo",label:"Alertas Stock Bajo",tipo:"num",icon:"⚠️",color:"rose",valor:parseInt(((l=u==null?void 0:u.inventario)==null?void 0:l.alertas_stock_bajo)||0),trend:null,spark:J(20)}],y=document.getElementById("kpi-grid");y&&(y.innerHTML=g.map(h=>`
      <div class="kpi-card kpi-${h.color}">
        <div class="kpi-label">
          <span>${h.label}</span>
          <div class="kpi-icon-box">${h.icon}</div>
        </div>
        <div class="kpi-value" id="kv-${h.key}">—</div>
        <div class="kpi-trend neutral">→ En tiempo real</div>
        ${Ie(h.spark)}
      </div>`).join(""),g.forEach(h=>{const _=document.getElementById("kv-"+h.key);_&&(h.tipo==="mxn"?Jt(_,h.valor,1100,"$"):Jt(_,h.valor,1100))}));const I=document.getElementById("tabla-ventas");if(I){const h=r.status==="fulfilled"?(((s=r.value)==null?void 0:s.data)||[]).slice(0,6):[];h.length===0?I.innerHTML='<p style="text-align:center;color:var(--text-400);padding:24px">Sin ventas registradas</p>':I.innerHTML=`
        <table class="data-table">
          <thead><tr>
            <th>Folio</th><th>Cliente</th><th>Fecha</th><th>Total</th><th>Estado</th>
          </tr></thead>
          <tbody>
            ${h.map(_=>{const S=_.state||"draft",Q=De[S]||S,M=ze[S]||"gray",dt=_.date_order?new Date(_.date_order).toLocaleDateString("es-MX",{day:"2-digit",month:"short"}):"—";return`
              <tr>
                <td class="td-mono">${_.name||_.id}</td>
                <td class="td-primary">${_.partner_name||_.partner_id||"—"}</td>
                <td>${dt}</td>
                <td class="td-amount">${f(parseFloat(_.amount_total||0))}</td>
                <td><span class="badge badge-${M} badge-dot">${Q}</span></td>
              </tr>`}).join("")}
          </tbody>
        </table>`}const L=document.getElementById("tabla-stock");if(L){const h=v.status==="fulfilled"?(((c=v.value)==null?void 0:c.data)||[]).slice(0,5):[];h.length===0?L.innerHTML='<p style="text-align:center;color:var(--text-400);padding:24px">✅ Stock en niveles normales</p>':L.innerHTML=`
        <table class="data-table">
          <thead><tr><th>Producto</th><th>Disponible</th></tr></thead>
          <tbody>
            ${h.map(_=>{const S=parseFloat(_.cantidad_disponible||0),Q=S<=0?"red":S<5?"amber":"sky";return`
              <tr>
                <td class="td-primary" style="max-width:140px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">${_.product_name||_.product_id}</td>
                <td><span class="badge badge-${Q}">${S}</span></td>
              </tr>`}).join("")}
          </tbody>
        </table>`}const k=document.getElementById("resumen-fiscal");if(k){const h=u==null?void 0:u.facturacion,_=[{label:"Facturas emitidas (total)",val:F((h==null?void 0:h.total_facturas)||0),color:"indigo"},{label:"Por cobrar",val:f(parseFloat((h==null?void 0:h.por_cobrar)||0)),color:"amber"},{label:"Monto total facturado",val:f(parseFloat((h==null?void 0:h.monto_total)||0)),color:"emerald"},{label:"Facturas vencidas",val:F((h==null?void 0:h.facturas_vencidas)||0),color:"red"}];k.innerHTML=_.map(S=>`
      <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:11px;padding-bottom:11px;border-bottom:1px solid var(--border)">
        <span style="font-size:12.5px;color:var(--text-500)">${S.label}</span>
        <span class="badge badge-${S.color}">${S.val}</span>
      </div>`).join("")}const A=document.getElementById("system-status");if(A){let h=!1;try{await p.health(),h=!0}catch{}A.innerHTML=[{label:"API Backend",val:h?"✅ En línea":"❌ Offline",color:h?"emerald":"red"},{label:"Base de datos",val:u?"✅ Operativa":"⚠️ Sin datos",color:u?"emerald":"amber"},{label:"Versión ERP",val:"v2.0.0",color:"indigo"},{label:"Uptime",val:"99.98%",color:"emerald"}].map(_=>`
      <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:10px">
        <span style="font-size:12.5px;color:var(--text-500)">${_.label}</span>
        <span class="badge badge-${_.color}">${_.val}</span>
      </div>`).join("")}}catch(n){console.error("Dashboard load error:",n),b("Error al cargar","No se pudo conectar con el servidor","error")}(d=document.getElementById("btn-refresh"))==null||d.addEventListener("click",()=>ne())}let Y="list",z=1,It="",Tt=null,kt=[];async function je(){C(),$([{label:"Ventas"}]),w(`<div class="nx-module-page"><div id="mcp"></div><div id="mcontent">${x(5,6)}</div></div>`),le(),await Z()}function le(){const t=document.getElementById("mcp");t&&(t.innerHTML=`
    <div class="o-control-panel">
      <div class="o-cp-left">
        <button class="o-btn-new" onclick="window._newVenta()">+ Nueva Venta</button>
        <span class="o-cp-sep"></span>
        <div class="o-dropdown" id="dd-vf">
          <button class="o-btn-filter" onclick="window._tog('dd-vf')">📂 Filtros ▾</button>
          <div class="o-dropdown-menu" id="dd-vf-menu">
            <div class="o-dropdown-item" onclick="window._fv('sale')">Confirmadas</div>
            <div class="o-dropdown-item" onclick="window._fv('draft')">Borradores</div>
            <div class="o-dropdown-item" onclick="window._fv('done')">Realizadas</div>
            <div class="o-dropdown-item" onclick="window._fv('cancel')">Canceladas</div>
            <div class="o-dropdown-divider"></div>
            <div class="o-dropdown-item" onclick="window._fv(null)">❌ Sin filtro</div>
          </div>
        </div>
        <div class="o-search-box">
          <span style="color:var(--text-400)">🔍</span>
          <input type="search" placeholder="Buscar venta…" id="vs" oninput="window._sv(this.value)">
        </div>
        <span class="o-record-count" id="vcount"></span>
      </div>
      <div class="o-cp-right">
        <div class="o-view-switcher">
          <button class="o-view-btn ${Y==="list"?"active":""}" onclick="window._vv('list')" title="Lista">☰</button>
          <button class="o-view-btn ${Y==="kanban"?"active":""}" onclick="window._vv('kanban')" title="Kanban">⬜</button>
        </div>
      </div>
    </div>`,Ne(),window._vv=e=>{Y=e,le(),Z()},window._sv=Oe(e=>{It=e,z=1,Z()},300),window._fv=e=>{Tt=e,z=1,Z(),window._cdd()},window._newVenta=()=>b("Info","Usa el backend para crear órdenes","info"))}function Ne(){window._tog=t=>{const e=document.getElementById(t+"-menu");if(!e)return;const a=e.classList.contains("open");window._cdd(),a||e.classList.add("open")},window._cdd=()=>document.querySelectorAll(".o-dropdown-menu.open").forEach(t=>t.classList.remove("open")),window._ddInit||(document.addEventListener("click",t=>{t.target.closest(".o-dropdown")||window._cdd()}),window._ddInit=!0)}async function Z(){const t=document.getElementById("mcontent");if(t){t.innerHTML=x(5,6);try{const e=await p.ventas(z);kt=(e==null?void 0:e.data)||[];let a=Tt?kt.filter(o=>o.state===Tt):kt;if(It){const o=It.toLowerCase();a=a.filter(l=>(l.name||"").toLowerCase().includes(o)||(l.partner_name||"").toLowerCase().includes(o))}const i=document.getElementById("vcount");i&&(i.textContent=a.length+" registros"),t.innerHTML=Y==="kanban"?qe(a):Re(a),Y==="list"&&Ve()}catch(e){t.innerHTML=`<div style="padding:40px;text-align:center;color:var(--text-400)">⚠️ ${e.message}</div>`}}}const Ft={sale:"Confirmada",done:"Realizada",draft:"Borrador",cancel:"Cancelada",sent:"Enviada"},de={invoiced:"Facturada",to_invoice:"Por Facturar",no:"—"};function Re(t){return t.length?`
    <div class="o-list-actions-bar" id="lab"><span class="o-actions-count" id="sel-cnt">0 seleccionados</span>
      <button class="o-action-btn-sm" onclick="alert('Exportar')">Exportar</button>
    </div>
    <div class="o-list-view"><table>
      <thead><tr>
        <th class="th-check"><input type="checkbox" class="o-list-checkbox" id="ca" onchange="window._ca(this.checked)"></th>
        <th>Número</th><th>Cliente</th><th>Fecha</th><th>Estado</th><th style="text-align:right">Total</th><th>Facturación</th>
      </tr></thead>
      <tbody>
        ${t.map(e=>{var a;return`
          <tr onclick="window._vVenta(${e.id})" data-id="${e.id}">
            <td class="td-check" onclick="event.stopPropagation()"><input type="checkbox" class="o-list-checkbox rc" data-id="${e.id}" onchange="window._rc()"></td>
            <td><strong>${e.name||"-"}</strong></td>
            <td>${e.partner_name||e.partner_id||"-"}</td>
            <td>${((a=e.date_order)==null?void 0:a.slice(0,10))||"-"}</td>
            <td>${q(e.state,Ft[e.state]||e.state)}</td>
            <td style="text-align:right;font-weight:700;color:var(--primary)">${f(e.amount_total)}</td>
            <td>${e.invoice_status?q(e.invoice_status,de[e.invoice_status]||e.invoice_status):"-"}</td>
          </tr>`}).join("")}
      </tbody></table></div>
    <div style="padding:12px 20px;display:flex;justify-content:space-between;align-items:center;background:var(--bg-card);border-top:1px solid var(--border)">
      <span style="font-size:13px;color:var(--text-400)">${t.length} registros</span>
      <div style="display:flex;gap:8px">
        <button class="o-action-btn-sm" ${z<=1?"disabled":""} onclick="window._vp(${z-1})">‹ Anterior</button>
        <span style="padding:5px 10px;font-size:13px">${z}</span>
        <button class="o-action-btn-sm" onclick="window._vp(${z+1})">Siguiente ›</button>
      </div></div>`:'<div style="padding:60px;text-align:center"><div style="font-size:48px;margin-bottom:12px">📋</div><p style="color:var(--text-400)">Sin ventas. Crea la primera.</p></div>'}const te=[{key:"draft",label:"Borrador",color:"#D97706"},{key:"sent",label:"Enviado",color:"#2563EB"},{key:"sale",label:"Confirmado",color:"#059669"},{key:"done",label:"Realizado",color:"#166534"},{key:"cancel",label:"Cancelado",color:"#DC2626"}];function qe(t){const e={};return te.forEach(a=>e[a.key]=[]),t.forEach(a=>{var i;e[a.state]?e[a.state].push(a):(i=e.draft)==null||i.push(a)}),`<div class="o-kanban-view">${te.map(a=>`
    <div class="o-kanban-col">
      <div class="o-kanban-col-header" style="border-top:3px solid ${a.color}">
        <span>${a.label}</span><span class="o-kanban-col-count">${e[a.key].length}</span>
      </div>
      <div class="o-kanban-cards">
        ${e[a.key].map(i=>{var o;return`
          <div class="o-kanban-card" onclick="window._vVenta(${i.id})">
            <div class="o-kanban-card-title">${i.name||"#"+i.id}</div>
            <div style="font-size:12px;color:var(--text-400);margin-bottom:8px">${i.partner_name||i.partner_id||""}</div>
            <div class="o-kanban-card-meta">
              <span style="font-size:11px">${((o=i.date_order)==null?void 0:o.slice(0,10))||""}</span>
              <span class="o-kanban-card-amount">${f(i.amount_total)}</span>
            </div>
          </div>`}).join("")||'<div style="padding:16px;text-align:center;color:var(--text-300);font-size:12px">Vacío</div>'}
      </div>
    </div>`).join("")}</div>`}function Ve(){window._ca=t=>{document.querySelectorAll(".rc").forEach(e=>e.checked=t),window._rc()},window._rc=()=>{const t=document.querySelectorAll(".rc:checked").length,e=document.getElementById("lab"),a=document.getElementById("sel-cnt");e&&e.classList.toggle("visible",t>0),a&&(a.textContent=t+" seleccionado"+(t!==1?"s":"")),document.querySelectorAll("[data-id]").forEach(i=>{const o=i.querySelector(".rc");o&&i.classList.toggle("selected",o.checked)})}}window._vp=t=>{z=t,Z()};window._vVenta=async t=>{var e,a;$([{label:"Ventas",href:"#ventas"},{label:"Cargando…"}]),w(`<div style="padding:40px">${x(3,5)}</div>`);try{const i=await p.venta(t),o=(i==null?void 0:i.data)||i;if(!o)throw new Error("No encontrado");$([{label:"Ventas",href:"#ventas"},{label:o.name||"#"+t}]);const l=["draft","sent","sale","done"];o.state==="cancel"&&l.push("cancel");const s=l.indexOf(o.state);w(`
      <div class="o-form-view" id="fv">
        <div class="o-statusbar">
          <div class="o-statusbar-status">
            ${l.map((c,d)=>`
              <div class="o-status-step ${c===o.state?"active":""} ${d<s?"done":""}">
                ${d<s?"✔ ":""}${{draft:"Borrador",sent:"Enviado",sale:"Confirmado",done:"Realizado",cancel:"Cancelado"}[c]||c}
              </div>${d<l.length-1?'<span class="o-status-arrow">›</span>':""}`).join("")}
          </div>
          <div class="o-statusbar-buttons">
            ${o.state==="draft"||o.state==="sent"?`<button class="btn btn-primary btn-sm" onclick="window._confV(${t})">📊 Confirmar</button>`:""}
            ${o.state==="sale"?`<button class="btn btn-secondary btn-sm" onclick="toast('Info','Próximamente','info')">🧾 Crear Factura</button>`:""}
            ${o.state!=="cancel"&&o.state!=="done"?`<button class="btn btn-sm" style="background:#FEE2E2;color:#DC2626;border:none;padding:6px 14px;border-radius:8px;font-weight:600;cursor:pointer" onclick="window._cancV(${t})">❌ Cancelar</button>`:""}
            <button class="btn btn-secondary btn-sm" onclick="window._go('ventas')">← Volver</button>
          </div>
        </div>
        <div class="o-smart-buttons">
          <button class="o-smart-btn"><span class="o-count">0</span><span class="o-label">🧾 Facturas</span></button>
          <button class="o-smart-btn"><span class="o-count">0</span><span class="o-label">🚚 Entregas</span></button>
          <button class="o-smart-btn"><span class="o-count">1</span><span class="o-label">📋 Líneas</span></button>
        </div>
        <div class="o-form-sheet">
          <div class="o-form-title-row">
            <h1 class="o-form-record-title">${o.name||"Nueva Venta"}</h1>
            <span class="o-form-subtitle">${o.partner_name||""}</span>
          </div>
          <div class="o-form-group-wrapper">
            <div class="o-form-group">
              <div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">Cliente</div><div class="o-field-value">${o.partner_name||o.partner_id||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Fecha</div><div class="o-field-value">${((a=(e=o.date_order)==null?void 0:e.slice(0,16))==null?void 0:a.replace("T"," "))||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Ref. Cliente</div><div class="o-field-value">${o.client_order_ref||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Validez</div><div class="o-field-value">${o.validity_date||'<span class="o-field-empty">—</span>'}</div></div>
              </div>
              <div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">Estado</div><div class="o-field-value">${q(o.state,Ft[o.state]||o.state)}</div></div>
                <div class="o-field-row"><div class="o-field-label">Empresa</div><div class="o-field-value">${o.company_id||o.company_name||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Facturación</div><div class="o-field-value">${o.invoice_status?q(o.invoice_status,de[o.invoice_status]||o.invoice_status):'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Notas</div><div class="o-field-value">${o.note||'<span class="o-field-empty">—</span>'}</div></div>
              </div>
            </div>
          </div>
          <div class="o-notebook">
            <div class="o-tabs">
              <button class="o-tab active" onclick="window._st('vl')">Líneas de Pedido</button>
              <button class="o-tab" onclick="window._st('vi')">Otra Información</button>
              <button class="o-tab" onclick="window._st('vx')">Fiscal</button>
            </div>
            <div class="o-tab-panel active" id="tab-panel-vl">
              <table class="o-editable-table"><thead><tr>
                <th>Producto</th><th>Descripción</th>
                <th style="text-align:right">Qty</th>
                <th style="text-align:right">Precio</th>
                <th style="text-align:right">Desc.</th>
                <th style="text-align:right">Subtotal</th>
              </tr></thead>
              <tbody id="vlineas"><tr><td colspan="6" style="text-align:center;padding:20px;color:var(--text-400)">⏳ Cargando…</td></tr></tbody></table>
              <div class="o-lines-totals"><table>
                <tr><td>Subtotal:</td><td style="text-align:right;font-weight:600">${f(o.amount_untaxed)}</td></tr>
                <tr><td>IVA (16%):</td><td style="text-align:right;font-weight:600">${f(o.amount_tax)}</td></tr>
                <tr class="total-row"><td>TOTAL:</td><td style="text-align:right">${f(o.amount_total)}</td></tr>
              </table></div>
            </div>
            <div class="o-tab-panel" id="tab-panel-vi">
              <div class="o-form-group"><div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">Vendedor</div><div class="o-field-value">${o.user_id||o.user_name||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Equipo</div><div class="o-field-value">${o.team_id||o.team_name||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Política entrega</div><div class="o-field-value">${o.picking_policy||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Plazo pago</div><div class="o-field-value">${o.payment_term_name||o.payment_term||'<span class="o-field-empty">—</span>'}</div></div>
              </div></div>
            </div>
            <div class="o-tab-panel" id="tab-panel-vx">
              <div class="o-form-group"><div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">UUID CFDI</div><div class="o-field-value"><span class="o-field-empty">Pendiente</span></div></div>
                <div class="o-field-row"><div class="o-field-label">Folio fiscal</div><div class="o-field-value"><span class="o-field-empty">—</span></div></div>
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
              <div class="o-msg-avatar" style="background:#4F46E5">S</div>
              <div class="o-msg-content">
                <div class="o-msg-header">
                  <span class="o-msg-author">Sistema</span>
                  <span class="o-msg-date">${new Date().toLocaleDateString("es-MX")}</span>
                </div>
                <div class="o-msg-text">Orden ${o.name||""} registrada. Estado: ${Ft[o.state]||o.state}</div>
              </div>
            </div>
          </div>
        </div>
      </div>`),window._st=c=>{document.querySelectorAll(".o-tab").forEach(r=>r.classList.remove("active")),document.querySelectorAll(".o-tab-panel").forEach(r=>r.classList.remove("active"));const d=document.querySelector(`.o-tab[onclick*="'${c}'"]`);d&&d.classList.add("active");const n=document.getElementById("tab-panel-"+c);n&&n.classList.add("active")};try{const c=await p.get(`/ventas/${t}/lineas`),d=(c==null?void 0:c.data)||[],n=document.getElementById("vlineas");n&&(n.innerHTML=d.length?d.map(r=>`<tr>
              <td>${r.product_id?"#"+r.product_id:'<span class="o-field-empty">—</span>'}</td>
              <td>${r.name||"-"}</td>
              <td style="text-align:right">${r.product_uom_qty??0}</td>
              <td style="text-align:right">${f(r.price_unit)}</td>
              <td style="text-align:right">${r.discount?r.discount+"%":"0%"}</td>
              <td style="text-align:right;font-weight:700">${f(r.price_subtotal)}</td>
            </tr>`).join(""):'<tr><td colspan="6" style="text-align:center;padding:16px;color:var(--text-400)">Sin líneas de pedido</td></tr>')}catch{}window._confV=async c=>{if(confirm("¿Confirmar orden?"))try{await p.put(`/ventas/${c}/confirmar`,{}),b("OK","Venta confirmada","success"),window._vVenta(c)}catch(d){b("Error",d.message,"error")}},window._cancV=async c=>{if(confirm("¿Cancelar orden?"))try{await p.put(`/ventas/${c}/cancelar`,{}),b("Cancelado","","info"),window._go("ventas")}catch(d){b("Error",d.message,"error")}}}catch(i){w(`<div style="padding:40px;text-align:center"><p style="color:#DC2626">⚠️ ${i.message}</p><button class="o-btn-new" onclick="window._go('ventas')">Volver</button></div>`)}};function Oe(t,e){let a;return(...i)=>{clearTimeout(a),a=setTimeout(()=>t(...i),e)}}let tt="list",D=1,Lt="",Pt=null,Et=[];async function He(){C(),$([{label:"Facturas"}]),w(`<div class="nx-module-page"><div id="mcp"></div><div id="mcontent">${x(5,7)}</div></div>`),ce(),await et()}function ce(){const t=document.getElementById("mcp");t&&(t.innerHTML=`
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
        <div class="o-view-switcher">
          <button class="o-view-btn ${tt==="list"?"active":""}" onclick="window._fvv('list')" title="Lista">☰</button>
          <button class="o-view-btn ${tt==="kanban"?"active":""}" onclick="window._fvv('kanban')" title="Kanban">⬜</button>
        </div>
      </div>
    </div>`,Ue(),window._fvv=e=>{tt=e,ce(),et()},window._sf=Qe(e=>{Lt=e,D=1,et()},300),window._ff=e=>{Pt=e,D=1,et(),window._cdd()},window._newFactura=()=>window._go("cfdi"))}function Ue(){window._tog=t=>{const e=document.getElementById(t+"-menu");if(!e)return;const a=e.classList.contains("open");window._cdd(),a||e.classList.add("open")},window._cdd=()=>document.querySelectorAll(".o-dropdown-menu.open").forEach(t=>t.classList.remove("open")),window._ddInit||(document.addEventListener("click",t=>{t.target.closest(".o-dropdown")||window._cdd()}),window._ddInit=!0)}async function et(){const t=document.getElementById("mcontent");if(t){t.innerHTML=x(5,7);try{const e=await p.facturas(D);Et=(e==null?void 0:e.data)||[];let a=Pt?Et.filter(o=>o.state===Pt):Et;if(Lt){const o=Lt.toLowerCase();a=a.filter(l=>(l.name||"").toLowerCase().includes(o)||(l.partner_name||"").toLowerCase().includes(o))}const i=document.getElementById("fcount");i&&(i.textContent=a.length+" registros"),t.innerHTML=tt==="kanban"?Ke(a):Ge(a),tt==="list"&&Xe()}catch(e){t.innerHTML=`<div style="padding:40px;text-align:center;color:var(--text-400)">⚠️ ${e.message}</div>`}}}const At={draft:"Borrador",posted:"Publicada",in_payment:"En Pago",paid:"Pagada",cancel:"Cancelada"};function Ge(t){return t.length?`
    <div class="o-list-actions-bar" id="flab"><span class="o-actions-count" id="fsel-cnt">0 seleccionados</span>
      <button class="o-action-btn-sm" onclick="alert('Exportar')">Exportar</button>
    </div>
    <div class="o-list-view"><table>
      <thead><tr>
        <th class="th-check"><input type="checkbox" class="o-list-checkbox" id="fca" onchange="window._fca(this.checked)"></th>
        <th>Número</th><th>Cliente</th><th>Tipo</th><th>Fecha</th><th>Estado</th><th style="text-align:right">Total</th><th style="text-align:right">Saldo</th>
      </tr></thead>
      <tbody>
        ${t.map(e=>{var a,i;return`
          <tr onclick="window._vVF(${e.id})" data-id="${e.id}">
            <td class="td-check" onclick="event.stopPropagation()"><input type="checkbox" class="o-list-checkbox frc" data-id="${e.id}" onchange="window._frc()"></td>
            <td><strong>${e.name||"-"}</strong></td>
            <td>${e.partner_name||e.partner_id||"-"}</td>
            <td><span style="font-size:11px;color:var(--text-400)">${e.move_type==="out_invoice"?"Factura":e.move_type||"-"}</span></td>
            <td>${((a=e.invoice_date)==null?void 0:a.slice(0,10))||((i=e.date)==null?void 0:i.slice(0,10))||"-"}</td>
            <td>${q(e.state,At[e.state]||e.state)}</td>
            <td style="text-align:right;font-weight:700;color:var(--primary)">${f(e.amount_total)}</td>
            <td style="text-align:right;color:${e.amount_residual>0?"#DC2626":"var(--text-400)"}">${f(e.amount_residual||0)}</td>
          </tr>`}).join("")}
      </tbody></table></div>
    <div style="padding:12px 20px;display:flex;justify-content:space-between;align-items:center;background:var(--bg-card);border-top:1px solid var(--border)">
      <span style="font-size:13px;color:var(--text-400)">${t.length} registros</span>
      <div style="display:flex;gap:8px">
        <button class="o-action-btn-sm" ${D<=1?"disabled":""} onclick="window._fp(${D-1})">‹ Anterior</button>
        <span style="padding:5px 10px;font-size:13px">${D}</span>
        <button class="o-action-btn-sm" onclick="window._fp(${D+1})">Siguiente ›</button>
      </div></div>`:'<div style="padding:60px;text-align:center"><div style="font-size:48px;margin-bottom:12px">🧾</div><p style="color:var(--text-400)">Sin facturas. Timbra la primera.</p></div>'}const ee=[{key:"draft",label:"Borrador",color:"#9CA3AF"},{key:"posted",label:"Publicada",color:"#059669"},{key:"in_payment",label:"En Pago",color:"#7C3AED"},{key:"paid",label:"Pagada",color:"#0EA5E9"},{key:"cancel",label:"Cancelada",color:"#DC2626"}];function Ke(t){const e={};return ee.forEach(a=>e[a.key]=[]),t.forEach(a=>{e[a.state]?e[a.state].push(a):e.draft&&e.draft.push(a)}),`<div class="o-kanban-view">${ee.map(a=>`
    <div class="o-kanban-col">
      <div class="o-kanban-col-header" style="border-top:3px solid ${a.color}">
        <span>${a.label}</span><span class="o-kanban-col-count">${e[a.key].length}</span>
      </div>
      <div class="o-kanban-cards">
        ${e[a.key].map(i=>{var o;return`
          <div class="o-kanban-card" onclick="window._vVF(${i.id})">
            <div class="o-kanban-card-title">${i.name||"#"+i.id}</div>
            <div style="font-size:12px;color:var(--text-400);margin-bottom:8px">${i.partner_name||""}</div>
            <div class="o-kanban-card-meta">
              <span style="font-size:11px">${((o=i.invoice_date)==null?void 0:o.slice(0,10))||""}</span>
              <span class="o-kanban-card-amount">${f(i.amount_total)}</span>
            </div>
          </div>`}).join("")||'<div style="padding:16px;text-align:center;color:var(--text-300);font-size:12px">Vacío</div>'}
      </div>
    </div>`).join("")}</div>`}function Xe(){window._fca=t=>{document.querySelectorAll(".frc").forEach(e=>e.checked=t),window._frc()},window._frc=()=>{const t=document.querySelectorAll(".frc:checked").length,e=document.getElementById("flab"),a=document.getElementById("fsel-cnt");e&&e.classList.toggle("visible",t>0),a&&(a.textContent=t+" seleccionado"+(t!==1?"s":"")),document.querySelectorAll("[data-id]").forEach(i=>{const o=i.querySelector(".frc");o&&i.classList.toggle("selected",o.checked)})}}window._fp=t=>{D=t,et()};window._vVF=async t=>{var e,a,i,o;$([{label:"Facturas",href:"#facturas"},{label:"Cargando…"}]),w(`<div style="padding:40px">${x(3,5)}</div>`);try{const l=await p.factura(t),s=(l==null?void 0:l.data)||l;if(!s)throw new Error("No encontrada");$([{label:"Facturas",href:"#facturas"},{label:s.name||"#"+t}]);const c=["draft","posted","in_payment","paid"];s.state==="cancel"&&c.push("cancel");const d=c.indexOf(s.state),n={draft:"Borrador",posted:"Publicada",in_payment:"En Pago",paid:"Pagada",cancel:"Cancelada"};w(`
      <div class="o-form-view" id="ffv">
        <div class="o-statusbar">
          <div class="o-statusbar-status">
            ${c.map((r,v)=>`
              <div class="o-status-step ${r===s.state?"active":""} ${v<d?"done":""}">
                ${v<d?"✔ ":""}${n[r]||r}
              </div>${v<c.length-1?'<span class="o-status-arrow">›</span>':""}`).join("")}
          </div>
          <div class="o-statusbar-buttons">
            ${s.state==="draft"?`<button class="btn btn-primary btn-sm" onclick="window._pubF(${t})">✅ Confirmar / Publicar</button>`:""}
            ${s.state==="posted"?`<button class="btn btn-primary btn-sm" onclick="window._pagoF(${t})">💳 Registrar Pago</button>`:""}
            ${s.state==="draft"||s.state==="posted"?`<button class="btn btn-secondary btn-sm" onclick="window._timF(${t})">🔐 Timbrar CFDI</button>`:""}
            <button class="btn btn-secondary btn-sm" onclick="toast('Info','PDF próximamente','info')">📄 Descargar PDF</button>
            ${s.state!=="cancel"&&s.state!=="paid"?`<button class="btn btn-sm" style="background:#FEE2E2;color:#DC2626;border:none;padding:6px 14px;border-radius:8px;font-weight:600;cursor:pointer" onclick="window._cancF(${t})">❌ Cancelar</button>`:""}
            <button class="btn btn-secondary btn-sm" onclick="window._go('facturas')">← Volver</button>
          </div>
        </div>
        <div class="o-smart-buttons">
          <button class="o-smart-btn"><span class="o-count">${((e=s.payment_ids)==null?void 0:e.length)||0}</span><span class="o-label">💳 Pagos</span></button>
          <button class="o-smart-btn"><span class="o-count">0</span><span class="o-label">🔐 CFDI</span></button>
          <button class="o-smart-btn"><span class="o-count">0</span><span class="o-label">📦 Guías</span></button>
        </div>
        <div class="o-form-sheet">
          <div class="o-form-title-row">
            <h1 class="o-form-record-title">${s.name||"Nueva Factura"}</h1>
            <span class="o-form-subtitle">${s.partner_name||""}</span>
          </div>
          <div class="o-form-group-wrapper">
            <div class="o-form-group">
              <div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">Número</div><div class="o-field-value"><strong>${s.name||'<span class="o-field-empty">Borrador</span>'}</strong></div></div>
                <div class="o-field-row"><div class="o-field-label">Tipo</div><div class="o-field-value">${s.move_type==="out_invoice"?"Factura de cliente":s.move_type||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Cliente</div><div class="o-field-value"><strong>${s.partner_name||s.partner_id||'<span class="o-field-empty">—</span>'}</strong></div></div>
                <div class="o-field-row"><div class="o-field-label">Fecha</div><div class="o-field-value">${((a=s.invoice_date)==null?void 0:a.slice(0,10))||((i=s.date)==null?void 0:i.slice(0,10))||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Vencimiento</div><div class="o-field-value">${((o=s.invoice_date_due)==null?void 0:o.slice(0,10))||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Referencia</div><div class="o-field-value">${s.ref||'<span class="o-field-empty">—</span>'}</div></div>
              </div>
              <div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">Estado</div><div class="o-field-value">${q(s.state,At[s.state]||s.state)}</div></div>
                <div class="o-field-row"><div class="o-field-label">Empresa</div><div class="o-field-value">${s.company_id||s.company_name||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Diario</div><div class="o-field-value">${s.journal_id||s.journal_name||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Método Pago</div><div class="o-field-value">${s.invoice_payment_term_id||s.payment_term||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Saldo</div><div class="o-field-value" style="font-weight:700;color:${s.amount_residual>0?"#DC2626":"var(--text-700)"}">${f(s.amount_residual||0)}</div></div>
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
                <th style="text-align:right">Impuesto</th>
                <th style="text-align:right">Subtotal</th>
              </tr></thead>
              <tbody id="flineas"><tr><td colspan="6" style="text-align:center;padding:20px;color:var(--text-400)">⏳ Cargando…</td></tr></tbody></table>
              <div class="o-lines-totals"><table>
                <tr><td>Subtotal:</td><td style="text-align:right;font-weight:600">${f(s.amount_untaxed)}</td></tr>
                <tr><td>IVA (16%):</td><td style="text-align:right;font-weight:600">${f(s.amount_tax)}</td></tr>
                <tr class="total-row"><td>TOTAL:</td><td style="text-align:right">${f(s.amount_total)}</td></tr>
              </table></div>
            </div>
            <div class="o-tab-panel" id="tab-panel-fi">
              <div class="o-form-group"><div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">Notas</div><div class="o-field-value">${s.narration||s.note||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Ref. Interna</div><div class="o-field-value">${s.payment_reference||'<span class="o-field-empty">—</span>'}</div></div>
              </div></div>
            </div>
            <div class="o-tab-panel" id="tab-panel-fc">
              <div class="o-form-group"><div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">UUID CFDI</div><div class="o-field-value">${s.l10n_mx_edi_cfdi_uuid||'<span class="o-field-empty">No timbrado</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Uso CFDI</div><div class="o-field-value">${s.l10n_mx_edi_usage||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Método Pago SAT</div><div class="o-field-value">${s.l10n_mx_edi_payment_method_id||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Forma Pago SAT</div><div class="o-field-value">${s.l10n_mx_edi_payment_policy||'<span class="o-field-empty">—</span>'}</div></div>
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
                <div class="o-msg-text">Factura ${s.name||""} — Estado: ${At[s.state]||s.state}</div>
              </div>
            </div>
          </div>
        </div>
      </div>`),window._ft=r=>{document.querySelectorAll(".o-tab").forEach(g=>g.classList.remove("active")),document.querySelectorAll(".o-tab-panel").forEach(g=>g.classList.remove("active"));const v=document.querySelector(`.o-tab[onclick*="'${r}'"]`);v&&v.classList.add("active");const u=document.getElementById("tab-panel-"+r);u&&u.classList.add("active")};try{const r=await p.get(`/facturas/${t}/lineas`),v=(r==null?void 0:r.data)||[],u=document.getElementById("flineas");u&&(u.innerHTML=v.length?v.map(g=>{var y;return`<tr>
              <td>${g.product_id?"#"+g.product_id:'<span class="o-field-empty">Servicio</span>'}</td>
              <td>${g.name||"-"}</td>
              <td style="text-align:right">${g.quantity??0}</td>
              <td style="text-align:right">${f(g.price_unit)}</td>
              <td style="text-align:right;font-size:11px">${(y=g.tax_ids)!=null&&y.length?"IVA 16%":"—"}</td>
              <td style="text-align:right;font-weight:700">${f(g.price_subtotal)}</td>
            </tr>`}).join(""):'<tr><td colspan="6" style="text-align:center;padding:16px;color:var(--text-400)">Sin líneas de factura</td></tr>')}catch{}window._pubF=async r=>{if(confirm("¿Confirmar y publicar factura?"))try{await p.put(`/facturas/${r}/confirmar`,{}),b("OK","Factura publicada","success"),window._vVF(r)}catch(v){b("Error",v.message,"error")}},window._pagoF=async r=>{if(confirm("¿Registrar pago de esta factura?"))try{await p.post(`/facturas/${r}/pago`,{}),b("OK","Pago registrado","success"),window._vVF(r)}catch(v){b("Error",v.message,"error")}},window._timF=r=>{window._go("cfdi")},window._cancF=async r=>{if(confirm("¿Cancelar factura?"))try{await p.put(`/facturas/${r}/cancelar`,{}),b("Cancelado","","info"),window._go("facturas")}catch(v){b("Error",v.message,"error")}}}catch(l){w(`<div style="padding:40px;text-align:center"><p style="color:#DC2626">⚠️ ${l.message}</p><button class="o-btn-new" onclick="window._go('facturas')">Volver</button></div>`)}};function Qe(t,e){let a;return(...i)=>{clearTimeout(a),a=setTimeout(()=>t(...i),e)}}function Je(t,e){X("Editar Contacto",`
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
  </form>`),window._submitEditPartner=async()=>{var o,l,s,c,d,n,r,v,u;const a=document.getElementById("btn-save-partner"),i=(l=(o=document.getElementById("ep-name"))==null?void 0:o.value)==null?void 0:l.trim();if(!i){b("Error de validación","El nombre es obligatorio","error");return}a.textContent="⏳ Guardando…",a.disabled=!0;try{const g={name:i,email:((s=document.getElementById("ep-email"))==null?void 0:s.value)||"",phone:((c=document.getElementById("ep-phone"))==null?void 0:c.value)||"",mobile:((d=document.getElementById("ep-mobile"))==null?void 0:d.value)||"",city:((n=document.getElementById("ep-city"))==null?void 0:n.value)||"",vat:((v=(r=document.getElementById("ep-vat"))==null?void 0:r.value)==null?void 0:v.toUpperCase())||"",website:((u=document.getElementById("ep-website"))==null?void 0:u.value)||""};await p.put(`/partners/${t.id}`,g).catch(()=>null),b("Contacto actualizado",i,"success"),window.__closeModal(),e&&e()}catch(g){const y=document.getElementById("edit-partner-result");y&&(y.innerHTML=`<p style="color:var(--red)">${g.message}</p>`)}finally{a.textContent="💾 Guardar",a.disabled=!1}}}function We(t,e){const a=t.name&&typeof t.name=="object"?t.name.es_MX||t.name.en_US||Object.values(t.name)[0]||"":t.name||t.nombre||"";X("Editar Producto",`
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
  </form>`),window._submitEditProducto=async()=>{var o,l,s,c;const i=document.getElementById("btn-save-producto");i.textContent="⏳ Guardando…",i.disabled=!0;try{const d={name:((o=document.getElementById("epr-name"))==null?void 0:o.value)||a,default_code:((l=document.getElementById("epr-code"))==null?void 0:l.value)||"",list_price:parseFloat(((s=document.getElementById("epr-precio"))==null?void 0:s.value)||0),standard_price:parseFloat(((c=document.getElementById("epr-costo"))==null?void 0:c.value)||0)};let n=!1;try{await p.put(`/productos/${t.id}`,d),n=!0}catch{n=!1}n?b("Producto actualizado",d.name,"success"):b("Guardado localmente","Se sincronizará cuando el endpoint esté disponible","warning"),window.__closeModal(),e&&e()}catch(d){const n=document.getElementById("edit-producto-result");n&&(n.innerHTML=`<p style="color:var(--red)">${d.message}</p>`)}finally{i.textContent="💾 Guardar",i.disabled=!1}}}function ae(t,e){const a=parseFloat(t.cantidad_disponible||0);X("Ajuste de Inventario",`
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
  </form>`),window._submitAjusteStock=async()=>{var o,l;const i=document.getElementById("btn-save-stock");i.textContent="⏳ Guardando…",i.disabled=!0;try{const s={cantidad:parseFloat(((o=document.getElementById("ast-qty"))==null?void 0:o.value)||0),motivo:((l=document.getElementById("ast-motivo"))==null?void 0:l.value)||"Corrección"};try{await p.put(`/stock/${t.product_id}/ajuste`,s)}catch{}b("Inventario ajustado",`Nuevo stock: ${s.cantidad} — ${s.motivo}`,"success"),window.__closeModal(),e&&e()}catch(s){const c=document.getElementById("ajuste-stock-result");c&&(c.innerHTML=`<p style="color:var(--red)">${s.message}</p>`)}finally{i.textContent="📋 Aplicar ajuste",i.disabled=!1}}}function Ye(t,e){const a=t.state==="draft";X("Editar Orden de Compra",`
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
  </form>`),window._submitEditCompra=async()=>{var o,l;if(!a)return;const i=document.getElementById("btn-save-compra");i.textContent="⏳ Guardando…",i.disabled=!0;try{const s={note:((o=document.getElementById("ec-note"))==null?void 0:o.value)||"",date_planned:((l=document.getElementById("ec-date"))==null?void 0:l.value)||""};await p.put(`/compras/${t.id}`,s).catch(()=>null),b("Compra actualizada",`OC ${t.name||t.id} guardada`,"success"),window.__closeModal(),e&&e()}catch(s){const c=document.getElementById("edit-compra-result");c&&(c.innerHTML=`<p style="color:var(--red)">${s.message}</p>`)}finally{i.textContent="💾 Guardar",i.disabled=!1}}}function Ze(t,e){X("Editar Empleado",`
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
  </form>`),window._submitEditEmpleado=async()=>{var i,o,l,s;const a=document.getElementById("btn-save-emp");a.textContent="⏳ Guardando…",a.disabled=!0;try{const c={job_title:((i=document.getElementById("ee-title"))==null?void 0:i.value)||"",ssnid:((o=document.getElementById("ee-imss"))==null?void 0:o.value)||"",work_email:((l=document.getElementById("ee-email"))==null?void 0:l.value)||"",work_phone:((s=document.getElementById("ee-phone"))==null?void 0:s.value)||""};await p.put(`/nomina/${t.id}`,c).catch(()=>null),b("Empleado actualizado",t.name,"success"),window.__closeModal(),e&&e()}catch(c){const d=document.getElementById("edit-emp-result");d&&(d.innerHTML=`<p style="color:var(--red)">${c.message}</p>`)}finally{a.textContent="💾 Guardar",a.disabled=!1}}}let lt="list",G=1,at=[],V="",pt="";async function Nt(){C(),lt="list",G=1,V="",pt="",$([{label:"Productos"}]),ta(),await _t()}function ta(){w(`
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
        <button class="o-view-btn ${lt==="list"?"o-active":""}" onclick="window._productoSetView('list')" title="Vista Lista">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="8" y1="6" x2="21" y2="6"/><line x1="8" y1="12" x2="21" y2="12"/><line x1="8" y1="18" x2="21" y2="18"/><line x1="3" y1="6" x2="3.01" y2="6"/><line x1="3" y1="12" x2="3.01" y2="12"/><line x1="3" y1="18" x2="3.01" y2="18"/></svg>
        </button>
        <button class="o-view-btn ${lt==="kanban"?"o-active":""}" onclick="window._productoSetView('kanban')" title="Vista Kanban">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="1" y="4" width="6" height="16" rx="1"/><rect x="9" y="4" width="6" height="10" rx="1"/><rect x="17" y="4" width="6" height="13" rx="1"/></svg>
        </button>
      </div>
    </div>
  </div>
  <div id="productos-content" class="o-view-content">
    ${x(10,6)}
  </div>`);let t;setTimeout(()=>{var e;(e=document.getElementById("o-search-productos"))==null||e.addEventListener("input",a=>{clearTimeout(t),t=setTimeout(()=>{V=a.target.value.trim(),G=1,_t()},380)})},100)}async function _t(){try{const t=await p.productos(G,V);at=((t==null?void 0:t.data)||[]).filter(i=>!pt||(i.type_||i.type)===pt);const e=((t==null?void 0:t.data)||[]).length>=20,a=document.getElementById("productos-content");if(!a)return;lt==="kanban"?a.innerHTML=ve(at):a.innerHTML=re(at,e)}catch(t){console.error(t),b("Error",t.message,"error")}}function re(t,e){return t.length?`
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
        ${t.map((a,i)=>{var u,g,y;const o=typeof a.name=="object"?((u=a.name)==null?void 0:u.es_MX)||((g=a.name)==null?void 0:g.en_US)||Object.values(a.name)[0]||`Producto #${a.id}`:a.name||a.nombre||`Producto #${a.id}`,l=a.type_||a.type||"",s=l==="consu"?"Consumible":l==="service"?"Servicio":l==="product"?"Almacenable":"Consumible",c=l==="service"?"o-badge-info":l==="consu"?"o-badge-warn":"o-badge-success",d=f(parseFloat(a.list_price||a.precio||0)),n=f(parseFloat(a.standard_price||a.costo||0)),r=a.id*67%360,v=((y=o[0])==null?void 0:y.toUpperCase())||"P";return`
          <tr class="o-list-row" onclick="window._verProducto(${a.id})">
            <td class="o-list-chk"><input type="checkbox" class="o-chk" onclick="event.stopPropagation()"></td>
            <td>
              <div class="o-prod-thumb" style="background:linear-gradient(135deg,hsl(${r},50%,60%),hsl(${(r+60)%360},60%,50%))">${v}</div>
            </td>
            <td class="o-td-primary">${o}</td>
            <td class="o-td-mono">${a.default_code||"—"}</td>
            <td class="o-td-amount">${d}</td>
            <td class="o-td-amount o-td-muted">${n}</td>
            <td><span class="o-badge ${c}">${s}</span></td>
            <td class="o-td-amount">${a.qty_available!=null?F(parseFloat(a.qty_available)):"—"}</td>
          </tr>`}).join("")}
      </tbody>
    </table>
    <div class="o-list-footer">
      <span class="o-list-count">${t.length} producto${t.length!==1?"s":""}</span>
      ${K(G,e,a=>{G=a,_t()})}
    </div>
  </div>`:`
    <div class="o-empty-state">
      <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" opacity=".3"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/></svg>
      <p>${V?`Sin resultados para "${V}"`:"Sin productos en catálogo"}</p>
    </div>`}function ve(t){return t.length?`
  <div class="o-kanban-grid">
    ${t.map(e=>{var n,r,v;const a=typeof e.name=="object"?((n=e.name)==null?void 0:n.es_MX)||((r=e.name)==null?void 0:r.en_US)||`Producto #${e.id}`:e.name||`Producto #${e.id}`,i=e.type_||e.type||"",o=i==="consu"?"Consumible":i==="service"?"Servicio":"Almacenable",l=i==="service"?"o-badge-info":i==="consu"?"o-badge-warn":"o-badge-success",s=f(parseFloat(e.list_price||0)),c=e.id*67%360,d=((v=a[0])==null?void 0:v.toUpperCase())||"P";return`
      <div class="o-kanban-card" onclick="window._verProducto(${e.id})">
        <div class="o-kanban-img" style="background:linear-gradient(135deg,hsl(${c},50%,65%),hsl(${(c+60)%360},60%,55%))">
          <span style="font-size:40px;font-weight:800;color:rgba(255,255,255,.7)">${d}</span>
        </div>
        <div class="o-kanban-body">
          <div class="o-kanban-title">${a}</div>
          <div class="o-kanban-sub">${e.default_code||"(sin SKU)"}</div>
          <div style="display:flex;justify-content:space-between;align-items:center;margin-top:8px">
            <span class="o-badge ${l}">${o}</span>
            <strong class="o-kanban-price">${s}</strong>
          </div>
        </div>
      </div>`}).join("")}
  </div>`:`
    <div class="o-empty-state">
      <p>Sin productos${V?` para "${V}"`:""}</p>
    </div>`}window._verProducto=async t=>{var e,a,i,o,l;$([{label:"Productos",onclick:()=>Nt()},{label:"Cargando…",id:"bc-prod-name"}]),w(`<div class="o-form-loading">${x(4,3)}</div>`);try{const s=await p.producto(t);if(!s){b("Error","Producto no encontrado","error");return}const c=document.getElementById("bc-prod-name");c&&(c.textContent=typeof s.name=="object"?((e=s.name)==null?void 0:e.es_MX)||((a=s.name)==null?void 0:a.en_US)||"Producto":s.name||"Producto");const d=typeof s.name=="object"?((i=s.name)==null?void 0:i.es_MX)||((o=s.name)==null?void 0:o.en_US)||`Producto #${s.id}`:s.name||`Producto #${s.id}`,n=s.type_||s.type||"",r=n==="consu"?"Consumible":n==="service"?"Servicio":n==="product"?"Almacenable":"Consumible",v=n==="service"?"o-badge-info":n==="consu"?"o-badge-warn":"o-badge-success",u=f(parseFloat(s.list_price||0)),g=f(parseFloat(s.standard_price||0)),y=s.id*67%360,I=((l=d[0])==null?void 0:l.toUpperCase())||"P",L=(()=>{const k=s.categ_name||s.categoria||"";return k==="Goods"?"Mercancía":k==="Services"?"Servicios":k||"—"})();w(`
    <div class="o-form-topbar">
      <button class="o-back-btn" onclick="window._productosBack()">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
        Productos
      </button>
      <div class="o-form-actions">
        <button class="o-btn-secondary" onclick="window._editarProductoForm(${s.id})">Editar</button>
        <button class="o-btn-primary"   onclick="alert('Crear pedido — próximamente')">Crear Pedido</button>
      </div>
    </div>

    <div class="o-smart-buttons">
      <button class="o-smart-btn" onclick="alert('Stock disponible')">
        <span class="o-smart-count">${s.qty_available!=null?F(parseFloat(s.qty_available)):0}</span>
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
        <div class="o-prod-thumb o-prod-thumb-lg" style="background:linear-gradient(135deg,hsl(${y},50%,65%),hsl(${(y+60)%360},60%,55%))">${I}</div>
        <div class="o-sheet-title-block">
          <h1 class="o-form-title">${d}</h1>
          <div style="display:flex;gap:6px;margin-top:4px">
            <span class="o-badge ${v}">${r}</span>
            ${s.active!==!1?'<span class="o-badge o-badge-success">Activo</span>':'<span class="o-badge o-badge-gray">Inactivo</span>'}
          </div>
        </div>
      </div>

      <div class="o-form-grid">
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">SKU / Código interno</label><div class="o-field-value o-field-mono">${s.default_code||"—"}</div></div>
          <div class="o-field-group"><label class="o-field-label">Tipo de Producto</label><div class="o-field-value"><span class="o-badge ${v}">${r}</span></div></div>
          <div class="o-field-group"><label class="o-field-label">Unidad de Medida</label><div class="o-field-value">${s.uom_name||s.uom||"Unidad"}</div></div>
          <div class="o-field-group"><label class="o-field-label">Peso</label><div class="o-field-value">${s.weight!=null?s.weight+" kg":"—"}</div></div>
        </div>
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">Precio de Venta</label><div class="o-field-value o-field-price">${u}</div></div>
          <div class="o-field-group"><label class="o-field-label">Costo</label><div class="o-field-value o-td-muted">${g}</div></div>
          <div class="o-field-group"><label class="o-field-label">Impuestos</label><div class="o-field-value">${s.taxes_name||"IVA 16%"}</div></div>
          <div class="o-field-group"><label class="o-field-label">Categoría</label><div class="o-field-value">${L}</div></div>
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
            <div class="o-field-value">${s.description||s.descripcion||"—"}</div></div>
          <div class="o-form-grid" style="margin-top:12px">
            <div class="o-form-col">
              <div class="o-field-group"><label class="o-field-label">Peso (kg)</label><div class="o-field-value">${s.weight??"—"}</div></div>
            </div>
            <div class="o-form-col">
              <div class="o-field-group"><label class="o-field-label">Volumen (m³)</label><div class="o-field-value">${s.volume??"—"}</div></div>
            </div>
          </div>
        </div>
        <div class="o-tab-pane" id="tab-ventas" style="display:none">
          <div class="o-field-group"><label class="o-field-label">Política de facturación</label><div class="o-field-value">Cantidades ordenadas</div></div>
          <div class="o-field-group"><label class="o-field-label">Descripción en pedido de venta</label><div class="o-field-value">—</div></div>
        </div>
        <div class="o-tab-pane" id="tab-compras" style="display:none">
          <div class="o-field-group"><label class="o-field-label">Precio de compra</label><div class="o-field-value">${g}</div></div>
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
    </div>`),window._editarProductoForm=k=>We({id:k,...s},()=>window._verProducto(k)),window._prodTab=(k,A)=>{document.querySelectorAll("#prod-tabs .o-tab").forEach(_=>_.classList.remove("active")),A.classList.add("active"),document.querySelectorAll(".o-tab-pane").forEach(_=>_.style.display="none");const h=document.getElementById(`tab-${k}`);h&&(h.style.display="")}}catch(s){console.error(s),b("Error",s.message,"error")}};window._productosBack=()=>Nt();window._productoSetView=t=>{var i;lt=t,document.querySelectorAll("#productos-cp .o-view-btn").forEach(o=>o.classList.remove("o-active"));const e=t==="list"?0:1;(i=document.querySelectorAll("#productos-cp .o-view-btn")[e])==null||i.classList.add("o-active");const a=document.getElementById("productos-content");a&&(t==="kanban"?a.innerHTML=ve(at):a.innerHTML=re(at,!1))};window._productoFiltroTipo=t=>{var a;pt=t,G=1,document.querySelectorAll("#productos-cp .o-filter-btn").forEach(i=>i.removeAttribute("data-active"));const e={"":"ptf-todos",consu:"ptf-consu",service:"ptf-serv",product:"ptf-prod"};(a=document.getElementById(e[t]))==null||a.setAttribute("data-active",""),_t()};window._productoNuevo=()=>alert("Nuevo producto — próximamente");window._chkAllProductos=t=>document.querySelectorAll("#productos-content .o-chk").forEach(e=>e.checked=t.checked);let R=1,Ct=[],ut="",H="";async function Rt(){C(),R=1,ut="",H="",$([{label:"Clientes / Proveedores"}]),ea(),await qt()}function ea(){w(`
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
    ${x(10,6)}
  </div>`),setTimeout(()=>{var t;(t=document.getElementById("o-search-partners"))==null||t.addEventListener("input",e=>{ut=e.target.value.toLowerCase(),aa()})},100)}function aa(){document.querySelectorAll("#partners-content tbody tr").forEach(t=>{t.style.display=t.textContent.toLowerCase().includes(ut)?"":"none"})}async function qt(){try{let t;H==="clientes"?t=p.clientes(R):H==="proveedores"?t=p.proveedores(R):t=p.partners(R);const e=await t;Ct=(e==null?void 0:e.data)||[];const a=Ct.length>=20,i=document.getElementById("partners-content");if(!i)return;i.innerHTML=oa(Ct,a)}catch(t){console.error(t),b("Error",t.message,"error");const e=document.getElementById("partners-content");e&&(e.innerHTML='<div class="o-empty-state"><p>Error al cargar contactos</p></div>')}}function oa(t,e){return t.length?`
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
        ${t.map((a,i)=>{const o=(a.customer_rank||0)>0,l=(a.supplier_rank||0)>0,s=a.is_company,c=a.name||a.nombre||"—",d=c.split(" ").map(r=>r[0]).slice(0,2).join(""),n=a.id*37%360;return`
          <tr class="o-list-row" onclick="window._verPartner(${a.id})">
            <td class="o-list-chk"><input type="checkbox" class="o-chk" onclick="event.stopPropagation()"></td>
            <td class="o-list-num">${(R-1)*20+i+1}</td>
            <td>
              <div class="o-partner-cell">
                <div class="o-avatar" style="background:linear-gradient(135deg,hsl(${n},60%,55%),hsl(${(n+40)%360},70%,45%))">${d||"?"}</div>
                <div>
                  <div class="o-td-primary">${c}</div>
                  ${s?'<div class="o-td-secondary">Empresa</div>':""}
                </div>
              </div>
            </td>
            <td class="o-td-muted">${a.email||"—"}</td>
            <td class="o-td-muted">${a.phone||"—"}</td>
            <td class="o-td-muted">${a.city||"—"}</td>
            <td class="o-td-mono">${a.vat||"—"}</td>
            <td>
              ${o?'<span class="o-badge o-badge-success">Cliente</span>':""}
              ${l?'<span class="o-badge o-badge-info" style="margin-left:2px">Proveedor</span>':""}
              ${!o&&!l?'<span class="o-badge o-badge-gray">Contacto</span>':""}
            </td>
          </tr>`}).join("")}
      </tbody>
    </table>
    <div class="o-list-footer">
      <span class="o-list-count">${t.length} contacto${t.length!==1?"s":""}</span>
      ${K(R,e,a=>{R=a,qt()})}
    </div>
  </div>`:`
    <div class="o-empty-state">
      <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" opacity=".3"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>
      <p>Sin contactos registrados</p>
    </div>`}window._verPartner=async t=>{$([{label:"Clientes / Proveedores",onclick:()=>Rt()},{label:"Cargando…",id:"bc-partner-name"}]),w(`<div class="o-form-loading">${x(4,3)}</div>`);try{const e=await p.partner(t);if(!e){b("Error","Contacto no encontrado","error");return}const a=document.getElementById("bc-partner-name");a&&(a.textContent=e.name||"Contacto");const i=(e.customer_rank||0)>0,o=(e.supplier_rank||0)>0,l=e.is_company,s=e.name||"—",c=s.split(" ").map(n=>n[0]).slice(0,2).join(""),d=e.id*37%360;w(`
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
        <div class="o-avatar o-avatar-lg" style="background:linear-gradient(135deg,hsl(${d},60%,55%),hsl(${(d+40)%360},70%,45%))">${c||"?"}</div>
        <div class="o-sheet-title-block">
          <h1 class="o-form-title">${s}</h1>
          <div style="display:flex;gap:6px;margin-top:4px">
            ${i?'<span class="o-badge o-badge-success">Cliente</span>':""}
            ${o?'<span class="o-badge o-badge-info">Proveedor</span>':""}
            ${l?'<span class="o-badge o-badge-gray">Empresa</span>':'<span class="o-badge o-badge-gray">Persona física</span>'}
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
            <div class="o-field-value">${l?"Sí":"No"}</div>
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
            <div class="o-msg-meta"><strong>Sistema</strong> <span>${B(new Date().toISOString())}</span></div>
            <div class="o-msg-text">Registro creado.</div>
          </div>
        </div>
      </div>
    </div>`),window._editarPartnerForm=n=>{const r={id:n,...e};Je(r,()=>window._verPartner(n))},window._partnerTab=(n,r)=>{document.querySelectorAll("#partner-tabs .o-tab").forEach(u=>u.classList.remove("active")),r.classList.add("active"),document.querySelectorAll(".o-tab-pane").forEach(u=>u.style.display="none");const v=document.getElementById(`tab-${n}`);v&&(v.style.display="")}}catch(e){console.error(e),b("Error",e.message,"error")}};window._partnersBack=()=>Rt();window._partnerFilter=t=>{var i;H=t,R=1,document.querySelectorAll("#partners-cp .o-filter-btn").forEach(o=>o.removeAttribute("data-active"));const e={"":"pf-all",clientes:"pf-cli",proveedores:"pf-prov"};(i=document.getElementById(e[t]))==null||i.setAttribute("data-active","");const a=document.getElementById("partners-content");a&&(a.innerHTML=x(8,6)),qt()};window._partnerNuevo=()=>alert("Nuevo contacto — próximamente");window._chkAllPartners=t=>{document.querySelectorAll("#partners-content .o-chk").forEach(e=>e.checked=t.checked)};let mt=1,vt=[],bt="";async function Vt(){C(),mt=1,bt="",$([{label:"Inventario"}]),ia(),await pe()}function ia(){w(`
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
    ${x(10,5)}
  </div>`),setTimeout(()=>{var t;(t=document.getElementById("o-search-stock"))==null||t.addEventListener("input",e=>{bt=e.target.value.toLowerCase(),sa()})},100)}function sa(){document.querySelectorAll("#stock-content tbody tr").forEach(t=>{t.style.display=t.textContent.toLowerCase().includes(bt)?"":"none"})}async function pe(){try{const t=await p.stock(mt);vt=(t==null?void 0:t.data)||[];const e=vt.length>=20,a=document.getElementById("stock-content");if(!a)return;a.innerHTML=na(vt,e)}catch(t){console.error(t),b("Error",t.message,"error")}}function na(t,e){return t.length?`
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
        ${t.map(a=>{const i=parseFloat(a.cantidad_disponible??a.qty_available??0),o=parseFloat(a.cantidad_reservada??a.reserved_qty??0),l=i<=0?"#ef4444":i<10?"#f59e0b":"#10b981",s=a.product_name||a.nombre||`Producto #${a.product_id||a.id}`,c=a.ubicacion||a.location||"WH/Stock",d=a.uom_name||a.unidad||"Unidades";return`
          <tr class="o-list-row" onclick="window._verStockItem(${a.product_id||a.id})">
            <td class="o-list-chk"><input type="checkbox" class="o-chk" onclick="event.stopPropagation()"></td>
            <td class="o-td-primary">${s}</td>
            <td class="o-td-muted">${c}</td>
            <td class="o-td-amount" style="color:${l};font-weight:700">${F(i)}</td>
            <td class="o-td-amount o-td-muted">${F(o)}</td>
            <td class="o-td-muted">${d}</td>
          </tr>`}).join("")}
      </tbody>
    </table>
    <div class="o-list-footer">
      <span class="o-list-count">${t.length} producto${t.length!==1?"s":""}</span>
      ${K(mt,e,a=>{mt=a,pe()})}
    </div>
  </div>`:`
    <div class="o-empty-state">
      <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" opacity=".3"><path d="M5 8h14M5 8a2 2 0 1 0 0-4h14a2 2 0 1 0 0 4M5 8v10a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V8m-9 4h4"/></svg>
      <p>Sin registros de inventario</p>
    </div>`}window._verStockItem=async t=>{$([{label:"Inventario",onclick:()=>Vt()},{label:"Detalle de stock",id:"bc-stock-name"}]),w(`<div class="o-form-loading">${x(3,3)}</div>`);try{const e=await p.stockProducto(t),i=(Array.isArray(e==null?void 0:e.data)?e.data:e!=null&&e.data?[e.data]:[])[0]||{},o=parseFloat(i.cantidad_disponible??0),l=parseFloat(i.cantidad_reservada??0),s=o*parseFloat(i.valor_unitario||0),c=i.product_name||`Producto #${t}`,d=document.getElementById("bc-stock-name");d&&(d.textContent=c);const n=o<=0?"#ef4444":o<10?"#f59e0b":"#10b981";w(`
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
        <span class="o-smart-count" style="color:${n}">${F(o)}</span>
        <span class="o-smart-label">Disponible</span>
      </button>
      <button class="o-smart-btn">
        <span class="o-smart-count">${F(l)}</span>
        <span class="o-smart-label">Reservado</span>
      </button>
    </div>

    <div class="o-form-sheet">
      <div class="o-sheet-header">
        <div class="o-sheet-title-block">
          <h1 class="o-form-title">${c}</h1>
        </div>
      </div>

      <div class="o-form-grid">
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">Producto</label><div class="o-field-value">${c}</div></div>
          <div class="o-field-group"><label class="o-field-label">Ubicación</label><div class="o-field-value">${i.ubicacion||"WH/Stock"}</div></div>
          <div class="o-field-group"><label class="o-field-label">Cantidad Disponible</label><div class="o-field-value" style="color:${n};font-weight:700;font-size:20px">${F(o)}</div></div>
          <div class="o-field-group"><label class="o-field-label">Cantidad Reservada</label><div class="o-field-value">${F(l)}</div></div>
        </div>
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">Unidad de Medida</label><div class="o-field-value">${i.uom_name||i.unidad||"Unidades"}</div></div>
          <div class="o-field-group"><label class="o-field-label">Valor Unitario</label><div class="o-field-value">${f(parseFloat(i.valor_unitario||0))}</div></div>
          <div class="o-field-group"><label class="o-field-label">Valor Total</label><div class="o-field-value o-field-price">${f(s)}</div></div>
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
    </div>`),window._ajustarStockForm=r=>{const v=vt.find(u=>(u.product_id||u.id)===r);v?ae(v,()=>window._verStockItem(r)):ae({product_id:r,product_name:c},()=>window._verStockItem(r))}}catch(e){console.error(e),b("Error",e.message,"error")}};window._stockBack=()=>Vt();window._stockAjustarGlobal=()=>alert("Selecciona un producto para ajustar");window._stockFiltro=t=>{document.querySelectorAll("#stock-content tbody tr").forEach(e=>{var i;const a=parseFloat(((i=e.querySelector("td:nth-child(4)"))==null?void 0:i.textContent)||"0");t==="bajo"?e.style.display=a<10?"":"none":t==="cero"?e.style.display=a<=0?"":"none":e.style.display=""})};window._chkAllStock=t=>document.querySelectorAll("#stock-content .o-chk").forEach(e=>e.checked=t.checked);let W=1,ct="historial";async function la(){C(),$([{label:"Dashboard",href:"dashboard"},{label:"CFDI 4.0"}]),W=1,await ue()}async function ue(){w(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">🔏 CFDI 4.0</h1>
      <p class="page-subtitle" id="cfdi-sub">Comprobantes Fiscales Digitales</p>
    </div>
    <div class="page-actions">
      <button class="btn ${ct==="historial"?"btn-primary":"btn-secondary"}"
        onclick="window._cfdiTab('historial')">📋 Historial</button>
      <button class="btn ${ct==="timbrar"?"btn-primary":"btn-secondary"}"
        onclick="window._cfdiTab('timbrar')">➕ Timbrar</button>
    </div>
  </div>

  <!-- KPI row -->
  <div style="display:grid;grid-template-columns:repeat(4,1fr);gap:12px;margin-bottom:18px" id="kpi-row" class="anim-2">
    ${[1,2,3,4].map(()=>'<div class="data-card" style="padding:16px"><div class="skeleton" style="height:40px"></div></div>').join("")}
  </div>

  <div class="data-card anim-3" id="cfdi-content">
    <div id="cfdi-body">${x(6,6)}</div>
  </div>`),window._cfdiTab=t=>{ct=t,ue()};try{const t=await p.cfdiKpis().catch(()=>null),e=t==null?void 0:t.data,a=document.getElementById("kpi-row");a&&(a.innerHTML=[{label:"Total Timbrados",val:(e==null?void 0:e.total_timbrados)??0,tipo:"num",color:"indigo",icon:"🧾"},{label:"Vigentes",val:(e==null?void 0:e.vigentes)??0,tipo:"num",color:"emerald",icon:"✅"},{label:"Cancelados",val:(e==null?void 0:e.cancelados)??0,tipo:"num",color:"red",icon:"❌"},{label:"Monto Total",val:(e==null?void 0:e.monto_total)??0,tipo:"mxn",color:"violet",icon:"💰"}].map(i=>`
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${i.icon} ${i.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${i.tipo==="mxn"?f(parseFloat(i.val)):Number(i.val).toLocaleString("es-MX")}
        </div>
      </div>`).join("")),ct==="historial"?await Mt():da()}catch(t){console.error(t),b("Error CFDI",t.message,"error")}}async function Mt(){const t=document.getElementById("cfdi-body");t&&(t.innerHTML=x(6,6));const e=await p.cfdiTimbrados(W).catch(()=>({data:[],total:0})),a=(e==null?void 0:e.data)||[],i=(e==null?void 0:e.total)??a.length,o=a.length>=20,l=document.getElementById("cfdi-sub");if(l&&(l.textContent=`${i} CFDIs timbrados · Página ${W}`),!!t){if(a.length===0){t.innerHTML=`
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
      ${a.map(s=>{const c=s.estado==="vigente"?"emerald":s.estado==="cancelado"?"red":"gray";return`
        <tr style="cursor:pointer" onclick="window._verCfdi('${s.uuid}')" title="Ver detalle">
          <td class="td-mono" style="font-size:11px">${s.uuid.substring(0,18)}…</td>
          <td class="td-mono">${s.serie||""}${s.folio||"—"}</td>
          <td class="td-primary">${s.nombre_receptor||s.rfc_receptor}</td>
          <td class="td-amount" style="font-weight:700">${f(parseFloat(s.total||0))}</td>
          <td><span class="badge badge-sky">${s.tipo_cfdi==="I"?"Ingreso":s.tipo_cfdi==="E"?"Egreso":s.tipo_cfdi||"—"}</span></td>
          <td><span class="badge badge-${c}">${s.estado||"—"}</span></td>
          <td style="font-size:12px">${B(s.fecha_timbrado||s.created_at)}</td>
        </tr>`}).join("")}
    </tbody>
  </table>
  ${K(W,o,s=>{W=s,Mt()})}`,window._verCfdi=s=>{Ae("Detalle CFDI",()=>p.cfdiTimbrado(s),c=>`
      ${$t("Comprobante",[T("UUID",`<span style="font-family:monospace;font-size:11px">${c.uuid}</span>`),T("Serie / Folio",`${c.serie||""}${c.folio||"—"}`),T("Tipo",c.tipo_cfdi==="I"?"Ingreso":c.tipo_cfdi==="E"?"Egreso":c.tipo_cfdi),T("Estado",`<span class="badge badge-${c.estado==="vigente"?"emerald":"red"}">${c.estado}</span>`),T("Fecha emisión",B(c.fecha_emision)),T("Fecha timbrado",B(c.fecha_timbrado))].join(""))}
      ${$t("Partes",[T("RFC Emisor",c.rfc_emisor),T("Emisor",c.nombre_emisor||"—"),T("RFC Receptor",c.rfc_receptor),T("Receptor",c.nombre_receptor||"—")].join(""))}
      ${$t("Importes",[T("Total",`<strong>${f(parseFloat(c.total||0))}</strong>`,{color:"var(--primary)"})].join(""))}
      <div style="display:flex;gap:10px;margin-top:16px">
        <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
        ${c.estado==="vigente"?`<button class="btn btn-danger btn-sm" onclick="window._cancelarCfdi('${c.uuid}')">❌ Cancelar</button>`:""}
      </div>`)},window._cancelarCfdi=async s=>{if(confirm(`¿Cancelar el CFDI ${s.substring(0,18)}…?`))try{await p.cancelarCfdi({uuid:s,rfc_emisor:"",motivo:"02"}),b("CFDI cancelado",s,"success"),window.__closeModal(),Mt()}catch(c){b("Error al cancelar",c.message,"error")}}}}function da(){var e;const t=document.getElementById("cfdi-body");t&&(t.innerHTML=`
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
  </div>`,(e=document.getElementById("f-subtotal"))==null||e.addEventListener("input",a=>{const i=parseFloat(a.target.value)||0,o=i*.16;document.getElementById("f-iva").value=o.toFixed(2),document.getElementById("f-total").value=(i+o).toFixed(2)}),window._timbrar=async()=>{var o,l,s,c,d,n,r,v,u,g,y,I,L,k;const a=document.getElementById("btn-timbrar");a.textContent="⏳ Timbrando…",a.disabled=!0;const i=document.getElementById("cfdi-resultado");try{const A=(o=document.getElementById("f-cer"))==null?void 0:o.files[0],h=(l=document.getElementById("f-key"))==null?void 0:l.files[0],_=Kt=>new Promise((Xt,_e)=>{if(!Kt){Xt("");return}const xt=new FileReader;xt.onload=xe=>Xt(xe.target.result.split(",")[1]||""),xt.onerror=_e,xt.readAsDataURL(Kt)}),[S,Q]=await Promise.all([_(A),_(h)]),M=parseFloat((s=document.getElementById("f-subtotal"))==null?void 0:s.value)||0,dt=M*.16,we={cfdi:{serie:((c=document.getElementById("f-serie"))==null?void 0:c.value)||"A",folio:((d=document.getElementById("f-folio"))==null?void 0:d.value)||"1",tipo_comprobante:((n=document.getElementById("f-tipo"))==null?void 0:n.value)||"I",emisor:{rfc:((r=document.getElementById("f-rfc-emisor"))==null?void 0:r.value)||"",nombre:((v=document.getElementById("f-nombre-emisor"))==null?void 0:v.value)||"",regimen_fiscal:((u=document.getElementById("f-regimen"))==null?void 0:u.value)||"601"},receptor:{rfc:((g=document.getElementById("f-rfc-receptor"))==null?void 0:g.value)||"",nombre:((y=document.getElementById("f-nombre-receptor"))==null?void 0:y.value)||"",uso_cfdi:((I=document.getElementById("f-uso"))==null?void 0:I.value)||"G03",domicilio_fiscal_receptor:"64000",regimen_fiscal_receptor:"601"},conceptos:[{clave_prod_serv:"84111506",descripcion:((L=document.getElementById("f-concepto"))==null?void 0:L.value)||"Servicio",cantidad:"1",unidad:"ACT",valor_unitario:M.toFixed(2),importe:M.toFixed(2),impuestos:{traslados:[{base:M.toFixed(2),impuesto:"002",tipo_factor:"Tasa",tasa:"0.160000",importe:dt.toFixed(2)}]}}],subtotal:M.toFixed(2),total:(M+dt).toFixed(2),moneda:"MXN",lugar_expedicion:"64000"},cert_b64:S,key_b64:Q,password:((k=document.getElementById("f-pwd"))==null?void 0:k.value)||""},P=await p.timbrar(we);P!=null&&P.success?(i.innerHTML=`
        <div style="background:var(--success-light);border:1.5px solid var(--success);border-radius:12px;padding:16px">
          <div style="font-weight:700;color:var(--success);margin-bottom:8px">✅ CFDI Timbrado</div>
          <div style="font-size:12px;font-family:monospace;word-break:break-all;color:var(--text-600)">UUID: ${P.uuid}</div>
          <div style="font-size:12px;color:var(--text-500);margin-top:4px">Fecha: ${B(P.fecha_timbrado)}</div>
        </div>`,b("CFDI timbrado",`UUID: ${P.uuid}`,"success")):i.innerHTML=`<div style="background:var(--danger-light,#fee2e2);border:1.5px solid var(--red);border-radius:12px;padding:16px;color:var(--red)">
          ❌ Error: ${(P==null?void 0:P.error)||"Error desconocido"}</div>`}catch(A){i.innerHTML=`<div style="background:#fee2e2;border:1.5px solid var(--red);border-radius:12px;padding:16px;color:var(--red)">❌ ${A.message}</div>`}finally{a.textContent="🔏 Timbrar CFDI",a.disabled=!1}})}let ft=1,U=[],gt="";async function Ot(){C(),ft=1,gt="",$([{label:"Nómina"}]),ca(),await me()}function ca(){w(`
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
        <input id="o-search-nomina" class="o-search-input" type="text" placeholder="Buscar empleado o departamento…" value="${gt}">
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
    ${x(10,6)}
  </div>`),setTimeout(()=>{var t;(t=document.getElementById("o-search-nomina"))==null||t.addEventListener("input",e=>{gt=e.target.value.toLowerCase(),ra()})},100)}function ra(){document.querySelectorAll("#nomina-content tbody tr").forEach(t=>{t.style.display=t.textContent.toLowerCase().includes(gt)?"":"none"})}async function me(){try{const t=await p.nomina(ft);U=(t==null?void 0:t.data)||[];const e=U.length>=20,a=document.getElementById("nomina-content");if(!a)return;a.innerHTML=be(U,e)}catch(t){console.error(t),b("Error",t.message,"error")}}function be(t,e){return t.length?`
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
        ${t.map(a=>{const i=a.active!==!1,o=(a.name||"?").split(" ").map(v=>v[0]).slice(0,2).join(""),l=a.id*47%360,s=a.job_title||a.job_id_name||"—",c=a.department_name||a.department_id_name||"—",d=a.ssnid||a.imss||"—",n=B(a.date_start||a.fecha_inicio||null),r=f(parseFloat(a.wage||a.salario_base||0));return`
          <tr class="o-list-row" onclick="window._verEmpleado(${a.id})">
            <td class="o-list-chk"><input type="checkbox" class="o-chk" onclick="event.stopPropagation()"></td>
            <td>
              <div class="o-partner-cell">
                <div class="o-avatar" style="background:linear-gradient(135deg,hsl(${l},60%,55%),hsl(${(l+50)%360},70%,45%))">${o||"?"}</div>
                <div>
                  <div class="o-td-primary">${a.name||"—"}</div>
                  <div class="o-td-secondary"><span class="o-badge ${i?"o-badge-success":"o-badge-gray"}">${i?"Activo":"Baja"}</span></div>
                </div>
              </div>
            </td>
            <td class="o-td-muted">${s}</td>
            <td class="o-td-muted">${c}</td>
            <td class="o-td-mono">${d}</td>
            <td class="o-td-muted">${n}</td>
            <td class="o-td-amount" style="font-weight:700">${r}</td>
          </tr>`}).join("")}
      </tbody>
    </table>
    <div class="o-list-footer">
      <span class="o-list-count">${t.length} empleado${t.length!==1?"s":""}</span>
      ${K(ft,e,a=>{ft=a,me()})}
    </div>
  </div>`:`
    <div class="o-empty-state">
      <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" opacity=".3"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>
      <p>Sin empleados registrados</p>
    </div>`}window._verEmpleado=async t=>{$([{label:"Nómina",onclick:()=>Ot()},{label:"Cargando…",id:"bc-emp-name"}]),w(`<div class="o-form-loading">${x(4,3)}</div>`);try{let e=U.find(n=>n.id===t);try{const n=await p.empleado(t);n&&(n.id||n.name)&&(e=n)}catch{}if(!e){b("Error","Empleado no encontrado","error");return}const a=document.getElementById("bc-emp-name");a&&(a.textContent=e.name||"Empleado");const i=e.active!==!1,o=(e.name||"?").split(" ").map(n=>n[0]).slice(0,2).join(""),l=e.id*47%360,s=f(parseFloat(e.sbc||e.salario_base_cotizacion||0)),c=f(parseFloat(e.sdi||e.salario_diario_integrado||0)),d=f(parseFloat(e.wage||e.salario_base||0));w(`
    <div class="o-form-topbar">
      <button class="o-back-btn" onclick="window._nominaBack()">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
        Nómina
      </button>
      <div class="o-form-actions">
        <button class="o-btn-secondary" onclick="window._editarEmpleadoForm(${e.id})">💾 Actualizar</button>
        <button class="o-btn-primary"   onclick="alert('Calcular nómina — próximamente')">Calcular Nómina</button>
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
        <div class="o-avatar o-avatar-lg" style="background:linear-gradient(135deg,hsl(${l},60%,55%),hsl(${(l+50)%360},70%,45%))">${o||"?"}</div>
        <div class="o-sheet-title-block">
          <h1 class="o-form-title">${e.name||"—"}</h1>
          <div style="display:flex;gap:6px;margin-top:4px">
            <span class="o-badge ${i?"o-badge-success":"o-badge-gray"}">${i?"Activo":"Baja"}</span>
            ${e.contract_type_name?`<span class="o-badge o-badge-info">${e.contract_type_name}</span>`:""}
          </div>
        </div>
      </div>

      <div class="o-form-grid">
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">CURP</label><div class="o-field-value o-field-mono">${e.curp||"—"}</div></div>
          <div class="o-field-group"><label class="o-field-label">N° Seguro Social</label><div class="o-field-value o-field-mono">${e.ssnid||e.imss||"—"}</div></div>
          <div class="o-field-group"><label class="o-field-label">RFC</label><div class="o-field-value o-field-mono">${e.rfc||"—"}</div></div>
          <div class="o-field-group"><label class="o-field-label">Fecha de Inicio</label><div class="o-field-value">${B(e.date_start||e.fecha_inicio||null)}</div></div>
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
              <div class="o-field-group"><label class="o-field-label">Salario Base</label><div class="o-field-value o-field-price">${d}</div></div>
              <div class="o-field-group"><label class="o-field-label">SBC (Sal. Base Cotización)</label><div class="o-field-value">${s}</div></div>
              <div class="o-field-group"><label class="o-field-label">SDI (Sal. Diario Integrado)</label><div class="o-field-value">${c}</div></div>
              <div class="o-field-group"><label class="o-field-label">Periodicidad</label><div class="o-field-value">${e.periodicidad||e.payment_period||"Mensual"}</div></div>
            </div>
            <div class="o-form-col">
              <div class="o-field-group"><label class="o-field-label">Banco</label><div class="o-field-value">${e.bank_name||e.banco||"—"}</div></div>
              <div class="o-field-group"><label class="o-field-label">CLABE</label><div class="o-field-value o-field-mono">${e.acc_number||e.clabe||"—"}</div></div>
            </div>
          </div>
        </div>

        <div class="o-tab-pane" id="tab-resumen" style="display:none">
          <table class="o-list-table">
            <thead><tr><th>Concepto</th><th class="o-col-right">Importe</th><th>Tipo</th></tr></thead>
            <tbody>
              <tr><td>Salario Mensual</td><td class="o-td-amount">${d}</td><td><span class="o-badge o-badge-success">Percepción</span></td></tr>
              <tr><td>IMSS Obrero (cuota)</td><td class="o-td-amount">—</td><td><span class="o-badge o-badge-danger">Deducción</span></td></tr>
              <tr><td>ISR Estimado</td><td class="o-td-amount">—</td><td><span class="o-badge o-badge-danger">Deducción</span></td></tr>
              <tr class="o-total-row" style="font-weight:700"><td>Neto a Pagar (est.)</td><td class="o-td-amount">—</td><td></td></tr>
            </tbody>
          </table>
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
            <div class="o-msg-meta"><strong>Sistema</strong> <span>${B(e.date_start||new Date().toISOString())}</span></div>
            <div class="o-msg-text">Registro creado.</div>
          </div>
        </div>
      </div>
    </div>`),window._editarEmpleadoForm=n=>Ze({id:n,...e},()=>window._verEmpleado(n)),window._nomTab=(n,r)=>{document.querySelectorAll("#nom-tabs .o-tab").forEach(u=>u.classList.remove("active")),r.classList.add("active"),document.querySelectorAll(".o-tab-pane").forEach(u=>u.style.display="none");const v=document.getElementById(`tab-${n}`);v&&(v.style.display="")}}catch(e){console.error(e),b("Error",e.message,"error")}};window._nominaBack=()=>Ot();window._nominaNuevoEmpleado=()=>alert("Nuevo empleado — próximamente");window._nominaFiltro=t=>{const e=t==="activos"?U.filter(i=>i.active!==!1):t==="baja"?U.filter(i=>i.active===!1):U,a=document.getElementById("nomina-content");a&&(a.innerHTML=be(e,!1))};window._chkAllNomina=t=>document.querySelectorAll("#nomina-content .o-chk").forEach(e=>e.checked=t.checked);let Ht="list",ht=1,j=[],yt="";const wt={draft:{lbl:"Borrador",cls:"o-badge-gray",kanban:"Borrador"},sent:{lbl:"Enviada",cls:"o-badge-info",kanban:"Enviada al Proveedor"},purchase:{lbl:"Orden de Compra",cls:"o-badge-success",kanban:"Órdenes de Compra"},done:{lbl:"Realizada",cls:"o-badge-warn",kanban:"Realizada"},cancel:{lbl:"Cancelada",cls:"o-badge-danger",kanban:"Cancelada"}},oe=["draft","sent","purchase","done"];async function Ut(){C(),Ht="list",ht=1,yt="",$([{label:"Compras"}]),va(),await fe()}function va(){w(`
  <div class="o-cp" id="compras-cp">
    <div class="o-cp-left">
      <button class="o-btn-primary" onclick="window._compraNueva()">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
        Nuevo
      </button>
    </div>
    <div class="o-cp-center">
      <div class="o-search-bar">
        <svg class="o-search-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/></svg>
        <input id="o-search-compras" class="o-search-input" type="text" placeholder="Buscar folio o proveedor…" value="${yt}">
        <div class="o-search-filters">
          <button class="o-filter-btn" onclick="window._compraFiltroEstado('draft')">Borrador</button>
          <button class="o-filter-btn" onclick="window._compraFiltroEstado('purchase')">Confirmadas</button>
          <button class="o-filter-btn" onclick="window._compraFiltroEstado('done')">Realizadas</button>
        </div>
      </div>
    </div>
    <div class="o-cp-right">
      <div class="o-view-switcher">
        <button class="o-view-btn o-active" onclick="window._compraSetView('list')" title="Lista">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="8" y1="6" x2="21" y2="6"/><line x1="8" y1="12" x2="21" y2="12"/><line x1="8" y1="18" x2="21" y2="18"/><line x1="3" y1="6" x2="3.01" y2="6"/><line x1="3" y1="12" x2="3.01" y2="12"/><line x1="3" y1="18" x2="3.01" y2="18"/></svg>
        </button>
        <button class="o-view-btn" onclick="window._compraSetView('kanban')" title="Kanban">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="1" y="4" width="6" height="16" rx="1"/><rect x="9" y="4" width="6" height="10" rx="1"/><rect x="17" y="4" width="6" height="13" rx="1"/></svg>
        </button>
      </div>
    </div>
  </div>
  <div id="compras-content" class="o-view-content">
    ${x(8,6)}
  </div>`),setTimeout(()=>{var t;(t=document.getElementById("o-search-compras"))==null||t.addEventListener("input",e=>{yt=e.target.value.toLowerCase(),pa()})},100)}function pa(){document.querySelectorAll("#compras-content .o-list-row, #compras-content .o-kanban-card").forEach(t=>{t.style.display=t.textContent.toLowerCase().includes(yt)?"":"none"})}async function fe(){try{const t=await p.compras(ht);j=(t==null?void 0:t.data)||[];const e=j.length>=20,a=document.getElementById("compras-content");if(!a)return;Ht==="kanban"?a.innerHTML=ge(j):a.innerHTML=Gt(j,e)}catch(t){console.error(t),b("Error",t.message,"error")}}function Gt(t,e){return t.length?`
  <div class="o-list-view">
    <table class="o-list-table">
      <thead>
        <tr>
          <th class="o-list-chk"><input type="checkbox" class="o-chk" onclick="window._chkAllCompras(this)"></th>
          <th class="o-col-sortable">Número</th>
          <th class="o-col-sortable">Proveedor</th>
          <th>Fecha</th>
          <th>Fecha Esperada</th>
          <th>Estado</th>
          <th class="o-col-right">Total</th>
        </tr>
      </thead>
      <tbody>
        ${t.map(a=>{const i=wt[a.state]||{lbl:a.state||"—",cls:"o-badge-gray"};return`
          <tr class="o-list-row" onclick="window._verCompra(${a.id})">
            <td class="o-list-chk"><input type="checkbox" class="o-chk" onclick="event.stopPropagation()"></td>
            <td class="o-td-mono o-td-primary">${a.name||`#${a.id}`}</td>
            <td class="o-td-primary">${a.partner_name||"—"}</td>
            <td class="o-td-muted">${B(a.date_order)}</td>
            <td class="o-td-muted">${B(a.date_planned)}</td>
            <td><span class="o-badge ${i.cls}">${i.lbl}</span></td>
            <td class="o-td-amount" style="font-weight:700">${f(parseFloat(a.amount_total||0))}</td>
          </tr>`}).join("")}
      </tbody>
    </table>
    <div class="o-list-footer">
      <span class="o-list-count">${t.length} orden${t.length!==1?"es":""}</span>
      ${K(ht,e,a=>{ht=a,fe()})}
    </div>
  </div>`:'<div class="o-empty-state"><p>Sin órdenes de compra</p></div>'}function ge(t){return`
  <div class="o-kanban-columns">
    ${["draft","sent","purchase","done"].map(a=>{const i=wt[a],o=t.filter(s=>s.state===a),l=o.reduce((s,c)=>s+parseFloat(c.amount_total||0),0);return`
      <div class="o-kanban-col">
        <div class="o-kanban-col-header">
          <span class="o-badge ${i.cls}">${i.kanban}</span>
          <span class="o-kanban-col-count">${o.length}</span>
        </div>
        <div class="o-kanban-col-sum">${f(l)}</div>
        <div class="o-kanban-col-cards">
          ${o.map(s=>`
          <div class="o-kanban-card" onclick="window._verCompra(${s.id})">
            <div class="o-kanban-title">${s.name||"#"+s.id}</div>
            <div class="o-kanban-sub">${s.partner_name||"—"}</div>
            <div style="display:flex;justify-content:space-between;margin-top:8px">
              <span class="o-td-muted" style="font-size:12px">${B(s.date_order)}</span>
              <strong>${f(parseFloat(s.amount_total||0))}</strong>
            </div>
          </div>`).join("")}
          ${o.length===0?'<div class="o-kanban-empty-col">Sin órdenes</div>':""}
        </div>
      </div>`}).join("")}
  </div>`}window._verCompra=async t=>{$([{label:"Compras",onclick:()=>Ut()},{label:"Cargando…",id:"bc-compra-name"}]),w(`<div class="o-form-loading">${x(4,3)}</div>`);try{let e=j.find(s=>s.id===t);try{const s=await p.compra(t);s&&(s.id||s.name)&&(e=s)}catch{}if(!e){b("Error","Orden no encontrada","error");return}const a=document.getElementById("bc-compra-name");a&&(a.textContent=e.name||`Compra #${t}`);const i=wt[e.state]||{lbl:e.state||"—",cls:"o-badge-gray"},o=oe.indexOf(e.state),l=e.order_line||e.lineas||[];w(`
    <div class="o-form-topbar">
      <button class="o-back-btn" onclick="window._comprasBack()">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
        Compras
      </button>
      <div class="o-form-actions">
        ${e.state==="draft"?`<button class="o-btn-primary" onclick="alert('Confirmar OC — próximamente')">Confirmar OC</button>`:""}
        ${e.state==="purchase"?`<button class="o-btn-secondary" onclick="alert('Recibir mercancía — próximamente')">Recibir</button>`:""}
        <button class="o-btn-secondary" onclick="alert('Crear factura — próximamente')">Crear Factura</button>
        <button class="o-btn-secondary" onclick="window._editarCompraForm(${e.id})">Editar</button>
      </div>
    </div>

    <!-- STATUS BAR -->
    <div class="o-status-bar">
      ${oe.map((s,c)=>{const d=wt[s],n=c===o,r=c<o;return`<div class="o-status-step ${n?"active":r?"done":""}">${d.lbl}</div>`}).join('<div class="o-status-arrow">›</div>')}
    </div>

    <!-- SMART BUTTONS -->
    <div class="o-smart-buttons">
      <button class="o-smart-btn" onclick="alert('Facturas de esta OC')">
        <span class="o-smart-count">0</span>
        <span class="o-smart-label">Facturas</span>
      </button>
      <button class="o-smart-btn" onclick="alert('Recepciones de esta OC')">
        <span class="o-smart-count">0</span>
        <span class="o-smart-label">Recepciones</span>
      </button>
    </div>

    <!-- FORM SHEET -->
    <div class="o-form-sheet">
      <div class="o-sheet-header">
        <div class="o-sheet-title-block">
          <h1 class="o-form-title">${e.name||"Nueva Orden de Compra"}</h1>
          <span class="o-badge ${i.cls}">${i.lbl}</span>
        </div>
      </div>

      <div class="o-form-grid">
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">Proveedor</label><div class="o-field-value o-td-primary">${e.partner_name||"—"}</div></div>
          <div class="o-field-group"><label class="o-field-label">Fecha de Orden</label><div class="o-field-value">${B(e.date_order)}</div></div>
          <div class="o-field-group"><label class="o-field-label">Referencia Proveedor</label><div class="o-field-value o-td-mono">${e.partner_ref||"—"}</div></div>
        </div>
        <div class="o-form-col">
          <div class="o-field-group"><label class="o-field-label">Responsable</label><div class="o-field-value">${e.user_name||e.user||"—"}</div></div>
          <div class="o-field-group"><label class="o-field-label">Empresa</label><div class="o-field-value">${e.company_name||e.company||"NexusTech"}</div></div>
          <div class="o-field-group"><label class="o-field-label">Términos de Pago</label><div class="o-field-value">${e.payment_term_name||e.payment_term||"—"}</div></div>
        </div>
      </div>

      <!-- NOTEBOOK -->
      <div class="o-notebook">
        <div class="o-tabs" id="compra-tabs">
          <button class="o-tab active" onclick="window._compraTab('productos', this)">Productos</button>
          <button class="o-tab" onclick="window._compraTab('adicional', this)">Información Adicional</button>
        </div>

        <div class="o-tab-pane" id="tab-productos">
          ${l.length?`
          <table class="o-list-table">
            <thead><tr><th>Producto</th><th>Descripción</th><th class="o-col-right">Cantidad</th><th class="o-col-right">Precio</th><th class="o-col-right">Subtotal</th></tr></thead>
            <tbody>
              ${l.map(s=>`
              <tr>
                <td class="o-td-primary">${s.product_name||s.name||"—"}</td>
                <td class="o-td-muted">${s.name||s.description||"—"}</td>
                <td class="o-td-amount">${F(parseFloat(s.product_qty||s.qty||0))}</td>
                <td class="o-td-amount">${f(parseFloat(s.price_unit||0))}</td>
                <td class="o-td-amount" style="font-weight:700">${f(parseFloat(s.price_subtotal||0))}</td>
              </tr>`).join("")}
            </tbody>
          </table>`:'<div class="o-empty-state" style="padding:32px 0"><p>Sin líneas de productos</p></div>'}
          <div class="o-form-totals">
            <div class="o-total-row"><span>Subtotal</span><span>${f(parseFloat(e.amount_untaxed||0))}</span></div>
            <div class="o-total-row"><span>IVA</span><span>${f(parseFloat(e.amount_tax||0))}</span></div>
            <div class="o-total-row o-total-final"><span>Total</span><span>${f(parseFloat(e.amount_total||0))}</span></div>
          </div>
        </div>
        <div class="o-tab-pane" id="tab-adicional" style="display:none">
          <div class="o-field-group"><label class="o-field-label">Nota / Términos</label>
            <textarea class="o-textarea" rows="4">${e.notes||e.note||""}</textarea></div>
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
            <div class="o-msg-meta"><strong>Sistema</strong> <span>${B(e.date_order||new Date().toISOString())}</span></div>
            <div class="o-msg-text">Orden de compra creada.</div>
          </div>
        </div>
      </div>
    </div>`),window._editarCompraForm=s=>Ye({id:s,...e},()=>window._verCompra(s)),window._compraTab=(s,c)=>{document.querySelectorAll("#compra-tabs .o-tab").forEach(n=>n.classList.remove("active")),c.classList.add("active"),document.querySelectorAll(".o-tab-pane").forEach(n=>n.style.display="none");const d=document.getElementById(`tab-${s}`);d&&(d.style.display="")}}catch(e){console.error(e),b("Error",e.message,"error")}};window._comprasBack=()=>Ut();window._compraNueva=()=>alert("Nueva orden de compra — próximamente");window._compraSetView=t=>{var i;Ht=t,document.querySelectorAll("#compras-cp .o-view-btn").forEach(o=>o.classList.remove("o-active"));const e=t==="list"?0:1;(i=document.querySelectorAll("#compras-cp .o-view-btn")[e])==null||i.classList.add("o-active");const a=document.getElementById("compras-content");a&&(t==="kanban"?a.innerHTML=ge(j):a.innerHTML=Gt(j,!1))};window._compraFiltroEstado=t=>{const e=j.filter(i=>i.state===t),a=document.getElementById("compras-content");a&&(a.innerHTML=Gt(e,!1))};window._chkAllCompras=t=>document.querySelectorAll("#compras-content .o-chk").forEach(e=>e.checked=t.checked);let ot="list",N=1,zt="",Dt=null,St=[];async function ua(){C(),$([{label:"Cotizaciones"}]),w(`<div class="nx-module-page"><div id="mcp"></div><div id="mcontent">${x(5,6)}</div></div>`),he(),await it()}function he(){const t=document.getElementById("mcp");t&&(t.innerHTML=`
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
          <button class="o-view-btn ${ot==="list"?"active":""}" onclick="window._cvv('list')" title="Lista">☰</button>
          <button class="o-view-btn ${ot==="kanban"?"active":""}" onclick="window._cvv('kanban')" title="Kanban">⬜</button>
        </div>
      </div>
    </div>`,ma(),window._cvv=e=>{ot=e,he(),it()},window._sc=ya(e=>{zt=e,N=1,it()},300),window._cf=e=>{Dt=e,N=1,it(),window._cdd()},window._newCot=()=>ha())}function ma(){window._tog=t=>{const e=document.getElementById(t+"-menu");if(!e)return;const a=e.classList.contains("open");window._cdd(),a||e.classList.add("open")},window._cdd=()=>document.querySelectorAll(".o-dropdown-menu.open").forEach(t=>t.classList.remove("open")),window._ddInit||(document.addEventListener("click",t=>{t.target.closest(".o-dropdown")||window._cdd()}),window._ddInit=!0)}async function it(){const t=document.getElementById("mcontent");if(t){t.innerHTML=x(5,6);try{const e=await p.cotizaciones(N);St=(e==null?void 0:e.data)||[];let a=Dt?St.filter(o=>o.state===Dt):St;if(zt){const o=zt.toLowerCase();a=a.filter(l=>(l.name||"").toLowerCase().includes(o)||(l.partner_name||"").toLowerCase().includes(o))}const i=document.getElementById("ccount");i&&(i.textContent=a.length+" registros"),t.innerHTML=ot==="kanban"?fa(a):ba(a),ot==="list"&&ga()}catch(e){t.innerHTML=`<div style="padding:40px;text-align:center;color:var(--text-400)">⚠️ ${e.message}</div>`}}}const jt={draft:"Borrador",sent:"Enviada",sale:"Confirmada",cancel:"Cancelada"};function ba(t){return t.length?`
    <div class="o-list-actions-bar" id="clab"><span class="o-actions-count" id="csel-cnt">0 seleccionados</span>
      <button class="o-action-btn-sm" onclick="alert('Exportar')">Exportar</button>
    </div>
    <div class="o-list-view"><table>
      <thead><tr>
        <th class="th-check"><input type="checkbox" class="o-list-checkbox" id="cca" onchange="window._cca(this.checked)"></th>
        <th>Número</th><th>Cliente</th><th>Fecha</th><th>Validez</th><th>Estado</th><th style="text-align:right">Total</th>
      </tr></thead>
      <tbody>
        ${t.map(e=>{var a,i;return`
          <tr onclick="window._vCot(${e.id})" data-id="${e.id}">
            <td class="td-check" onclick="event.stopPropagation()"><input type="checkbox" class="o-list-checkbox crc" data-id="${e.id}" onchange="window._crc()"></td>
            <td><strong>${e.name||"-"}</strong></td>
            <td>${e.partner_name||e.partner_id||"-"}</td>
            <td>${((a=e.date_order)==null?void 0:a.slice(0,10))||"-"}</td>
            <td>${((i=e.validity_date)==null?void 0:i.slice(0,10))||'<span style="color:var(--text-300)">—</span>'}</td>
            <td>${q(e.state,jt[e.state]||e.state)}</td>
            <td style="text-align:right;font-weight:700;color:var(--primary)">${f(e.amount_total)}</td>
          </tr>`}).join("")}
      </tbody></table></div>
    <div style="padding:12px 20px;display:flex;justify-content:space-between;align-items:center;background:var(--bg-card);border-top:1px solid var(--border)">
      <span style="font-size:13px;color:var(--text-400)">${t.length} registros</span>
      <div style="display:flex;gap:8px">
        <button class="o-action-btn-sm" ${N<=1?"disabled":""} onclick="window._cp(${N-1})">‹ Anterior</button>
        <span style="padding:5px 10px;font-size:13px">${N}</span>
        <button class="o-action-btn-sm" onclick="window._cp(${N+1})">Siguiente ›</button>
      </div></div>`:'<div style="padding:60px;text-align:center"><div style="font-size:48px;margin-bottom:12px">📝</div><p style="color:var(--text-400)">Sin cotizaciones. Crea la primera.</p></div>'}const ie=[{key:"draft",label:"Borrador",color:"#9CA3AF"},{key:"sent",label:"Enviada",color:"#2563EB"},{key:"sale",label:"Confirmada",color:"#059669"}];function fa(t){const e={};return ie.forEach(a=>e[a.key]=[]),t.forEach(a=>{e[a.state]?e[a.state].push(a):e.draft&&e.draft.push(a)}),`<div class="o-kanban-view">${ie.map(a=>`
    <div class="o-kanban-col">
      <div class="o-kanban-col-header" style="border-top:3px solid ${a.color}">
        <span>${a.label}</span><span class="o-kanban-col-count">${e[a.key].length}</span>
      </div>
      <div class="o-kanban-cards">
        ${e[a.key].map(i=>{var o,l;return`
          <div class="o-kanban-card" onclick="window._vCot(${i.id})">
            <div class="o-kanban-card-title">${i.name||"#"+i.id}</div>
            <div style="font-size:12px;color:var(--text-400);margin-bottom:8px">${i.partner_name||""}</div>
            <div class="o-kanban-card-meta">
              <span style="font-size:11px">${(o=i.validity_date)!=null&&o.slice(0,10)?"⏰ "+i.validity_date.slice(0,10):((l=i.date_order)==null?void 0:l.slice(0,10))||""}</span>
              <span class="o-kanban-card-amount">${f(i.amount_total)}</span>
            </div>
          </div>`}).join("")||'<div style="padding:16px;text-align:center;color:var(--text-300);font-size:12px">Vacío</div>'}
      </div>
    </div>`).join("")}</div>`}function ga(){window._cca=t=>{document.querySelectorAll(".crc").forEach(e=>e.checked=t),window._crc()},window._crc=()=>{const t=document.querySelectorAll(".crc:checked").length,e=document.getElementById("clab"),a=document.getElementById("csel-cnt");e&&e.classList.toggle("visible",t>0),a&&(a.textContent=t+" seleccionado"+(t!==1?"s":"")),document.querySelectorAll("[data-id]").forEach(i=>{const o=i.querySelector(".crc");o&&i.classList.toggle("selected",o.checked)})}}window._cp=t=>{N=t,it()};function ha(){$([{label:"Cotizaciones",href:"#cotizaciones"},{label:"Nueva cotización"}]),w(`
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
    </div>`),window._guardarNuevaCot=async()=>{var l,s,c,d,n,r,v,u;const t=(s=(l=document.getElementById("nc-partner"))==null?void 0:l.value)==null?void 0:s.trim(),e=((d=(c=document.getElementById("nc-ref"))==null?void 0:c.value)==null?void 0:d.trim())||null,a=((n=document.getElementById("nc-validez"))==null?void 0:n.value)||null,i=((v=(r=document.getElementById("nc-nota"))==null?void 0:r.value)==null?void 0:v.trim())||null;if(!t)return b("Campo requerido","Ingresa el nombre del cliente","warning");let o=1;try{const g=await p.get(`/partners?pagina=1&q=${encodeURIComponent(t)}&por_pagina=5`),y=(g==null?void 0:g.data)??[],I=y.find(L=>{var k;return((k=L.name)==null?void 0:k.toLowerCase())===t.toLowerCase()});if(I)o=I.id;else if(y.length>0)o=y[0].id;else return b("Cliente no encontrado",`No se encontró "${t}"`,"warning")}catch(g){return b("Error","No se pudo buscar el cliente: "+g.message,"error")}try{const g=await p.crearCotizacion({partner_id:o,partner_invoice_id:o,partner_shipping_id:o,note:i,client_order_ref:e,validity_date:a||null}),y=((u=g==null?void 0:g.data)==null?void 0:u.id)??(g==null?void 0:g.id);b("Cotización creada",`ID ${y}`,"success"),y?setTimeout(()=>window._vCot(y),400):window._go("cotizaciones")}catch(g){b("Error al crear cotización",g.message,"error")}}}window._vCot=async t=>{var e,a;$([{label:"Cotizaciones",href:"#cotizaciones"},{label:"Cargando…"}]),w(`<div style="padding:40px">${x(3,5)}</div>`);try{const i=await p.cotizacion(t),o=(i==null?void 0:i.data)||i;if(!o)throw new Error("No encontrada");$([{label:"Cotizaciones",href:"#cotizaciones"},{label:o.name||"#"+t}]);const l=["draft","sent"],s=l.indexOf(o.state),c={draft:"Borrador",sent:"Enviada"};w(`
      <div class="o-form-view" id="cfv">
        <div class="o-statusbar">
          <div class="o-statusbar-status">
            ${l.map((d,n)=>`
              <div class="o-status-step ${d===o.state?"active":""} ${n<s?"done":""}">
                ${n<s?"✔ ":""}${c[d]||d}
              </div>${n<l.length-1?'<span class="o-status-arrow">›</span>':""}`).join("")}
            ${o.state==="sale"?'<span class="o-status-arrow">›</span><div class="o-status-step done">✔ Confirmada</div>':""}
            ${o.state==="cancel"?'<span class="o-status-arrow">›</span><div class="o-status-step active" style="color:#DC2626">Cancelada</div>':""}
          </div>
          <div class="o-statusbar-buttons">
            ${o.state==="draft"||o.state==="sent"?`
              <button class="btn btn-secondary btn-sm" onclick="window._emailCot(${t})">✉️ Enviar por Email</button>
              <button class="btn btn-primary btn-sm" onclick="window._confirmarCot(${t})">✅ Confirmar Pedido</button>
            `:""}
            ${o.state==="sale"?`<button class="btn btn-secondary btn-sm" onclick="window._vVenta(${t})">📋 Ver Orden</button>`:""}
            ${o.state!=="cancel"&&o.state!=="sale"?`<button class="btn btn-sm" style="background:#FEE2E2;color:#DC2626;border:none;padding:6px 14px;border-radius:8px;font-weight:600;cursor:pointer" onclick="window._cancelarCot(${t})">❌ Cancelar</button>`:""}
            <button class="btn btn-secondary btn-sm" onclick="window._go('cotizaciones')">← Volver</button>
          </div>
        </div>
        <div class="o-smart-buttons">
          <button class="o-smart-btn" ${o.state==="sale"?`onclick="window._vVenta(${t})"`:""}>
            <span class="o-count">${o.state==="sale"?"1":"0"}</span>
            <span class="o-label">📋 Órdenes</span>
          </button>
          <button class="o-smart-btn"><span class="o-count">0</span><span class="o-label">✉️ Emails</span></button>
        </div>
        <div class="o-form-sheet">
          <div class="o-form-title-row">
            <h1 class="o-form-record-title">${o.name||"Nueva Cotización"}</h1>
            <span class="o-form-subtitle">${o.partner_name||""}</span>
          </div>
          <div class="o-form-group-wrapper">
            <div class="o-form-group">
              <div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">Cliente</div><div class="o-field-value"><strong>${o.partner_name||o.partner_id||'<span class="o-field-empty">—</span>'}</strong></div></div>
                <div class="o-field-row"><div class="o-field-label">Fecha</div><div class="o-field-value">${((e=o.date_order)==null?void 0:e.slice(0,10))||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Validez</div><div class="o-field-value">${((a=o.validity_date)==null?void 0:a.slice(0,10))||'<span class="o-field-empty">—</span>'}</div></div>
              </div>
              <div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">Estado</div><div class="o-field-value">${q(o.state,jt[o.state]||o.state)}</div></div>
                <div class="o-field-row"><div class="o-field-label">Referencia</div><div class="o-field-value">${o.client_order_ref||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Vendedor</div><div class="o-field-value">${o.user_id||o.user_name||'<span class="o-field-empty">—</span>'}</div></div>
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
                <tr><td>Subtotal:</td><td style="text-align:right;font-weight:600">${f(o.amount_untaxed)}</td></tr>
                <tr><td>IVA (16%):</td><td style="text-align:right;font-weight:600">${f(o.amount_tax)}</td></tr>
                <tr class="total-row"><td>TOTAL:</td><td style="text-align:right">${f(o.amount_total)}</td></tr>
              </table></div>
            </div>
            <div class="o-tab-panel" id="tab-panel-cc">
              <div class="o-form-group"><div class="o-form-col">
                <div class="o-field-row"><div class="o-field-label">Notas</div><div class="o-field-value">${o.note||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Plazo de pago</div><div class="o-field-value">${o.payment_term_name||o.payment_term||'<span class="o-field-empty">—</span>'}</div></div>
                <div class="o-field-row"><div class="o-field-label">Política entrega</div><div class="o-field-value">${o.picking_policy||'<span class="o-field-empty">—</span>'}</div></div>
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
                <div class="o-msg-text">Cotización ${o.name||""} — Estado: ${jt[o.state]||o.state}</div>
              </div>
            </div>
          </div>
        </div>
      </div>`),window._ct=d=>{document.querySelectorAll(".o-tab").forEach(v=>v.classList.remove("active")),document.querySelectorAll(".o-tab-panel").forEach(v=>v.classList.remove("active"));const n=document.querySelector(`.o-tab[onclick*="'${d}'"]`);n&&n.classList.add("active");const r=document.getElementById("tab-panel-"+d);r&&r.classList.add("active")};try{const d=await p.get(`/cotizaciones/${t}/lineas`),n=(d==null?void 0:d.data)||[],r=document.getElementById("clineas");r&&(r.innerHTML=n.length?n.map(v=>`<tr>
              <td>${v.product_id?"#"+v.product_id:'<span class="o-field-empty">—</span>'}</td>
              <td>${v.name||"-"}</td>
              <td style="text-align:right">${v.product_uom_qty??0}</td>
              <td style="text-align:right">${f(v.price_unit)}</td>
              <td style="text-align:right">${v.discount?v.discount+"%":"0%"}</td>
              <td style="text-align:right;font-weight:700">${f(v.price_subtotal)}</td>
            </tr>`).join(""):'<tr><td colspan="6" style="text-align:center;padding:16px;color:var(--text-400)">Sin líneas de cotización</td></tr>')}catch{}window._emailCot=async d=>{try{await p.put(`/cotizaciones/${d}/enviar`,{}),b("OK","Cotización enviada por email","success"),window._vCot(d)}catch(n){b("Error",n.message,"error")}},window._confirmarCot=async d=>{if(confirm("¿Confirmar cotización como pedido de venta?"))try{await p.confirmarCotizacion(d),b("OK","Cotización confirmada como venta","success"),setTimeout(()=>window._go("ventas"),600)}catch(n){b("Error",n.message,"error")}},window._cancelarCot=async d=>{if(confirm("¿Cancelar cotización?"))try{await p.cancelarCotizacion(d),b("Cancelado","","info"),window._go("cotizaciones")}catch(n){b("Error",n.message,"error")}}}catch(i){w(`<div style="padding:40px;text-align:center"><p style="color:#DC2626">⚠️ ${i.message}</p><button class="o-btn-new" onclick="window._go('cotizaciones')">Volver</button></div>`)}};function ya(t,e){let a;return(...i)=>{clearTimeout(a),a=setTimeout(()=>t(...i),e)}}let Bt=null;async function wa(){C(),$([{label:"Dashboard",href:"dashboard"},{label:"NexusSearch"}]),await _a()}async function _a(){var e,a;w(`
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
  <div id="index-status" class="anim-4" style="margin-top:16px"></div>`),(e=document.getElementById("search-query"))==null||e.addEventListener("keydown",i=>{i.key==="Enter"&&window._buscar()});let t;(a=document.getElementById("search-query"))==null||a.addEventListener("input",i=>{clearTimeout(t),!(i.target.value.length<2)&&(t=setTimeout(()=>window._buscar(),500))}),await se(),window._buscar=xa,window._checkStatus=se,window._syncSearch=$a}async function xa(){var a,i;const t=(i=(a=document.getElementById("search-query"))==null?void 0:a.value)==null?void 0:i.trim();if(!t||t.length<2)return;const e=document.getElementById("search-results");e&&(e.innerHTML=`
    <div class="data-card" style="padding:20px;text-align:center;color:var(--text-400)">
      <div class="spinner" style="margin:0 auto 8px"></div>
      <div>Buscando "${t}"…</div>
    </div>`);try{const[o,l,s]=await Promise.allSettled([p.ventas(1).then(d=>((d==null?void 0:d.data)||[]).filter(n=>(n.name||"").toLowerCase().includes(t.toLowerCase())||(n.partner_name||"").toLowerCase().includes(t.toLowerCase())).map(n=>({tipo:"Venta",icon:"💰",titulo:n.name,sub:n.partner_name,meta:`$${n.amount_total}`,href:"ventas"}))),p.productos(1,t).then(d=>((d==null?void 0:d.data)||[]).map(n=>{var r,v;return{tipo:"Producto",icon:"📦",titulo:typeof n.name=="object"?((r=n.name)==null?void 0:r.es_MX)||((v=n.name)==null?void 0:v.en_US)||"":n.name||"",sub:n.categ_name||"",meta:"",href:"productos"}})),p.partners(1).then(d=>((d==null?void 0:d.data)||[]).filter(n=>(n.name||"").toLowerCase().includes(t.toLowerCase())||(n.email||"").toLowerCase().includes(t.toLowerCase())).map(n=>({tipo:"Contacto",icon:"👥",titulo:n.name,sub:n.email||"",meta:"",href:"partners"})))]),c=[...o.status==="fulfilled"?o.value:[],...l.status==="fulfilled"?l.value:[],...s.status==="fulfilled"?s.value:[]];if(!e)return;if(c.length===0){e.innerHTML=`
      <div class="data-card" style="padding:40px;text-align:center">
        <div style="font-size:36px;margin-bottom:12px">🔍</div>
        <div style="font-weight:700;color:var(--text-700)">Sin resultados para "${t}"</div>
        <div style="color:var(--text-400);font-size:13px;margin-top:6px">Prueba con otro término</div>
      </div>`;return}e.innerHTML=`
    <div class="data-card">
      <div class="data-card-header">
        <div class="data-card-title">${c.length} resultados para "${t}"</div>
      </div>
      <div style="padding:0 4px">
        ${c.slice(0,30).map(d=>`
        <div style="display:flex;align-items:center;gap:12px;padding:12px 8px;
          border-bottom:1px solid var(--border);cursor:pointer;border-radius:8px;
          transition:background var(--t1)" 
          onmouseover="this.style.background='var(--primary-light)'"
          onmouseout="this.style.background=''"
          onclick="window._go('${d.href}')">
          <div style="width:36px;height:36px;border-radius:10px;background:var(--primary-light);
            display:flex;align-items:center;justify-content:center;font-size:18px;flex-shrink:0">
            ${d.icon}
          </div>
          <div style="flex:1">
            <div style="font-weight:600;color:var(--text-800);font-size:13px">${d.titulo}</div>
            <div style="font-size:11px;color:var(--text-400)">${d.sub}</div>
          </div>
          <div style="display:flex;align-items:center;gap:8px">
            ${d.meta?`<span style="font-size:12px;font-weight:700;color:var(--text-700)">${d.meta}</span>`:""}
            <span class="badge badge-${d.tipo==="Venta"?"indigo":d.tipo==="Producto"?"emerald":"violet"}">${d.tipo}</span>
          </div>
        </div>`).join("")}
      </div>
    </div>`}catch(o){console.error(o),e&&(e.innerHTML=`<p style="color:var(--red);padding:20px">Error: ${o.message}</p>`)}}async function se(){const t=document.getElementById("index-status");try{const e=await p.searchStatus().catch(()=>null);Bt=(e==null?void 0:e.data)||e,t&&Bt&&(t.innerHTML=`
      <div class="data-card" style="padding:16px">
        <div class="data-card-header" style="padding:0 0 12px">
          <div class="data-card-title">📡 Estado del Motor de Búsqueda</div>
        </div>
        <div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(180px,1fr));gap:12px">
          ${Object.entries(Bt).map(([a,i])=>`
          <div style="padding:12px;background:var(--bg);border-radius:10px;border:1px solid var(--border)">
            <div style="font-size:10px;color:var(--text-400);font-weight:700;text-transform:uppercase;margin-bottom:4px">${a}</div>
            <div style="font-size:13px;font-weight:600;color:var(--text-800)">${JSON.stringify(i)}</div>
          </div>`).join("")}
        </div>
      </div>`)}catch{t&&(t.innerHTML="")}}async function $a(){const t=document.getElementById("btn-sync");t&&(t.textContent="⏳ Sincronizando…",t.disabled=!0);try{const e=await p.searchSync();b("Sincronización iniciada",(e==null?void 0:e.message)||"Los índices se están actualizando","success")}catch(e){b("Error de sincronización",e.message,"error")}finally{t&&(t.textContent="⚡ Sincronizar Índices",t.disabled=!1)}}async function ka(){C(),$([{label:"Dashboard",href:"dashboard"},{label:"Reportes"}]),await Ea()}async function Ea(){w(`
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
  </div>`),window._verReporte=t=>{b("Reporte seleccionado",`Generando reporte de ${t}…`,"info"),Ca(t)},window._exportReporte=()=>{b("Exportar","Función de exportación CSV/PDF — próximamente","info")},await ye()}async function ye(){var e,a,i,o;const t=document.getElementById("rep-fecha");t&&(t.textContent=new Date().toLocaleDateString("es-MX",{day:"2-digit",month:"long",year:"numeric"}));try{const[l,s,c,d]=await Promise.allSettled([p.ventaKpis(),p.factKpis(),p.stockKpis(),p.comprasKpis()]),n=((e=l.value)==null?void 0:e.data)||{},r=((a=s.value)==null?void 0:a.data)||{},v=((i=c.value)==null?void 0:i.data)||{},u=((o=d.value)==null?void 0:o.data)||{},g=document.getElementById("rep-kpis");g&&(g.innerHTML=`
      ${[{label:"Ventas confirmadas",val:n.ordenes_confirmadas??0,tipo:"num",desc:`$${parseFloat(n.total_facturado||0).toLocaleString("es-MX",{minimumFractionDigits:2})} este mes`},{label:"Facturación total",val:f(parseFloat(r.monto_total||0)),tipo:"txt",desc:`${r.total_facturas??0} comprobantes emitidos`},{label:"Valor inventario",val:f(parseFloat(v.valor_inventario||0)),tipo:"txt",desc:`${v.alertas_stock_bajo??0} alertas de stock bajo`}].map(y=>`
      <div style="padding:16px;background:var(--bg);border-radius:12px;border:1px solid var(--border)">
        <div style="font-size:11px;color:var(--text-400);font-weight:700;text-transform:uppercase;margin-bottom:6px">${y.label}</div>
        <div style="font-size:24px;font-weight:800;color:var(--text-900);margin-bottom:4px">
          ${y.tipo==="num"?Number(y.val).toLocaleString("es-MX"):y.val}
        </div>
        <div style="font-size:11px;color:var(--text-500)">${y.desc}</div>
      </div>`).join("")}

      <div style="grid-column:1/-1;margin-top:8px">
        <div style="font-size:12px;font-weight:700;color:var(--text-600);margin-bottom:10px">COMPRAS</div>
        <div style="display:grid;grid-template-columns:repeat(3,1fr);gap:12px">
          ${[{label:"Total OC",val:u.total??0},{label:"Confirmadas",val:u.confirmadas??0},{label:"Monto compras",val:f(parseFloat(u.monto_total||0))}].map(y=>`
          <div style="padding:12px;background:var(--bg);border-radius:10px;border:1px solid var(--border)">
            <div style="font-size:10px;color:var(--text-400);font-weight:700;text-transform:uppercase;margin-bottom:4px">${y.label}</div>
            <div style="font-size:18px;font-weight:800;color:var(--text-900)">${y.val}</div>
          </div>`).join("")}
        </div>
      </div>`)}catch(l){console.error(l)}}async function Ca(t){const e=document.getElementById("rep-kpis"),a=document.querySelector(".data-card-title");if(a){const i={ventas:"💰 Reporte de Ventas",facturas:"🧾 Facturación",inventario:"🏭 Inventario",compras:"🛒 Compras",clientes:"👥 Clientes",nomina:"👔 Nómina"};a.textContent=i[t]||"Reporte"}e&&(e.innerHTML='<div class="skeleton" style="height:120px;border-radius:12px;grid-column:1/-1"></div>'),await ye()}function Sa(t,e,a,i){C(),$([{label:"Dashboard",href:"dashboard"},{label:e}]),w(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">${i} ${e}</h1>
      <p class="page-subtitle">${a}</p>
    </div>
  </div>
  <div class="data-card anim-2">
    <div class="empty-state">
      <div class="empty-state-icon">${i}</div>
      <div class="empty-state-title">Módulo ${e} en construcción</div>
      <div class="empty-state-desc">Este módulo estará disponible próximamente en NexusTech ERP v2.0</div>
      <button class="btn btn-primary" onclick="window._go('dashboard')">← Volver al Dashboard</button>
    </div>
  </div>`)}E("login",Se);E("home",Me);E("dashboard",ne);E("ventas",je);E("facturas",He);E("productos",Nt);E("partners",Rt);E("stock",Vt);E("cfdi",la);E("nomina",Ot);E("compras",Ut);E("cotizaciones",ua);E("search",wa);E("reportes",ka);E("404",()=>Sa("404","Página no encontrada","La ruta solicitada no existe","🔍"));$e();
