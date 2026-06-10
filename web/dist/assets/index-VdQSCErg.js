(function(){const e=document.createElement("link").relList;if(e&&e.supports&&e.supports("modulepreload"))return;for(const o of document.querySelectorAll('link[rel="modulepreload"]'))a(o);new MutationObserver(o=>{for(const r of o)if(r.type==="childList")for(const s of r.addedNodes)s.tagName==="LINK"&&s.rel==="modulepreload"&&a(s)}).observe(document,{childList:!0,subtree:!0});function i(o){const r={};return o.integrity&&(r.integrity=o.integrity),o.referrerPolicy&&(r.referrerPolicy=o.referrerPolicy),o.crossOrigin==="use-credentials"?r.credentials="include":o.crossOrigin==="anonymous"?r.credentials="omit":r.credentials="same-origin",r}function a(o){if(o.ep)return;o.ep=!0;const r=i(o);fetch(o.href,r)}})();const et={isLoggedIn:()=>!!localStorage.getItem("nx_token"),getUser:()=>{try{return JSON.parse(localStorage.getItem("nx_user")||"{}")}catch{return{}}},setSession(t,e){localStorage.setItem("nx_token",t),localStorage.setItem("nx_user",JSON.stringify(e))},clear(){localStorage.removeItem("nx_token"),localStorage.removeItem("nx_user")}},nt={};function S(t,e){nt[t]=e}function at(t){window.location.hash=t}function Pt(){window.addEventListener("hashchange",yt),yt()}function yt(){const t=window.location.hash.replace("#","")||"dashboard";if(!et.isLoggedIn()&&t!=="login"){at("login");return}if(et.isLoggedIn()&&t==="login"){at("dashboard");return}const e=nt[t];e?e():nt[404]&&nt[404]()}const Lt="/api/v1";function jt(){return localStorage.getItem("nx_token")}class Dt extends Error{constructor(e,i){super(i),this.status=e}}async function f(t,e,i){const a=jt(),o=await fetch(Lt+e,{method:t,headers:{"Content-Type":"application/json",...a?{Authorization:`Bearer ${a}`}:{}},...i!==void 0?{body:JSON.stringify(i)}:{}});if(o.status===401)return localStorage.removeItem("nx_token"),localStorage.removeItem("nx_user"),window.location.hash="login",null;if(!o.ok)throw new Dt(o.status,await o.text());return(o.headers.get("content-type")||"").includes("application/json")?o.json():o.text()}const x={get:t=>f("GET",t),post:(t,e)=>f("POST",t,e),put:(t,e)=>f("PUT",t,e),del:t=>f("DELETE",t),login:(t,e)=>f("POST","/auth/login",{login:t,password:e}),logout:()=>f("POST","/auth/logout",{}),dashboard:()=>f("GET","/dashboard"),ventaKpis:()=>f("GET","/ventas/kpis"),factKpis:()=>f("GET","/facturas/kpis"),stockKpis:()=>f("GET","/stock/kpis"),ventas:(t=1)=>f("GET",`/ventas?pagina=${t}`),venta:t=>f("GET",`/ventas/${t}`),facturas:(t=1)=>f("GET",`/facturas?pagina=${t}`),factura:t=>f("GET",`/facturas/${t}`),porCobrar:()=>f("GET","/facturas/por-cobrar"),productos:(t=1,e="")=>f("GET",`/productos?pagina=${t}&q=${encodeURIComponent(e)}`),producto:t=>f("GET",`/productos/${t}`),partners:(t=1)=>f("GET",`/partners?pagina=${t}`),partner:t=>f("GET",`/partners/${t}`),clientes:(t=1)=>f("GET",`/clientes?pagina=${t}`),proveedores:(t=1)=>f("GET",`/proveedores?pagina=${t}`),stock:(t=1)=>f("GET",`/stock?pagina=${t}`),stockKpis:()=>f("GET","/stock/kpis"),stockBajo:()=>f("GET","/stock/bajo"),stockProducto:t=>f("GET",`/stock/producto/${t}`),cfdiTimbrados:(t=1)=>f("GET",`/cfdi/timbrados?pagina=${t}`),cfdiTimbrado:t=>f("GET",`/cfdi/timbrados/${t}`),cfdiKpis:()=>f("GET","/cfdi/kpis"),timbrar:t=>f("POST","/cfdi/timbrar",t),cancelarCfdi:t=>f("POST","/cfdi/cancelar",t),nomina:(t=1)=>f("GET",`/nomina?pagina=${t}`),nominaKpis:()=>f("GET","/nomina/kpis"),compras:(t=1)=>f("GET",`/compras?pagina=${t}`),comprasKpis:()=>f("GET","/compras/kpis"),cotizaciones:(t=1)=>f("GET",`/cotizaciones?pagina=${t}`),cotizacionKpis:()=>f("GET","/cotizaciones/kpis"),cotizacion:t=>f("GET",`/cotizaciones/${t}`),crearCotizacion:t=>f("POST","/cotizaciones",t),confirmarCotizacion:t=>f("PUT",`/cotizaciones/${t}/confirmar`),cancelarCotizacion:t=>f("PUT",`/cotizaciones/${t}/cancelar`),actualizarCotizacion:(t,e)=>f("PUT",`/cotizaciones/${t}`,e),agregarLinea:(t,e)=>f("POST",`/cotizaciones/${t}/lineas`,e),eliminarLinea:(t,e)=>f("DELETE",`/cotizaciones/${t}/lineas/${e}`),searchSync:()=>f("POST","/search/sync",{}),searchStatus:()=>f("GET","/search/status"),health:()=>f("GET","/health"),putVenta:(t,e)=>f("PUT",`/ventas/${t}`,e),putPartner:(t,e)=>f("PUT",`/partners/${t}`,e),putProducto:(t,e)=>f("PUT",`/productos/${t}`,e),putCompra:(t,e)=>f("PUT",`/compras/${t}`,e),putEmpleado:(t,e)=>f("PUT",`/nomina/${t}`,e),ajusteStock:(t,e)=>f("PUT",`/stock/${t}/ajuste`,e)};function zt(){const t=document.getElementById("__shell");t&&t.remove(),document.getElementById("app").innerHTML=`
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
  </div>`;const e=document.getElementById("lbtn"),i=document.getElementById("lu"),a=document.getElementById("lp"),o=document.getElementById("lerr");async function r(){if(e.disabled)return;const s=i.value.trim(),p=a.value;if(!s||!p){o.textContent="Ingresa usuario y contraseña",o.classList.add("show");return}e.disabled=!0,e.textContent="Verificando...",o.classList.remove("show");try{const l=await x.login(s,p),d=(l==null?void 0:l.data)||l,y=(d==null?void 0:d.access_token)||(d==null?void 0:d.token);if(y){et.setSession(y,{nombre:d.email||s,email:d.email||s,user_id:d.user_id,company_id:d.company_id}),document.getElementById("app").innerHTML="",at("dashboard");return}o.textContent="Error inesperado del servidor. Intenta de nuevo.",o.classList.add("show")}catch(l){o.textContent=(l==null?void 0:l.status)===401?"Credenciales incorrectas. Verifica tu usuario y contraseña.":`Error de conexión: ${(l==null?void 0:l.message)||"No se pudo contactar el servidor"}`,o.classList.add("show")}e.disabled=!1,e.textContent="Acceder al sistema"}e.addEventListener("click",r),a.addEventListener("keydown",s=>s.key==="Enter"&&r()),i.addEventListener("keydown",s=>s.key==="Enter"&&a.focus()),setTimeout(()=>i.focus(),100)}function q(t,e=0){return t==null||t===""?"—":Number(t).toLocaleString("es-MX",{minimumFractionDigits:e,maximumFractionDigits:e})}function E(t){return t==null?"—":(t=parseFloat(t)||0,Math.abs(t)>=1e6?`$${(t/1e6).toFixed(2)}M`:Math.abs(t)>=1e3?`$${(t/1e3).toFixed(1)}k`:`$${q(t,2)}`)}function st(t){return t==null?"—":Number(t).toLocaleString("es-MX")}function L(t){return t?new Date(t).toLocaleDateString("es-MX",{day:"2-digit",month:"short",year:"numeric"}):"—"}function k(t,e="",i="info"){const a={success:"✅",error:"❌",info:"ℹ️",warning:"⚠️"};let o=document.getElementById("__toasts");o||(o=document.createElement("div"),o.id="__toasts",o.className="toast-container",document.body.appendChild(o));const r=document.createElement("div");r.className=`toast ${i}`,r.innerHTML=`
    <span class="toast-icon">${a[i]||"ℹ️"}</span>
    <div><div class="toast-title">${t}</div>${e?`<div class="toast-msg">${e}</div>`:""}</div>`,o.appendChild(r),requestAnimationFrame(()=>r.classList.add("show")),setTimeout(()=>{r.classList.remove("show"),setTimeout(()=>r.remove(),400)},3800)}function ht(t,e,i=900,a="",o=""){if(!t)return;const r=performance.now(),s=String(e).includes(".");function p(l){const d=Math.min((l-r)/i,1),y=1-Math.pow(1-d,3),w=e*y;t.textContent=a+(s?w.toLocaleString("es-MX",{minimumFractionDigits:2,maximumFractionDigits:2}):Math.round(w).toLocaleString("es-MX"))+o,d<1&&requestAnimationFrame(p)}requestAnimationFrame(p)}function Rt(t){if(!(t!=null&&t.length))return"";const e=Math.max(...t,1);return`<div class="sparkline">${t.map((i,a)=>`<div class="spark-bar${a===t.length-1?" active":""}" style="height:${Math.max(4,Math.round(i/e*100))}%"></div>`).join("")}</div>`}function At(t=5,e=6){return`<tbody>${Array.from({length:e},()=>`<tr>${Array.from({length:t},()=>`<td><div class="skeleton" style="height:14px;width:${60+Math.random()*30}%"></div></td>`).join("")}</tr>`).join("")}</tbody>`}function T(t=5,e=4){return`<table class="data-table"><thead><tr>${Array.from({length:e},()=>`<th><div class="skeleton" style="height:12px;width:${40+Math.random()*40}%"></div></th>`).join("")}</tr></thead>${At(e,t)}</table>`}function Nt(t=5){return Array.from({length:t},()=>`
  <div class="kpi-card kpi-gray">
    <div class="kpi-label"><div class="skeleton" style="height:13px;width:60%"></div></div>
    <div class="kpi-value"><div class="skeleton" style="height:28px;width:70%"></div></div>
    <div><div class="skeleton" style="height:11px;width:40%;margin-top:6px"></div></div>
  </div>`).join("")}const Vt={sale:"emerald",done:"indigo",draft:"gray",sent:"sky",cancel:"red",posted:"emerald",in_payment:"violet",paid:"emerald",partial:"amber"};function ot(t,e){return`<span class="badge badge-${Vt[t]||"gray"} badge-dot">${e}</span>`}function V(t,e,i){return window.__pagNav=i,`
  <div class="data-table-footer">
    <span style="color:var(--text-400)">Página ${t}</span>
    <div class="pagination">
      <button class="pag-btn" ${t<=1?"disabled":""} onclick="window.__pagNav(${t-1})">&#8592; Anterior</button>
      <span class="pag-btn active">${t}</span>
      <button class="pag-btn" ${e?"":"disabled"} onclick="window.__pagNav(${t+1})">Siguiente &#8594;</button>
    </div>
  </div>`}let O=null;function j(t,e,i={}){let a=document.getElementById("__modal-overlay");a||(a=document.createElement("div"),a.id="__modal-overlay",a.innerHTML=`
      <div id="__modal-drawer">
        <div id="__modal-header">
          <span id="__modal-title"></span>
          <button id="__modal-close" onclick="window.__closeModal()">✕</button>
        </div>
        <div id="__modal-body"></div>
      </div>`,document.body.appendChild(a),a.addEventListener("click",o=>{o.target===a&&window.__closeModal()})),document.getElementById("__modal-title").textContent=t,document.getElementById("__modal-body").innerHTML=e,a.classList.add("open"),document.body.style.overflow="hidden",O&&document.removeEventListener("keydown",O),O=o=>{o.key==="Escape"&&window.__closeModal()},document.addEventListener("keydown",O),i.onMounted&&setTimeout(i.onMounted,10)}function bt(){const t=document.getElementById("__modal-overlay");t&&t.classList.remove("open"),document.body.style.overflow="",O&&(document.removeEventListener("keydown",O),O=null)}window.__closeModal=bt;async function H(t,e,i){j(t,`
    <div style="display:flex;flex-direction:column;gap:12px;padding:8px 0">
      ${[1,2,3,4,5].map(()=>'<div class="skeleton" style="height:52px;border-radius:10px"></div>').join("")}
    </div>`);try{const a=await e(),o=(a==null?void 0:a.data)??a;document.getElementById("__modal-body").innerHTML=i(o)}catch(a){document.getElementById("__modal-body").innerHTML=`<p style="color:var(--red);padding:24px">Error: ${a.message}</p>`}}function v(t,e,i={}){const a=e??"—",o=i.color?`color:${i.color}`:"";return`
  <div style="display:flex;justify-content:space-between;align-items:flex-start;
    padding:10px 0;border-bottom:1px solid var(--border)">
    <span style="font-size:12px;color:var(--text-400);font-weight:600;min-width:140px">${t}</span>
    <span style="font-size:13px;font-weight:500;text-align:right;${o}">${a}</span>
  </div>`}function I(t,e){return`
  <div style="margin-bottom:20px">
    <div style="font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.06em;
      color:var(--text-400);margin-bottom:8px;padding-bottom:6px;
      border-bottom:2px solid var(--primary)">${t}</div>
    ${e}
  </div>`}const xt=[{id:"dashboard",icon:"📊",label:"Dashboard",section:"Principal"},{id:"ventas",icon:"💰",label:"Ventas",section:"Principal"},{id:"facturas",icon:"🧾",label:"Facturación",section:"Principal"},{id:"productos",icon:"📦",label:"Productos",section:"Principal"},{id:"partners",icon:"👥",label:"Clientes",section:"Principal"},{id:"stock",icon:"🏭",label:"Inventario",section:"Principal"},{id:"cfdi",icon:"🔏",label:"CFDI 4.0",section:"Fiscal",badge:"NUEVO"},{id:"nomina",icon:"👔",label:"Nómina IMSS",section:"Fiscal"},{id:"compras",icon:"🛒",label:"Compras",section:"Operaciones"},{id:"search",icon:"🔍",label:"NexusSearch",section:"Sistema"},{id:"reportes",icon:"📈",label:"Reportes",section:"Sistema"}];function F(){if(document.getElementById("__shell"))return;const t=et.getUser(),e=(t.nombre||t.name||"AD").substring(0,2).toUpperCase(),i=[...new Set(xt.map(a=>a.section))];document.getElementById("app").innerHTML=`
  <div class="app-shell" id="__shell">
    <!-- SIDEBAR -->
    <nav class="sidebar" id="__sidebar">
      <div class="sidebar-brand">
        <div class="brand-logo">N</div>
        <div>
          <div class="brand-name">NexusTech</div>
          <div class="brand-version">ERP v2.0</div>
        </div>
      </div>

      <div class="sidebar-nav">
        ${i.map(a=>`
        <div class="nav-section">
          <div class="nav-section-title">${a}</div>
          ${xt.filter(o=>o.section===a).map(o=>`
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
  </div>`,window._go=a=>{at(a)},window._logout=()=>{et.clear();const a=document.getElementById("__shell");a&&a.remove(),at("login"),k("Sesión cerrada","Hasta pronto","info")},window.addEventListener("hashchange",wt),wt()}function B(t){const e=document.getElementById("__page");e&&(e.innerHTML=t,e.scrollTop=0)}function M(t){const e=document.getElementById("__breadcrumb");e&&(e.innerHTML=t.map((i,a)=>`
    <span class="breadcrumb-item"${a<t.length-1&&i.href?` onclick="window._go('${i.href}')"`:""}>
      ${i.label}
      ${a<t.length-1?'<span class="breadcrumb-sep">/</span>':""}
    </span>`).join(""))}function wt(){const t=window.location.hash.replace("#","")||"dashboard";document.querySelectorAll(".nav-link").forEach(e=>{e.classList.toggle("active",e.id===`nl-${t}`)})}const Ht={sale:"indigo",done:"emerald",draft:"gray",cancel:"red",sent:"sky",posted:"emerald"},Gt={sale:"Confirmada",done:"Entregada",draft:"Borrador",cancel:"Cancelada",sent:"Enviada"};function X(t,e=10){return Array.from({length:e},()=>Math.max(5,Math.round(t*(.6+Math.random()*.8))))}async function Ct(){var t,e,i,a,o,r,s,p,l;F(),M([{label:"Dashboard"}]),B(`
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
  <div class="kpi-grid anim-2" id="kpi-grid">${Nt(5)}</div>

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
      <div id="tabla-ventas">${T(6,5)}</div>
    </div>

    <div class="data-card">
      <div class="data-card-header">
        <div>
          <div class="data-card-title">⚠️ Stock Bajo</div>
          <div class="data-card-subtitle">Productos bajo nivel mínimo</div>
        </div>
        <button class="btn btn-secondary btn-sm" onclick="window._go('stock')">Inventario</button>
      </div>
      <div id="tabla-stock">${T(5,4)}</div>
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
      <div id="resumen-fiscal">${T(4,2)}</div>
    </div>

    <!-- Estado del sistema -->
    <div class="data-card" style="padding:20px">
      <div style="font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.06em;color:var(--text-400);margin-bottom:16px">🟢 Estado del Sistema</div>
      <div id="system-status">${T(4,2)}</div>
    </div>
  </div>`);try{const[d,y,w]=await Promise.allSettled([x.dashboard(),x.ventas(1),x.stockBajo()]),u=d.status==="fulfilled"?(t=d.value)==null?void 0:t.data:null,n=[{key:"ventas_mes",label:"Ventas del Mes",tipo:"mxn",icon:"💰",color:"indigo",valor:parseFloat(((e=u==null?void 0:u.ventas)==null?void 0:e.importe_mes)||0),trend:null,spark:X(100)},{key:"facturas",label:"Facturas Emitidas",tipo:"num",icon:"🧾",color:"emerald",valor:parseInt(((i=u==null?void 0:u.facturacion)==null?void 0:i.total_facturas)||0),trend:null,spark:X(50)},{key:"cobrar",label:"Por Cobrar",tipo:"mxn",icon:"📋",color:"amber",valor:parseFloat(((a=u==null?void 0:u.facturacion)==null?void 0:a.por_cobrar)||0),trend:null,spark:X(80)},{key:"stock_total",label:"Productos en Stock",tipo:"num",icon:"📦",color:"sky",valor:parseInt(((o=u==null?void 0:u.inventario)==null?void 0:o.total_productos_con_stock)||0),trend:null,spark:X(80)},{key:"stock_bajo",label:"Alertas Stock Bajo",tipo:"num",icon:"⚠️",color:"rose",valor:parseInt(((r=u==null?void 0:u.inventario)==null?void 0:r.alertas_stock_bajo)||0),trend:null,spark:X(20)}],c=document.getElementById("kpi-grid");c&&(c.innerHTML=n.map(g=>`
      <div class="kpi-card kpi-${g.color}">
        <div class="kpi-label">
          <span>${g.label}</span>
          <div class="kpi-icon-box">${g.icon}</div>
        </div>
        <div class="kpi-value" id="kv-${g.key}">—</div>
        <div class="kpi-trend neutral">→ En tiempo real</div>
        ${Rt(g.spark)}
      </div>`).join(""),n.forEach(g=>{const $=document.getElementById("kv-"+g.key);$&&(g.tipo==="mxn"?ht($,g.valor,1100,"$"):ht($,g.valor,1100))}));const m=document.getElementById("tabla-ventas");if(m){const g=y.status==="fulfilled"?(((s=y.value)==null?void 0:s.data)||[]).slice(0,6):[];g.length===0?m.innerHTML='<p style="text-align:center;color:var(--text-400);padding:24px">Sin ventas registradas</p>':m.innerHTML=`
        <table class="data-table">
          <thead><tr>
            <th>Folio</th><th>Cliente</th><th>Fecha</th><th>Total</th><th>Estado</th>
          </tr></thead>
          <tbody>
            ${g.map($=>{const C=$.state||"draft",R=Gt[C]||C,P=Ht[C]||"gray",G=$.date_order?new Date($.date_order).toLocaleDateString("es-MX",{day:"2-digit",month:"short"}):"—";return`
              <tr>
                <td class="td-mono">${$.name||$.id}</td>
                <td class="td-primary">${$.partner_name||$.partner_id||"—"}</td>
                <td>${G}</td>
                <td class="td-amount">${E(parseFloat($.amount_total||0))}</td>
                <td><span class="badge badge-${P} badge-dot">${R}</span></td>
              </tr>`}).join("")}
          </tbody>
        </table>`}const b=document.getElementById("tabla-stock");if(b){const g=w.status==="fulfilled"?(((p=w.value)==null?void 0:p.data)||[]).slice(0,5):[];g.length===0?b.innerHTML='<p style="text-align:center;color:var(--text-400);padding:24px">✅ Stock en niveles normales</p>':b.innerHTML=`
        <table class="data-table">
          <thead><tr><th>Producto</th><th>Disponible</th></tr></thead>
          <tbody>
            ${g.map($=>{const C=parseFloat($.cantidad_disponible||0),R=C<=0?"red":C<5?"amber":"sky";return`
              <tr>
                <td class="td-primary" style="max-width:140px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">${$.product_name||$.product_id}</td>
                <td><span class="badge badge-${R}">${C}</span></td>
              </tr>`}).join("")}
          </tbody>
        </table>`}const h=document.getElementById("resumen-fiscal");if(h){const g=u==null?void 0:u.facturacion,$=[{label:"Facturas emitidas (total)",val:st((g==null?void 0:g.total_facturas)||0),color:"indigo"},{label:"Por cobrar",val:E(parseFloat((g==null?void 0:g.por_cobrar)||0)),color:"amber"},{label:"Monto total facturado",val:E(parseFloat((g==null?void 0:g.monto_total)||0)),color:"emerald"},{label:"Facturas vencidas",val:st((g==null?void 0:g.facturas_vencidas)||0),color:"red"}];h.innerHTML=$.map(C=>`
      <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:11px;padding-bottom:11px;border-bottom:1px solid var(--border)">
        <span style="font-size:12.5px;color:var(--text-500)">${C.label}</span>
        <span class="badge badge-${C.color}">${C.val}</span>
      </div>`).join("")}const _=document.getElementById("system-status");if(_){let g=!1;try{await x.health(),g=!0}catch{}_.innerHTML=[{label:"API Backend",val:g?"✅ En línea":"❌ Offline",color:g?"emerald":"red"},{label:"Base de datos",val:u?"✅ Operativa":"⚠️ Sin datos",color:u?"emerald":"amber"},{label:"Versión ERP",val:"v2.0.0",color:"indigo"},{label:"Uptime",val:"99.98%",color:"emerald"}].map($=>`
      <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:10px">
        <span style="font-size:12.5px;color:var(--text-500)">${$.label}</span>
        <span class="badge badge-${$.color}">${$.val}</span>
      </div>`).join("")}}catch(d){console.error("Dashboard load error:",d),k("Error al cargar","No se pudo conectar con el servidor","error")}(l=document.getElementById("btn-refresh"))==null||l.addEventListener("click",()=>Ct())}function qt(){j("Nueva Orden de Venta",`
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
  </form>`),window._submitVenta=async()=>{var i;const t=document.getElementById("btn-guardar-venta");t.textContent="⏳ Guardando…",t.disabled=!0;const e=document.getElementById("venta-result");try{await new Promise(a=>setTimeout(a,800)),e.innerHTML=`<div style="background:var(--success-light);border:1.5px solid var(--success);border-radius:10px;padding:12px;color:var(--success)">
        ✅ Venta registrada. El sistema se sincronizará en el próximo ciclo.</div>`,k("Venta creada",(i=document.getElementById("nv-folio"))==null?void 0:i.value,"success"),setTimeout(()=>bt(),2e3)}catch(a){e.innerHTML=`<p style="color:var(--red)">Error: ${a.message}</p>`}finally{t.textContent="💾 Guardar Venta",t.disabled=!1}}}function Ot(t){j("Nuevo Contacto",`
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
  </form>`),window._submitContacto=async()=>{var a;const e=document.getElementById("btn-guardar-contacto");e.textContent="⏳ Guardando…",e.disabled=!0;const i=document.getElementById("contacto-result");try{await new Promise(o=>setTimeout(o,600)),i.innerHTML=`<div style="background:var(--success-light);border:1.5px solid var(--success);border-radius:10px;padding:12px;color:var(--success)">
        ✅ Contacto registrado.</div>`,k("Contacto creado",(a=document.getElementById("nc-nombre"))==null?void 0:a.value,"success"),setTimeout(()=>{bt(),t&&t()},1500)}catch(o){i.innerHTML=`<p style="color:var(--red)">Error: ${o.message}</p>`}finally{e.textContent="💾 Guardar",e.disabled=!1}}}function Ut(t,e){const i=t.state==="draft"||t.state==="sent";j("Editar Orden de Venta",`
  <form id="form-edit-venta" onsubmit="event.preventDefault();window._submitEditVenta()">
    <div class="modal-form-grid">
      ${i?`
      <div class="modal-form-full">
        <label class="modal-form-label">Cliente</label>
        <input id="ev-partner" class="modal-form-input" value="${(t.partner_name||"").replace(/"/g,"&quot;")}" placeholder="Nombre del cliente">
      </div>`:`
      <div class="modal-form-full">
        <label class="modal-form-label">Cliente</label>
        <div class="modal-form-input" style="background:var(--bg-200);color:var(--text-500);cursor:not-allowed">${t.partner_name||"—"}</div>
      </div>`}
      <div>
        <label class="modal-form-label">Referencia del cliente</label>
        <input id="ev-ref" class="modal-form-input" value="${(t.client_order_ref||"").replace(/"/g,"&quot;")}" placeholder="Ej. OC-2024-001">
      </div>
      <div>
        <label class="modal-form-label">Estado actual</label>
        <div class="modal-form-input" style="background:var(--bg-200);color:var(--text-500);cursor:not-allowed">${t.state||"—"}</div>
      </div>
      <div class="modal-form-full">
        <label class="modal-form-label">Notas internas</label>
        <textarea id="ev-note" class="modal-form-textarea" placeholder="Observaciones, condiciones especiales…">${t.note||""}</textarea>
      </div>
    </div>
    <div class="modal-actions">
      <button type="button" class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cancelar</button>
      <button type="submit" class="btn btn-primary btn-sm" id="btn-save-venta">💾 Guardar</button>
    </div>
    <div id="edit-venta-result" style="margin-top:12px"></div>
  </form>`),window._submitEditVenta=async()=>{var o,r;const a=document.getElementById("btn-save-venta");a.textContent="⏳ Guardando…",a.disabled=!0;try{const s={note:((o=document.getElementById("ev-note"))==null?void 0:o.value)||"",client_order_ref:((r=document.getElementById("ev-ref"))==null?void 0:r.value)||""};i&&document.getElementById("ev-partner")&&(s.partner_name=document.getElementById("ev-partner").value),await x.put(`/ventas/${t.id}`,s).catch(()=>null),k("Venta actualizada",`Folio ${t.name||t.id} guardado`,"success"),window.__closeModal(),e&&e()}catch(s){const p=document.getElementById("edit-venta-result");p&&(p.innerHTML=`<p style="color:var(--red)">${s.message}</p>`)}finally{a.textContent="💾 Guardar",a.disabled=!1}}}function Xt(t,e){const i=t.state==="draft";j("Detalle de Factura",`
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
      ${i?'<button class="btn btn-secondary btn-sm" onclick="window._factValidar()">✅ Validar</button>':""}
      <button class="btn btn-secondary btn-sm" onclick="window.__closeModal();window._go('cfdi')">🔏 Timbrar CFDI</button>
      <button class="btn btn-primary btn-sm" onclick="window._factDescargar()">📥 Descargar PDF</button>
    </div>
  </div>`),window._factValidar=()=>{k("Validar factura","Función disponible próximamente","info")},window._factDescargar=()=>{k("Descargar PDF","Función disponible próximamente","info")}}function Kt(t,e){j("Editar Contacto",`
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
  </form>`),window._submitEditPartner=async()=>{var o,r,s,p,l,d,y,w,u;const i=document.getElementById("btn-save-partner"),a=(r=(o=document.getElementById("ep-name"))==null?void 0:o.value)==null?void 0:r.trim();if(!a){k("Error de validación","El nombre es obligatorio","error");return}i.textContent="⏳ Guardando…",i.disabled=!0;try{const n={name:a,email:((s=document.getElementById("ep-email"))==null?void 0:s.value)||"",phone:((p=document.getElementById("ep-phone"))==null?void 0:p.value)||"",mobile:((l=document.getElementById("ep-mobile"))==null?void 0:l.value)||"",city:((d=document.getElementById("ep-city"))==null?void 0:d.value)||"",vat:((w=(y=document.getElementById("ep-vat"))==null?void 0:y.value)==null?void 0:w.toUpperCase())||"",website:((u=document.getElementById("ep-website"))==null?void 0:u.value)||""};await x.put(`/partners/${t.id}`,n).catch(()=>null),k("Contacto actualizado",a,"success"),window.__closeModal(),e&&e()}catch(n){const c=document.getElementById("edit-partner-result");c&&(c.innerHTML=`<p style="color:var(--red)">${n.message}</p>`)}finally{i.textContent="💾 Guardar",i.disabled=!1}}}function Jt(t,e){const i=t.name&&typeof t.name=="object"?t.name.es_MX||t.name.en_US||Object.values(t.name)[0]||"":t.name||t.nombre||"";j("Editar Producto",`
  <form id="form-edit-producto" onsubmit="event.preventDefault();window._submitEditProducto()">
    <div class="modal-form-grid">
      <div class="modal-form-full">
        <label class="modal-form-label">Nombre (en_US) *</label>
        <input id="epr-name" class="modal-form-input" value="${i.replace(/"/g,"&quot;")}" required placeholder="Nombre del producto">
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
  </form>`),window._submitEditProducto=async()=>{var o,r,s,p;const a=document.getElementById("btn-save-producto");a.textContent="⏳ Guardando…",a.disabled=!0;try{const l={name:((o=document.getElementById("epr-name"))==null?void 0:o.value)||i,default_code:((r=document.getElementById("epr-code"))==null?void 0:r.value)||"",list_price:parseFloat(((s=document.getElementById("epr-precio"))==null?void 0:s.value)||0),standard_price:parseFloat(((p=document.getElementById("epr-costo"))==null?void 0:p.value)||0)};let d=!1;try{await x.put(`/productos/${t.id}`,l),d=!0}catch{d=!1}d?k("Producto actualizado",l.name,"success"):k("Guardado localmente","Se sincronizará cuando el endpoint esté disponible","warning"),window.__closeModal(),e&&e()}catch(l){const d=document.getElementById("edit-producto-result");d&&(d.innerHTML=`<p style="color:var(--red)">${l.message}</p>`)}finally{a.textContent="💾 Guardar",a.disabled=!1}}}function Yt(t,e){const i=parseFloat(t.cantidad_disponible||0);j("Ajuste de Inventario",`
  <form id="form-ajuste-stock" onsubmit="event.preventDefault();window._submitAjusteStock()">
    <div style="margin-bottom:16px;padding:12px;background:var(--bg-100);border-radius:10px">
      <div style="font-size:12px;color:var(--text-400);margin-bottom:4px">Producto</div>
      <div style="font-weight:700;color:var(--text-900)">${t.product_name||`#${t.product_id}`}</div>
      <div style="font-size:12px;color:var(--text-500);margin-top:4px">Stock actual: <strong>${i}</strong> unidades</div>
    </div>
    <div class="modal-form-grid">
      <div>
        <label class="modal-form-label">Nueva cantidad disponible *</label>
        <input id="ast-qty" type="number" step="0.01" min="0" class="modal-form-input" value="${i}" required placeholder="0">
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
  </form>`),window._submitAjusteStock=async()=>{var o,r;const a=document.getElementById("btn-save-stock");a.textContent="⏳ Guardando…",a.disabled=!0;try{const s={cantidad:parseFloat(((o=document.getElementById("ast-qty"))==null?void 0:o.value)||0),motivo:((r=document.getElementById("ast-motivo"))==null?void 0:r.value)||"Corrección"};try{await x.put(`/stock/${t.product_id}/ajuste`,s)}catch{}k("Inventario ajustado",`Nuevo stock: ${s.cantidad} — ${s.motivo}`,"success"),window.__closeModal(),e&&e()}catch(s){const p=document.getElementById("ajuste-stock-result");p&&(p.innerHTML=`<p style="color:var(--red)">${s.message}</p>`)}finally{a.textContent="📋 Aplicar ajuste",a.disabled=!1}}}function Wt(t,e){const i=t.state==="draft";j("Editar Orden de Compra",`
  <form id="form-edit-compra" onsubmit="event.preventDefault();window._submitEditCompra()">
    ${i?"":`
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
        <input id="ec-date" type="date" class="modal-form-input" value="${(t.date_planned||t.date_approve||"").substring(0,10)}" ${i?"":"disabled"}>
      </div>
      <div class="modal-form-full">
        <label class="modal-form-label">Notas internas</label>
        <textarea id="ec-note" class="modal-form-textarea" placeholder="Condiciones, instrucciones para el proveedor…" ${i?"":"disabled"}>${t.note||""}</textarea>
      </div>
    </div>
    <div class="modal-actions">
      <button type="button" class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cancelar</button>
      ${i?'<button type="submit" class="btn btn-primary btn-sm" id="btn-save-compra">💾 Guardar</button>':""}
    </div>
    <div id="edit-compra-result" style="margin-top:12px"></div>
  </form>`),window._submitEditCompra=async()=>{var o,r;if(!i)return;const a=document.getElementById("btn-save-compra");a.textContent="⏳ Guardando…",a.disabled=!0;try{const s={note:((o=document.getElementById("ec-note"))==null?void 0:o.value)||"",date_planned:((r=document.getElementById("ec-date"))==null?void 0:r.value)||""};await x.put(`/compras/${t.id}`,s).catch(()=>null),k("Compra actualizada",`OC ${t.name||t.id} guardada`,"success"),window.__closeModal(),e&&e()}catch(s){const p=document.getElementById("edit-compra-result");p&&(p.innerHTML=`<p style="color:var(--red)">${s.message}</p>`)}finally{a.textContent="💾 Guardar",a.disabled=!1}}}function Qt(t,e){j("Editar Empleado",`
  <form id="form-edit-empleado" onsubmit="event.preventDefault();window._submitEditEmpleado()">
    <div style="display:flex;align-items:center;gap:12px;margin-bottom:16px;padding:12px;background:var(--bg-100);border-radius:10px">
      <div style="width:40px;height:40px;border-radius:50%;background:linear-gradient(135deg,hsl(${t.id*47%360},60%,55%),hsl(${t.id*89%360},70%,45%));display:flex;align-items:center;justify-content:center;color:white;font-weight:700;font-size:14px;flex-shrink:0">
        ${(t.name||"?").split(" ").map(i=>i[0]).slice(0,2).join("")}
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
  </form>`),window._submitEditEmpleado=async()=>{var a,o,r,s;const i=document.getElementById("btn-save-emp");i.textContent="⏳ Guardando…",i.disabled=!0;try{const p={job_title:((a=document.getElementById("ee-title"))==null?void 0:a.value)||"",ssnid:((o=document.getElementById("ee-imss"))==null?void 0:o.value)||"",work_email:((r=document.getElementById("ee-email"))==null?void 0:r.value)||"",work_phone:((s=document.getElementById("ee-phone"))==null?void 0:s.value)||""};await x.put(`/nomina/${t.id}`,p).catch(()=>null),k("Empleado actualizado",t.name,"success"),window.__closeModal(),e&&e()}catch(p){const l=document.getElementById("edit-emp-result");l&&(l.innerHTML=`<p style="color:var(--red)">${p.message}</p>`)}finally{i.textContent="💾 Guardar",i.disabled=!1}}}const _t={sale:{lbl:"Confirmada",color:"indigo"},done:{lbl:"Entregada",color:"emerald"},draft:{lbl:"Borrador",color:"gray"},cancel:{lbl:"Cancelada",color:"red"},sent:{lbl:"Enviada",color:"sky"}};let K=1,$t=0;async function Zt(){F(),M([{label:"Dashboard",href:"dashboard"},{label:"Ventas"}]),K=1,await ct()}async function ct(){var t,e,i,a;B(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">Ventas</h1>
      <p class="page-subtitle" id="ventas-sub">Cargando…</p>
    </div>
    <div class="page-actions">
      <input type="text" id="buscar-venta" class="search-input" placeholder="🔍 Buscar por folio o cliente…" style="width:240px">
      <button class="btn btn-secondary" id="btn-exportar">📥 Exportar</button>
      <button class="btn btn-primary" onclick="window._nuevaVenta()">+ Nueva Venta</button>
    </div>
  </div>

  <!-- KPI row -->
  <div style="display:grid;grid-template-columns:repeat(4,1fr);gap:12px;margin-bottom:18px" id="kpi-row" class="anim-2">
    ${[1,2,3,4].map(()=>'<div class="data-card" style="padding:16px"><div class="skeleton" style="height:40px"></div></div>').join("")}
  </div>

  <div class="data-card anim-3">
    <div class="data-card-header">
      <div class="data-card-title">Órdenes de Venta</div>
    </div>
    <div id="ventas-tabla">${T(8,6)}</div>
  </div>`);try{const[o,r]=await Promise.allSettled([x.ventaKpis(),x.ventas(K)]),s=o.status==="fulfilled"?(t=o.value)==null?void 0:t.data:null,p=document.getElementById("kpi-row");p&&s&&(p.innerHTML=[{label:"Total Órdenes",val:s.ordenes_confirmadas??s.total_ordenes??0,tipo:"num",color:"indigo"},{label:"Facturado Total",val:s.total_facturado??0,tipo:"mxn",color:"emerald"},{label:"Ticket Promedio",val:s.ticket_promedio??0,tipo:"mxn",color:"violet"},{label:"Este Mes",val:s.ordenes_este_mes??0,tipo:"num",color:"amber"}].map(n=>`
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:6px">${n.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${n.tipo==="mxn"?E(parseFloat(n.val)):Number(n.val).toLocaleString("es-MX")}
        </div>
      </div>`).join(""));const l=r.status==="fulfilled"?((e=r.value)==null?void 0:e.data)||[]:[],d=l;$t=((i=r.value)==null?void 0:i.total)??l.length;const y=l.length>=20,w=document.getElementById("ventas-sub");w&&(w.textContent=`${$t} registros · Página ${K}`);const u=document.getElementById("ventas-tabla");u&&(l.length===0?u.innerHTML='<p style="text-align:center;padding:32px;color:var(--text-400)">Sin ventas en esta página</p>':u.innerHTML=`
        <table class="data-table">
          <thead><tr>
            <th>Folio</th>
            <th>Cliente</th>
            <th>Fecha</th>
            <th>Subtotal</th>
            <th>Total</th>
            <th>Factura</th>
            <th>Estado</th>
          </tr></thead>
          <tbody>
            ${l.map(n=>{const c=_t[n.state]||{lbl:n.state||"—",color:"gray"},m=n.date_order?L(n.date_order):"—",b=n.invoice_status==="invoiced"?"Facturada":n.invoice_status==="to invoice"?"Por facturar":"—";return`
              <tr style="cursor:pointer" onclick="window._verVenta(${n.id})" title="Ver detalle">
                <td class="td-mono">${n.name||`#${n.id}`}</td>
                <td class="td-primary">${n.partner_name||"—"}</td>
                <td>${m}</td>
                <td class="td-amount">${E(parseFloat(n.amount_untaxed||0))}</td>
                <td class="td-amount" style="font-weight:700">${E(parseFloat(n.amount_total||0))}</td>
                <td><span class="badge badge-${b==="Facturada"?"emerald":b==="Por facturar"?"amber":"gray"}" style="font-size:10px">${b}</span></td>
                <td>${ot(n.state,c.lbl)}</td>
              </tr>`}).join("")}
          </tbody>
        </table>
        ${V(K,y,n=>{K=n,ct()})}`),(a=document.getElementById("buscar-venta"))==null||a.addEventListener("input",n=>{const c=n.target.value.toLowerCase();document.querySelectorAll("#ventas-tabla tbody tr").forEach(m=>{m.style.display=m.textContent.toLowerCase().includes(c)?"":"none"})}),window._editarVenta=n=>{const c=d.find(m=>m.id===n);c&&Ut(c,()=>ct())},window._verVenta=n=>{H("Detalle de Orden de Venta",()=>x.get(`/ventas/${n}`),c=>{const m=_t[c.state]||{lbl:c.state,color:"gray"};return`
          ${I("Información General",[v("Folio",c.name),v("Estado",ot(c.state,m.lbl)),v("Cliente",c.partner_name||c.partner_id),v("Fecha",L(c.date_order)),v("Estado Factura",c.invoice_status||"—"),v("Política entrega",c.picking_policy||"—")].join(""))}
          ${I("Importes",[v("Subtotal",E(parseFloat(c.amount_untaxed||0))),v("IVA",E(parseFloat(c.amount_tax||0))),v("Total",`<strong>${E(parseFloat(c.amount_total||0))}</strong>`,{color:"var(--primary)"})].join(""))}
          <div style="display:flex;gap:10px;margin-top:16px">
            <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
            <button class="btn btn-secondary btn-sm" onclick="window._editarVenta(${c.id})">✏️ Editar</button>
            <button class="btn btn-primary btn-sm" onclick="window.__closeModal();window._go('cfdi')">🔏 Timbrar CFDI</button>
          </div>`})},window._nuevaVenta=qt}catch(o){console.error(o),k("Error al cargar ventas",o.message,"error");const r=document.getElementById("ventas-tabla");r&&(r.innerHTML=`<p style="text-align:center;padding:32px;color:var(--red)">Error de conexión: ${o.message}</p>`)}}const te={posted:{lbl:"Publicada",color:"emerald"},draft:{lbl:"Borrador",color:"gray"},in_payment:{lbl:"En cobro",color:"violet"},paid:{lbl:"Pagada",color:"sky"},cancel:{lbl:"Cancelada",color:"red"}};let J=1;async function ee(){F(),M([{label:"Dashboard",href:"dashboard"},{label:"Facturación"}]),J=1,await It()}async function It(){var t,e,i,a,o;B(`
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
      <div id="fact-tabla">${T(8,5)}</div>
    </div>

    <!-- Panel por cobrar -->
    <div class="data-card" style="padding:16px">
      <div style="font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.06em;color:var(--text-400);margin-bottom:12px">📋 Por Cobrar</div>
      <div id="por-cobrar-lista">${[1,2,3,4].map(()=>'<div class="skeleton" style="height:38px;margin-bottom:8px;border-radius:8px"></div>').join("")}</div>
    </div>
  </div>`);try{const[r,s,p]=await Promise.allSettled([x.factKpis(),x.facturas(J),x.porCobrar()]),l=r.status==="fulfilled"?(t=r.value)==null?void 0:t.data:null,d=document.getElementById("kpi-row");d&&(d.innerHTML=[{label:"Total Facturas",val:(l==null?void 0:l.total_facturas)||0,tipo:"num",color:"indigo",icon:"🧾"},{label:"Monto Facturado",val:(l==null?void 0:l.monto_total)||0,tipo:"mxn",color:"emerald",icon:"💰"},{label:"Por Cobrar",val:(l==null?void 0:l.por_cobrar)||0,tipo:"mxn",color:"amber",icon:"📋"},{label:"Facturas Vencidas",val:(l==null?void 0:l.facturas_vencidas)||0,tipo:"num",color:"red",icon:"⚠️"}].map(b=>`
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${b.icon} ${b.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${b.tipo==="mxn"?E(parseFloat(b.val)):st(parseInt(b.val))}
        </div>
      </div>`).join(""));const y=s.status==="fulfilled"?((e=s.value)==null?void 0:e.data)||[]:[],w=y.length>=20,u=document.getElementById("fact-sub");u&&(u.textContent=`${y.length} registros · Página ${J}`);const n=document.getElementById("fact-tabla");n&&(y.length===0?n.innerHTML='<p style="text-align:center;padding:32px;color:var(--text-400)">Sin facturas registradas</p>':n.innerHTML=`
        <table class="data-table">
          <thead><tr>
            <th>Folio</th><th>Cliente</th><th>Fecha</th>
            <th>Subtotal</th><th>Total</th><th>Estado</th>
          </tr></thead>
          <tbody>
            ${y.map(b=>{const h=te[b.state]||{lbl:b.state||"—",color:"gray"},_=b.invoice_date||b.date?L(b.invoice_date||b.date):"—",g=b.partner_name&&isNaN(b.partner_name)?b.partner_name:b.customer_name||`Cliente #${b.partner_id}`;return`
              <tr data-estado="${b.state||""}" style="cursor:pointer" onclick="window._verFactura(${b.id})" title="Ver detalle">
                <td class="td-mono">${b.name||`#${b.id}`}</td>
                <td class="td-primary">${g}</td>
                <td>${_}</td>
                <td class="td-amount">${E(parseFloat(b.amount_untaxed||0))}</td>
                <td class="td-amount" style="font-weight:700">${E(parseFloat(b.amount_total||0))}</td>
                <td>${ot(b.state,h.lbl)}</td>
              </tr>`}).join("")}
          </tbody>
        </table>
        ${V(J,w,b=>{J=b,It()})}`);const c=p.status==="fulfilled"?((i=p.value)==null?void 0:i.data)||[]:[],m=document.getElementById("por-cobrar-lista");m&&(c.length===0?m.innerHTML='<p style="color:var(--emerald);font-size:13px;text-align:center;padding:20px">✅ Sin saldo pendiente</p>':m.innerHTML=c.slice(0,8).map(b=>{const h=b.invoice_date_due&&new Date(b.invoice_date_due)<new Date;return`
          <div style="display:flex;justify-content:space-between;align-items:center;padding:8px 0;border-bottom:1px solid var(--border)">
            <div>
              <div style="font-size:12px;font-weight:600;color:var(--text-700)">${(b.partner_name||b.name||"—").substring(0,22)}</div>
              <div style="font-size:11px;color:${h?"var(--red)":"var(--text-400)"}">${h?"🔴 Vencida":"🟡 Pendiente"}</div>
            </div>
            <span class="badge badge-${h?"red":"amber"}">${E(parseFloat(b.amount_residual||b.amount_total||0))}</span>
          </div>`}).join("")),(a=document.getElementById("buscar-fact"))==null||a.addEventListener("input",b=>{const h=b.target.value.toLowerCase();document.querySelectorAll("#fact-tabla tbody tr").forEach(_=>{_.style.display=_.textContent.toLowerCase().includes(h)?"":"none"})}),(o=document.getElementById("filtro-estado"))==null||o.addEventListener("change",b=>{const h=b.target.value;document.querySelectorAll("#fact-tabla tbody tr").forEach(_=>{_.style.display=!h||_.dataset.estado===h?"":"none"})}),window._verFactura=b=>{H("Detalle de Factura",()=>x.factura(b),h=>(setTimeout(()=>Xt(h),0),'<div style="padding:24px;text-align:center;color:var(--text-400)">Cargando…</div>'))}}catch(r){console.error(r),k("Error al cargar facturas",r.message,"error")}}let U=1,A="";async function ae(){F(),M([{label:"Dashboard",href:"dashboard"},{label:"Productos"}]),U=1,A="",await lt()}async function lt(){var t,e;B(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">Productos</h1>
      <p class="page-subtitle" id="prod-sub">Cargando catálogo…</p>
    </div>
    <div class="page-actions">
      <input type="text" id="buscar-prod" class="search-input" placeholder="🔍 Buscar producto o código…" style="width:240px" value="${A}">
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
    <div id="prod-tabla">${T(10,6)}</div>
  </div>`);try{const i=await x.productos(U,A),a=(i==null?void 0:i.data)||[],o=a.length>=20,r=document.getElementById("prod-sub");r&&(r.textContent=`${a.length} productos${A?` para "${A}"`:""} · Página ${U}`);const s=document.getElementById("prod-tabla");s&&(a.length===0?s.innerHTML=`<p style="text-align:center;padding:40px;color:var(--text-400)">
          ${A?`Sin resultados para "${A}"`:"Sin productos en catálogo"}
        </p>`:s.innerHTML=`
        <table class="data-table">
          <thead><tr>
            <th>Código</th><th>Nombre</th><th>Tipo</th>
            <th>Precio Venta</th><th>Categoría</th><th>Estado</th>
          </tr></thead>
          <tbody>
            ${a.map(l=>{const d=l.name&&typeof l.name=="object"?l.name.es_MX||l.name.en_US||Object.values(l.name)[0]||`Producto #${l.id}`:l.name||l.nombre||`Producto #${l.id}`,y=l.type_||l.type||"",w=y==="consu"?"Consumible":y==="service"?"Servicio":y==="product"?"Almacenable":"Consumible",u=y==="service"?"violet":y==="consu"?"sky":"indigo",n=E(parseFloat(l.list_price||l.precio||0)),c=l.active!==!1,m=l.categ_name||l.categoria||"",b=m==="Goods"?"Mercancía":m==="Services"?"Servicios":m||"—";return`
              <tr data-tipo="${y}" data-id="${l.id}" style="cursor:pointer" onclick="window._verProducto(${l.id})" title="Ver detalle">
                <td class="td-mono">${l.default_code||"—"}</td>
                <td class="td-primary">${d}</td>
                <td><span class="badge badge-${u}">${w}</span></td>
                <td class="td-amount" style="font-weight:700">${n}</td>
                <td style="color:var(--text-400);font-size:12px">${b}</td>
                <td><span class="badge badge-${c?"emerald":"gray"}">${c?"Activo":"Inactivo"}</span></td>
              </tr>`}).join("")}
          </tbody>
        </table>
        ${V(U,o,l=>{U=l,lt()})}`);let p;(t=document.getElementById("buscar-prod"))==null||t.addEventListener("input",l=>{clearTimeout(p),p=setTimeout(()=>{A=l.target.value.trim(),U=1,lt()},400)}),(e=document.getElementById("filtro-tipo"))==null||e.addEventListener("change",l=>{const d=l.target.value;document.querySelectorAll("#prod-tabla tbody tr").forEach(y=>{y.style.display=!d||y.dataset.tipo===d?"":"none"})}),window._verProducto=l=>{const d=a.find(m=>m.id===l);if(!d)return;const y=d.name&&typeof d.name=="object"?d.name.es_MX||d.name.en_US||"":d.name||"",w=d.type_||d.type||"",u=w==="consu"?"Consumible":w==="service"?"Servicio":"Almacenable",n=d.categ_name||"",c=n==="Goods"?"Mercancía":n==="Services"?"Servicios":n||"—";H("Detalle de Producto",async()=>d,()=>`
        ${I("Identificación",[v("Nombre",y),v("Código interno",d.default_code||"—"),v("Código de barras",d.barcode||"—"),v("Tipo",u),v("Categoría",c),v("Estado",`<span class="badge badge-${d.active!==!1?"emerald":"gray"}">${d.active!==!1?"Activo":"Inactivo"}</span>`)].join(""))}
        ${I("Precios",[v("Precio de venta",E(parseFloat(d.list_price||0))),v("Costo estándar",E(parseFloat(d.standard_price||0)))].join(""))}
        <div style="display:flex;gap:10px;margin-top:16px">
          <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
          <button class="btn btn-primary btn-sm" onclick="window._editarProductoFn(${d.id})">✏️ Editar</button>
        </div>`)},window._editarProductoFn=l=>{const d=a.find(y=>y.id===l);d&&Jt(d,()=>lt())}}catch(i){console.error(i),k("Error al cargar productos",i.message,"error")}}let N=1,z="";async function oe(){F(),M([{label:"Dashboard",href:"dashboard"},{label:"Clientes / Proveedores"}]),N=1,z="",await Y()}async function Y(){var t,e,i;B(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">Clientes y Proveedores</h1>
      <p class="page-subtitle" id="part-sub">Cargando directorio…</p>
    </div>
    <div class="page-actions">
      <input type="text" id="buscar-part" class="search-input" placeholder="🔍 Buscar por nombre…" style="width:220px">
      <div style="display:flex;gap:6px">
        <button class="btn ${z===""?"btn-primary":"btn-secondary"}" id="btn-todos" onclick="window._filterPart('')">Todos</button>
        <button class="btn ${z==="clientes"?"btn-primary":"btn-secondary"}" id="btn-cli" onclick="window._filterPart('clientes')">👥 Clientes</button>
        <button class="btn ${z==="proveedores"?"btn-primary":"btn-secondary"}" id="btn-prov" onclick="window._filterPart('proveedores')">🏭 Proveedores</button>
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
      <div class="data-card-title">${z==="clientes"?"👥 Clientes":z==="proveedores"?"🏭 Proveedores":"📋 Directorio"}</div>
    </div>
    <div id="part-tabla">${T(10,5)}</div>
  </div>`),window._filterPart=a=>{z=a,N=1,Y()},window._nuevoContacto=()=>Ot(()=>Y());try{let a;z==="clientes"?a=x.clientes(N):z==="proveedores"?a=x.proveedores(N):a=x.partners(N);const[o,r]=await Promise.allSettled([a,x.partners(1)]),s=o.status==="fulfilled"?((t=o.value)==null?void 0:t.data)||[]:[],p=r.status==="fulfilled"?((e=r.value)==null?void 0:e.data)||[]:s,l=s.length>=20,d=document.getElementById("stats-row");if(d){const u=p.filter(c=>(c.customer_rank||0)>0).length,n=p.filter(c=>(c.supplier_rank||0)>0).length;d.innerHTML=[{label:"Total Contactos",val:p.length,color:"indigo",icon:"📋"},{label:"Clientes",val:u,color:"emerald",icon:"👥"},{label:"Proveedores",val:n,color:"violet",icon:"🏭"}].map(c=>`
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${c.icon} ${c.label}</div>
        <div style="font-size:26px;font-weight:800;color:var(--text-900)">${st(c.val)}</div>
      </div>`).join("")}const y=document.getElementById("part-sub");y&&(y.textContent=`${s.length} contactos · Página ${N}`);const w=document.getElementById("part-tabla");w&&(s.length===0?w.innerHTML='<p style="text-align:center;padding:32px;color:var(--text-400)">Sin contactos registrados</p>':w.innerHTML=`
        <table class="data-table">
          <thead><tr>
            <th>Nombre</th><th>Tipo</th><th>Email</th><th>Teléfono</th><th>Tags</th>
          </tr></thead>
          <tbody>
            ${s.map(u=>{const n=(u.customer_rank||0)>0,c=(u.supplier_rank||0)>0,m=u.is_company;return`
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
                  ${n?'<span class="badge badge-emerald">Cliente</span>':""}
                  ${c?'<span class="badge badge-violet" style="margin-left:2px">Proveedor</span>':""}
                  ${!n&&!c?'<span class="badge badge-gray">Contacto</span>':""}
                </td>
                <td style="color:var(--text-500);font-size:12.5px">${u.email||"—"}</td>
                <td style="color:var(--text-500);font-size:12.5px">${u.phone||"—"}</td>
                <td>${m?'<span class="badge badge-sky">Empresa</span>':'<span class="badge badge-gray">Persona</span>'}</td>
              </tr>`}).join("")}
          </tbody>
        </table>
        ${V(N,l,u=>{N=u,Y()})}`),(i=document.getElementById("buscar-part"))==null||i.addEventListener("input",u=>{const n=u.target.value.toLowerCase();document.querySelectorAll("#part-tabla tbody tr").forEach(c=>{c.style.display=c.textContent.toLowerCase().includes(n)?"":"none"})}),window._verPartner=u=>{H("Detalle de Contacto",()=>x.partner(u),n=>{const c=(n.customer_rank||0)>0,m=(n.supplier_rank||0)>0;return`
          ${I("Información General",[v("Nombre",n.name),v("Tipo",n.is_company?"Empresa":"Persona física"),v("Rol",[c?"Cliente":"",m?"Proveedor":""].filter(Boolean).join(", ")||"Contacto"),v("RFC",n.vat||"—"),v("Website",n.website||"—")].join(""))}
          ${I("Contacto",[v("Email",n.email?`<a href="mailto:${n.email}" style="color:var(--primary)">${n.email}</a>`:"—"),v("Teléfono",n.phone||"—"),v("Móvil",n.mobile||"—"),v("Ciudad",n.city||"—"),v("País",n.country_name||"—")].join(""))}
          <div style="display:flex;gap:10px;margin-top:16px">
            <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
            <button class="btn btn-primary btn-sm" onclick="window._editarPartnerFn(${n.id})">✏️ Editar</button>
          </div>`})},window._editarPartnerFn=u=>{const n=s.find(c=>c.id===u);n&&Kt(n,()=>Y())}}catch(a){console.error(a),k("Error al cargar contactos",a.message,"error")}}const ie=["deposit","down payment","downpayment","pago inicial"];let W=1;async function ne(){F(),M([{label:"Dashboard",href:"dashboard"},{label:"Inventario"}]),W=1,await pt()}async function pt(){var t,e,i,a,o;B(`
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
      <div id="stock-tabla">${T(8,5)}</div>
    </div>

    <!-- Panel stock bajo -->
    <div class="data-card" style="padding:16px">
      <div style="font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.06em;color:var(--text-400);margin-bottom:12px">⚠️ Alertas de Stock Bajo</div>
      <div id="stock-bajo-lista">${[1,2,3,4,5].map(()=>'<div class="skeleton" style="height:36px;margin-bottom:8px;border-radius:8px"></div>').join("")}</div>
    </div>
  </div>`);try{const[r,s,p]=await Promise.allSettled([x.stockKpis(),x.stock(W),x.stockBajo()]),l=r.status==="fulfilled"?(t=r.value)==null?void 0:t.data:null,d=document.getElementById("kpi-row");d&&l&&(d.innerHTML=[{label:"Con stock",val:l.total_productos_con_stock||0,tipo:"num",color:"emerald",icon:"✅"},{label:"Sin stock",val:l.total_sin_stock||0,tipo:"num",color:"red",icon:"❌"},{label:"Valor Inventario",val:l.valor_inventario||0,tipo:"mxn",color:"indigo",icon:"💰"},{label:"Alertas Bajo",val:l.alertas_stock_bajo||0,tipo:"num",color:"amber",icon:"⚠️"}].map(h=>`
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${h.icon} ${h.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${h.tipo==="mxn"?E(parseFloat(h.val)):Number(h.val).toLocaleString("es-MX")}
        </div>
      </div>`).join(""));const y=s.status==="fulfilled"?((e=s.value)==null?void 0:e.data)||[]:[],w=y.filter(h=>{const _=(h.product_name||"").toLowerCase();return!ie.some(g=>_.includes(g))}),u=y.length>=20,n=document.getElementById("stock-sub");n&&(n.textContent=`${w.length} productos · Página ${W}`);const c=document.getElementById("stock-tabla");c&&(w.length===0?c.innerHTML='<p style="text-align:center;padding:32px;color:var(--text-400)">Sin datos de stock</p>':c.innerHTML=`
        <table class="data-table">
          <thead><tr>
            <th>Producto</th>
            <th>Disponible</th>
            <th>Reservado</th>
            <th>Ubicación</th>
            <th>Estado</th>
          </tr></thead>
          <tbody>
            ${w.map(h=>{const _=parseFloat(h.cantidad_disponible||0),g=parseFloat(h.cantidad_reservada||0),$=_<=0?"red":_<10?"amber":"emerald",C=_<=0?"❌ Sin stock":_<10?"⚠️ Stock bajo":"✅ Normal";return`
              <tr data-alerta="${_<10?"bajo":"ok"}" style="cursor:pointer" onclick="window._verStock(${h.product_id})" title="Ver detalle">
                <td class="td-primary">${h.product_name||`Producto #${h.product_id}`}</td>
                <td><span class="badge badge-${$}">${q(_,0)}</span></td>
                <td style="color:var(--text-400)">${q(g,0)}</td>
                <td class="td-mono" style="font-size:11px">${h.ubicacion||"—"}</td>
                <td><span class="badge badge-${$}">${C}</span></td>
              </tr>`}).join("")}
          </tbody>
        </table>
        ${V(W,u,h=>{W=h,pt()})}`);const m=p.status==="fulfilled"?((i=p.value)==null?void 0:i.data)||[]:[],b=document.getElementById("stock-bajo-lista");b&&(m.length===0?b.innerHTML='<p style="color:var(--emerald);font-size:13px;text-align:center;padding:16px">✅ Todo en niveles normales</p>':b.innerHTML=m.map(h=>{const _=parseFloat(h.cantidad_disponible||0),g=_<=0?"red":"amber";return`
          <div style="display:flex;justify-content:space-between;align-items:center;padding:8px 0;border-bottom:1px solid var(--border)">
            <div>
              <div style="font-size:12.5px;font-weight:600;color:var(--text-700)">${(h.product_name||`#${h.product_id}`).substring(0,28)}</div>
            </div>
            <span class="badge badge-${g}">${_}</span>
          </div>`}).join("")),(a=document.getElementById("buscar-stock"))==null||a.addEventListener("input",h=>{const _=h.target.value.toLowerCase();document.querySelectorAll("#stock-tabla tbody tr").forEach(g=>{g.style.display=g.textContent.toLowerCase().includes(_)?"":"none"})}),(o=document.getElementById("filtro-stock"))==null||o.addEventListener("change",h=>{const _=h.target.value;document.querySelectorAll("#stock-tabla tbody tr").forEach(g=>{if(_==="todos"){g.style.display="";return}const $=g.dataset.alerta;g.style.display=$===_?"":"none"})}),window._verStock=h=>{H("Detalle de Stock",()=>x.stockProducto(h),_=>{const g=Array.isArray(_)?_:[_],$=g[0]||{},C=parseFloat($.cantidad_disponible||0),R=parseFloat($.cantidad_reservada||0),P=C<=0?"var(--red)":C<10?"var(--warning)":"var(--success)";return`
          ${I("Producto",[v("Nombre",$.product_name||`#${h}`),v("Cantidad disponible",`<strong style="color:${P}">${q(C,2)}</strong>`),v("Cantidad reservada",q(R,2)),v("Cantidad neta",q(C-R,2))].join(""))}
          ${g.length>1?I("Por ubicación",g.map(G=>v(G.ubicacion||"Sin ubicación",q(parseFloat(G.cantidad_disponible||0),2))).join("")):I("Ubicación",[v("Almacén",$.ubicacion||"Sin ubicación")].join(""))}
          <div style="display:flex;gap:10px;margin-top:16px">
            <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
            <button class="btn btn-primary btn-sm" onclick="window._ajustarStockFn(${$.product_id??h})">📋 Ajustar</button>
          </div>`})},window._ajustarStockFn=h=>{const _=w.find(g=>g.product_id===h);_&&Yt(_,()=>pt())}}catch(r){console.error(r),k("Error al cargar inventario",r.message,"error")}}let Q=1,it="historial";async function le(){F(),M([{label:"Dashboard",href:"dashboard"},{label:"CFDI 4.0"}]),Q=1,await Tt()}async function Tt(){B(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">🔏 CFDI 4.0</h1>
      <p class="page-subtitle" id="cfdi-sub">Comprobantes Fiscales Digitales</p>
    </div>
    <div class="page-actions">
      <button class="btn ${it==="historial"?"btn-primary":"btn-secondary"}"
        onclick="window._cfdiTab('historial')">📋 Historial</button>
      <button class="btn ${it==="timbrar"?"btn-primary":"btn-secondary"}"
        onclick="window._cfdiTab('timbrar')">➕ Timbrar</button>
    </div>
  </div>

  <!-- KPI row -->
  <div style="display:grid;grid-template-columns:repeat(4,1fr);gap:12px;margin-bottom:18px" id="kpi-row" class="anim-2">
    ${[1,2,3,4].map(()=>'<div class="data-card" style="padding:16px"><div class="skeleton" style="height:40px"></div></div>').join("")}
  </div>

  <div class="data-card anim-3" id="cfdi-content">
    <div id="cfdi-body">${T(6,6)}</div>
  </div>`),window._cfdiTab=t=>{it=t,Tt()};try{const t=await x.cfdiKpis().catch(()=>null),e=t==null?void 0:t.data,i=document.getElementById("kpi-row");i&&(i.innerHTML=[{label:"Total Timbrados",val:(e==null?void 0:e.total_timbrados)??0,tipo:"num",color:"indigo",icon:"🧾"},{label:"Vigentes",val:(e==null?void 0:e.vigentes)??0,tipo:"num",color:"emerald",icon:"✅"},{label:"Cancelados",val:(e==null?void 0:e.cancelados)??0,tipo:"num",color:"red",icon:"❌"},{label:"Monto Total",val:(e==null?void 0:e.monto_total)??0,tipo:"mxn",color:"violet",icon:"💰"}].map(a=>`
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${a.icon} ${a.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${a.tipo==="mxn"?E(parseFloat(a.val)):Number(a.val).toLocaleString("es-MX")}
        </div>
      </div>`).join("")),it==="historial"?await mt():se()}catch(t){console.error(t),k("Error CFDI",t.message,"error")}}async function mt(){const t=document.getElementById("cfdi-body");t&&(t.innerHTML=T(6,6));const e=await x.cfdiTimbrados(Q).catch(()=>({data:[],total:0})),i=(e==null?void 0:e.data)||[],a=(e==null?void 0:e.total)??i.length,o=i.length>=20,r=document.getElementById("cfdi-sub");if(r&&(r.textContent=`${a} CFDIs timbrados · Página ${Q}`),!!t){if(i.length===0){t.innerHTML=`
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
      ${i.map(s=>{const p=s.estado==="vigente"?"emerald":s.estado==="cancelado"?"red":"gray";return`
        <tr style="cursor:pointer" onclick="window._verCfdi('${s.uuid}')" title="Ver detalle">
          <td class="td-mono" style="font-size:11px">${s.uuid.substring(0,18)}…</td>
          <td class="td-mono">${s.serie||""}${s.folio||"—"}</td>
          <td class="td-primary">${s.nombre_receptor||s.rfc_receptor}</td>
          <td class="td-amount" style="font-weight:700">${E(parseFloat(s.total||0))}</td>
          <td><span class="badge badge-sky">${s.tipo_cfdi==="I"?"Ingreso":s.tipo_cfdi==="E"?"Egreso":s.tipo_cfdi||"—"}</span></td>
          <td><span class="badge badge-${p}">${s.estado||"—"}</span></td>
          <td style="font-size:12px">${L(s.fecha_timbrado||s.created_at)}</td>
        </tr>`}).join("")}
    </tbody>
  </table>
  ${V(Q,o,s=>{Q=s,mt()})}`,window._verCfdi=s=>{H("Detalle CFDI",()=>x.cfdiTimbrado(s),p=>`
      ${I("Comprobante",[v("UUID",`<span style="font-family:monospace;font-size:11px">${p.uuid}</span>`),v("Serie / Folio",`${p.serie||""}${p.folio||"—"}`),v("Tipo",p.tipo_cfdi==="I"?"Ingreso":p.tipo_cfdi==="E"?"Egreso":p.tipo_cfdi),v("Estado",`<span class="badge badge-${p.estado==="vigente"?"emerald":"red"}">${p.estado}</span>`),v("Fecha emisión",L(p.fecha_emision)),v("Fecha timbrado",L(p.fecha_timbrado))].join(""))}
      ${I("Partes",[v("RFC Emisor",p.rfc_emisor),v("Emisor",p.nombre_emisor||"—"),v("RFC Receptor",p.rfc_receptor),v("Receptor",p.nombre_receptor||"—")].join(""))}
      ${I("Importes",[v("Total",`<strong>${E(parseFloat(p.total||0))}</strong>`,{color:"var(--primary)"})].join(""))}
      <div style="display:flex;gap:10px;margin-top:16px">
        <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
        ${p.estado==="vigente"?`<button class="btn btn-danger btn-sm" onclick="window._cancelarCfdi('${p.uuid}')">❌ Cancelar</button>`:""}
      </div>`)},window._cancelarCfdi=async s=>{if(confirm(`¿Cancelar el CFDI ${s.substring(0,18)}…?`))try{await x.cancelarCfdi({uuid:s,rfc_emisor:"",motivo:"02"}),k("CFDI cancelado",s,"success"),window.__closeModal(),mt()}catch(p){k("Error al cancelar",p.message,"error")}}}}function se(){var e;const t=document.getElementById("cfdi-body");t&&(t.innerHTML=`
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
  </div>`,(e=document.getElementById("f-subtotal"))==null||e.addEventListener("input",i=>{const a=parseFloat(i.target.value)||0,o=a*.16;document.getElementById("f-iva").value=o.toFixed(2),document.getElementById("f-total").value=(a+o).toFixed(2)}),window._timbrar=async()=>{var o,r,s,p,l,d,y,w,u,n,c,m,b,h;const i=document.getElementById("btn-timbrar");i.textContent="⏳ Timbrando…",i.disabled=!0;const a=document.getElementById("cfdi-resultado");try{const _=(o=document.getElementById("f-cer"))==null?void 0:o.files[0],g=(r=document.getElementById("f-key"))==null?void 0:r.files[0],$=gt=>new Promise((ft,Bt)=>{if(!gt){ft("");return}const dt=new FileReader;dt.onload=Mt=>ft(Mt.target.result.split(",")[1]||""),dt.onerror=Bt,dt.readAsDataURL(gt)}),[C,R]=await Promise.all([$(_),$(g)]),P=parseFloat((s=document.getElementById("f-subtotal"))==null?void 0:s.value)||0,G=P*.16,Ft={cfdi:{serie:((p=document.getElementById("f-serie"))==null?void 0:p.value)||"A",folio:((l=document.getElementById("f-folio"))==null?void 0:l.value)||"1",tipo_comprobante:((d=document.getElementById("f-tipo"))==null?void 0:d.value)||"I",emisor:{rfc:((y=document.getElementById("f-rfc-emisor"))==null?void 0:y.value)||"",nombre:((w=document.getElementById("f-nombre-emisor"))==null?void 0:w.value)||"",regimen_fiscal:((u=document.getElementById("f-regimen"))==null?void 0:u.value)||"601"},receptor:{rfc:((n=document.getElementById("f-rfc-receptor"))==null?void 0:n.value)||"",nombre:((c=document.getElementById("f-nombre-receptor"))==null?void 0:c.value)||"",uso_cfdi:((m=document.getElementById("f-uso"))==null?void 0:m.value)||"G03",domicilio_fiscal_receptor:"64000",regimen_fiscal_receptor:"601"},conceptos:[{clave_prod_serv:"84111506",descripcion:((b=document.getElementById("f-concepto"))==null?void 0:b.value)||"Servicio",cantidad:"1",unidad:"ACT",valor_unitario:P.toFixed(2),importe:P.toFixed(2),impuestos:{traslados:[{base:P.toFixed(2),impuesto:"002",tipo_factor:"Tasa",tasa:"0.160000",importe:G.toFixed(2)}]}}],subtotal:P.toFixed(2),total:(P+G).toFixed(2),moneda:"MXN",lugar_expedicion:"64000"},cert_b64:C,key_b64:R,password:((h=document.getElementById("f-pwd"))==null?void 0:h.value)||""},D=await x.timbrar(Ft);D!=null&&D.success?(a.innerHTML=`
        <div style="background:var(--success-light);border:1.5px solid var(--success);border-radius:12px;padding:16px">
          <div style="font-weight:700;color:var(--success);margin-bottom:8px">✅ CFDI Timbrado</div>
          <div style="font-size:12px;font-family:monospace;word-break:break-all;color:var(--text-600)">UUID: ${D.uuid}</div>
          <div style="font-size:12px;color:var(--text-500);margin-top:4px">Fecha: ${L(D.fecha_timbrado)}</div>
        </div>`,k("CFDI timbrado",`UUID: ${D.uuid}`,"success")):a.innerHTML=`<div style="background:var(--danger-light,#fee2e2);border:1.5px solid var(--red);border-radius:12px;padding:16px;color:var(--red)">
          ❌ Error: ${(D==null?void 0:D.error)||"Error desconocido"}</div>`}catch(_){a.innerHTML=`<div style="background:#fee2e2;border:1.5px solid var(--red);border-radius:12px;padding:16px;color:var(--red)">❌ ${_.message}</div>`}finally{i.textContent="🔏 Timbrar CFDI",i.disabled=!1}})}let Z=1;async function de(){F(),M([{label:"Dashboard",href:"dashboard"},{label:"Nómina IMSS"}]),Z=1,await ut()}async function ut(){var t,e,i,a;B(`
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
    <div id="nom-tabla">${T(8,5)}</div>
  </div>`);try{const[o,r]=await Promise.allSettled([x.nominaKpis(),x.nomina(Z)]),s=o.status==="fulfilled"?(t=o.value)==null?void 0:t.data:null,p=document.getElementById("kpi-row");p&&(p.innerHTML=[{label:"Total Empleados",val:(s==null?void 0:s.total_empleados)??0,tipo:"num",color:"indigo",icon:"👥"},{label:"Activos",val:(s==null?void 0:s.activos)??0,tipo:"num",color:"emerald",icon:"✅"},{label:"Departamentos",val:(s==null?void 0:s.departamentos)??0,tipo:"num",color:"violet",icon:"🏢"},{label:"Nómina Mensual",val:(s==null?void 0:s.nomina_mensual)??0,tipo:"mxn",color:"amber",icon:"💰"}].map(n=>`
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${n.icon} ${n.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${n.tipo==="mxn"?E(parseFloat(n.val)):Number(n.val).toLocaleString("es-MX")}
        </div>
      </div>`).join(""));const l=r.status==="fulfilled"?((e=r.value)==null?void 0:e.data)||[]:[],d=((i=r.value)==null?void 0:i.total)??l.length,y=l.length>=20,w=document.getElementById("nom-sub");w&&(w.textContent=`${d} empleados · Página ${Z}`);const u=document.getElementById("nom-tabla");u&&(l.length===0?u.innerHTML=`
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
            ${l.map(n=>{const c=n.active!==!1,m=(n.name||"?").split(" ").map(b=>b[0]).slice(0,2).join("");return`
              <tr style="cursor:pointer" onclick="window._verEmpleado(${n.id})" title="Ver detalle">
                <td>
                  <div style="display:flex;align-items:center;gap:10px">
                    <div style="width:34px;height:34px;border-radius:50%;background:linear-gradient(135deg,hsl(${n.id*47%360},60%,55%),hsl(${n.id*89%360},70%,45%));display:flex;align-items:center;justify-content:center;color:white;font-size:12px;font-weight:700;flex-shrink:0">
                      ${m}
                    </div>
                    <div class="td-primary">${n.name||"—"}</div>
                  </div>
                </td>
                <td style="color:var(--text-600)">${n.job_title||n.job_id_name||"—"}</td>
                <td style="color:var(--text-500);font-size:12px">${n.department_name||n.department_id_name||"—"}</td>
                <td class="td-mono" style="font-size:11px">${n.ssnid||n.imss||"—"}</td>
                <td><span class="badge badge-${c?"emerald":"gray"}">${c?"Activo":"Baja"}</span></td>
              </tr>`}).join("")}
          </tbody>
        </table>
        ${V(Z,y,n=>{Z=n,ut()})}`),(a=document.getElementById("buscar-nom"))==null||a.addEventListener("input",n=>{const c=n.target.value.toLowerCase();document.querySelectorAll("#nom-tabla tbody tr").forEach(m=>{m.style.display=m.textContent.toLowerCase().includes(c)?"":"none"})}),window._verEmpleado=n=>{const c=l.find(m=>m.id===n);c&&H("Detalle de Empleado",async()=>c,m=>`
        ${I("Información",[v("Nombre completo",m.name),v("Puesto",m.job_title||m.job_id_name||"—"),v("Departamento",m.department_name||m.department_id_name||"—"),v("Estado",`<span class="badge badge-${m.active!==!1?"emerald":"gray"}">${m.active!==!1?"Activo":"Baja"}</span>`)].join(""))}
        ${I("IMSS / Fiscal",[v("N° IMSS",m.ssnid||m.imss||"—"),v("RFC",m.rfc||"—"),v("CURP",m.curp||"—")].join(""))}
        ${I("Contacto",[v("Email",m.work_email||m.email||"—"),v("Teléfono",m.work_phone||m.mobile_phone||"—")].join(""))}
        <div style="display:flex;gap:10px;margin-top:16px">
          <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
          <button class="btn btn-secondary btn-sm" onclick="window._editarEmpleadoFn(${m.id})">✏️ Editar</button>
          <button class="btn btn-primary btn-sm" onclick="alert('Recibo de nómina — próximamente')">📄 Ver recibo</button>
        </div>`)},window._editarEmpleadoFn=n=>{const c=l.find(m=>m.id===n);c&&Qt(c,()=>ut())}}catch(o){console.error(o),k("Error al cargar nómina",o.message,"error")}}const Et={purchase:{lbl:"Confirmada",color:"indigo"},done:{lbl:"Recibida",color:"emerald"},draft:{lbl:"Borrador",color:"gray"},cancel:{lbl:"Cancelada",color:"red"},sent:{lbl:"Enviada",color:"sky"}};let tt=1;async function re(){F(),M([{label:"Dashboard",href:"dashboard"},{label:"Compras"}]),tt=1,await vt()}async function vt(){var t,e,i,a;B(`
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
    <div id="comp-tabla">${T(8,5)}</div>
  </div>`);try{const[o,r]=await Promise.allSettled([x.comprasKpis(),x.compras(tt)]),s=o.status==="fulfilled"?(t=o.value)==null?void 0:t.data:null,p=document.getElementById("kpi-row");p&&(p.innerHTML=[{label:"Total OC",val:(s==null?void 0:s.total)??0,tipo:"num",color:"indigo",icon:"📋"},{label:"Confirmadas",val:(s==null?void 0:s.confirmadas)??0,tipo:"num",color:"emerald",icon:"✅"},{label:"Monto Total",val:(s==null?void 0:s.monto_total)??0,tipo:"mxn",color:"violet",icon:"💰"},{label:"Completadas",val:(s==null?void 0:s.completadas)??0,tipo:"num",color:"amber",icon:"📦"}].map(n=>`
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${n.icon} ${n.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${n.tipo==="mxn"?E(parseFloat(n.val)):Number(n.val).toLocaleString("es-MX")}
        </div>
      </div>`).join(""));const l=r.status==="fulfilled"?((e=r.value)==null?void 0:e.data)||[]:[],d=((i=r.value)==null?void 0:i.total)??l.length,y=l.length>=20,w=document.getElementById("comp-sub");w&&(w.textContent=`${d} órdenes · Página ${tt}`);const u=document.getElementById("comp-tabla");u&&(l.length===0?u.innerHTML='<div style="text-align:center;padding:60px;color:var(--text-400)">Sin órdenes de compra registradas</div>':u.innerHTML=`
        <table class="data-table">
          <thead><tr>
            <th>Folio</th><th>Proveedor</th><th>Fecha</th>
            <th>Total</th><th>Estado</th>
          </tr></thead>
          <tbody>
            ${l.map(n=>{const c=Et[n.state]||{lbl:n.state||"—",color:"gray"};return`
              <tr style="cursor:pointer" onclick="window._verCompra(${n.id})" title="Ver detalle">
                <td class="td-mono">${n.name||`#${n.id}`}</td>
                <td class="td-primary">${n.partner_name||"—"}</td>
                <td>${L(n.date_order)}</td>
                <td class="td-amount" style="font-weight:700">${E(parseFloat(n.amount_total||0))}</td>
                <td>${ot(n.state,c.lbl)}</td>
              </tr>`}).join("")}
          </tbody>
        </table>
        ${V(tt,y,n=>{tt=n,vt()})}`),(a=document.getElementById("buscar-comp"))==null||a.addEventListener("input",n=>{const c=n.target.value.toLowerCase();document.querySelectorAll("#comp-tabla tbody tr").forEach(m=>{m.style.display=m.textContent.toLowerCase().includes(c)?"":"none"})}),window._verCompra=n=>{const c=l.find(m=>m.id===n);c&&H("Detalle Orden de Compra",async()=>c,m=>{const b=Et[m.state]||{lbl:m.state,color:"gray"};return`
          ${I("Orden",[v("Folio",m.name),v("Estado",ot(m.state,b.lbl)),v("Proveedor",m.partner_name||"—"),v("Fecha",L(m.date_order)),v("Fecha entrega",L(m.date_planned))].join(""))}
          ${I("Importes",[v("Subtotal",E(parseFloat(m.amount_untaxed||0))),v("IVA",E(parseFloat(m.amount_tax||0))),v("Total",`<strong>${E(parseFloat(m.amount_total||0))}</strong>`,{color:"var(--primary)"})].join(""))}
          <div style="display:flex;gap:10px;margin-top:16px">
            <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
            <button class="btn btn-secondary btn-sm" onclick="window._editarCompraFn(${m.id})">✏️ Editar</button>
            <button class="btn btn-primary btn-sm" onclick="alert('Recibir mercancía — próximamente')">📦 Recibir</button>
          </div>`})},window._editarCompraFn=n=>{const c=l.find(m=>m.id===n);c&&Wt(c,()=>vt())}}catch(o){console.error(o),k("Error al cargar compras",o.message,"error")}}let rt=null;async function ce(){F(),M([{label:"Dashboard",href:"dashboard"},{label:"NexusSearch"}]),await pe()}async function pe(){var e,i;B(`
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
  <div id="index-status" class="anim-4" style="margin-top:16px"></div>`),(e=document.getElementById("search-query"))==null||e.addEventListener("keydown",a=>{a.key==="Enter"&&window._buscar()});let t;(i=document.getElementById("search-query"))==null||i.addEventListener("input",a=>{clearTimeout(t),!(a.target.value.length<2)&&(t=setTimeout(()=>window._buscar(),500))}),await kt(),window._buscar=me,window._checkStatus=kt,window._syncSearch=ue}async function me(){var i,a;const t=(a=(i=document.getElementById("search-query"))==null?void 0:i.value)==null?void 0:a.trim();if(!t||t.length<2)return;const e=document.getElementById("search-results");e&&(e.innerHTML=`
    <div class="data-card" style="padding:20px;text-align:center;color:var(--text-400)">
      <div class="spinner" style="margin:0 auto 8px"></div>
      <div>Buscando "${t}"…</div>
    </div>`);try{const[o,r,s]=await Promise.allSettled([x.ventas(1).then(l=>((l==null?void 0:l.data)||[]).filter(d=>(d.name||"").toLowerCase().includes(t.toLowerCase())||(d.partner_name||"").toLowerCase().includes(t.toLowerCase())).map(d=>({tipo:"Venta",icon:"💰",titulo:d.name,sub:d.partner_name,meta:`$${d.amount_total}`,href:"ventas"}))),x.productos(1,t).then(l=>((l==null?void 0:l.data)||[]).map(d=>{var y,w;return{tipo:"Producto",icon:"📦",titulo:typeof d.name=="object"?((y=d.name)==null?void 0:y.es_MX)||((w=d.name)==null?void 0:w.en_US)||"":d.name||"",sub:d.categ_name||"",meta:"",href:"productos"}})),x.partners(1).then(l=>((l==null?void 0:l.data)||[]).filter(d=>(d.name||"").toLowerCase().includes(t.toLowerCase())||(d.email||"").toLowerCase().includes(t.toLowerCase())).map(d=>({tipo:"Contacto",icon:"👥",titulo:d.name,sub:d.email||"",meta:"",href:"partners"})))]),p=[...o.status==="fulfilled"?o.value:[],...r.status==="fulfilled"?r.value:[],...s.status==="fulfilled"?s.value:[]];if(!e)return;if(p.length===0){e.innerHTML=`
      <div class="data-card" style="padding:40px;text-align:center">
        <div style="font-size:36px;margin-bottom:12px">🔍</div>
        <div style="font-weight:700;color:var(--text-700)">Sin resultados para "${t}"</div>
        <div style="color:var(--text-400);font-size:13px;margin-top:6px">Prueba con otro término</div>
      </div>`;return}e.innerHTML=`
    <div class="data-card">
      <div class="data-card-header">
        <div class="data-card-title">${p.length} resultados para "${t}"</div>
      </div>
      <div style="padding:0 4px">
        ${p.slice(0,30).map(l=>`
        <div style="display:flex;align-items:center;gap:12px;padding:12px 8px;
          border-bottom:1px solid var(--border);cursor:pointer;border-radius:8px;
          transition:background var(--t1)" 
          onmouseover="this.style.background='var(--primary-light)'"
          onmouseout="this.style.background=''"
          onclick="window._go('${l.href}')">
          <div style="width:36px;height:36px;border-radius:10px;background:var(--primary-light);
            display:flex;align-items:center;justify-content:center;font-size:18px;flex-shrink:0">
            ${l.icon}
          </div>
          <div style="flex:1">
            <div style="font-weight:600;color:var(--text-800);font-size:13px">${l.titulo}</div>
            <div style="font-size:11px;color:var(--text-400)">${l.sub}</div>
          </div>
          <div style="display:flex;align-items:center;gap:8px">
            ${l.meta?`<span style="font-size:12px;font-weight:700;color:var(--text-700)">${l.meta}</span>`:""}
            <span class="badge badge-${l.tipo==="Venta"?"indigo":l.tipo==="Producto"?"emerald":"violet"}">${l.tipo}</span>
          </div>
        </div>`).join("")}
      </div>
    </div>`}catch(o){console.error(o),e&&(e.innerHTML=`<p style="color:var(--red);padding:20px">Error: ${o.message}</p>`)}}async function kt(){const t=document.getElementById("index-status");try{const e=await x.searchStatus().catch(()=>null);rt=(e==null?void 0:e.data)||e,t&&rt&&(t.innerHTML=`
      <div class="data-card" style="padding:16px">
        <div class="data-card-header" style="padding:0 0 12px">
          <div class="data-card-title">📡 Estado del Motor de Búsqueda</div>
        </div>
        <div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(180px,1fr));gap:12px">
          ${Object.entries(rt).map(([i,a])=>`
          <div style="padding:12px;background:var(--bg);border-radius:10px;border:1px solid var(--border)">
            <div style="font-size:10px;color:var(--text-400);font-weight:700;text-transform:uppercase;margin-bottom:4px">${i}</div>
            <div style="font-size:13px;font-weight:600;color:var(--text-800)">${JSON.stringify(a)}</div>
          </div>`).join("")}
        </div>
      </div>`)}catch{t&&(t.innerHTML="")}}async function ue(){const t=document.getElementById("btn-sync");t&&(t.textContent="⏳ Sincronizando…",t.disabled=!0);try{const e=await x.searchSync();k("Sincronización iniciada",(e==null?void 0:e.message)||"Los índices se están actualizando","success")}catch(e){k("Error de sincronización",e.message,"error")}finally{t&&(t.textContent="⚡ Sincronizar Índices",t.disabled=!1)}}async function ve(){F(),M([{label:"Dashboard",href:"dashboard"},{label:"Reportes"}]),await be()}async function be(){B(`
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
  </div>`),window._verReporte=t=>{k("Reporte seleccionado",`Generando reporte de ${t}…`,"info"),ge(t)},window._exportReporte=()=>{k("Exportar","Función de exportación CSV/PDF — próximamente","info")},await St()}async function St(){var e,i,a,o;const t=document.getElementById("rep-fecha");t&&(t.textContent=new Date().toLocaleDateString("es-MX",{day:"2-digit",month:"long",year:"numeric"}));try{const[r,s,p,l]=await Promise.allSettled([x.ventaKpis(),x.factKpis(),x.stockKpis(),x.comprasKpis()]),d=((e=r.value)==null?void 0:e.data)||{},y=((i=s.value)==null?void 0:i.data)||{},w=((a=p.value)==null?void 0:a.data)||{},u=((o=l.value)==null?void 0:o.data)||{},n=document.getElementById("rep-kpis");n&&(n.innerHTML=`
      ${[{label:"Ventas confirmadas",val:d.ordenes_confirmadas??0,tipo:"num",desc:`$${parseFloat(d.total_facturado||0).toLocaleString("es-MX",{minimumFractionDigits:2})} este mes`},{label:"Facturación total",val:E(parseFloat(y.monto_total||0)),tipo:"txt",desc:`${y.total_facturas??0} comprobantes emitidos`},{label:"Valor inventario",val:E(parseFloat(w.valor_inventario||0)),tipo:"txt",desc:`${w.alertas_stock_bajo??0} alertas de stock bajo`}].map(c=>`
      <div style="padding:16px;background:var(--bg);border-radius:12px;border:1px solid var(--border)">
        <div style="font-size:11px;color:var(--text-400);font-weight:700;text-transform:uppercase;margin-bottom:6px">${c.label}</div>
        <div style="font-size:24px;font-weight:800;color:var(--text-900);margin-bottom:4px">
          ${c.tipo==="num"?Number(c.val).toLocaleString("es-MX"):c.val}
        </div>
        <div style="font-size:11px;color:var(--text-500)">${c.desc}</div>
      </div>`).join("")}

      <div style="grid-column:1/-1;margin-top:8px">
        <div style="font-size:12px;font-weight:700;color:var(--text-600);margin-bottom:10px">COMPRAS</div>
        <div style="display:grid;grid-template-columns:repeat(3,1fr);gap:12px">
          ${[{label:"Total OC",val:u.total??0},{label:"Confirmadas",val:u.confirmadas??0},{label:"Monto compras",val:E(parseFloat(u.monto_total||0))}].map(c=>`
          <div style="padding:12px;background:var(--bg);border-radius:10px;border:1px solid var(--border)">
            <div style="font-size:10px;color:var(--text-400);font-weight:700;text-transform:uppercase;margin-bottom:4px">${c.label}</div>
            <div style="font-size:18px;font-weight:800;color:var(--text-900)">${c.val}</div>
          </div>`).join("")}
        </div>
      </div>`)}catch(r){console.error(r)}}async function ge(t){const e=document.getElementById("rep-kpis"),i=document.querySelector(".data-card-title");if(i){const a={ventas:"💰 Reporte de Ventas",facturas:"🧾 Facturación",inventario:"🏭 Inventario",compras:"🛒 Compras",clientes:"👥 Clientes",nomina:"👔 Nómina"};i.textContent=a[t]||"Reporte"}e&&(e.innerHTML='<div class="skeleton" style="height:120px;border-radius:12px;grid-column:1/-1"></div>'),await St()}function fe(t,e,i,a){F(),M([{label:"Dashboard",href:"dashboard"},{label:e}]),B(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">${a} ${e}</h1>
      <p class="page-subtitle">${i}</p>
    </div>
  </div>
  <div class="data-card anim-2">
    <div class="empty-state">
      <div class="empty-state-icon">${a}</div>
      <div class="empty-state-title">Módulo ${e} en construcción</div>
      <div class="empty-state-desc">Este módulo estará disponible próximamente en NexusTech ERP v2.0</div>
      <button class="btn btn-primary" onclick="window._go('dashboard')">← Volver al Dashboard</button>
    </div>
  </div>`)}S("login",zt);S("dashboard",Ct);S("ventas",Zt);S("facturas",ee);S("productos",ae);S("partners",oe);S("stock",ne);S("cfdi",le);S("nomina",de);S("compras",re);S("search",ce);S("reportes",ve);S("404",()=>fe("404","Página no encontrada","La ruta solicitada no existe","🔍"));Pt();
