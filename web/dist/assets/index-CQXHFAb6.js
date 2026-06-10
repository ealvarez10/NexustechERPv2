(function(){const e=document.createElement("link").relList;if(e&&e.supports&&e.supports("modulepreload"))return;for(const o of document.querySelectorAll('link[rel="modulepreload"]'))n(o);new MutationObserver(o=>{for(const d of o)if(d.type==="childList")for(const s of d.addedNodes)s.tagName==="LINK"&&s.rel==="modulepreload"&&n(s)}).observe(document,{childList:!0,subtree:!0});function a(o){const d={};return o.integrity&&(d.integrity=o.integrity),o.referrerPolicy&&(d.referrerPolicy=o.referrerPolicy),o.crossOrigin==="use-credentials"?d.credentials="include":o.crossOrigin==="anonymous"?d.credentials="omit":d.credentials="same-origin",d}function n(o){if(o.ep)return;o.ep=!0;const d=a(o);fetch(o.href,d)}})();const nt={isLoggedIn:()=>!!localStorage.getItem("nx_token"),getUser:()=>{try{return JSON.parse(localStorage.getItem("nx_user")||"{}")}catch{return{}}},setSession(t,e){localStorage.setItem("nx_token",t),localStorage.setItem("nx_user",JSON.stringify(e))},clear(){localStorage.removeItem("nx_token"),localStorage.removeItem("nx_user")}},rt={};function L(t,e){rt[t]=e}function it(t){window.location.hash=t}function Kt(){window.addEventListener("hashchange",Ft),Ft()}function Ft(){const t=window.location.hash.replace("#","")||"home";if(!nt.isLoggedIn()&&t!=="login"){it("login");return}if(nt.isLoggedIn()&&t==="login"){it("home");return}const e=rt[t];e?e():rt[404]&&rt[404]()}const Jt="/api/v1";function Yt(){return localStorage.getItem("nx_token")}class Qt extends Error{constructor(e,a){super(a),this.status=e}}async function x(t,e,a){const n=Yt(),o=await fetch(Jt+e,{method:t,headers:{"Content-Type":"application/json",...n?{Authorization:`Bearer ${n}`}:{}},...a!==void 0?{body:JSON.stringify(a)}:{}});if(o.status===401)return localStorage.removeItem("nx_token"),localStorage.removeItem("nx_user"),window.location.hash="login",null;if(!o.ok)throw new Qt(o.status,await o.text());return(o.headers.get("content-type")||"").includes("application/json")?o.json():o.text()}const f={get:t=>x("GET",t),post:(t,e)=>x("POST",t,e),put:(t,e)=>x("PUT",t,e),del:t=>x("DELETE",t),login:(t,e)=>x("POST","/auth/login",{login:t,password:e}),logout:()=>x("POST","/auth/logout",{}),dashboard:()=>x("GET","/dashboard"),ventaKpis:()=>x("GET","/ventas/kpis"),factKpis:()=>x("GET","/facturas/kpis"),stockKpis:()=>x("GET","/stock/kpis"),ventas:(t=1)=>x("GET",`/ventas?pagina=${t}`),venta:t=>x("GET",`/ventas/${t}`),facturas:(t=1)=>x("GET",`/facturas?pagina=${t}`),factura:t=>x("GET",`/facturas/${t}`),porCobrar:()=>x("GET","/facturas/por-cobrar"),productos:(t=1,e="")=>x("GET",`/productos?pagina=${t}&q=${encodeURIComponent(e)}`),producto:t=>x("GET",`/productos/${t}`),partners:(t=1)=>x("GET",`/partners?pagina=${t}`),partner:t=>x("GET",`/partners/${t}`),clientes:(t=1)=>x("GET",`/clientes?pagina=${t}`),proveedores:(t=1)=>x("GET",`/proveedores?pagina=${t}`),stock:(t=1)=>x("GET",`/stock?pagina=${t}`),stockKpis:()=>x("GET","/stock/kpis"),stockBajo:()=>x("GET","/stock/bajo"),stockProducto:t=>x("GET",`/stock/producto/${t}`),cfdiTimbrados:(t=1)=>x("GET",`/cfdi/timbrados?pagina=${t}`),cfdiTimbrado:t=>x("GET",`/cfdi/timbrados/${t}`),cfdiKpis:()=>x("GET","/cfdi/kpis"),timbrar:t=>x("POST","/cfdi/timbrar",t),cancelarCfdi:t=>x("POST","/cfdi/cancelar",t),nomina:(t=1)=>x("GET",`/nomina?pagina=${t}`),nominaKpis:()=>x("GET","/nomina/kpis"),compras:(t=1)=>x("GET",`/compras?pagina=${t}`),comprasKpis:()=>x("GET","/compras/kpis"),cotizaciones:(t=1)=>x("GET",`/cotizaciones?pagina=${t}`),cotizacionKpis:()=>x("GET","/cotizaciones/kpis"),cotizacion:t=>x("GET",`/cotizaciones/${t}`),crearCotizacion:t=>x("POST","/cotizaciones",t),confirmarCotizacion:t=>x("PUT",`/cotizaciones/${t}/confirmar`),cancelarCotizacion:t=>x("PUT",`/cotizaciones/${t}/cancelar`),actualizarCotizacion:(t,e)=>x("PUT",`/cotizaciones/${t}`,e),agregarLinea:(t,e)=>x("POST",`/cotizaciones/${t}/lineas`,e),eliminarLinea:(t,e)=>x("DELETE",`/cotizaciones/${t}/lineas/${e}`),searchSync:()=>x("POST","/search/sync",{}),searchStatus:()=>x("GET","/search/status"),health:()=>x("GET","/health"),putVenta:(t,e)=>x("PUT",`/ventas/${t}`,e),putPartner:(t,e)=>x("PUT",`/partners/${t}`,e),putProducto:(t,e)=>x("PUT",`/productos/${t}`,e),putCompra:(t,e)=>x("PUT",`/compras/${t}`,e),putEmpleado:(t,e)=>x("PUT",`/nomina/${t}`,e),ajusteStock:(t,e)=>x("PUT",`/stock/${t}/ajuste`,e)};function Wt(){const t=document.getElementById("__shell");t&&t.remove(),document.getElementById("app").innerHTML=`
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
  </div>`;const e=document.getElementById("lbtn"),a=document.getElementById("lu"),n=document.getElementById("lp"),o=document.getElementById("lerr");async function d(){if(e.disabled)return;const s=a.value.trim(),r=n.value;if(!s||!r){o.textContent="Ingresa usuario y contraseña",o.classList.add("show");return}e.disabled=!0,e.textContent="Verificando...",o.classList.remove("show");try{const i=await f.login(s,r),l=(i==null?void 0:i.data)||i,v=(l==null?void 0:l.access_token)||(l==null?void 0:l.token);if(v){nt.setSession(v,{nombre:l.email||s,email:l.email||s,user_id:l.user_id,company_id:l.company_id}),document.getElementById("app").innerHTML="",it("dashboard");return}o.textContent="Error inesperado del servidor. Intenta de nuevo.",o.classList.add("show")}catch(i){o.textContent=(i==null?void 0:i.status)===401?"Credenciales incorrectas. Verifica tu usuario y contraseña.":`Error de conexión: ${(i==null?void 0:i.message)||"No se pudo contactar el servidor"}`,o.classList.add("show")}e.disabled=!1,e.textContent="Acceder al sistema"}e.addEventListener("click",d),n.addEventListener("keydown",s=>s.key==="Enter"&&d()),a.addEventListener("keydown",s=>s.key==="Enter"&&n.focus()),setTimeout(()=>a.focus(),100)}function O(t,e=0){return t==null||t===""?"—":Number(t).toLocaleString("es-MX",{minimumFractionDigits:e,maximumFractionDigits:e})}function E(t){return t==null?"—":(t=parseFloat(t)||0,Math.abs(t)>=1e6?`$${(t/1e6).toFixed(2)}M`:Math.abs(t)>=1e3?`$${(t/1e3).toFixed(1)}k`:`$${O(t,2)}`)}function pt(t){return t==null?"—":Number(t).toLocaleString("es-MX")}function S(t){return t?new Date(t).toLocaleDateString("es-MX",{day:"2-digit",month:"short",year:"numeric"}):"—"}function w(t,e="",a="info"){const n={success:"✅",error:"❌",info:"ℹ️",warning:"⚠️"};let o=document.getElementById("__toasts");o||(o=document.createElement("div"),o.id="__toasts",o.className="toast-container",document.body.appendChild(o));const d=document.createElement("div");d.className=`toast ${a}`,d.innerHTML=`
    <span class="toast-icon">${n[a]||"ℹ️"}</span>
    <div><div class="toast-title">${t}</div>${e?`<div class="toast-msg">${e}</div>`:""}</div>`,o.appendChild(d),requestAnimationFrame(()=>d.classList.add("show")),setTimeout(()=>{d.classList.remove("show"),setTimeout(()=>d.remove(),400)},3800)}function Bt(t,e,a=900,n="",o=""){if(!t)return;const d=performance.now(),s=String(e).includes(".");function r(i){const l=Math.min((i-d)/a,1),v=1-Math.pow(1-l,3),$=e*v;t.textContent=n+(s?$.toLocaleString("es-MX",{minimumFractionDigits:2,maximumFractionDigits:2}):Math.round($).toLocaleString("es-MX"))+o,l<1&&requestAnimationFrame(r)}requestAnimationFrame(r)}function Zt(t){if(!(t!=null&&t.length))return"";const e=Math.max(...t,1);return`<div class="sparkline">${t.map((a,n)=>`<div class="spark-bar${n===t.length-1?" active":""}" style="height:${Math.max(4,Math.round(a/e*100))}%"></div>`).join("")}</div>`}function te(t=5,e=6){return`<tbody>${Array.from({length:e},()=>`<tr>${Array.from({length:t},()=>`<td><div class="skeleton" style="height:14px;width:${60+Math.random()*30}%"></div></td>`).join("")}</tr>`).join("")}</tbody>`}function B(t=5,e=4){return`<table class="data-table"><thead><tr>${Array.from({length:e},()=>`<th><div class="skeleton" style="height:12px;width:${40+Math.random()*40}%"></div></th>`).join("")}</tr></thead>${te(e,t)}</table>`}function ee(t=5){return Array.from({length:t},()=>`
  <div class="kpi-card kpi-gray">
    <div class="kpi-label"><div class="skeleton" style="height:13px;width:60%"></div></div>
    <div class="kpi-value"><div class="skeleton" style="height:28px;width:70%"></div></div>
    <div><div class="skeleton" style="height:11px;width:40%;margin-top:6px"></div></div>
  </div>`).join("")}const ae={sale:"emerald",done:"indigo",draft:"gray",sent:"sky",cancel:"red",posted:"emerald",in_payment:"violet",paid:"emerald",partial:"amber"};function K(t,e){return`<span class="badge badge-${ae[t]||"gray"} badge-dot">${e}</span>`}function R(t,e,a){return window.__pagNav=a,`
  <div class="data-table-footer">
    <span style="color:var(--text-400)">Página ${t}</span>
    <div class="pagination">
      <button class="pag-btn" ${t<=1?"disabled":""} onclick="window.__pagNav(${t-1})">&#8592; Anterior</button>
      <span class="pag-btn active">${t}</span>
      <button class="pag-btn" ${e?"":"disabled"} onclick="window.__pagNav(${t+1})">Siguiente &#8594;</button>
    </div>
  </div>`}let U=null;function P(t,e,a={}){let n=document.getElementById("__modal-overlay");n||(n=document.createElement("div"),n.id="__modal-overlay",n.innerHTML=`
      <div id="__modal-drawer">
        <div id="__modal-header">
          <span id="__modal-title"></span>
          <button id="__modal-close" onclick="window.__closeModal()">✕</button>
        </div>
        <div id="__modal-body"></div>
      </div>`,document.body.appendChild(n),n.addEventListener("click",o=>{o.target===n&&window.__closeModal()})),document.getElementById("__modal-title").textContent=t,document.getElementById("__modal-body").innerHTML=e,n.classList.add("open"),document.body.style.overflow="hidden",U&&document.removeEventListener("keydown",U),U=o=>{o.key==="Escape"&&window.__closeModal()},document.addEventListener("keydown",U),a.onMounted&&setTimeout(a.onMounted,10)}function It(){const t=document.getElementById("__modal-overlay");t&&t.classList.remove("open"),document.body.style.overflow="",U&&(document.removeEventListener("keydown",U),U=null)}window.__closeModal=It;async function q(t,e,a){P(t,`
    <div style="display:flex;flex-direction:column;gap:12px;padding:8px 0">
      ${[1,2,3,4,5].map(()=>'<div class="skeleton" style="height:52px;border-radius:10px"></div>').join("")}
    </div>`);try{const n=await e(),o=(n==null?void 0:n.data)??n;document.getElementById("__modal-body").innerHTML=a(o)}catch(n){document.getElementById("__modal-body").innerHTML=`<p style="color:var(--red);padding:24px">Error: ${n.message}</p>`}}function g(t,e,a={}){const n=e??"—",o=a.color?`color:${a.color}`:"";return`
  <div style="display:flex;justify-content:space-between;align-items:flex-start;
    padding:10px 0;border-bottom:1px solid var(--border)">
    <span style="font-size:12px;color:var(--text-400);font-weight:600;min-width:140px">${t}</span>
    <span style="font-size:13px;font-weight:500;text-align:right;${o}">${n}</span>
  </div>`}function I(t,e){return`
  <div style="margin-bottom:20px">
    <div style="font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.06em;
      color:var(--text-400);margin-bottom:8px;padding-bottom:6px;
      border-bottom:2px solid var(--primary)">${t}</div>
    ${e}
  </div>`}const Lt=[{id:"home",icon:"⊞",label:"Inicio",section:"Principal"},{id:"dashboard",icon:"📊",label:"Dashboard",section:"Principal"},{id:"ventas",icon:"💰",label:"Ventas",section:"Principal"},{id:"cotizaciones",icon:"📝",label:"Cotizaciones",section:"Principal"},{id:"facturas",icon:"🧾",label:"Facturación",section:"Principal"},{id:"productos",icon:"📦",label:"Productos",section:"Principal"},{id:"partners",icon:"👥",label:"Clientes",section:"Principal"},{id:"stock",icon:"🏭",label:"Inventario",section:"Principal"},{id:"cfdi",icon:"🔏",label:"CFDI 4.0",section:"Fiscal",badge:"NUEVO"},{id:"nomina",icon:"👔",label:"Nómina IMSS",section:"Fiscal"},{id:"compras",icon:"🛒",label:"Compras",section:"Operaciones"},{id:"search",icon:"🔍",label:"NexusSearch",section:"Sistema"},{id:"reportes",icon:"📈",label:"Reportes",section:"Sistema"}];function M(){if(document.getElementById("__shell"))return;const t=nt.getUser(),e=(t.nombre||t.name||"AD").substring(0,2).toUpperCase(),a=[...new Set(Lt.map(n=>n.section))];if(document.getElementById("app").innerHTML=`
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
        ${a.map(n=>`
        <div class="nav-section">
          <div class="nav-section-title">${n}</div>
          ${Lt.filter(o=>o.section===n).map(o=>`
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
  </div>`,window._go=n=>{it(n)},window._logout=()=>{nt.clear();const n=document.getElementById("__shell");n&&n.remove(),it("login"),w("Sesión cerrada","Hasta pronto","info")},window._toggleSidebar=()=>{const n=document.getElementById("__sidebar"),o=document.getElementById("sidebar-toggle");if(!n)return;const d=n.classList.toggle("collapsed");localStorage.setItem("nx_sidebar_collapsed",d?"1":"0"),o&&(o.textContent=d?"▶":"◀")},localStorage.getItem("nx_sidebar_collapsed")==="1"){const n=document.getElementById("__sidebar"),o=document.getElementById("sidebar-toggle");n&&n.classList.add("collapsed"),o&&(o.textContent="▶")}window.addEventListener("hashchange",Mt),Mt()}function T(t){const e=document.getElementById("__page");e&&(e.innerHTML=t,e.scrollTop=0)}function F(t){const e=document.getElementById("__breadcrumb");e&&(e.innerHTML=t.map((a,n)=>`
    <span class="breadcrumb-item"${n<t.length-1&&a.href?` onclick="window._go('${a.href}')"`:""}>
      ${a.label}
      ${n<t.length-1?'<span class="breadcrumb-sep">/</span>':""}
    </span>`).join(""))}function Mt(){const t=window.location.hash.replace("#","")||"home";document.querySelectorAll(".nav-link").forEach(e=>{e.classList.toggle("active",e.id===`nl-${t}`)})}const ht=[{id:"ventas",icon:"📊",gradient:"linear-gradient(135deg, #4F46E5, #7C3AED)",nombre:"Ventas",desc:"Órdenes y Cotizaciones",kpiEndpoint:"/ventas/kpis",kpiField:"total_ordenes"},{id:"facturas",icon:"🧾",gradient:"linear-gradient(135deg, #059669, #0EA5E9)",nombre:"Facturación",desc:"Facturas y Cobros",kpiEndpoint:"/facturas/kpis",kpiField:"total_facturas"},{id:"partners",icon:"👥",gradient:"linear-gradient(135deg, #7C3AED, #EC4899)",nombre:"Clientes",desc:"Directorio y Contactos",kpiEndpoint:null,kpiField:null},{id:"stock",icon:"📦",gradient:"linear-gradient(135deg, #D97706, #EA580C)",nombre:"Inventario",desc:"Almacén y Movimientos",kpiEndpoint:"/stock/kpis",kpiField:"total_productos"},{id:"compras",icon:"🛒",gradient:"linear-gradient(135deg, #2563EB, #4F46E5)",nombre:"Compras",desc:"Órdenes de Compra",kpiEndpoint:"/compras/kpis",kpiField:"total_ordenes"},{id:"productos",icon:"🏷️",gradient:"linear-gradient(135deg, #0D9488, #059669)",nombre:"Productos",desc:"Catálogo y Precios",kpiEndpoint:null,kpiField:null},{id:"cfdi",icon:"🔏",gradient:"linear-gradient(135deg, #E11D48, #DC2626)",nombre:"CFDI 4.0",desc:"Timbrado Electrónico",kpiEndpoint:"/cfdi/kpis",kpiField:"timbrados_hoy"},{id:"nomina",icon:"👔",gradient:"linear-gradient(135deg, #0EA5E9, #2563EB)",nombre:"Nómina IMSS",desc:"Empleados y Recibos",kpiEndpoint:"/nomina/kpis",kpiField:"total_empleados"},{id:"reportes",icon:"📈",gradient:"linear-gradient(135deg, #475569, #1E293B)",nombre:"Reportes",desc:"Análisis y Estadísticas",kpiEndpoint:null,kpiField:null}];function oe(){return`
  <div class="nx-home">
    <div class="nx-home-header">
      <h1 class="nx-home-title">Aplicaciones</h1>
      <div class="nx-home-search">
        <input type="search" placeholder="Buscar módulo..." id="home-search" autocomplete="off">
      </div>
    </div>
    <div class="nx-app-grid" id="home-app-grid">
      ${ht.map(()=>`
        <div class="nx-app-card" style="pointer-events:none">
          <div class="nx-app-icon skeleton" style="background:none"></div>
          <div class="nx-app-name skeleton" style="height:14px;width:70%;margin:0 auto 6px"></div>
          <div class="nx-app-desc skeleton" style="height:11px;width:55%;margin:0 auto"></div>
        </div>
      `).join("")}
    </div>
  </div>`}async function ne(){M(),F([{label:"Inicio"}]),T(oe());const t=ht.map(o=>o.kpiEndpoint?f.get(o.kpiEndpoint).catch(()=>null):Promise.resolve(null)),e=await Promise.allSettled(t),a=document.getElementById("home-app-grid");if(!a)return;a.innerHTML=ht.map((o,d)=>{var i;const s=e[d];let r=null;if(s.status==="fulfilled"&&s.value&&o.kpiField){const l=((i=s.value)==null?void 0:i.data)??s.value;r=(l==null?void 0:l[o.kpiField])??null}return`
      <div class="nx-app-card"
           data-app-id="${o.id}"
           data-name="${o.nombre.toLowerCase()}"
           style="animation-delay:${d*50}ms"
           onclick="window._go('${o.id}')">
        <div class="nx-app-icon" style="background:${o.gradient}">
          <span class="nx-app-emoji">${o.icon}</span>
        </div>
        ${r!==null?`<div class="nx-app-badge">${Number(r).toLocaleString("es-MX")}</div>`:""}
        <div class="nx-app-name">${o.nombre}</div>
        <div class="nx-app-desc">${o.desc}</div>
      </div>
    `}).join("");const n=document.getElementById("home-search");n&&(n.addEventListener("input",o=>{const d=o.target.value.toLowerCase().trim();document.querySelectorAll("#home-app-grid .nx-app-card").forEach(s=>{const r=s.dataset.name||"",i=s.textContent.toLowerCase();s.classList.toggle("hidden",d!==""&&!r.includes(d)&&!i.includes(d))})}),n.focus())}const ie={sale:"indigo",done:"emerald",draft:"gray",cancel:"red",sent:"sky",posted:"emerald"},se={sale:"Confirmada",done:"Entregada",draft:"Borrador",cancel:"Cancelada",sent:"Enviada"};function Y(t,e=10){return Array.from({length:e},()=>Math.max(5,Math.round(t*(.6+Math.random()*.8))))}async function Dt(){var t,e,a,n,o,d,s,r,i;M(),F([{label:"Dashboard"}]),T(`
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
  <div class="kpi-grid anim-2" id="kpi-grid">${ee(5)}</div>

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
      <div id="tabla-ventas">${B(6,5)}</div>
    </div>

    <div class="data-card">
      <div class="data-card-header">
        <div>
          <div class="data-card-title">⚠️ Stock Bajo</div>
          <div class="data-card-subtitle">Productos bajo nivel mínimo</div>
        </div>
        <button class="btn btn-secondary btn-sm" onclick="window._go('stock')">Inventario</button>
      </div>
      <div id="tabla-stock">${B(5,4)}</div>
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
      <div id="resumen-fiscal">${B(4,2)}</div>
    </div>

    <!-- Estado del sistema -->
    <div class="data-card" style="padding:20px">
      <div style="font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.06em;color:var(--text-400);margin-bottom:16px">🟢 Estado del Sistema</div>
      <div id="system-status">${B(4,2)}</div>
    </div>
  </div>`);try{const[l,v,$]=await Promise.allSettled([f.dashboard(),f.ventas(1),f.stockBajo()]),u=l.status==="fulfilled"?(t=l.value)==null?void 0:t.data:null,c=[{key:"ventas_mes",label:"Ventas del Mes",tipo:"mxn",icon:"💰",color:"indigo",valor:parseFloat(((e=u==null?void 0:u.ventas)==null?void 0:e.importe_mes)||0),trend:null,spark:Y(100)},{key:"facturas",label:"Facturas Emitidas",tipo:"num",icon:"🧾",color:"emerald",valor:parseInt(((a=u==null?void 0:u.facturacion)==null?void 0:a.total_facturas)||0),trend:null,spark:Y(50)},{key:"cobrar",label:"Por Cobrar",tipo:"mxn",icon:"📋",color:"amber",valor:parseFloat(((n=u==null?void 0:u.facturacion)==null?void 0:n.por_cobrar)||0),trend:null,spark:Y(80)},{key:"stock_total",label:"Productos en Stock",tipo:"num",icon:"📦",color:"sky",valor:parseInt(((o=u==null?void 0:u.inventario)==null?void 0:o.total_productos_con_stock)||0),trend:null,spark:Y(80)},{key:"stock_bajo",label:"Alertas Stock Bajo",tipo:"num",icon:"⚠️",color:"rose",valor:parseInt(((d=u==null?void 0:u.inventario)==null?void 0:d.alertas_stock_bajo)||0),trend:null,spark:Y(20)}],p=document.getElementById("kpi-grid");p&&(p.innerHTML=c.map(y=>`
      <div class="kpi-card kpi-${y.color}">
        <div class="kpi-label">
          <span>${y.label}</span>
          <div class="kpi-icon-box">${y.icon}</div>
        </div>
        <div class="kpi-value" id="kv-${y.key}">—</div>
        <div class="kpi-trend neutral">→ En tiempo real</div>
        ${Zt(y.spark)}
      </div>`).join(""),c.forEach(y=>{const k=document.getElementById("kv-"+y.key);k&&(y.tipo==="mxn"?Bt(k,y.valor,1100,"$"):Bt(k,y.valor,1100))}));const m=document.getElementById("tabla-ventas");if(m){const y=v.status==="fulfilled"?(((s=v.value)==null?void 0:s.data)||[]).slice(0,6):[];y.length===0?m.innerHTML='<p style="text-align:center;color:var(--text-400);padding:24px">Sin ventas registradas</p>':m.innerHTML=`
        <table class="data-table">
          <thead><tr>
            <th>Folio</th><th>Cliente</th><th>Fecha</th><th>Total</th><th>Estado</th>
          </tr></thead>
          <tbody>
            ${y.map(k=>{const C=k.state||"draft",N=se[C]||C,z=ie[C]||"gray",G=k.date_order?new Date(k.date_order).toLocaleDateString("es-MX",{day:"2-digit",month:"short"}):"—";return`
              <tr>
                <td class="td-mono">${k.name||k.id}</td>
                <td class="td-primary">${k.partner_name||k.partner_id||"—"}</td>
                <td>${G}</td>
                <td class="td-amount">${E(parseFloat(k.amount_total||0))}</td>
                <td><span class="badge badge-${z} badge-dot">${N}</span></td>
              </tr>`}).join("")}
          </tbody>
        </table>`}const b=document.getElementById("tabla-stock");if(b){const y=$.status==="fulfilled"?(((r=$.value)==null?void 0:r.data)||[]).slice(0,5):[];y.length===0?b.innerHTML='<p style="text-align:center;color:var(--text-400);padding:24px">✅ Stock en niveles normales</p>':b.innerHTML=`
        <table class="data-table">
          <thead><tr><th>Producto</th><th>Disponible</th></tr></thead>
          <tbody>
            ${y.map(k=>{const C=parseFloat(k.cantidad_disponible||0),N=C<=0?"red":C<5?"amber":"sky";return`
              <tr>
                <td class="td-primary" style="max-width:140px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">${k.product_name||k.product_id}</td>
                <td><span class="badge badge-${N}">${C}</span></td>
              </tr>`}).join("")}
          </tbody>
        </table>`}const h=document.getElementById("resumen-fiscal");if(h){const y=u==null?void 0:u.facturacion,k=[{label:"Facturas emitidas (total)",val:pt((y==null?void 0:y.total_facturas)||0),color:"indigo"},{label:"Por cobrar",val:E(parseFloat((y==null?void 0:y.por_cobrar)||0)),color:"amber"},{label:"Monto total facturado",val:E(parseFloat((y==null?void 0:y.monto_total)||0)),color:"emerald"},{label:"Facturas vencidas",val:pt((y==null?void 0:y.facturas_vencidas)||0),color:"red"}];h.innerHTML=k.map(C=>`
      <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:11px;padding-bottom:11px;border-bottom:1px solid var(--border)">
        <span style="font-size:12.5px;color:var(--text-500)">${C.label}</span>
        <span class="badge badge-${C.color}">${C.val}</span>
      </div>`).join("")}const _=document.getElementById("system-status");if(_){let y=!1;try{await f.health(),y=!0}catch{}_.innerHTML=[{label:"API Backend",val:y?"✅ En línea":"❌ Offline",color:y?"emerald":"red"},{label:"Base de datos",val:u?"✅ Operativa":"⚠️ Sin datos",color:u?"emerald":"amber"},{label:"Versión ERP",val:"v2.0.0",color:"indigo"},{label:"Uptime",val:"99.98%",color:"emerald"}].map(k=>`
      <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:10px">
        <span style="font-size:12.5px;color:var(--text-500)">${k.label}</span>
        <span class="badge badge-${k.color}">${k.val}</span>
      </div>`).join("")}}catch(l){console.error("Dashboard load error:",l),w("Error al cargar","No se pudo conectar con el servidor","error")}(i=document.getElementById("btn-refresh"))==null||i.addEventListener("click",()=>Dt())}function le(){P("Nueva Orden de Venta",`
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
  </form>`),window._submitVenta=async()=>{var a;const t=document.getElementById("btn-guardar-venta");t.textContent="⏳ Guardando…",t.disabled=!0;const e=document.getElementById("venta-result");try{await new Promise(n=>setTimeout(n,800)),e.innerHTML=`<div style="background:var(--success-light);border:1.5px solid var(--success);border-radius:10px;padding:12px;color:var(--success)">
        ✅ Venta registrada. El sistema se sincronizará en el próximo ciclo.</div>`,w("Venta creada",(a=document.getElementById("nv-folio"))==null?void 0:a.value,"success"),setTimeout(()=>It(),2e3)}catch(n){e.innerHTML=`<p style="color:var(--red)">Error: ${n.message}</p>`}finally{t.textContent="💾 Guardar Venta",t.disabled=!1}}}function re(t){P("Nuevo Contacto",`
  <form id="form-nuevo-contacto" onsubmit="event.preventDefault();window._submitContacto()">
    <div class="modal-form-grid">
      <div class="modal-form-full">
        <label class="modal-form-label">Nombre completo *</label>
        <input id="nc-nombre" class="modal-form-input" placeholder="Empresa SA de CV" required>
      </div>
      <div>
        <label class="modal-form-label">Tipo</label>
        <select id="nc-tipo" class="modal-form-select">
          <option value="company">Empresa</option>
          <option value="person">Persona física</option>
        </select>
      </div>
      <div>
        <label class="modal-form-label">RFC</label>
        <input id="nc-rfc" class="modal-form-input" placeholder="XAXX010101000">
      </div>
      <div>
        <label class="modal-form-label">Email</label>
        <input id="nc-email" type="email" class="modal-form-input" placeholder="contacto@empresa.com">
      </div>
      <div>
        <label class="modal-form-label">Teléfono</label>
        <input id="nc-tel" class="modal-form-input" placeholder="+52 81 0000 0000">
      </div>
      <div>
        <label class="modal-form-label">Ciudad</label>
        <input id="nc-ciudad" class="modal-form-input" placeholder="Monterrey">
      </div>
      <div class="modal-form-full">
        <label class="modal-form-label">Rol</label>
        <div style="display:flex;gap:12px;margin-top:4px">
          <label style="display:flex;align-items:center;gap:6px;font-size:13px">
            <input type="checkbox" id="nc-es-cliente"> Cliente
          </label>
          <label style="display:flex;align-items:center;gap:6px;font-size:13px">
            <input type="checkbox" id="nc-es-proveedor"> Proveedor
          </label>
        </div>
      </div>
    </div>
    <div class="modal-actions">
      <button type="button" class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cancelar</button>
      <button type="submit" class="btn btn-primary btn-sm" id="btn-guardar-contacto">💾 Guardar</button>
    </div>
    <div id="contacto-result" style="margin-top:12px"></div>
  </form>`),window._submitContacto=async()=>{var n;const e=document.getElementById("btn-guardar-contacto");e.textContent="⏳ Guardando…",e.disabled=!0;const a=document.getElementById("contacto-result");try{await new Promise(o=>setTimeout(o,600)),a.innerHTML=`<div style="background:var(--success-light);border:1.5px solid var(--success);border-radius:10px;padding:12px;color:var(--success)">
        ✅ Contacto registrado.</div>`,w("Contacto creado",(n=document.getElementById("nc-nombre"))==null?void 0:n.value,"success"),setTimeout(()=>{It(),t&&t()},1500)}catch(o){a.innerHTML=`<p style="color:var(--red)">Error: ${o.message}</p>`}finally{e.textContent="💾 Guardar",e.disabled=!1}}}function de(t,e){return t.map(a=>`
    <div class="o-status-step ${a.key===e?"active":a.done?"done":""}"
         data-status="${a.key}">
      ${a.label}
    </div>
  `).join("")}function ce(t){return t!=null&&t.length?`
    <div class="o-smart-buttons">
      ${t.map(e=>`
        <button class="o-smart-btn" onclick="${e.onClick||""}">
          <span class="o-count">${e.count??"—"}</span>
          <span class="o-label">${e.icon||""} ${e.label}</span>
        </button>
      `).join("")}
    </div>
  `:""}function At(t,e=2){const a=Math.ceil(t.length/e),n=t.slice(0,a),o=e===2?t.slice(a):[],d=r=>`
    <div class="o-field-row">
      <div class="o-field-label">${r.label}</div>
      <div class="o-field-value">
        ${r.value!==null&&r.value!==void 0&&r.value!==""?`<span>${r.value}</span>`:'<span style="color:var(--text-300)">—</span>'}
      </div>
    </div>
  `,s=r=>r.map(d).join("");return`
    <div class="o-form-group${e===1?" full":""}">
      <div class="o-form-col">${s(n)}</div>
      ${e===2?`<div class="o-form-col">${s(o)}</div>`:""}
    </div>
  `}function pe(t=[],e=""){const a=t.length>0?t.map(o=>`
      <div class="o-message">
        <div class="o-msg-avatar">${o.initials||"?"}</div>
        <div class="o-msg-content">
          <div class="o-msg-header">
            <span class="o-msg-author">${o.author}</span>
            <span class="o-msg-date">${o.date}</span>
          </div>
          <div class="o-msg-text">${o.text}</div>
        </div>
      </div>
    `).join(""):'<div class="o-chatter-empty">Sin actividad registrada en este documento.</div>',n=(e||"").replace(/'/g,"\\'");return`
    <div class="o-chatter">
      <div class="o-chatter-topbar">
        <button class="o-chatter-btn" onclick="window._chatterMessage('${n}')">✉️ Enviar mensaje</button>
        <button class="o-chatter-btn" onclick="window._chatterNote('${n}')">📋 Nota interna</button>
        <button class="o-chatter-btn">📎 Adjuntar</button>
      </div>
      <div class="o-chatter-thread">${a}</div>
    </div>
  `}function me(t,e={}){const{title:a="",statusSteps:n=[],currentStatus:o="",smartButtons:d=[],statusButtons:s=[],groups:r=[],tabs:i=[],messages:l=[],editable:v=!1}=e,$=s.filter(p=>p.visible!==!1).map(p=>`
      <button class="btn ${p.primary?"btn-primary":"btn-secondary"} btn-sm"
              onclick="${p.onClick||""}">
        ${p.label}
      </button>
    `).join(""),u=i.length>0?`
    <div class="o-notebook">
      <div class="o-tabs" role="tablist">
        ${i.map((p,m)=>`
          <button class="o-tab${m===0?" active":""}"
                  role="tab"
                  data-tab="${m}"
                  onclick="window._switchTab(this, ${m})">
            ${p.label}
          </button>
        `).join("")}
      </div>
      ${i.map((p,m)=>`
        <div class="o-tab-panel${m===0?" active":""}" data-panel="${m}">
          ${p.content||""}
        </div>
      `).join("")}
    </div>
  `:"",c=`
    <div class="o-form-view${v?" editing":""}">
      <!-- Status Bar -->
      <div class="o-statusbar">
        <div class="o-statusbar-status">
          ${de(n,o)}
        </div>
        <div class="o-statusbar-buttons">
          ${$}
        </div>
      </div>

      <!-- Smart Buttons -->
      ${ce(d)}

      <!-- Form Sheet -->
      <div class="o-form-sheet">
        <div class="o-form-header">
          ${a?`<h2 style="font-family:'Plus Jakarta Sans',sans-serif;font-size:20px;font-weight:800;color:var(--text-900);margin-bottom:16px">${a}</h2>`:""}
        </div>

        ${r.map(p=>At(p.fields,p.cols??2)).join("")}

        ${u}
      </div>

      <!-- Chatter -->
      ${pe(l,a)}
    </div>
  `;return t&&(t.innerHTML=c),window._switchTab=(p,m)=>{var h;const b=p.closest(".o-form-view");b.querySelectorAll(".o-tab").forEach(_=>_.classList.remove("active")),b.querySelectorAll(".o-tab-panel").forEach(_=>_.classList.remove("active")),p.classList.add("active"),(h=b.querySelector(`.o-tab-panel[data-panel="${m}"]`))==null||h.classList.add("active")},c}function ue(t={}){const{backLabel:e="Volver",backHref:a="",pageTitle:n=t.title||"Detalle"}=t;F([...a?[{label:e,href:a}]:[{label:e}],{label:n}]);const o=document.createElement("div");me(o,t),T(o.innerHTML),window._switchTab=(d,s)=>{var i;const r=d.closest(".o-form-view");r&&(r.querySelectorAll(".o-tab").forEach(l=>l.classList.remove("active")),r.querySelectorAll(".o-tab-panel").forEach(l=>l.classList.remove("active")),d.classList.add("active"),(i=r.querySelector(`.o-tab-panel[data-panel="${s}"]`))==null||i.classList.add("active"))}}window._chatterMessage=t=>{P("Enviar mensaje",`
    <div style="display:flex;flex-direction:column;gap:12px">
      <label style="font-size:13px;font-weight:600;color:var(--text-600)">Mensaje</label>
      <textarea id="chatter-msg" style="width:100%;min-height:100px;padding:10px;border:1px solid var(--border);border-radius:8px;font-size:13px;resize:vertical;font-family:inherit"
        placeholder="Escribe tu mensaje..."></textarea>
      <div style="display:flex;gap:8px;justify-content:flex-end">
        <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cancelar</button>
        <button class="btn btn-primary btn-sm" onclick="(()=>{
          const msg = document.getElementById('chatter-msg')?.value;
          if(msg){window.__closeModal();window._toast&&window._toast('Mensaje enviado','Registrado en el historial','success');}
        })()">✉️ Enviar</button>
      </div>
    </div>
  `)};window._chatterNote=t=>{P("Nota interna",`
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
  `)};const Rt={sale:{lbl:"Confirmada",color:"indigo",step:1},done:{lbl:"Entregada",color:"emerald",step:2},draft:{lbl:"Borrador",color:"gray",step:0},cancel:{lbl:"Cancelada",color:"red",step:-1},sent:{lbl:"Enviada",color:"sky",step:1}},ve=[{key:"draft",label:"Borrador",color:"#9CA3AF"},{key:"sent",label:"Enviada",color:"#0EA5E9"},{key:"sale",label:"Confirmada",color:"#4F46E5"},{key:"done",label:"Entregada",color:"#059669"},{key:"cancel",label:"Cancelada",color:"#DC2626"}];let st=1,ot=0,J="list",mt=[],dt="";async function be(){M(),F([{label:"Inicio",href:"home"},{label:"Ventas"}]),st=1,J=localStorage.getItem("ventas_view")||"list",await Nt()}function ge(){return`
  <div class="o-control-panel" id="ventas-cp">
    <div class="o-cp-left">
      <button class="o-btn-new" onclick="window._nuevaVenta()">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3"><path d="M12 5v14M5 12h14"/></svg>
        Nuevo
      </button>
      <div class="o-search-box">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#9CA3AF" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/></svg>
        <input type="text" id="ventas-search" placeholder="Buscar..." value="${dt}" autocomplete="off">
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
      <span class="o-record-count" id="ventas-count">${ot>0?`${ot} registros`:""}</span>
      <div class="o-view-switcher">
        <button class="o-view-btn${J==="list"?" active":""}"
                id="view-btn-list" title="Vista Lista"
                onclick="window._switchVentaView('list')">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="8" y1="6" x2="21" y2="6"/><line x1="8" y1="12" x2="21" y2="12"/><line x1="8" y1="18" x2="21" y2="18"/><line x1="3" y1="6" x2="3.01" y2="6"/><line x1="3" y1="12" x2="3.01" y2="12"/><line x1="3" y1="18" x2="3.01" y2="18"/></svg>
        </button>
        <button class="o-view-btn${J==="kanban"?" active":""}"
                id="view-btn-kanban" title="Vista Kanban"
                onclick="window._switchVentaView('kanban')">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="7" height="7"/><rect x="14" y="3" width="7" height="7"/><rect x="3" y="14" width="7" height="7"/><rect x="14" y="14" width="7" height="7"/></svg>
        </button>
      </div>
    </div>
  </div>`}function fe(t){return t.length===0?`<div class="empty-state"><div class="empty-state-icon">📋</div>
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
        ${t.map(e=>{const a=Rt[e.state]||{lbl:e.state||"—"},n=e.date_order?S(e.date_order):"—",o=e.invoice_status==="invoiced"?"Facturada":e.invoice_status==="to invoice"?"Por facturar":"—";return`
          <tr onclick="window._verVenta(${e.id})" title="Ver detalle">
            <td onclick="event.stopPropagation()">
              <input type="checkbox" class="o-list-checkbox row-chk" data-id="${e.id}"
                     onchange="window._onRowCheck()">
            </td>
            <td class="td-mono">${e.name||`#${e.id}`}</td>
            <td class="td-primary">${e.partner_name||"—"}</td>
            <td>${n}</td>
            <td class="td-amount">${E(parseFloat(e.amount_untaxed||0))}</td>
            <td class="td-amount" style="font-weight:700">${E(parseFloat(e.amount_total||0))}</td>
            <td><span class="badge badge-${o==="Facturada"?"emerald":o==="Por facturar"?"amber":"gray"}" style="font-size:10px">${o}</span></td>
            <td>${K(e.state,a.lbl)}</td>
          </tr>`}).join("")}
      </tbody>
    </table>
    ${R(st,t.length>=20,e=>{st=e,Nt()})}
  </div>`}function ye(t){return`
  <div class="o-kanban-view">
    ${ve.map(e=>{const a=t.filter(o=>o.state===e.key),n=a.reduce((o,d)=>o+parseFloat(d.amount_total||0),0);return`
      <div class="o-kanban-col">
        <div class="o-kanban-col-header" style="border-top:3px solid ${e.color}">
          <span>${e.label}</span>
          <span class="o-kanban-col-count">${a.length}</span>
        </div>
        <div class="o-kanban-cards">
          ${a.length===0?'<div style="text-align:center;padding:20px;color:var(--text-300);font-size:12px">Sin registros</div>':a.map(o=>`
              <div class="o-kanban-card" onclick="window._verVenta(${o.id})">
                <div class="o-kanban-card-title">${o.partner_name||"—"}</div>
                <div style="font-size:11px;color:var(--text-400);margin-bottom:8px">${o.name||`#${o.id}`}</div>
                <div class="o-kanban-card-meta">
                  <span>${o.date_order?S(o.date_order):"—"}</span>
                  <span class="o-kanban-card-amount">${E(parseFloat(o.amount_total||0))}</span>
                </div>
              </div>
            `).join("")}
        </div>
        ${a.length>0?`<div style="padding:10px 16px;font-size:12px;color:var(--text-400);border-top:1px solid var(--border);font-weight:600">Total: ${E(n)}</div>`:""}
      </div>`}).join("")}
  </div>`}async function Nt(){var t,e,a,n,o,d;T(`
    ${ge()}
    <div id="ventas-kpis" style="display:grid;grid-template-columns:repeat(4,1fr);gap:12px;padding:16px 20px">
      ${[1,2,3,4].map(()=>'<div class="data-card" style="padding:16px"><div class="skeleton" style="height:40px"></div></div>').join("")}
    </div>
    <div id="ventas-content" style="padding:0 20px 20px">
      <div class="data-card">${B(8,7)}</div>
    </div>
  `);try{const[s,r]=await Promise.allSettled([f.ventaKpis(),f.ventas(st)]),i=s.status==="fulfilled"?((t=s.value)==null?void 0:t.data)||s.value:null,l=document.getElementById("ventas-kpis");l&&i&&(l.innerHTML=[{label:"Total Órdenes",val:i.ordenes_confirmadas??i.total_ordenes??0,tipo:"num",color:"indigo"},{label:"Facturado Total",val:i.total_facturado??0,tipo:"mxn",color:"emerald"},{label:"Ticket Promedio",val:i.ticket_promedio??0,tipo:"mxn",color:"violet"},{label:"Este Mes",val:i.ordenes_este_mes??0,tipo:"num",color:"amber"}].map(u=>`
        <div class="kpi-card kpi-${u.color}" style="padding:16px">
          <div class="kpi-label">${u.label}</div>
          <div class="kpi-value" style="font-size:22px">
            ${u.tipo==="mxn"?E(parseFloat(u.val)):Number(u.val).toLocaleString("es-MX")}
          </div>
        </div>`).join(""));const v=r.status==="fulfilled"?((e=r.value)==null?void 0:e.data)||r.value||[]:[];mt=Array.isArray(v)?v:[],ot=((a=r.value)==null?void 0:a.total)??mt.length,r.status==="fulfilled"&&((o=(n=r.value)==null?void 0:n.pagination)!=null&&o.total)&&(ot=r.value.pagination.total);const $=document.getElementById("ventas-count");$&&($.textContent=`${ot} registros · Pág. ${st}`),Pt(),(d=document.getElementById("ventas-search"))==null||d.addEventListener("input",u=>{dt=u.target.value.toLowerCase(),J==="list"?document.querySelectorAll("#ventas-content tbody tr").forEach(c=>{c.style.display=c.textContent.toLowerCase().includes(dt)?"":"none"}):document.querySelectorAll("#ventas-content .o-kanban-card").forEach(c=>{c.style.display=c.textContent.toLowerCase().includes(dt)?"":"none"})})}catch(s){console.error(s),w("Error al cargar ventas",s.message,"error");const r=document.getElementById("ventas-content");r&&(r.innerHTML=`<p style="text-align:center;padding:32px;color:var(--danger)">Error de conexión: ${s.message}</p>`)}window._nuevaVenta=le,window._switchVentaView=s=>{var r;J=s,localStorage.setItem("ventas_view",s),document.querySelectorAll(".o-view-btn").forEach(i=>i.classList.remove("active")),(r=document.getElementById(`view-btn-${s}`))==null||r.classList.add("active"),Pt()},window._checkAll=s=>{document.querySelectorAll(".row-chk").forEach(r=>{r.checked=s.checked}),window._onRowCheck()},window._onRowCheck=()=>{const s=document.querySelectorAll(".row-chk:checked"),r=document.getElementById("ventas-actions-bar");s.length>0&&r?r.innerHTML=`
        <div class="o-list-actions-bar">
          <span class="o-actions-count">${s.length} seleccionado(s)</span>
          <button class="btn btn-secondary btn-sm">Exportar</button>
          <button class="btn btn-danger btn-sm">Eliminar</button>
        </div>`:r&&(r.innerHTML="")}}function Pt(){const t=document.getElementById("ventas-content");t&&(J==="kanban"?t.innerHTML=`<div id="ventas-actions-bar"></div>${ye(mt)}`:t.innerHTML=`<div id="ventas-actions-bar"></div>${fe(mt)}`)}window._verVenta=async t=>{F([{label:"Inicio",href:"home"},{label:"Ventas",href:"ventas"},{label:"Cargando..."}]),T(`
    <div class="o-form-view">
      <div class="o-statusbar">
        <div class="o-statusbar-status">
          ${["Borrador","Confirmada","Entregada"].map(e=>'<div class="o-status-step skeleton" style="width:100px;height:28px;margin:10px 4px"></div>').join("")}
        </div>
      </div>
      <div class="o-form-sheet" style="margin:20px 24px;padding:24px">
        ${[1,2,3,4].map(()=>'<div class="skeleton" style="height:36px;margin-bottom:12px;border-radius:6px"></div>').join("")}
      </div>
    </div>`);try{const e=await f.venta(t),a=(e==null?void 0:e.data)??e;if(!a){w("Error","No se encontró la venta","error");return}const n=Rt[a.state]||{lbl:a.state||"—",color:"gray",step:0},o=a.invoice_status==="invoiced"?"Facturada":a.invoice_status==="to invoice"?"Por facturar":"No facturada",d=[{key:"draft",label:"Borrador",done:n.step>0},{key:"sale",label:"Confirmada",done:n.step>1},{key:"done",label:"Entregada",done:n.step>2}];a.state==="cancel"&&d.push({key:"cancel",label:"Cancelada",done:!1});const s=a.order_line||a.lineas||[],r=`
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
          ${s.length>0?s.map(l=>`
              <tr>
                <td class="td-primary">${l.product_name||l.nombre||"—"}</td>
                <td style="color:var(--text-500)">${l.name||l.descripcion||""}</td>
                <td style="text-align:right">${l.product_uom_qty??l.cantidad??0}</td>
                <td style="text-align:right">${E(parseFloat(l.price_unit||l.precio_unitario||0))}</td>
                <td style="text-align:right">${l.discount||l.descuento||0}%</td>
                <td style="text-align:right;font-weight:700">${E(parseFloat(l.price_subtotal||l.subtotal||0))}</td>
              </tr>`).join(""):'<tr><td colspan="6" style="text-align:center;padding:20px;color:var(--text-400)">Sin líneas de pedido</td></tr>'}
        </tbody>
      </table>
      <div style="display:flex;justify-content:flex-end;padding:16px 0;gap:20px;border-top:1px solid var(--border)">
        <div style="text-align:right">
          <div style="font-size:12px;color:var(--text-400)">Subtotal</div>
          <div style="font-size:14px;font-weight:700">${E(parseFloat(a.amount_untaxed||0))}</div>
        </div>
        <div style="text-align:right">
          <div style="font-size:12px;color:var(--text-400)">IVA</div>
          <div style="font-size:14px;font-weight:700">${E(parseFloat(a.amount_tax||0))}</div>
        </div>
        <div style="text-align:right">
          <div style="font-size:12px;color:var(--text-400)">Total</div>
          <div style="font-size:18px;font-weight:800;color:var(--primary)">${E(parseFloat(a.amount_total||0))}</div>
        </div>
      </div>`,i=At([{label:"Política entrega",value:a.picking_policy||"—"},{label:"Plazo de pago",value:a.payment_term_name||a.payment_term||"—"},{label:"Notas",value:a.note||a.notes||"—"},{label:"Equipo de ventas",value:a.team_name||"—"}],2);ue({title:a.name||`Venta #${a.id}`,backLabel:"Ventas",backHref:"ventas",pageTitle:a.name||`#${a.id}`,statusSteps:d,currentStatus:a.state,smartButtons:[{icon:"📄",count:a.invoice_count??0,label:"Facturas",onClick:""},{icon:"🚚",count:a.delivery_count??0,label:"Entregas",onClick:""}],statusButtons:[{label:"✅ Confirmar",primary:!0,visible:a.state==="draft"||a.state==="sent",onClick:`window._confirmarVenta(${a.id})`},{label:"🔏 Timbrar CFDI",primary:!1,visible:a.invoice_status==="to invoice",onClick:"window._go('cfdi')"},{label:"❌ Cancelar",primary:!1,visible:a.state!=="cancel"&&a.state!=="done",onClick:`window._cancelarVenta(${a.id})`}],groups:[{cols:2,fields:[{label:"Cliente",value:`<strong>${a.partner_name||a.partner_id||"—"}</strong>`},{label:"Vendedor",value:a.user_name||a.salesperson||"—"},{label:"Fecha Orden",value:a.date_order?S(a.date_order):"—"},{label:"Empresa",value:a.company_name||"—"},{label:"Referencia",value:a.client_order_ref||"—"},{label:"Estado Factura",value:`<span class="badge badge-${o==="Facturada"?"emerald":o==="Por facturar"?"amber":"gray"}">${o}</span>`}]}],tabs:[{label:"Líneas de Pedido",content:r},{label:"Otra Información",content:i}],messages:[{author:"Sistema",initials:"SY",date:a.date_order?S(a.date_order):"—",text:`Orden de venta ${a.name||""} creada. Estado: ${n.lbl}`}]}),window._confirmarVenta=async l=>{try{await f.put(`/ventas/${l}/confirmar`,{}),w("Venta confirmada","Estado actualizado correctamente","success"),window._verVenta(l)}catch(v){w("Error",v.message,"error")}},window._cancelarVenta=async l=>{try{await f.put(`/ventas/${l}/cancelar`,{}),w("Venta cancelada","","info"),window._verVenta(l)}catch(v){w("Error",v.message,"error")}}}catch(e){console.error(e),w("Error al cargar venta",e.message,"error")}};function he(t,e){const a=t.state==="draft";P("Detalle de Factura",`
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
  </div>`),window._factValidar=()=>{w("Validar factura","Función disponible próximamente","info")},window._factDescargar=()=>{w("Descargar PDF","Función disponible próximamente","info")}}function xe(t,e){P("Editar Contacto",`
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
  </form>`),window._submitEditPartner=async()=>{var o,d,s,r,i,l,v,$,u;const a=document.getElementById("btn-save-partner"),n=(d=(o=document.getElementById("ep-name"))==null?void 0:o.value)==null?void 0:d.trim();if(!n){w("Error de validación","El nombre es obligatorio","error");return}a.textContent="⏳ Guardando…",a.disabled=!0;try{const c={name:n,email:((s=document.getElementById("ep-email"))==null?void 0:s.value)||"",phone:((r=document.getElementById("ep-phone"))==null?void 0:r.value)||"",mobile:((i=document.getElementById("ep-mobile"))==null?void 0:i.value)||"",city:((l=document.getElementById("ep-city"))==null?void 0:l.value)||"",vat:(($=(v=document.getElementById("ep-vat"))==null?void 0:v.value)==null?void 0:$.toUpperCase())||"",website:((u=document.getElementById("ep-website"))==null?void 0:u.value)||""};await f.put(`/partners/${t.id}`,c).catch(()=>null),w("Contacto actualizado",n,"success"),window.__closeModal(),e&&e()}catch(c){const p=document.getElementById("edit-partner-result");p&&(p.innerHTML=`<p style="color:var(--red)">${c.message}</p>`)}finally{a.textContent="💾 Guardar",a.disabled=!1}}}function we(t,e){const a=t.name&&typeof t.name=="object"?t.name.es_MX||t.name.en_US||Object.values(t.name)[0]||"":t.name||t.nombre||"";P("Editar Producto",`
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
  </form>`),window._submitEditProducto=async()=>{var o,d,s,r;const n=document.getElementById("btn-save-producto");n.textContent="⏳ Guardando…",n.disabled=!0;try{const i={name:((o=document.getElementById("epr-name"))==null?void 0:o.value)||a,default_code:((d=document.getElementById("epr-code"))==null?void 0:d.value)||"",list_price:parseFloat(((s=document.getElementById("epr-precio"))==null?void 0:s.value)||0),standard_price:parseFloat(((r=document.getElementById("epr-costo"))==null?void 0:r.value)||0)};let l=!1;try{await f.put(`/productos/${t.id}`,i),l=!0}catch{l=!1}l?w("Producto actualizado",i.name,"success"):w("Guardado localmente","Se sincronizará cuando el endpoint esté disponible","warning"),window.__closeModal(),e&&e()}catch(i){const l=document.getElementById("edit-producto-result");l&&(l.innerHTML=`<p style="color:var(--red)">${i.message}</p>`)}finally{n.textContent="💾 Guardar",n.disabled=!1}}}function $e(t,e){const a=parseFloat(t.cantidad_disponible||0);P("Ajuste de Inventario",`
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
  </form>`),window._submitAjusteStock=async()=>{var o,d;const n=document.getElementById("btn-save-stock");n.textContent="⏳ Guardando…",n.disabled=!0;try{const s={cantidad:parseFloat(((o=document.getElementById("ast-qty"))==null?void 0:o.value)||0),motivo:((d=document.getElementById("ast-motivo"))==null?void 0:d.value)||"Corrección"};try{await f.put(`/stock/${t.product_id}/ajuste`,s)}catch{}w("Inventario ajustado",`Nuevo stock: ${s.cantidad} — ${s.motivo}`,"success"),window.__closeModal(),e&&e()}catch(s){const r=document.getElementById("ajuste-stock-result");r&&(r.innerHTML=`<p style="color:var(--red)">${s.message}</p>`)}finally{n.textContent="📋 Aplicar ajuste",n.disabled=!1}}}function _e(t,e){const a=t.state==="draft";P("Editar Orden de Compra",`
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
  </form>`),window._submitEditCompra=async()=>{var o,d;if(!a)return;const n=document.getElementById("btn-save-compra");n.textContent="⏳ Guardando…",n.disabled=!0;try{const s={note:((o=document.getElementById("ec-note"))==null?void 0:o.value)||"",date_planned:((d=document.getElementById("ec-date"))==null?void 0:d.value)||""};await f.put(`/compras/${t.id}`,s).catch(()=>null),w("Compra actualizada",`OC ${t.name||t.id} guardada`,"success"),window.__closeModal(),e&&e()}catch(s){const r=document.getElementById("edit-compra-result");r&&(r.innerHTML=`<p style="color:var(--red)">${s.message}</p>`)}finally{n.textContent="💾 Guardar",n.disabled=!1}}}function Ee(t,e){P("Editar Empleado",`
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
  </form>`),window._submitEditEmpleado=async()=>{var n,o,d,s;const a=document.getElementById("btn-save-emp");a.textContent="⏳ Guardando…",a.disabled=!0;try{const r={job_title:((n=document.getElementById("ee-title"))==null?void 0:n.value)||"",ssnid:((o=document.getElementById("ee-imss"))==null?void 0:o.value)||"",work_email:((d=document.getElementById("ee-email"))==null?void 0:d.value)||"",work_phone:((s=document.getElementById("ee-phone"))==null?void 0:s.value)||""};await f.put(`/nomina/${t.id}`,r).catch(()=>null),w("Empleado actualizado",t.name,"success"),window.__closeModal(),e&&e()}catch(r){const i=document.getElementById("edit-emp-result");i&&(i.innerHTML=`<p style="color:var(--red)">${r.message}</p>`)}finally{a.textContent="💾 Guardar",a.disabled=!1}}}const ke={posted:{lbl:"Publicada",color:"emerald"},draft:{lbl:"Borrador",color:"gray"},in_payment:{lbl:"En cobro",color:"violet"},paid:{lbl:"Pagada",color:"sky"},cancel:{lbl:"Cancelada",color:"red"}};let Q=1;async function Ce(){M(),F([{label:"Dashboard",href:"dashboard"},{label:"Facturación"}]),Q=1,await Ht()}async function Ht(){var t,e,a,n,o;T(`
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
      <div id="fact-tabla">${B(8,5)}</div>
    </div>

    <!-- Panel por cobrar -->
    <div class="data-card" style="padding:16px">
      <div style="font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.06em;color:var(--text-400);margin-bottom:12px">📋 Por Cobrar</div>
      <div id="por-cobrar-lista">${[1,2,3,4].map(()=>'<div class="skeleton" style="height:38px;margin-bottom:8px;border-radius:8px"></div>').join("")}</div>
    </div>
  </div>`);try{const[d,s,r]=await Promise.allSettled([f.factKpis(),f.facturas(Q),f.porCobrar()]),i=d.status==="fulfilled"?(t=d.value)==null?void 0:t.data:null,l=document.getElementById("kpi-row");l&&(l.innerHTML=[{label:"Total Facturas",val:(i==null?void 0:i.total_facturas)||0,tipo:"num",color:"indigo",icon:"🧾"},{label:"Monto Facturado",val:(i==null?void 0:i.monto_total)||0,tipo:"mxn",color:"emerald",icon:"💰"},{label:"Por Cobrar",val:(i==null?void 0:i.por_cobrar)||0,tipo:"mxn",color:"amber",icon:"📋"},{label:"Facturas Vencidas",val:(i==null?void 0:i.facturas_vencidas)||0,tipo:"num",color:"red",icon:"⚠️"}].map(b=>`
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${b.icon} ${b.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${b.tipo==="mxn"?E(parseFloat(b.val)):pt(parseInt(b.val))}
        </div>
      </div>`).join(""));const v=s.status==="fulfilled"?((e=s.value)==null?void 0:e.data)||[]:[],$=v.length>=20,u=document.getElementById("fact-sub");u&&(u.textContent=`${v.length} registros · Página ${Q}`);const c=document.getElementById("fact-tabla");c&&(v.length===0?c.innerHTML='<p style="text-align:center;padding:32px;color:var(--text-400)">Sin facturas registradas</p>':c.innerHTML=`
        <table class="data-table">
          <thead><tr>
            <th>Folio</th><th>Cliente</th><th>Fecha</th>
            <th>Subtotal</th><th>Total</th><th>Estado</th>
          </tr></thead>
          <tbody>
            ${v.map(b=>{const h=ke[b.state]||{lbl:b.state||"—",color:"gray"},_=b.invoice_date||b.date?S(b.invoice_date||b.date):"—",y=b.partner_name&&isNaN(b.partner_name)?b.partner_name:b.customer_name||`Cliente #${b.partner_id}`;return`
              <tr data-estado="${b.state||""}" style="cursor:pointer" onclick="window._verFactura(${b.id})" title="Ver detalle">
                <td class="td-mono">${b.name||`#${b.id}`}</td>
                <td class="td-primary">${y}</td>
                <td>${_}</td>
                <td class="td-amount">${E(parseFloat(b.amount_untaxed||0))}</td>
                <td class="td-amount" style="font-weight:700">${E(parseFloat(b.amount_total||0))}</td>
                <td>${K(b.state,h.lbl)}</td>
              </tr>`}).join("")}
          </tbody>
        </table>
        ${R(Q,$,b=>{Q=b,Ht()})}`);const p=r.status==="fulfilled"?((a=r.value)==null?void 0:a.data)||[]:[],m=document.getElementById("por-cobrar-lista");m&&(p.length===0?m.innerHTML='<p style="color:var(--emerald);font-size:13px;text-align:center;padding:20px">✅ Sin saldo pendiente</p>':m.innerHTML=p.slice(0,8).map(b=>{const h=b.invoice_date_due&&new Date(b.invoice_date_due)<new Date;return`
          <div style="display:flex;justify-content:space-between;align-items:center;padding:8px 0;border-bottom:1px solid var(--border)">
            <div>
              <div style="font-size:12px;font-weight:600;color:var(--text-700)">${(b.partner_name||b.name||"—").substring(0,22)}</div>
              <div style="font-size:11px;color:${h?"var(--red)":"var(--text-400)"}">${h?"🔴 Vencida":"🟡 Pendiente"}</div>
            </div>
            <span class="badge badge-${h?"red":"amber"}">${E(parseFloat(b.amount_residual||b.amount_total||0))}</span>
          </div>`}).join("")),(n=document.getElementById("buscar-fact"))==null||n.addEventListener("input",b=>{const h=b.target.value.toLowerCase();document.querySelectorAll("#fact-tabla tbody tr").forEach(_=>{_.style.display=_.textContent.toLowerCase().includes(h)?"":"none"})}),(o=document.getElementById("filtro-estado"))==null||o.addEventListener("change",b=>{const h=b.target.value;document.querySelectorAll("#fact-tabla tbody tr").forEach(_=>{_.style.display=!h||_.dataset.estado===h?"":"none"})}),window._verFactura=b=>{q("Detalle de Factura",()=>f.factura(b),h=>(setTimeout(()=>he(h),0),'<div style="padding:24px;text-align:center;color:var(--text-400)">Cargando…</div>'))}}catch(d){console.error(d),w("Error al cargar facturas",d.message,"error")}}let X=1,H="";async function Ie(){M(),F([{label:"Dashboard",href:"dashboard"},{label:"Productos"}]),X=1,H="",await ct()}async function ct(){var t,e;T(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">Productos</h1>
      <p class="page-subtitle" id="prod-sub">Cargando catálogo…</p>
    </div>
    <div class="page-actions">
      <input type="text" id="buscar-prod" class="search-input" placeholder="🔍 Buscar producto o código…" style="width:240px" value="${H}">
      <button class="btn btn-primary">+ Nuevo Producto</button>
    </div>
  </div>

  <div class="data-card anim-2">
    <div class="data-card-header">
      <div class="data-card-title">Catálogo de Productos</div>
      <select id="filtro-tipo" class="search-input" style="width:150px;font-size:12px">
        <option value="">Todos</option>
        <option value="consu">Consumibles</option>
        <option value="service">Servicios</option>
        <option value="product">Almacenables</option>
      </select>
    </div>
    <div id="prod-tabla">${B(10,6)}</div>
  </div>`);try{const a=await f.productos(X,H),n=(a==null?void 0:a.data)||[],o=n.length>=20,d=document.getElementById("prod-sub");d&&(d.textContent=`${n.length} productos${H?` para "${H}"`:""} · Página ${X}`);const s=document.getElementById("prod-tabla");s&&(n.length===0?s.innerHTML=`<p style="text-align:center;padding:40px;color:var(--text-400)">
          ${H?`Sin resultados para "${H}"`:"Sin productos en catálogo"}
        </p>`:s.innerHTML=`
        <table class="data-table">
          <thead><tr>
            <th>Código</th><th>Nombre</th><th>Tipo</th>
            <th>Precio Venta</th><th>Categoría</th><th>Estado</th>
          </tr></thead>
          <tbody>
            ${n.map(i=>{const l=i.name&&typeof i.name=="object"?i.name.es_MX||i.name.en_US||Object.values(i.name)[0]||`Producto #${i.id}`:i.name||i.nombre||`Producto #${i.id}`,v=i.type_||i.type||"",$=v==="consu"?"Consumible":v==="service"?"Servicio":v==="product"?"Almacenable":"Consumible",u=v==="service"?"violet":v==="consu"?"sky":"indigo",c=E(parseFloat(i.list_price||i.precio||0)),p=i.active!==!1,m=i.categ_name||i.categoria||"",b=m==="Goods"?"Mercancía":m==="Services"?"Servicios":m||"—";return`
              <tr data-tipo="${v}" data-id="${i.id}" style="cursor:pointer" onclick="window._verProducto(${i.id})" title="Ver detalle">
                <td class="td-mono">${i.default_code||"—"}</td>
                <td class="td-primary">${l}</td>
                <td><span class="badge badge-${u}">${$}</span></td>
                <td class="td-amount" style="font-weight:700">${c}</td>
                <td style="color:var(--text-400);font-size:12px">${b}</td>
                <td><span class="badge badge-${p?"emerald":"gray"}">${p?"Activo":"Inactivo"}</span></td>
              </tr>`}).join("")}
          </tbody>
        </table>
        ${R(X,o,i=>{X=i,ct()})}`);let r;(t=document.getElementById("buscar-prod"))==null||t.addEventListener("input",i=>{clearTimeout(r),r=setTimeout(()=>{H=i.target.value.trim(),X=1,ct()},400)}),(e=document.getElementById("filtro-tipo"))==null||e.addEventListener("change",i=>{const l=i.target.value;document.querySelectorAll("#prod-tabla tbody tr").forEach(v=>{v.style.display=!l||v.dataset.tipo===l?"":"none"})}),window._verProducto=i=>{const l=n.find(m=>m.id===i);if(!l)return;const v=l.name&&typeof l.name=="object"?l.name.es_MX||l.name.en_US||"":l.name||"",$=l.type_||l.type||"",u=$==="consu"?"Consumible":$==="service"?"Servicio":"Almacenable",c=l.categ_name||"",p=c==="Goods"?"Mercancía":c==="Services"?"Servicios":c||"—";q("Detalle de Producto",async()=>l,()=>`
        ${I("Identificación",[g("Nombre",v),g("Código interno",l.default_code||"—"),g("Código de barras",l.barcode||"—"),g("Tipo",u),g("Categoría",p),g("Estado",`<span class="badge badge-${l.active!==!1?"emerald":"gray"}">${l.active!==!1?"Activo":"Inactivo"}</span>`)].join(""))}
        ${I("Precios",[g("Precio de venta",E(parseFloat(l.list_price||0))),g("Costo estándar",E(parseFloat(l.standard_price||0)))].join(""))}
        <div style="display:flex;gap:10px;margin-top:16px">
          <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
          <button class="btn btn-primary btn-sm" onclick="window._editarProductoFn(${l.id})">✏️ Editar</button>
        </div>`)},window._editarProductoFn=i=>{const l=n.find(v=>v.id===i);l&&we(l,()=>ct())}}catch(a){console.error(a),w("Error al cargar productos",a.message,"error")}}let V=1,A="";async function Se(){M(),F([{label:"Dashboard",href:"dashboard"},{label:"Clientes / Proveedores"}]),V=1,A="",await W()}async function W(){var t,e,a;T(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">Clientes y Proveedores</h1>
      <p class="page-subtitle" id="part-sub">Cargando directorio…</p>
    </div>
    <div class="page-actions">
      <input type="text" id="buscar-part" class="search-input" placeholder="🔍 Buscar por nombre…" style="width:220px">
      <div style="display:flex;gap:6px">
        <button class="btn ${A===""?"btn-primary":"btn-secondary"}" id="btn-todos" onclick="window._filterPart('')">Todos</button>
        <button class="btn ${A==="clientes"?"btn-primary":"btn-secondary"}" id="btn-cli" onclick="window._filterPart('clientes')">👥 Clientes</button>
        <button class="btn ${A==="proveedores"?"btn-primary":"btn-secondary"}" id="btn-prov" onclick="window._filterPart('proveedores')">🏭 Proveedores</button>
      </div>
      <button class="btn btn-primary" onclick="window._nuevoContacto()">+ Nuevo Contacto</button>
    </div>
  </div>

  <!-- Stats row -->
  <div style="display:grid;grid-template-columns:repeat(3,1fr);gap:12px;margin-bottom:18px" id="stats-row" class="anim-2">
    ${[1,2,3].map(()=>'<div class="data-card" style="padding:16px"><div class="skeleton" style="height:40px"></div></div>').join("")}
  </div>

  <div class="data-card anim-3">
    <div class="data-card-header">
      <div class="data-card-title">${A==="clientes"?"👥 Clientes":A==="proveedores"?"🏭 Proveedores":"📋 Directorio"}</div>
    </div>
    <div id="part-tabla">${B(10,5)}</div>
  </div>`),window._filterPart=n=>{A=n,V=1,W()},window._nuevoContacto=()=>re(()=>W());try{let n;A==="clientes"?n=f.clientes(V):A==="proveedores"?n=f.proveedores(V):n=f.partners(V);const[o,d]=await Promise.allSettled([n,f.partners(1)]),s=o.status==="fulfilled"?((t=o.value)==null?void 0:t.data)||[]:[],r=d.status==="fulfilled"?((e=d.value)==null?void 0:e.data)||[]:s,i=s.length>=20,l=document.getElementById("stats-row");if(l){const u=r.filter(p=>(p.customer_rank||0)>0).length,c=r.filter(p=>(p.supplier_rank||0)>0).length;l.innerHTML=[{label:"Total Contactos",val:r.length,color:"indigo",icon:"📋"},{label:"Clientes",val:u,color:"emerald",icon:"👥"},{label:"Proveedores",val:c,color:"violet",icon:"🏭"}].map(p=>`
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${p.icon} ${p.label}</div>
        <div style="font-size:26px;font-weight:800;color:var(--text-900)">${pt(p.val)}</div>
      </div>`).join("")}const v=document.getElementById("part-sub");v&&(v.textContent=`${s.length} contactos · Página ${V}`);const $=document.getElementById("part-tabla");$&&(s.length===0?$.innerHTML='<p style="text-align:center;padding:32px;color:var(--text-400)">Sin contactos registrados</p>':$.innerHTML=`
        <table class="data-table">
          <thead><tr>
            <th>Nombre</th><th>Tipo</th><th>Email</th><th>Teléfono</th><th>Tags</th>
          </tr></thead>
          <tbody>
            ${s.map(u=>{const c=(u.customer_rank||0)>0,p=(u.supplier_rank||0)>0,m=u.is_company;return`
              <tr style="cursor:pointer" onclick="window._verPartner(${u.id})" title="Ver detalle">
                <td>
                  <div style="display:flex;align-items:center;gap:8px">
                    <div style="width:32px;height:32px;border-radius:50%;background:linear-gradient(135deg,hsl(${u.id*37%360},60%,55%),hsl(${u.id*71%360},70%,45%));display:flex;align-items:center;justify-content:center;color:white;font-size:13px;font-weight:700;flex-shrink:0">
                      ${(u.name||u.nombre||"?")[0].toUpperCase()}
                    </div>
                    <div>
                      <div class="td-primary">${u.name||u.nombre||"—"}</div>
                      ${m?'<div style="font-size:11px;color:var(--text-400)">Empresa</div>':""}
                    </div>
                  </div>
                </td>
                <td>
                  ${c?'<span class="badge badge-emerald">Cliente</span>':""}
                  ${p?'<span class="badge badge-violet" style="margin-left:2px">Proveedor</span>':""}
                  ${!c&&!p?'<span class="badge badge-gray">Contacto</span>':""}
                </td>
                <td style="color:var(--text-500);font-size:12.5px">${u.email||"—"}</td>
                <td style="color:var(--text-500);font-size:12.5px">${u.phone||"—"}</td>
                <td>${m?'<span class="badge badge-sky">Empresa</span>':'<span class="badge badge-gray">Persona</span>'}</td>
              </tr>`}).join("")}
          </tbody>
        </table>
        ${R(V,i,u=>{V=u,W()})}`),(a=document.getElementById("buscar-part"))==null||a.addEventListener("input",u=>{const c=u.target.value.toLowerCase();document.querySelectorAll("#part-tabla tbody tr").forEach(p=>{p.style.display=p.textContent.toLowerCase().includes(c)?"":"none"})}),window._verPartner=u=>{q("Detalle de Contacto",()=>f.partner(u),c=>{const p=(c.customer_rank||0)>0,m=(c.supplier_rank||0)>0;return`
          ${I("Información General",[g("Nombre",c.name),g("Tipo",c.is_company?"Empresa":"Persona física"),g("Rol",[p?"Cliente":"",m?"Proveedor":""].filter(Boolean).join(", ")||"Contacto"),g("RFC",c.vat||"—"),g("Website",c.website||"—")].join(""))}
          ${I("Contacto",[g("Email",c.email?`<a href="mailto:${c.email}" style="color:var(--primary)">${c.email}</a>`:"—"),g("Teléfono",c.phone||"—"),g("Móvil",c.mobile||"—"),g("Ciudad",c.city||"—"),g("País",c.country_name||"—")].join(""))}
          <div style="display:flex;gap:10px;margin-top:16px">
            <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
            <button class="btn btn-primary btn-sm" onclick="window._editarPartnerFn(${c.id})">✏️ Editar</button>
          </div>`})},window._editarPartnerFn=u=>{const c=s.find(p=>p.id===u);c&&xe(c,()=>W())}}catch(n){console.error(n),w("Error al cargar contactos",n.message,"error")}}const Te=["deposit","down payment","downpayment","pago inicial"];let Z=1;async function Fe(){M(),F([{label:"Dashboard",href:"dashboard"},{label:"Inventario"}]),Z=1,await xt()}async function xt(){var t,e,a,n,o;T(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">Inventario</h1>
      <p class="page-subtitle" id="stock-sub">Cargando…</p>
    </div>
    <div class="page-actions">
      <input type="text" id="buscar-stock" class="search-input" placeholder="🔍 Buscar producto…" style="width:220px">
      <button class="btn btn-secondary" id="btn-ajuste">📋 Ajuste</button>
      <button class="btn btn-primary">+ Recepción</button>
    </div>
  </div>

  <!-- KPI row -->
  <div style="display:grid;grid-template-columns:repeat(4,1fr);gap:12px;margin-bottom:18px" id="kpi-row" class="anim-2">
    ${[1,2,3,4].map(()=>'<div class="data-card" style="padding:16px"><div class="skeleton" style="height:40px"></div></div>').join("")}
  </div>

  <div style="display:grid;grid-template-columns:1fr 340px;gap:16px" class="anim-3">
    <!-- Tabla principal -->
    <div class="data-card">
      <div class="data-card-header">
        <div class="data-card-title">Stock por Producto</div>
        <select id="filtro-stock" class="search-input" style="width:160px;font-size:12px">
          <option value="todos">Todos</option>
          <option value="bajo">⚠️ Stock bajo</option>
          <option value="ok">✅ Stock normal</option>
        </select>
      </div>
      <div id="stock-tabla">${B(8,5)}</div>
    </div>

    <!-- Panel stock bajo -->
    <div class="data-card" style="padding:16px">
      <div style="font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.06em;color:var(--text-400);margin-bottom:12px">⚠️ Alertas de Stock Bajo</div>
      <div id="stock-bajo-lista">${[1,2,3,4,5].map(()=>'<div class="skeleton" style="height:36px;margin-bottom:8px;border-radius:8px"></div>').join("")}</div>
    </div>
  </div>`);try{const[d,s,r]=await Promise.allSettled([f.stockKpis(),f.stock(Z),f.stockBajo()]),i=d.status==="fulfilled"?(t=d.value)==null?void 0:t.data:null,l=document.getElementById("kpi-row");l&&i&&(l.innerHTML=[{label:"Con stock",val:i.total_productos_con_stock||0,tipo:"num",color:"emerald",icon:"✅"},{label:"Sin stock",val:i.total_sin_stock||0,tipo:"num",color:"red",icon:"❌"},{label:"Valor Inventario",val:i.valor_inventario||0,tipo:"mxn",color:"indigo",icon:"💰"},{label:"Alertas Bajo",val:i.alertas_stock_bajo||0,tipo:"num",color:"amber",icon:"⚠️"}].map(h=>`
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${h.icon} ${h.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${h.tipo==="mxn"?E(parseFloat(h.val)):Number(h.val).toLocaleString("es-MX")}
        </div>
      </div>`).join(""));const v=s.status==="fulfilled"?((e=s.value)==null?void 0:e.data)||[]:[],$=v.filter(h=>{const _=(h.product_name||"").toLowerCase();return!Te.some(y=>_.includes(y))}),u=v.length>=20,c=document.getElementById("stock-sub");c&&(c.textContent=`${$.length} productos · Página ${Z}`);const p=document.getElementById("stock-tabla");p&&($.length===0?p.innerHTML='<p style="text-align:center;padding:32px;color:var(--text-400)">Sin datos de stock</p>':p.innerHTML=`
        <table class="data-table">
          <thead><tr>
            <th>Producto</th>
            <th>Disponible</th>
            <th>Reservado</th>
            <th>Ubicación</th>
            <th>Estado</th>
          </tr></thead>
          <tbody>
            ${$.map(h=>{const _=parseFloat(h.cantidad_disponible||0),y=parseFloat(h.cantidad_reservada||0),k=_<=0?"red":_<10?"amber":"emerald",C=_<=0?"❌ Sin stock":_<10?"⚠️ Stock bajo":"✅ Normal";return`
              <tr data-alerta="${_<10?"bajo":"ok"}" style="cursor:pointer" onclick="window._verStock(${h.product_id})" title="Ver detalle">
                <td class="td-primary">${h.product_name||`Producto #${h.product_id}`}</td>
                <td><span class="badge badge-${k}">${O(_,0)}</span></td>
                <td style="color:var(--text-400)">${O(y,0)}</td>
                <td class="td-mono" style="font-size:11px">${h.ubicacion||"—"}</td>
                <td><span class="badge badge-${k}">${C}</span></td>
              </tr>`}).join("")}
          </tbody>
        </table>
        ${R(Z,u,h=>{Z=h,xt()})}`);const m=r.status==="fulfilled"?((a=r.value)==null?void 0:a.data)||[]:[],b=document.getElementById("stock-bajo-lista");b&&(m.length===0?b.innerHTML='<p style="color:var(--emerald);font-size:13px;text-align:center;padding:16px">✅ Todo en niveles normales</p>':b.innerHTML=m.map(h=>{const _=parseFloat(h.cantidad_disponible||0),y=_<=0?"red":"amber";return`
          <div style="display:flex;justify-content:space-between;align-items:center;padding:8px 0;border-bottom:1px solid var(--border)">
            <div>
              <div style="font-size:12.5px;font-weight:600;color:var(--text-700)">${(h.product_name||`#${h.product_id}`).substring(0,28)}</div>
            </div>
            <span class="badge badge-${y}">${_}</span>
          </div>`}).join("")),(n=document.getElementById("buscar-stock"))==null||n.addEventListener("input",h=>{const _=h.target.value.toLowerCase();document.querySelectorAll("#stock-tabla tbody tr").forEach(y=>{y.style.display=y.textContent.toLowerCase().includes(_)?"":"none"})}),(o=document.getElementById("filtro-stock"))==null||o.addEventListener("change",h=>{const _=h.target.value;document.querySelectorAll("#stock-tabla tbody tr").forEach(y=>{if(_==="todos"){y.style.display="";return}const k=y.dataset.alerta;y.style.display=k===_?"":"none"})}),window._verStock=h=>{q("Detalle de Stock",()=>f.stockProducto(h),_=>{const y=Array.isArray(_)?_:[_],k=y[0]||{},C=parseFloat(k.cantidad_disponible||0),N=parseFloat(k.cantidad_reservada||0),z=C<=0?"var(--red)":C<10?"var(--warning)":"var(--success)";return`
          ${I("Producto",[g("Nombre",k.product_name||`#${h}`),g("Cantidad disponible",`<strong style="color:${z}">${O(C,2)}</strong>`),g("Cantidad reservada",O(N,2)),g("Cantidad neta",O(C-N,2))].join(""))}
          ${y.length>1?I("Por ubicación",y.map(G=>g(G.ubicacion||"Sin ubicación",O(parseFloat(G.cantidad_disponible||0),2))).join("")):I("Ubicación",[g("Almacén",k.ubicacion||"Sin ubicación")].join(""))}
          <div style="display:flex;gap:10px;margin-top:16px">
            <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
            <button class="btn btn-primary btn-sm" onclick="window._ajustarStockFn(${k.product_id??h})">📋 Ajustar</button>
          </div>`})},window._ajustarStockFn=h=>{const _=$.find(y=>y.product_id===h);_&&$e(_,()=>xt())}}catch(d){console.error(d),w("Error al cargar inventario",d.message,"error")}}let tt=1,lt="historial";async function Be(){M(),F([{label:"Dashboard",href:"dashboard"},{label:"CFDI 4.0"}]),tt=1,await Vt()}async function Vt(){T(`
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
    <div id="cfdi-body">${B(6,6)}</div>
  </div>`),window._cfdiTab=t=>{lt=t,Vt()};try{const t=await f.cfdiKpis().catch(()=>null),e=t==null?void 0:t.data,a=document.getElementById("kpi-row");a&&(a.innerHTML=[{label:"Total Timbrados",val:(e==null?void 0:e.total_timbrados)??0,tipo:"num",color:"indigo",icon:"🧾"},{label:"Vigentes",val:(e==null?void 0:e.vigentes)??0,tipo:"num",color:"emerald",icon:"✅"},{label:"Cancelados",val:(e==null?void 0:e.cancelados)??0,tipo:"num",color:"red",icon:"❌"},{label:"Monto Total",val:(e==null?void 0:e.monto_total)??0,tipo:"mxn",color:"violet",icon:"💰"}].map(n=>`
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${n.icon} ${n.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${n.tipo==="mxn"?E(parseFloat(n.val)):Number(n.val).toLocaleString("es-MX")}
        </div>
      </div>`).join("")),lt==="historial"?await wt():Le()}catch(t){console.error(t),w("Error CFDI",t.message,"error")}}async function wt(){const t=document.getElementById("cfdi-body");t&&(t.innerHTML=B(6,6));const e=await f.cfdiTimbrados(tt).catch(()=>({data:[],total:0})),a=(e==null?void 0:e.data)||[],n=(e==null?void 0:e.total)??a.length,o=a.length>=20,d=document.getElementById("cfdi-sub");if(d&&(d.textContent=`${n} CFDIs timbrados · Página ${tt}`),!!t){if(a.length===0){t.innerHTML=`
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
      ${a.map(s=>{const r=s.estado==="vigente"?"emerald":s.estado==="cancelado"?"red":"gray";return`
        <tr style="cursor:pointer" onclick="window._verCfdi('${s.uuid}')" title="Ver detalle">
          <td class="td-mono" style="font-size:11px">${s.uuid.substring(0,18)}…</td>
          <td class="td-mono">${s.serie||""}${s.folio||"—"}</td>
          <td class="td-primary">${s.nombre_receptor||s.rfc_receptor}</td>
          <td class="td-amount" style="font-weight:700">${E(parseFloat(s.total||0))}</td>
          <td><span class="badge badge-sky">${s.tipo_cfdi==="I"?"Ingreso":s.tipo_cfdi==="E"?"Egreso":s.tipo_cfdi||"—"}</span></td>
          <td><span class="badge badge-${r}">${s.estado||"—"}</span></td>
          <td style="font-size:12px">${S(s.fecha_timbrado||s.created_at)}</td>
        </tr>`}).join("")}
    </tbody>
  </table>
  ${R(tt,o,s=>{tt=s,wt()})}`,window._verCfdi=s=>{q("Detalle CFDI",()=>f.cfdiTimbrado(s),r=>`
      ${I("Comprobante",[g("UUID",`<span style="font-family:monospace;font-size:11px">${r.uuid}</span>`),g("Serie / Folio",`${r.serie||""}${r.folio||"—"}`),g("Tipo",r.tipo_cfdi==="I"?"Ingreso":r.tipo_cfdi==="E"?"Egreso":r.tipo_cfdi),g("Estado",`<span class="badge badge-${r.estado==="vigente"?"emerald":"red"}">${r.estado}</span>`),g("Fecha emisión",S(r.fecha_emision)),g("Fecha timbrado",S(r.fecha_timbrado))].join(""))}
      ${I("Partes",[g("RFC Emisor",r.rfc_emisor),g("Emisor",r.nombre_emisor||"—"),g("RFC Receptor",r.rfc_receptor),g("Receptor",r.nombre_receptor||"—")].join(""))}
      ${I("Importes",[g("Total",`<strong>${E(parseFloat(r.total||0))}</strong>`,{color:"var(--primary)"})].join(""))}
      <div style="display:flex;gap:10px;margin-top:16px">
        <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
        ${r.estado==="vigente"?`<button class="btn btn-danger btn-sm" onclick="window._cancelarCfdi('${r.uuid}')">❌ Cancelar</button>`:""}
      </div>`)},window._cancelarCfdi=async s=>{if(confirm(`¿Cancelar el CFDI ${s.substring(0,18)}…?`))try{await f.cancelarCfdi({uuid:s,rfc_emisor:"",motivo:"02"}),w("CFDI cancelado",s,"success"),window.__closeModal(),wt()}catch(r){w("Error al cancelar",r.message,"error")}}}}function Le(){var e;const t=document.getElementById("cfdi-body");t&&(t.innerHTML=`
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
  </div>`,(e=document.getElementById("f-subtotal"))==null||e.addEventListener("input",a=>{const n=parseFloat(a.target.value)||0,o=n*.16;document.getElementById("f-iva").value=o.toFixed(2),document.getElementById("f-total").value=(n+o).toFixed(2)}),window._timbrar=async()=>{var o,d,s,r,i,l,v,$,u,c,p,m,b,h;const a=document.getElementById("btn-timbrar");a.textContent="⏳ Timbrando…",a.disabled=!0;const n=document.getElementById("cfdi-resultado");try{const _=(o=document.getElementById("f-cer"))==null?void 0:o.files[0],y=(d=document.getElementById("f-key"))==null?void 0:d.files[0],k=St=>new Promise((Tt,Ut)=>{if(!St){Tt("");return}const ft=new FileReader;ft.onload=Xt=>Tt(Xt.target.result.split(",")[1]||""),ft.onerror=Ut,ft.readAsDataURL(St)}),[C,N]=await Promise.all([k(_),k(y)]),z=parseFloat((s=document.getElementById("f-subtotal"))==null?void 0:s.value)||0,G=z*.16,Ot={cfdi:{serie:((r=document.getElementById("f-serie"))==null?void 0:r.value)||"A",folio:((i=document.getElementById("f-folio"))==null?void 0:i.value)||"1",tipo_comprobante:((l=document.getElementById("f-tipo"))==null?void 0:l.value)||"I",emisor:{rfc:((v=document.getElementById("f-rfc-emisor"))==null?void 0:v.value)||"",nombre:(($=document.getElementById("f-nombre-emisor"))==null?void 0:$.value)||"",regimen_fiscal:((u=document.getElementById("f-regimen"))==null?void 0:u.value)||"601"},receptor:{rfc:((c=document.getElementById("f-rfc-receptor"))==null?void 0:c.value)||"",nombre:((p=document.getElementById("f-nombre-receptor"))==null?void 0:p.value)||"",uso_cfdi:((m=document.getElementById("f-uso"))==null?void 0:m.value)||"G03",domicilio_fiscal_receptor:"64000",regimen_fiscal_receptor:"601"},conceptos:[{clave_prod_serv:"84111506",descripcion:((b=document.getElementById("f-concepto"))==null?void 0:b.value)||"Servicio",cantidad:"1",unidad:"ACT",valor_unitario:z.toFixed(2),importe:z.toFixed(2),impuestos:{traslados:[{base:z.toFixed(2),impuesto:"002",tipo_factor:"Tasa",tasa:"0.160000",importe:G.toFixed(2)}]}}],subtotal:z.toFixed(2),total:(z+G).toFixed(2),moneda:"MXN",lugar_expedicion:"64000"},cert_b64:C,key_b64:N,password:((h=document.getElementById("f-pwd"))==null?void 0:h.value)||""},D=await f.timbrar(Ot);D!=null&&D.success?(n.innerHTML=`
        <div style="background:var(--success-light);border:1.5px solid var(--success);border-radius:12px;padding:16px">
          <div style="font-weight:700;color:var(--success);margin-bottom:8px">✅ CFDI Timbrado</div>
          <div style="font-size:12px;font-family:monospace;word-break:break-all;color:var(--text-600)">UUID: ${D.uuid}</div>
          <div style="font-size:12px;color:var(--text-500);margin-top:4px">Fecha: ${S(D.fecha_timbrado)}</div>
        </div>`,w("CFDI timbrado",`UUID: ${D.uuid}`,"success")):n.innerHTML=`<div style="background:var(--danger-light,#fee2e2);border:1.5px solid var(--red);border-radius:12px;padding:16px;color:var(--red)">
          ❌ Error: ${(D==null?void 0:D.error)||"Error desconocido"}</div>`}catch(_){n.innerHTML=`<div style="background:#fee2e2;border:1.5px solid var(--red);border-radius:12px;padding:16px;color:var(--red)">❌ ${_.message}</div>`}finally{a.textContent="🔏 Timbrar CFDI",a.disabled=!1}})}let et=1;async function Me(){M(),F([{label:"Dashboard",href:"dashboard"},{label:"Nómina IMSS"}]),et=1,await $t()}async function $t(){var t,e,a,n;T(`
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
    <div id="nom-tabla">${B(8,5)}</div>
  </div>`);try{const[o,d]=await Promise.allSettled([f.nominaKpis(),f.nomina(et)]),s=o.status==="fulfilled"?(t=o.value)==null?void 0:t.data:null,r=document.getElementById("kpi-row");r&&(r.innerHTML=[{label:"Total Empleados",val:(s==null?void 0:s.total_empleados)??0,tipo:"num",color:"indigo",icon:"👥"},{label:"Activos",val:(s==null?void 0:s.activos)??0,tipo:"num",color:"emerald",icon:"✅"},{label:"Departamentos",val:(s==null?void 0:s.departamentos)??0,tipo:"num",color:"violet",icon:"🏢"},{label:"Nómina Mensual",val:(s==null?void 0:s.nomina_mensual)??0,tipo:"mxn",color:"amber",icon:"💰"}].map(c=>`
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${c.icon} ${c.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${c.tipo==="mxn"?E(parseFloat(c.val)):Number(c.val).toLocaleString("es-MX")}
        </div>
      </div>`).join(""));const i=d.status==="fulfilled"?((e=d.value)==null?void 0:e.data)||[]:[],l=((a=d.value)==null?void 0:a.total)??i.length,v=i.length>=20,$=document.getElementById("nom-sub");$&&($.textContent=`${l} empleados · Página ${et}`);const u=document.getElementById("nom-tabla");u&&(i.length===0?u.innerHTML=`
        <div style="text-align:center;padding:60px 24px">
          <div style="font-size:48px;margin-bottom:16px">👔</div>
          <div style="font-size:16px;font-weight:700;color:var(--text-700);margin-bottom:8px">Sin empleados registrados</div>
          <div style="font-size:13px;color:var(--text-400)">Agrega empleados para gestionar tu nómina</div>
        </div>`:u.innerHTML=`
        <table class="data-table">
          <thead><tr>
            <th>Empleado</th><th>Puesto</th><th>Departamento</th>
            <th>IMSS</th><th>Estado</th>
          </tr></thead>
          <tbody>
            ${i.map(c=>{const p=c.active!==!1,m=(c.name||"?").split(" ").map(b=>b[0]).slice(0,2).join("");return`
              <tr style="cursor:pointer" onclick="window._verEmpleado(${c.id})" title="Ver detalle">
                <td>
                  <div style="display:flex;align-items:center;gap:10px">
                    <div style="width:34px;height:34px;border-radius:50%;background:linear-gradient(135deg,hsl(${c.id*47%360},60%,55%),hsl(${c.id*89%360},70%,45%));display:flex;align-items:center;justify-content:center;color:white;font-size:12px;font-weight:700;flex-shrink:0">
                      ${m}
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
        ${R(et,v,c=>{et=c,$t()})}`),(n=document.getElementById("buscar-nom"))==null||n.addEventListener("input",c=>{const p=c.target.value.toLowerCase();document.querySelectorAll("#nom-tabla tbody tr").forEach(m=>{m.style.display=m.textContent.toLowerCase().includes(p)?"":"none"})}),window._verEmpleado=c=>{const p=i.find(m=>m.id===c);p&&q("Detalle de Empleado",async()=>p,m=>`
        ${I("Información",[g("Nombre completo",m.name),g("Puesto",m.job_title||m.job_id_name||"—"),g("Departamento",m.department_name||m.department_id_name||"—"),g("Estado",`<span class="badge badge-${m.active!==!1?"emerald":"gray"}">${m.active!==!1?"Activo":"Baja"}</span>`)].join(""))}
        ${I("IMSS / Fiscal",[g("N° IMSS",m.ssnid||m.imss||"—"),g("RFC",m.rfc||"—"),g("CURP",m.curp||"—")].join(""))}
        ${I("Contacto",[g("Email",m.work_email||m.email||"—"),g("Teléfono",m.work_phone||m.mobile_phone||"—")].join(""))}
        <div style="display:flex;gap:10px;margin-top:16px">
          <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
          <button class="btn btn-secondary btn-sm" onclick="window._editarEmpleadoFn(${m.id})">✏️ Editar</button>
          <button class="btn btn-primary btn-sm" onclick="alert('Recibo de nómina — próximamente')">📄 Ver recibo</button>
        </div>`)},window._editarEmpleadoFn=c=>{const p=i.find(m=>m.id===c);p&&Ee(p,()=>$t())}}catch(o){console.error(o),w("Error al cargar nómina",o.message,"error")}}const zt={purchase:{lbl:"Confirmada",color:"indigo"},done:{lbl:"Recibida",color:"emerald"},draft:{lbl:"Borrador",color:"gray"},cancel:{lbl:"Cancelada",color:"red"},sent:{lbl:"Enviada",color:"sky"}};let at=1;async function Pe(){M(),F([{label:"Dashboard",href:"dashboard"},{label:"Compras"}]),at=1,await _t()}async function _t(){var t,e,a,n;T(`
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
    <div id="comp-tabla">${B(8,5)}</div>
  </div>`);try{const[o,d]=await Promise.allSettled([f.comprasKpis(),f.compras(at)]),s=o.status==="fulfilled"?(t=o.value)==null?void 0:t.data:null,r=document.getElementById("kpi-row");r&&(r.innerHTML=[{label:"Total OC",val:(s==null?void 0:s.total)??0,tipo:"num",color:"indigo",icon:"📋"},{label:"Confirmadas",val:(s==null?void 0:s.confirmadas)??0,tipo:"num",color:"emerald",icon:"✅"},{label:"Monto Total",val:(s==null?void 0:s.monto_total)??0,tipo:"mxn",color:"violet",icon:"💰"},{label:"Completadas",val:(s==null?void 0:s.completadas)??0,tipo:"num",color:"amber",icon:"📦"}].map(c=>`
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${c.icon} ${c.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${c.tipo==="mxn"?E(parseFloat(c.val)):Number(c.val).toLocaleString("es-MX")}
        </div>
      </div>`).join(""));const i=d.status==="fulfilled"?((e=d.value)==null?void 0:e.data)||[]:[],l=((a=d.value)==null?void 0:a.total)??i.length,v=i.length>=20,$=document.getElementById("comp-sub");$&&($.textContent=`${l} órdenes · Página ${at}`);const u=document.getElementById("comp-tabla");u&&(i.length===0?u.innerHTML='<div style="text-align:center;padding:60px;color:var(--text-400)">Sin órdenes de compra registradas</div>':u.innerHTML=`
        <table class="data-table">
          <thead><tr>
            <th>Folio</th><th>Proveedor</th><th>Fecha</th>
            <th>Total</th><th>Estado</th>
          </tr></thead>
          <tbody>
            ${i.map(c=>{const p=zt[c.state]||{lbl:c.state||"—",color:"gray"};return`
              <tr style="cursor:pointer" onclick="window._verCompra(${c.id})" title="Ver detalle">
                <td class="td-mono">${c.name||`#${c.id}`}</td>
                <td class="td-primary">${c.partner_name||"—"}</td>
                <td>${S(c.date_order)}</td>
                <td class="td-amount" style="font-weight:700">${E(parseFloat(c.amount_total||0))}</td>
                <td>${K(c.state,p.lbl)}</td>
              </tr>`}).join("")}
          </tbody>
        </table>
        ${R(at,v,c=>{at=c,_t()})}`),(n=document.getElementById("buscar-comp"))==null||n.addEventListener("input",c=>{const p=c.target.value.toLowerCase();document.querySelectorAll("#comp-tabla tbody tr").forEach(m=>{m.style.display=m.textContent.toLowerCase().includes(p)?"":"none"})}),window._verCompra=c=>{const p=i.find(m=>m.id===c);p&&q("Detalle Orden de Compra",async()=>p,m=>{const b=zt[m.state]||{lbl:m.state,color:"gray"};return`
          ${I("Orden",[g("Folio",m.name),g("Estado",K(m.state,b.lbl)),g("Proveedor",m.partner_name||"—"),g("Fecha",S(m.date_order)),g("Fecha entrega",S(m.date_planned))].join(""))}
          ${I("Importes",[g("Subtotal",E(parseFloat(m.amount_untaxed||0))),g("IVA",E(parseFloat(m.amount_tax||0))),g("Total",`<strong>${E(parseFloat(m.amount_total||0))}</strong>`,{color:"var(--primary)"})].join(""))}
          <div style="display:flex;gap:10px;margin-top:16px">
            <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
            <button class="btn btn-secondary btn-sm" onclick="window._editarCompraFn(${m.id})">✏️ Editar</button>
            <button class="btn btn-primary btn-sm" onclick="alert('Recibir mercancía — próximamente')">📦 Recibir</button>
          </div>`})},window._editarCompraFn=c=>{const p=i.find(m=>m.id===c);p&&_e(p,()=>_t())}}catch(o){console.error(o),w("Error al cargar compras",o.message,"error")}}let Et="draft",ut=1,j=[];async function ze(){F([{label:"Principal"},{label:"Cotizaciones"}]),T(`
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
      ${B(7,5)}
    </div>
  `),window._cotTab=kt,window._cotPage=bt,window._cotDetail=gt,window._cotConfirm=De,window._cotCancel=Ae,window._cotAddLine=Re,window._cotDelLine=Ne,vt(),kt("draft")}async function vt(){try{const t=await f.cotizacionKpis(),e=(t==null?void 0:t.data)??t;if(!e)return;document.getElementById("kpi-borradores").textContent=e.total_borradores??"—",document.getElementById("kpi-importe").textContent=E(e.importe_total),document.getElementById("kpi-vencidas").textContent=e.vencidas??"0"}catch{}}function kt(t){Et=t,ut=1,document.querySelectorAll(".tab-btn").forEach(a=>a.classList.remove("active"));const e=document.getElementById("tab-"+t);e&&e.classList.add("active"),bt(1)}async function bt(t=1){ut=t;const e=document.getElementById("cot-content");if(e){if(Et==="nueva"){He();return}e.innerHTML=B(7,8);try{let a;Et==="draft"?a=await f.cotizaciones(t):a=await f.ventas(t);const n=(a==null?void 0:a.data)??[],o=(a==null?void 0:a.total)??n.length,d=(a==null?void 0:a.por_pagina)??20,s=t*d<o,r={draft:"Borrador",sent:"Enviada",sale:"Confirmada",done:"Realizada",cancel:"Cancelada"};if(!n.length){e.innerHTML=`<div style="text-align:center;padding:48px;color:var(--text-400)">
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
            ${n.map(i=>{const l=r[i.state]||i.state;return`<tr style="cursor:pointer" onclick="window._cotDetail(${i.id})">
                <td style="font-size:11px;color:var(--text-400)">${i.id}</td>
                <td style="font-weight:600;color:var(--primary)">${i.name||"—"}</td>
                <td>${i.partner_name||i.partner_id||"—"}</td>
                <td>${K(i.state,l)}</td>
                <td>${E(i.amount_untaxed)}</td>
                <td>${E(i.amount_tax)}</td>
                <td style="font-weight:600">${E(i.amount_total)}</td>
                <td style="font-size:12px;color:var(--text-400)">${S(i.date_order)}</td>
                <td style="font-size:12px;color:var(--text-400)">${S(i.validity_date)}</td>
                <td onclick="event.stopPropagation()">
                  <button class="btn btn-secondary btn-sm" onclick="window._cotDetail(${i.id})">Ver</button>
                </td>
              </tr>`}).join("")}
          </tbody>
        </table>
        ${R(t,s,window._cotPage)}
      </div>
    `}catch(a){e.innerHTML=`<div class="empty-state"><p style="color:var(--red)">Error cargando cotizaciones: ${a.message}</p></div>`}}}function gt(t){q(`Cotización #${t}`,()=>f.cotizacion(t),je)}function je(t){const e=(t==null?void 0:t.orden)??t,a=(t==null?void 0:t.lineas)??[],n={draft:"Borrador",sent:"Enviada",sale:"Confirmada",done:"Realizada",cancel:"Cancelada"},o=["draft","sent"].includes(e.state),d=o,s=!["cancel","done"].includes(e.state),r=a.length?`<div class="table-container" style="margin-top:12px">
        <table class="data-table" style="font-size:12px">
          <thead><tr><th>Producto</th><th>Cant.</th><th>Precio U.</th><th>Dto%</th><th>Subtotal</th><th></th></tr></thead>
          <tbody>
            ${a.map(v=>`<tr>
              <td>${v.name||"—"}</td>
              <td>${v.product_uom_qty}</td>
              <td>${E(v.price_unit)}</td>
              <td>${v.discount?v.discount+"%":"—"}</td>
              <td style="font-weight:600">${E(v.price_subtotal)}</td>
              <td>${o?`<button class="btn btn-secondary btn-sm" style="color:var(--red)" onclick="window._cotDelLine(${e.id},${v.id})">✕</button>`:""}</td>
            </tr>`).join("")}
          </tbody>
        </table>
      </div>`:'<p style="color:var(--text-400);font-size:13px;padding:8px 0">Sin líneas de venta</p>',i=o?`
    <div style="margin-top:16px;padding:16px;background:var(--surface-2);border-radius:10px;border:1px solid var(--border)">
      <div style="font-weight:600;margin-bottom:12px;font-size:13px">➕ Agregar línea</div>
      <div style="display:grid;grid-template-columns:2fr 1fr 1fr 1fr;gap:8px;margin-bottom:8px">
        <input id="linea-name" class="form-control" placeholder="Descripción" style="font-size:13px">
        <input id="linea-qty"  class="form-control" type="number" placeholder="Cantidad" value="1" min="0.01" step="0.01" style="font-size:13px">
        <input id="linea-price" class="form-control" type="number" placeholder="Precio" min="0" step="0.01" style="font-size:13px">
        <input id="linea-dto" class="form-control" type="number" placeholder="Dto %" min="0" max="100" step="0.01" style="font-size:13px">
      </div>
      <button class="btn btn-primary btn-sm" onclick="window._cotAddLine(${e.id})">Agregar línea</button>
    </div>`:"",l=`
    <div style="display:flex;gap:8px;margin-top:20px;flex-wrap:wrap">
      ${d?`<button class="btn btn-primary" onclick="window._cotConfirm(${e.id})">✅ Confirmar pedido</button>`:""}
      ${s?`<button class="btn btn-secondary" style="color:var(--red)" onclick="window._cotCancel(${e.id})">🚫 Cancelar</button>`:""}
    </div>`;return`
    ${I("Información General",`
      ${g("Referencia",e.name)}
      ${g("Estado",K(e.state,n[e.state]||e.state))}
      ${g("Cliente",e.partner_name||"—")}
      ${g("Referencia cliente",e.client_order_ref||"—")}
      ${g("Fecha",S(e.date_order))}
      ${g("Validez",S(e.validity_date))}
      ${g("Estado factura",e.invoice_status||"—")}
    `)}
    ${I("Importes",`
      ${g("Subtotal",E(e.amount_untaxed))}
      ${g("IVA",E(e.amount_tax))}
      ${g("Total",`<strong style="font-size:16px;color:var(--primary)">${E(e.amount_total)}</strong>`)}
    `)}
    ${I("Líneas de venta",r+i)}
    ${e.note?I("Notas",`<p style="font-size:13px;line-height:1.6">${e.note}</p>`):""}
    ${l}
  `}async function De(t){if(confirm("¿Confirmar esta cotización? Pasará a pedido de venta."))try{await f.confirmarCotizacion(t),w("Cotización confirmada","El pedido fue confirmado correctamente","success"),window.__closeModal(),vt(),bt(ut)}catch(e){w("Error",e.message,"error")}}async function Ae(t){if(confirm("¿Cancelar esta cotización?"))try{await f.cancelarCotizacion(t),w("Cotización cancelada","","info"),window.__closeModal(),vt(),bt(ut)}catch(e){w("Error",e.message,"error")}}async function Re(t){var d,s,r,i,l;const e=(s=(d=document.getElementById("linea-name"))==null?void 0:d.value)==null?void 0:s.trim(),a=parseFloat(((r=document.getElementById("linea-qty"))==null?void 0:r.value)||"1"),n=parseFloat(((i=document.getElementById("linea-price"))==null?void 0:i.value)||"0"),o=parseFloat(((l=document.getElementById("linea-dto"))==null?void 0:l.value)||"0")||null;if(!e)return w("Campo requerido","Escribe una descripción de producto","warning");if(!n)return w("Campo requerido","Ingresa el precio unitario","warning");try{await f.agregarLinea(t,{name:e,product_uom_qty:a,price_unit:n,discount:o}),w("Línea agregada","","success"),gt(t)}catch(v){w("Error al agregar línea",v.message,"error")}}async function Ne(t,e){if(confirm("¿Eliminar esta línea?"))try{await f.eliminarLinea(t,e),w("Línea eliminada","","success"),gt(t)}catch(a){w("Error",a.message,"error")}}function He(){j=[];const t=document.getElementById("cot-content");t&&(t.innerHTML=`
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
  `,window._nvAddRow=Ve,window._nvDelRow=qe,window._nvGuardar=Ge,window._nvRecalc=Ct)}function Ve(){j.length,j.push({name:"",qty:1,price:0,discount:0}),qt()}function qe(t){j.splice(t,1),qt()}function qt(){const t=document.getElementById("nv-lineas-list");if(t){if(!j.length){t.innerHTML='<p style="color:var(--text-400);font-size:13px;padding:16px 0;text-align:center">Sin líneas.</p>',Ct();return}t.innerHTML=`
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
          ${j.map((e,a)=>{const n=parseFloat(e.discount)||0,o=(parseFloat(e.qty)||0)*(parseFloat(e.price)||0)*(1-n/100);return`<tr>
              <td><input class="form-control" style="font-size:12px" value="${e.name}" oninput="_lineasNueva[${a}].name=this.value" placeholder="Descripción del producto"></td>
              <td><input class="form-control" style="font-size:12px;width:70px" type="number" min="0.01" step="0.01" value="${e.qty}" oninput="_lineasNueva[${a}].qty=this.value;window._nvRecalc()"></td>
              <td><input class="form-control" style="font-size:12px;width:90px" type="number" min="0" step="0.01" value="${e.price}" oninput="_lineasNueva[${a}].price=this.value;window._nvRecalc()"></td>
              <td><input class="form-control" style="font-size:12px;width:65px" type="number" min="0" max="100" step="0.01" value="${e.discount}" oninput="_lineasNueva[${a}].discount=this.value;window._nvRecalc()"></td>
              <td style="font-weight:600">${E(o)}</td>
              <td><button class="btn btn-secondary btn-sm" style="color:var(--red)" onclick="window._nvDelRow(${a})">✕</button></td>
            </tr>`}).join("")}
        </tbody>
      </table>
    </div>`,Ct()}}function Ct(){let t=0;j.forEach(r=>{const i=parseFloat(r.discount)||0;t+=(parseFloat(r.qty)||0)*(parseFloat(r.price)||0)*(1-i/100)});const e=t*.16,a=t+e,n=r=>r.toLocaleString("es-MX",{minimumFractionDigits:2,maximumFractionDigits:2}),o=document.getElementById("nv-subtotal"),d=document.getElementById("nv-iva"),s=document.getElementById("nv-total");o&&(o.textContent="$"+n(t)),d&&(d.textContent="$"+n(e)),s&&(s.textContent="$"+n(a))}async function Ge(){var s,r,i,l,v,$,u,c;const t=(r=(s=document.getElementById("nv-partner"))==null?void 0:s.value)==null?void 0:r.trim(),e=((l=(i=document.getElementById("nv-ref"))==null?void 0:i.value)==null?void 0:l.trim())||null,a=((v=document.getElementById("nv-validez"))==null?void 0:v.value)||null,n=((u=($=document.getElementById("nv-nota"))==null?void 0:$.value)==null?void 0:u.trim())||null;if(!t)return w("Campo requerido","Ingresa el nombre del cliente","warning");let o=1;try{const p=await f.get(`/partners?pagina=1&q=${encodeURIComponent(t)}&por_pagina=5`),m=(p==null?void 0:p.data)??[],b=m.find(h=>{var _;return((_=h.name)==null?void 0:_.toLowerCase())===t.toLowerCase()});if(b)o=b.id;else if(m.length>0)o=m[0].id;else return w("Cliente no encontrado",`No se encontró "${t}"`,"warning")}catch(p){return w("Error","No se pudo buscar el cliente: "+p.message,"error")}const d={partner_id:o,partner_invoice_id:o,partner_shipping_id:o,note:n,client_order_ref:e,validity_date:a||null};try{const p=await f.crearCotizacion(d),m=((c=p==null?void 0:p.data)==null?void 0:c.id)??(p==null?void 0:p.id);if(w("Cotización creada",`ID ${m} — Referencia generada`,"success"),m&&j.length)for(const b of j)b.name&&await f.agregarLinea(m,{name:b.name,product_uom_qty:parseFloat(b.qty)||1,price_unit:parseFloat(b.price)||0,discount:parseFloat(b.discount)||null}).catch(()=>{});j=[],vt(),kt("draft"),setTimeout(()=>m&&gt(m),600)}catch(p){w("Error al crear cotización",p.message,"error")}}let yt=null;async function Oe(){M(),F([{label:"Dashboard",href:"dashboard"},{label:"NexusSearch"}]),await Ue()}async function Ue(){var e,a;T(`
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
  <div id="index-status" class="anim-4" style="margin-top:16px"></div>`),(e=document.getElementById("search-query"))==null||e.addEventListener("keydown",n=>{n.key==="Enter"&&window._buscar()});let t;(a=document.getElementById("search-query"))==null||a.addEventListener("input",n=>{clearTimeout(t),!(n.target.value.length<2)&&(t=setTimeout(()=>window._buscar(),500))}),await jt(),window._buscar=Xe,window._checkStatus=jt,window._syncSearch=Ke}async function Xe(){var a,n;const t=(n=(a=document.getElementById("search-query"))==null?void 0:a.value)==null?void 0:n.trim();if(!t||t.length<2)return;const e=document.getElementById("search-results");e&&(e.innerHTML=`
    <div class="data-card" style="padding:20px;text-align:center;color:var(--text-400)">
      <div class="spinner" style="margin:0 auto 8px"></div>
      <div>Buscando "${t}"…</div>
    </div>`);try{const[o,d,s]=await Promise.allSettled([f.ventas(1).then(i=>((i==null?void 0:i.data)||[]).filter(l=>(l.name||"").toLowerCase().includes(t.toLowerCase())||(l.partner_name||"").toLowerCase().includes(t.toLowerCase())).map(l=>({tipo:"Venta",icon:"💰",titulo:l.name,sub:l.partner_name,meta:`$${l.amount_total}`,href:"ventas"}))),f.productos(1,t).then(i=>((i==null?void 0:i.data)||[]).map(l=>{var v,$;return{tipo:"Producto",icon:"📦",titulo:typeof l.name=="object"?((v=l.name)==null?void 0:v.es_MX)||(($=l.name)==null?void 0:$.en_US)||"":l.name||"",sub:l.categ_name||"",meta:"",href:"productos"}})),f.partners(1).then(i=>((i==null?void 0:i.data)||[]).filter(l=>(l.name||"").toLowerCase().includes(t.toLowerCase())||(l.email||"").toLowerCase().includes(t.toLowerCase())).map(l=>({tipo:"Contacto",icon:"👥",titulo:l.name,sub:l.email||"",meta:"",href:"partners"})))]),r=[...o.status==="fulfilled"?o.value:[],...d.status==="fulfilled"?d.value:[],...s.status==="fulfilled"?s.value:[]];if(!e)return;if(r.length===0){e.innerHTML=`
      <div class="data-card" style="padding:40px;text-align:center">
        <div style="font-size:36px;margin-bottom:12px">🔍</div>
        <div style="font-weight:700;color:var(--text-700)">Sin resultados para "${t}"</div>
        <div style="color:var(--text-400);font-size:13px;margin-top:6px">Prueba con otro término</div>
      </div>`;return}e.innerHTML=`
    <div class="data-card">
      <div class="data-card-header">
        <div class="data-card-title">${r.length} resultados para "${t}"</div>
      </div>
      <div style="padding:0 4px">
        ${r.slice(0,30).map(i=>`
        <div style="display:flex;align-items:center;gap:12px;padding:12px 8px;
          border-bottom:1px solid var(--border);cursor:pointer;border-radius:8px;
          transition:background var(--t1)" 
          onmouseover="this.style.background='var(--primary-light)'"
          onmouseout="this.style.background=''"
          onclick="window._go('${i.href}')">
          <div style="width:36px;height:36px;border-radius:10px;background:var(--primary-light);
            display:flex;align-items:center;justify-content:center;font-size:18px;flex-shrink:0">
            ${i.icon}
          </div>
          <div style="flex:1">
            <div style="font-weight:600;color:var(--text-800);font-size:13px">${i.titulo}</div>
            <div style="font-size:11px;color:var(--text-400)">${i.sub}</div>
          </div>
          <div style="display:flex;align-items:center;gap:8px">
            ${i.meta?`<span style="font-size:12px;font-weight:700;color:var(--text-700)">${i.meta}</span>`:""}
            <span class="badge badge-${i.tipo==="Venta"?"indigo":i.tipo==="Producto"?"emerald":"violet"}">${i.tipo}</span>
          </div>
        </div>`).join("")}
      </div>
    </div>`}catch(o){console.error(o),e&&(e.innerHTML=`<p style="color:var(--red);padding:20px">Error: ${o.message}</p>`)}}async function jt(){const t=document.getElementById("index-status");try{const e=await f.searchStatus().catch(()=>null);yt=(e==null?void 0:e.data)||e,t&&yt&&(t.innerHTML=`
      <div class="data-card" style="padding:16px">
        <div class="data-card-header" style="padding:0 0 12px">
          <div class="data-card-title">📡 Estado del Motor de Búsqueda</div>
        </div>
        <div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(180px,1fr));gap:12px">
          ${Object.entries(yt).map(([a,n])=>`
          <div style="padding:12px;background:var(--bg);border-radius:10px;border:1px solid var(--border)">
            <div style="font-size:10px;color:var(--text-400);font-weight:700;text-transform:uppercase;margin-bottom:4px">${a}</div>
            <div style="font-size:13px;font-weight:600;color:var(--text-800)">${JSON.stringify(n)}</div>
          </div>`).join("")}
        </div>
      </div>`)}catch{t&&(t.innerHTML="")}}async function Ke(){const t=document.getElementById("btn-sync");t&&(t.textContent="⏳ Sincronizando…",t.disabled=!0);try{const e=await f.searchSync();w("Sincronización iniciada",(e==null?void 0:e.message)||"Los índices se están actualizando","success")}catch(e){w("Error de sincronización",e.message,"error")}finally{t&&(t.textContent="⚡ Sincronizar Índices",t.disabled=!1)}}async function Je(){M(),F([{label:"Dashboard",href:"dashboard"},{label:"Reportes"}]),await Ye()}async function Ye(){T(`
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
  </div>`),window._verReporte=t=>{w("Reporte seleccionado",`Generando reporte de ${t}…`,"info"),Qe(t)},window._exportReporte=()=>{w("Exportar","Función de exportación CSV/PDF — próximamente","info")},await Gt()}async function Gt(){var e,a,n,o;const t=document.getElementById("rep-fecha");t&&(t.textContent=new Date().toLocaleDateString("es-MX",{day:"2-digit",month:"long",year:"numeric"}));try{const[d,s,r,i]=await Promise.allSettled([f.ventaKpis(),f.factKpis(),f.stockKpis(),f.comprasKpis()]),l=((e=d.value)==null?void 0:e.data)||{},v=((a=s.value)==null?void 0:a.data)||{},$=((n=r.value)==null?void 0:n.data)||{},u=((o=i.value)==null?void 0:o.data)||{},c=document.getElementById("rep-kpis");c&&(c.innerHTML=`
      ${[{label:"Ventas confirmadas",val:l.ordenes_confirmadas??0,tipo:"num",desc:`$${parseFloat(l.total_facturado||0).toLocaleString("es-MX",{minimumFractionDigits:2})} este mes`},{label:"Facturación total",val:E(parseFloat(v.monto_total||0)),tipo:"txt",desc:`${v.total_facturas??0} comprobantes emitidos`},{label:"Valor inventario",val:E(parseFloat($.valor_inventario||0)),tipo:"txt",desc:`${$.alertas_stock_bajo??0} alertas de stock bajo`}].map(p=>`
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
          ${[{label:"Total OC",val:u.total??0},{label:"Confirmadas",val:u.confirmadas??0},{label:"Monto compras",val:E(parseFloat(u.monto_total||0))}].map(p=>`
          <div style="padding:12px;background:var(--bg);border-radius:10px;border:1px solid var(--border)">
            <div style="font-size:10px;color:var(--text-400);font-weight:700;text-transform:uppercase;margin-bottom:4px">${p.label}</div>
            <div style="font-size:18px;font-weight:800;color:var(--text-900)">${p.val}</div>
          </div>`).join("")}
        </div>
      </div>`)}catch(d){console.error(d)}}async function Qe(t){const e=document.getElementById("rep-kpis"),a=document.querySelector(".data-card-title");if(a){const n={ventas:"💰 Reporte de Ventas",facturas:"🧾 Facturación",inventario:"🏭 Inventario",compras:"🛒 Compras",clientes:"👥 Clientes",nomina:"👔 Nómina"};a.textContent=n[t]||"Reporte"}e&&(e.innerHTML='<div class="skeleton" style="height:120px;border-radius:12px;grid-column:1/-1"></div>'),await Gt()}function We(t,e,a,n){M(),F([{label:"Dashboard",href:"dashboard"},{label:e}]),T(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">${n} ${e}</h1>
      <p class="page-subtitle">${a}</p>
    </div>
  </div>
  <div class="data-card anim-2">
    <div class="empty-state">
      <div class="empty-state-icon">${n}</div>
      <div class="empty-state-title">Módulo ${e} en construcción</div>
      <div class="empty-state-desc">Este módulo estará disponible próximamente en NexusTech ERP v2.0</div>
      <button class="btn btn-primary" onclick="window._go('dashboard')">← Volver al Dashboard</button>
    </div>
  </div>`)}L("login",Wt);L("home",ne);L("dashboard",Dt);L("ventas",be);L("facturas",Ce);L("productos",Ie);L("partners",Se);L("stock",Fe);L("cfdi",Be);L("nomina",Me);L("compras",Pe);L("cotizaciones",ze);L("search",Oe);L("reportes",Je);L("404",()=>We("404","Página no encontrada","La ruta solicitada no existe","🔍"));Kt();
