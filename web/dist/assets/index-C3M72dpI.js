(function(){const e=document.createElement("link").relList;if(e&&e.supports&&e.supports("modulepreload"))return;for(const o of document.querySelectorAll('link[rel="modulepreload"]'))a(o);new MutationObserver(o=>{for(const c of o)if(c.type==="childList")for(const r of c.addedNodes)r.tagName==="LINK"&&r.rel==="modulepreload"&&a(r)}).observe(document,{childList:!0,subtree:!0});function s(o){const c={};return o.integrity&&(c.integrity=o.integrity),o.referrerPolicy&&(c.referrerPolicy=o.referrerPolicy),o.crossOrigin==="use-credentials"?c.credentials="include":o.crossOrigin==="anonymous"?c.credentials="omit":c.credentials="same-origin",c}function a(o){if(o.ep)return;o.ep=!0;const c=s(o);fetch(o.href,c)}})();const tt={isLoggedIn:()=>!!localStorage.getItem("nx_token"),getUser:()=>{try{return JSON.parse(localStorage.getItem("nx_user")||"{}")}catch{return{}}},setSession(t,e){localStorage.setItem("nx_token",t),localStorage.setItem("nx_user",JSON.stringify(e))},clear(){localStorage.removeItem("nx_token"),localStorage.removeItem("nx_user")}},ot={};function S(t,e){ot[t]=e}function et(t){window.location.hash=t}function Pt(){window.addEventListener("hashchange",vt),vt()}function vt(){const t=window.location.hash.replace("#","")||"dashboard";if(!tt.isLoggedIn()&&t!=="login"){et("login");return}if(tt.isLoggedIn()&&t==="login"){et("dashboard");return}const e=ot[t];e?e():ot[404]&&ot[404]()}const jt="/api/v1";function Dt(){return localStorage.getItem("nx_token")}class Rt extends Error{constructor(e,s){super(s),this.status=e}}async function _(t,e,s){const a=Dt(),o=await fetch(jt+e,{method:t,headers:{"Content-Type":"application/json",...a?{Authorization:`Bearer ${a}`}:{}},...s!==void 0?{body:JSON.stringify(s)}:{}});if(o.status===401)return localStorage.removeItem("nx_token"),localStorage.removeItem("nx_user"),window.location.hash="login",null;if(!o.ok)throw new Rt(o.status,await o.text());return(o.headers.get("content-type")||"").includes("application/json")?o.json():o.text()}const h={get:t=>_("GET",t),post:(t,e)=>_("POST",t,e),put:(t,e)=>_("PUT",t,e),del:t=>_("DELETE",t),login:(t,e)=>_("POST","/auth/login",{login:t,password:e}),logout:()=>_("POST","/auth/logout",{}),dashboard:()=>_("GET","/dashboard"),ventaKpis:()=>_("GET","/ventas/kpis"),factKpis:()=>_("GET","/facturas/kpis"),stockKpis:()=>_("GET","/stock/kpis"),ventas:(t=1)=>_("GET",`/ventas?pagina=${t}`),venta:t=>_("GET",`/ventas/${t}`),facturas:(t=1)=>_("GET",`/facturas?pagina=${t}`),factura:t=>_("GET",`/facturas/${t}`),porCobrar:()=>_("GET","/facturas/por-cobrar"),productos:(t=1,e="")=>_("GET",`/productos?pagina=${t}&q=${encodeURIComponent(e)}`),producto:t=>_("GET",`/productos/${t}`),partners:(t=1)=>_("GET",`/partners?pagina=${t}`),partner:t=>_("GET",`/partners/${t}`),clientes:(t=1)=>_("GET",`/clientes?pagina=${t}`),proveedores:(t=1)=>_("GET",`/proveedores?pagina=${t}`),stock:(t=1)=>_("GET",`/stock?pagina=${t}`),stockKpis:()=>_("GET","/stock/kpis"),stockBajo:()=>_("GET","/stock/bajo"),stockProducto:t=>_("GET",`/stock/producto/${t}`),cfdiTimbrados:(t=1)=>_("GET",`/cfdi/timbrados?pagina=${t}`),cfdiTimbrado:t=>_("GET",`/cfdi/timbrados/${t}`),cfdiKpis:()=>_("GET","/cfdi/kpis"),timbrar:t=>_("POST","/cfdi/timbrar",t),cancelarCfdi:t=>_("POST","/cfdi/cancelar",t),nomina:(t=1)=>_("GET",`/nomina?pagina=${t}`),nominaKpis:()=>_("GET","/nomina/kpis"),compras:(t=1)=>_("GET",`/compras?pagina=${t}`),comprasKpis:()=>_("GET","/compras/kpis"),searchSync:()=>_("POST","/search/sync",{}),searchStatus:()=>_("GET","/search/status"),health:()=>_("GET","/health")};function zt(){const t=document.getElementById("__shell");t&&t.remove(),document.getElementById("app").innerHTML=`
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
  </div>`;const e=document.getElementById("lbtn"),s=document.getElementById("lu"),a=document.getElementById("lp"),o=document.getElementById("lerr");async function c(){if(e.disabled)return;const r=s.value.trim(),b=a.value;if(!r||!b){o.textContent="Ingresa usuario y contraseña",o.classList.add("show");return}e.disabled=!0,e.textContent="Verificando...",o.classList.remove("show");try{const i=await h.login(r,b),d=(i==null?void 0:i.data)||i,y=(d==null?void 0:d.access_token)||(d==null?void 0:d.token);if(y){tt.setSession(y,{nombre:d.email||r,email:d.email||r,user_id:d.user_id,company_id:d.company_id}),document.getElementById("app").innerHTML="",et("dashboard");return}o.textContent="Error inesperado del servidor. Intenta de nuevo.",o.classList.add("show")}catch(i){o.textContent=(i==null?void 0:i.status)===401?"Credenciales incorrectas. Verifica tu usuario y contraseña.":`Error de conexión: ${(i==null?void 0:i.message)||"No se pudo contactar el servidor"}`,o.classList.add("show")}e.disabled=!1,e.textContent="Acceder al sistema"}e.addEventListener("click",c),a.addEventListener("keydown",r=>r.key==="Enter"&&c()),s.addEventListener("keydown",r=>r.key==="Enter"&&a.focus()),setTimeout(()=>s.focus(),100)}function G(t,e=0){return t==null||t===""?"—":Number(t).toLocaleString("es-MX",{minimumFractionDigits:e,maximumFractionDigits:e})}function E(t){return t==null?"—":(t=parseFloat(t)||0,Math.abs(t)>=1e6?`$${(t/1e6).toFixed(2)}M`:Math.abs(t)>=1e3?`$${(t/1e3).toFixed(1)}k`:`$${G(t,2)}`)}function nt(t){return t==null?"—":Number(t).toLocaleString("es-MX")}function F(t){return t?new Date(t).toLocaleDateString("es-MX",{day:"2-digit",month:"short",year:"numeric"}):"—"}function I(t,e="",s="info"){const a={success:"✅",error:"❌",info:"ℹ️",warning:"⚠️"};let o=document.getElementById("__toasts");o||(o=document.createElement("div"),o.id="__toasts",o.className="toast-container",document.body.appendChild(o));const c=document.createElement("div");c.className=`toast ${s}`,c.innerHTML=`
    <span class="toast-icon">${a[s]||"ℹ️"}</span>
    <div><div class="toast-title">${t}</div>${e?`<div class="toast-msg">${e}</div>`:""}</div>`,o.appendChild(c),requestAnimationFrame(()=>c.classList.add("show")),setTimeout(()=>{c.classList.remove("show"),setTimeout(()=>c.remove(),400)},3800)}function bt(t,e,s=900,a="",o=""){if(!t)return;const c=performance.now(),r=String(e).includes(".");function b(i){const d=Math.min((i-c)/s,1),y=1-Math.pow(1-d,3),x=e*y;t.textContent=a+(r?x.toLocaleString("es-MX",{minimumFractionDigits:2,maximumFractionDigits:2}):Math.round(x).toLocaleString("es-MX"))+o,d<1&&requestAnimationFrame(b)}requestAnimationFrame(b)}function At(t){if(!(t!=null&&t.length))return"";const e=Math.max(...t,1);return`<div class="sparkline">${t.map((s,a)=>`<div class="spark-bar${a===t.length-1?" active":""}" style="height:${Math.max(4,Math.round(s/e*100))}%"></div>`).join("")}</div>`}function Nt(t=5,e=6){return`<tbody>${Array.from({length:e},()=>`<tr>${Array.from({length:t},()=>`<td><div class="skeleton" style="height:14px;width:${60+Math.random()*30}%"></div></td>`).join("")}</tr>`).join("")}</tbody>`}function T(t=5,e=4){return`<table class="data-table"><thead><tr>${Array.from({length:e},()=>`<th><div class="skeleton" style="height:12px;width:${40+Math.random()*40}%"></div></th>`).join("")}</tr></thead>${Nt(e,t)}</table>`}function Ht(t=5){return Array.from({length:t},()=>`
  <div class="kpi-card kpi-gray">
    <div class="kpi-label"><div class="skeleton" style="height:13px;width:60%"></div></div>
    <div class="kpi-value"><div class="skeleton" style="height:28px;width:70%"></div></div>
    <div><div class="skeleton" style="height:11px;width:40%;margin-top:6px"></div></div>
  </div>`).join("")}const Vt={sale:"emerald",done:"indigo",draft:"gray",sent:"sky",cancel:"red",posted:"emerald",in_payment:"violet",paid:"emerald",partial:"amber"};function U(t,e){return`<span class="badge badge-${Vt[t]||"gray"} badge-dot">${e}</span>`}function N(t,e,s){return window.__pagNav=s,`
  <div class="data-table-footer">
    <span style="color:var(--text-400)">Página ${t}</span>
    <div class="pagination">
      <button class="pag-btn" ${t<=1?"disabled":""} onclick="window.__pagNav(${t-1})">&#8592; Anterior</button>
      <span class="pag-btn active">${t}</span>
      <button class="pag-btn" ${e?"":"disabled"} onclick="window.__pagNav(${t+1})">Siguiente &#8594;</button>
    </div>
  </div>`}let O=null;function ct(t,e,s={}){let a=document.getElementById("__modal-overlay");a||(a=document.createElement("div"),a.id="__modal-overlay",a.innerHTML=`
      <div id="__modal-drawer">
        <div id="__modal-header">
          <span id="__modal-title"></span>
          <button id="__modal-close" onclick="window.__closeModal()">✕</button>
        </div>
        <div id="__modal-body"></div>
      </div>`,document.body.appendChild(a),a.addEventListener("click",o=>{o.target===a&&window.__closeModal()})),document.getElementById("__modal-title").textContent=t,document.getElementById("__modal-body").innerHTML=e,a.classList.add("open"),document.body.style.overflow="hidden",O&&document.removeEventListener("keydown",O),O=o=>{o.key==="Escape"&&window.__closeModal()},document.addEventListener("keydown",O),s.onMounted&&setTimeout(s.onMounted,10)}function pt(){const t=document.getElementById("__modal-overlay");t&&t.classList.remove("open"),document.body.style.overflow="",O&&(document.removeEventListener("keydown",O),O=null)}window.__closeModal=pt;async function H(t,e,s){ct(t,`
    <div style="display:flex;flex-direction:column;gap:12px;padding:8px 0">
      ${[1,2,3,4,5].map(()=>'<div class="skeleton" style="height:52px;border-radius:10px"></div>').join("")}
    </div>`);try{const a=await e(),o=(a==null?void 0:a.data)??a;document.getElementById("__modal-body").innerHTML=s(o)}catch(a){document.getElementById("__modal-body").innerHTML=`<p style="color:var(--red);padding:24px">Error: ${a.message}</p>`}}function p(t,e,s={}){const a=e??"—",o=s.color?`color:${s.color}`:"";return`
  <div style="display:flex;justify-content:space-between;align-items:flex-start;
    padding:10px 0;border-bottom:1px solid var(--border)">
    <span style="font-size:12px;color:var(--text-400);font-weight:600;min-width:140px">${t}</span>
    <span style="font-size:13px;font-weight:500;text-align:right;${o}">${a}</span>
  </div>`}function C(t,e){return`
  <div style="margin-bottom:20px">
    <div style="font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.06em;
      color:var(--text-400);margin-bottom:8px;padding-bottom:6px;
      border-bottom:2px solid var(--primary)">${t}</div>
    ${e}
  </div>`}const gt=[{id:"dashboard",icon:"📊",label:"Dashboard",section:"Principal"},{id:"ventas",icon:"💰",label:"Ventas",section:"Principal"},{id:"facturas",icon:"🧾",label:"Facturación",section:"Principal"},{id:"productos",icon:"📦",label:"Productos",section:"Principal"},{id:"partners",icon:"👥",label:"Clientes",section:"Principal"},{id:"stock",icon:"🏭",label:"Inventario",section:"Principal"},{id:"cfdi",icon:"🔏",label:"CFDI 4.0",section:"Fiscal",badge:"NUEVO"},{id:"nomina",icon:"👔",label:"Nómina IMSS",section:"Fiscal"},{id:"compras",icon:"🛒",label:"Compras",section:"Operaciones"},{id:"search",icon:"🔍",label:"NexusSearch",section:"Sistema"},{id:"reportes",icon:"📈",label:"Reportes",section:"Sistema"}];function M(){if(document.getElementById("__shell"))return;const t=tt.getUser(),e=(t.nombre||t.name||"AD").substring(0,2).toUpperCase(),s=[...new Set(gt.map(a=>a.section))];document.getElementById("app").innerHTML=`
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
        ${s.map(a=>`
        <div class="nav-section">
          <div class="nav-section-title">${a}</div>
          ${gt.filter(o=>o.section===a).map(o=>`
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
  </div>`,window._go=a=>{et(a)},window._logout=()=>{tt.clear();const a=document.getElementById("__shell");a&&a.remove(),et("login"),I("Sesión cerrada","Hasta pronto","info")},window.addEventListener("hashchange",ft),ft()}function B(t){const e=document.getElementById("__page");e&&(e.innerHTML=t,e.scrollTop=0)}function L(t){const e=document.getElementById("__breadcrumb");e&&(e.innerHTML=t.map((s,a)=>`
    <span class="breadcrumb-item"${a<t.length-1&&s.href?` onclick="window._go('${s.href}')"`:""}>
      ${s.label}
      ${a<t.length-1?'<span class="breadcrumb-sep">/</span>':""}
    </span>`).join(""))}function ft(){const t=window.location.hash.replace("#","")||"dashboard";document.querySelectorAll(".nav-link").forEach(e=>{e.classList.toggle("active",e.id===`nl-${t}`)})}const Gt={sale:"indigo",done:"emerald",draft:"gray",cancel:"red",sent:"sky",posted:"emerald"},Ot={sale:"Confirmada",done:"Entregada",draft:"Borrador",cancel:"Cancelada",sent:"Enviada"};function X(t,e=10){return Array.from({length:e},()=>Math.max(5,Math.round(t*(.6+Math.random()*.8))))}async function $t(){var t,e,s,a,o,c,r,b,i;M(),L([{label:"Dashboard"}]),B(`
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
  <div class="kpi-grid anim-2" id="kpi-grid">${Ht(5)}</div>

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
  </div>`);try{const[d,y,x]=await Promise.allSettled([h.dashboard(),h.ventas(1),h.stockBajo()]),l=d.status==="fulfilled"?(t=d.value)==null?void 0:t.data:null,n=[{key:"ventas_mes",label:"Ventas del Mes",tipo:"mxn",icon:"💰",color:"indigo",valor:parseFloat(((e=l==null?void 0:l.ventas)==null?void 0:e.importe_mes)||0),trend:null,spark:X(100)},{key:"facturas",label:"Facturas Emitidas",tipo:"num",icon:"🧾",color:"emerald",valor:parseInt(((s=l==null?void 0:l.facturacion)==null?void 0:s.total_facturas)||0),trend:null,spark:X(50)},{key:"cobrar",label:"Por Cobrar",tipo:"mxn",icon:"📋",color:"amber",valor:parseFloat(((a=l==null?void 0:l.facturacion)==null?void 0:a.por_cobrar)||0),trend:null,spark:X(80)},{key:"stock_total",label:"Productos en Stock",tipo:"num",icon:"📦",color:"sky",valor:parseInt(((o=l==null?void 0:l.inventario)==null?void 0:o.total_productos_con_stock)||0),trend:null,spark:X(80)},{key:"stock_bajo",label:"Alertas Stock Bajo",tipo:"num",icon:"⚠️",color:"rose",valor:parseInt(((c=l==null?void 0:l.inventario)==null?void 0:c.alertas_stock_bajo)||0),trend:null,spark:X(20)}],u=document.getElementById("kpi-grid");u&&(u.innerHTML=n.map(f=>`
      <div class="kpi-card kpi-${f.color}">
        <div class="kpi-label">
          <span>${f.label}</span>
          <div class="kpi-icon-box">${f.icon}</div>
        </div>
        <div class="kpi-value" id="kv-${f.key}">—</div>
        <div class="kpi-trend neutral">→ En tiempo real</div>
        ${At(f.spark)}
      </div>`).join(""),n.forEach(f=>{const $=document.getElementById("kv-"+f.key);$&&(f.tipo==="mxn"?bt($,f.valor,1100,"$"):bt($,f.valor,1100))}));const m=document.getElementById("tabla-ventas");if(m){const f=y.status==="fulfilled"?(((r=y.value)==null?void 0:r.data)||[]).slice(0,6):[];f.length===0?m.innerHTML='<p style="text-align:center;color:var(--text-400);padding:24px">Sin ventas registradas</p>':m.innerHTML=`
        <table class="data-table">
          <thead><tr>
            <th>Folio</th><th>Cliente</th><th>Fecha</th><th>Total</th><th>Estado</th>
          </tr></thead>
          <tbody>
            ${f.map($=>{const k=$.state||"draft",R=Ot[k]||k,P=Gt[k]||"gray",V=$.date_order?new Date($.date_order).toLocaleDateString("es-MX",{day:"2-digit",month:"short"}):"—";return`
              <tr>
                <td class="td-mono">${$.name||$.id}</td>
                <td class="td-primary">${$.partner_name||$.partner_id||"—"}</td>
                <td>${V}</td>
                <td class="td-amount">${E(parseFloat($.amount_total||0))}</td>
                <td><span class="badge badge-${P} badge-dot">${R}</span></td>
              </tr>`}).join("")}
          </tbody>
        </table>`}const g=document.getElementById("tabla-stock");if(g){const f=x.status==="fulfilled"?(((b=x.value)==null?void 0:b.data)||[]).slice(0,5):[];f.length===0?g.innerHTML='<p style="text-align:center;color:var(--text-400);padding:24px">✅ Stock en niveles normales</p>':g.innerHTML=`
        <table class="data-table">
          <thead><tr><th>Producto</th><th>Disponible</th></tr></thead>
          <tbody>
            ${f.map($=>{const k=parseFloat($.cantidad_disponible||0),R=k<=0?"red":k<5?"amber":"sky";return`
              <tr>
                <td class="td-primary" style="max-width:140px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">${$.product_name||$.product_id}</td>
                <td><span class="badge badge-${R}">${k}</span></td>
              </tr>`}).join("")}
          </tbody>
        </table>`}const v=document.getElementById("resumen-fiscal");if(v){const f=l==null?void 0:l.facturacion,$=[{label:"Facturas emitidas (total)",val:nt((f==null?void 0:f.total_facturas)||0),color:"indigo"},{label:"Por cobrar",val:E(parseFloat((f==null?void 0:f.por_cobrar)||0)),color:"amber"},{label:"Monto total facturado",val:E(parseFloat((f==null?void 0:f.monto_total)||0)),color:"emerald"},{label:"Facturas vencidas",val:nt((f==null?void 0:f.facturas_vencidas)||0),color:"red"}];v.innerHTML=$.map(k=>`
      <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:11px;padding-bottom:11px;border-bottom:1px solid var(--border)">
        <span style="font-size:12.5px;color:var(--text-500)">${k.label}</span>
        <span class="badge badge-${k.color}">${k.val}</span>
      </div>`).join("")}const w=document.getElementById("system-status");if(w){let f=!1;try{await h.health(),f=!0}catch{}w.innerHTML=[{label:"API Backend",val:f?"✅ En línea":"❌ Offline",color:f?"emerald":"red"},{label:"Base de datos",val:l?"✅ Operativa":"⚠️ Sin datos",color:l?"emerald":"amber"},{label:"Versión ERP",val:"v2.0.0",color:"indigo"},{label:"Uptime",val:"99.98%",color:"emerald"}].map($=>`
      <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:10px">
        <span style="font-size:12.5px;color:var(--text-500)">${$.label}</span>
        <span class="badge badge-${$.color}">${$.val}</span>
      </div>`).join("")}}catch(d){console.error("Dashboard load error:",d),I("Error al cargar","No se pudo conectar con el servidor","error")}(i=document.getElementById("btn-refresh"))==null||i.addEventListener("click",()=>$t())}function qt(){ct("Nueva Orden de Venta",`
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
  </form>`),window._submitVenta=async()=>{var s;const t=document.getElementById("btn-guardar-venta");t.textContent="⏳ Guardando…",t.disabled=!0;const e=document.getElementById("venta-result");try{await new Promise(a=>setTimeout(a,800)),e.innerHTML=`<div style="background:var(--success-light);border:1.5px solid var(--success);border-radius:10px;padding:12px;color:var(--success)">
        ✅ Venta registrada. El sistema se sincronizará en el próximo ciclo.</div>`,I("Venta creada",(s=document.getElementById("nv-folio"))==null?void 0:s.value,"success"),setTimeout(()=>pt(),2e3)}catch(a){e.innerHTML=`<p style="color:var(--red)">Error: ${a.message}</p>`}finally{t.textContent="💾 Guardar Venta",t.disabled=!1}}}function Ut(t){ct("Nuevo Contacto",`
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
  </form>`),window._submitContacto=async()=>{var a;const e=document.getElementById("btn-guardar-contacto");e.textContent="⏳ Guardando…",e.disabled=!0;const s=document.getElementById("contacto-result");try{await new Promise(o=>setTimeout(o,600)),s.innerHTML=`<div style="background:var(--success-light);border:1.5px solid var(--success);border-radius:10px;padding:12px;color:var(--success)">
        ✅ Contacto registrado.</div>`,I("Contacto creado",(a=document.getElementById("nc-nombre"))==null?void 0:a.value,"success"),setTimeout(()=>{pt(),t&&t()},1500)}catch(o){s.innerHTML=`<p style="color:var(--red)">Error: ${o.message}</p>`}finally{e.textContent="💾 Guardar",e.disabled=!1}}}const yt={sale:{lbl:"Confirmada",color:"indigo"},done:{lbl:"Entregada",color:"emerald"},draft:{lbl:"Borrador",color:"gray"},cancel:{lbl:"Cancelada",color:"red"},sent:{lbl:"Enviada",color:"sky"}};let K=1,ht=0;async function Xt(){M(),L([{label:"Dashboard",href:"dashboard"},{label:"Ventas"}]),K=1,await Et()}async function Et(){var t,e,s,a;B(`
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
  </div>`);try{const[o,c]=await Promise.allSettled([h.ventaKpis(),h.ventas(K)]),r=o.status==="fulfilled"?(t=o.value)==null?void 0:t.data:null,b=document.getElementById("kpi-row");b&&r&&(b.innerHTML=[{label:"Total Órdenes",val:r.ordenes_confirmadas??r.total_ordenes??0,tipo:"num",color:"indigo"},{label:"Facturado Total",val:r.total_facturado??0,tipo:"mxn",color:"emerald"},{label:"Ticket Promedio",val:r.ticket_promedio??0,tipo:"mxn",color:"violet"},{label:"Este Mes",val:r.ordenes_este_mes??0,tipo:"num",color:"amber"}].map(l=>`
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:6px">${l.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${l.tipo==="mxn"?E(parseFloat(l.val)):Number(l.val).toLocaleString("es-MX")}
        </div>
      </div>`).join(""));const i=c.status==="fulfilled"?((e=c.value)==null?void 0:e.data)||[]:[];ht=((s=c.value)==null?void 0:s.total)??i.length;const d=i.length>=20,y=document.getElementById("ventas-sub");y&&(y.textContent=`${ht} registros · Página ${K}`);const x=document.getElementById("ventas-tabla");x&&(i.length===0?x.innerHTML='<p style="text-align:center;padding:32px;color:var(--text-400)">Sin ventas en esta página</p>':x.innerHTML=`
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
            ${i.map(l=>{const n=yt[l.state]||{lbl:l.state||"—",color:"gray"},u=l.date_order?F(l.date_order):"—",m=l.invoice_status==="invoiced"?"Facturada":l.invoice_status==="to invoice"?"Por facturar":"—";return`
              <tr style="cursor:pointer" onclick="window._verVenta(${l.id})" title="Ver detalle">
                <td class="td-mono">${l.name||`#${l.id}`}</td>
                <td class="td-primary">${l.partner_name||"—"}</td>
                <td>${u}</td>
                <td class="td-amount">${E(parseFloat(l.amount_untaxed||0))}</td>
                <td class="td-amount" style="font-weight:700">${E(parseFloat(l.amount_total||0))}</td>
                <td><span class="badge badge-${m==="Facturada"?"emerald":m==="Por facturar"?"amber":"gray"}" style="font-size:10px">${m}</span></td>
                <td>${U(l.state,n.lbl)}</td>
              </tr>`}).join("")}
          </tbody>
        </table>
        ${N(K,d,l=>{K=l,Et()})}`),(a=document.getElementById("buscar-venta"))==null||a.addEventListener("input",l=>{const n=l.target.value.toLowerCase();document.querySelectorAll("#ventas-tabla tbody tr").forEach(u=>{u.style.display=u.textContent.toLowerCase().includes(n)?"":"none"})}),window._verVenta=l=>{H("Detalle de Orden de Venta",()=>h.get(`/ventas/${l}`),n=>{const u=yt[n.state]||{lbl:n.state,color:"gray"};return`
          ${C("Información General",[p("Folio",n.name),p("Estado",U(n.state,u.lbl)),p("Cliente",n.partner_name||n.partner_id),p("Fecha",F(n.date_order)),p("Estado Factura",n.invoice_status||"—"),p("Política entrega",n.picking_policy||"—")].join(""))}
          ${C("Importes",[p("Subtotal",E(parseFloat(n.amount_untaxed||0))),p("IVA",E(parseFloat(n.amount_tax||0))),p("Total",`<strong>${E(parseFloat(n.amount_total||0))}</strong>`,{color:"var(--primary)"})].join(""))}
          <div style="display:flex;gap:10px;margin-top:16px">
            <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
            <button class="btn btn-primary btn-sm" onclick="alert('Crear CFDI — próximamente')">🔏 Timbrar CFDI</button>
          </div>`})},window._nuevaVenta=qt}catch(o){console.error(o),I("Error al cargar ventas",o.message,"error");const c=document.getElementById("ventas-tabla");c&&(c.innerHTML=`<p style="text-align:center;padding:32px;color:var(--red)">Error de conexión: ${o.message}</p>`)}}const xt={posted:{lbl:"Publicada",color:"emerald"},draft:{lbl:"Borrador",color:"gray"},in_payment:{lbl:"En cobro",color:"violet"},paid:{lbl:"Pagada",color:"sky"},cancel:{lbl:"Cancelada",color:"red"}};let J=1;async function Kt(){M(),L([{label:"Dashboard",href:"dashboard"},{label:"Facturación"}]),J=1,await kt()}async function kt(){var t,e,s,a,o;B(`
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
  </div>`);try{const[c,r,b]=await Promise.allSettled([h.factKpis(),h.facturas(J),h.porCobrar()]),i=c.status==="fulfilled"?(t=c.value)==null?void 0:t.data:null,d=document.getElementById("kpi-row");d&&(d.innerHTML=[{label:"Total Facturas",val:(i==null?void 0:i.total_facturas)||0,tipo:"num",color:"indigo",icon:"🧾"},{label:"Monto Facturado",val:(i==null?void 0:i.monto_total)||0,tipo:"mxn",color:"emerald",icon:"💰"},{label:"Por Cobrar",val:(i==null?void 0:i.por_cobrar)||0,tipo:"mxn",color:"amber",icon:"📋"},{label:"Facturas Vencidas",val:(i==null?void 0:i.facturas_vencidas)||0,tipo:"num",color:"red",icon:"⚠️"}].map(g=>`
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${g.icon} ${g.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${g.tipo==="mxn"?E(parseFloat(g.val)):nt(parseInt(g.val))}
        </div>
      </div>`).join(""));const y=r.status==="fulfilled"?((e=r.value)==null?void 0:e.data)||[]:[],x=y.length>=20,l=document.getElementById("fact-sub");l&&(l.textContent=`${y.length} registros · Página ${J}`);const n=document.getElementById("fact-tabla");n&&(y.length===0?n.innerHTML='<p style="text-align:center;padding:32px;color:var(--text-400)">Sin facturas registradas</p>':n.innerHTML=`
        <table class="data-table">
          <thead><tr>
            <th>Folio</th><th>Cliente</th><th>Fecha</th>
            <th>Subtotal</th><th>Total</th><th>Estado</th>
          </tr></thead>
          <tbody>
            ${y.map(g=>{const v=xt[g.state]||{lbl:g.state||"—",color:"gray"},w=g.invoice_date||g.date?F(g.invoice_date||g.date):"—",f=g.partner_name&&isNaN(g.partner_name)?g.partner_name:g.customer_name||`Cliente #${g.partner_id}`;return`
              <tr data-estado="${g.state||""}" style="cursor:pointer" onclick="window._verFactura(${g.id})" title="Ver detalle">
                <td class="td-mono">${g.name||`#${g.id}`}</td>
                <td class="td-primary">${f}</td>
                <td>${w}</td>
                <td class="td-amount">${E(parseFloat(g.amount_untaxed||0))}</td>
                <td class="td-amount" style="font-weight:700">${E(parseFloat(g.amount_total||0))}</td>
                <td>${U(g.state,v.lbl)}</td>
              </tr>`}).join("")}
          </tbody>
        </table>
        ${N(J,x,g=>{J=g,kt()})}`);const u=b.status==="fulfilled"?((s=b.value)==null?void 0:s.data)||[]:[],m=document.getElementById("por-cobrar-lista");m&&(u.length===0?m.innerHTML='<p style="color:var(--emerald);font-size:13px;text-align:center;padding:20px">✅ Sin saldo pendiente</p>':m.innerHTML=u.slice(0,8).map(g=>{const v=g.invoice_date_due&&new Date(g.invoice_date_due)<new Date;return`
          <div style="display:flex;justify-content:space-between;align-items:center;padding:8px 0;border-bottom:1px solid var(--border)">
            <div>
              <div style="font-size:12px;font-weight:600;color:var(--text-700)">${(g.partner_name||g.name||"—").substring(0,22)}</div>
              <div style="font-size:11px;color:${v?"var(--red)":"var(--text-400)"}">${v?"🔴 Vencida":"🟡 Pendiente"}</div>
            </div>
            <span class="badge badge-${v?"red":"amber"}">${E(parseFloat(g.amount_residual||g.amount_total||0))}</span>
          </div>`}).join("")),(a=document.getElementById("buscar-fact"))==null||a.addEventListener("input",g=>{const v=g.target.value.toLowerCase();document.querySelectorAll("#fact-tabla tbody tr").forEach(w=>{w.style.display=w.textContent.toLowerCase().includes(v)?"":"none"})}),(o=document.getElementById("filtro-estado"))==null||o.addEventListener("change",g=>{const v=g.target.value;document.querySelectorAll("#fact-tabla tbody tr").forEach(w=>{w.style.display=!v||w.dataset.estado===v?"":"none"})}),window._verFactura=g=>{H("Detalle de Factura",()=>h.factura(g),v=>{const w=xt[v.state]||{lbl:v.state,color:"gray"},f=v.partner_name&&isNaN(v.partner_name)?v.partner_name:`Cliente #${v.partner_id}`;return`
          ${C("Comprobante",[p("Folio",v.name),p("Estado",U(v.state,w.lbl)),p("Cliente",f),p("Fecha emisión",F(v.invoice_date||v.date)),p("Vencimiento",F(v.invoice_date_due)),p("Referencia",v.ref||"—"),p("Diario",v.journal_name||"—")].join(""))}
          ${C("Importes",[p("Subtotal",E(parseFloat(v.amount_untaxed||0))),p("IVA",E(parseFloat(v.amount_tax||0))),p("Total",`<strong>${E(parseFloat(v.amount_total||0))}</strong>`,{color:"var(--primary)"}),p("Saldo pendiente",E(parseFloat(v.amount_residual||0)),{color:v.amount_residual>0?"var(--warning)":"var(--success)"})].join(""))}
          <div style="display:flex;gap:10px;margin-top:16px">
            <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
            <button class="btn btn-primary btn-sm" onclick="window._go('cfdi')">🔏 Timbrar CFDI</button>
          </div>`})}}catch(c){console.error(c),I("Error al cargar facturas",c.message,"error")}}let q=1,z="";async function Jt(){M(),L([{label:"Dashboard",href:"dashboard"},{label:"Productos"}]),q=1,z="",await lt()}async function lt(){var t,e;B(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">Productos</h1>
      <p class="page-subtitle" id="prod-sub">Cargando catálogo…</p>
    </div>
    <div class="page-actions">
      <input type="text" id="buscar-prod" class="search-input" placeholder="🔍 Buscar producto o código…" style="width:240px" value="${z}">
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
  </div>`);try{const s=await h.productos(q,z),a=(s==null?void 0:s.data)||[],o=a.length>=20,c=document.getElementById("prod-sub");c&&(c.textContent=`${a.length} productos${z?` para "${z}"`:""} · Página ${q}`);const r=document.getElementById("prod-tabla");r&&(a.length===0?r.innerHTML=`<p style="text-align:center;padding:40px;color:var(--text-400)">
          ${z?`Sin resultados para "${z}"`:"Sin productos en catálogo"}
        </p>`:r.innerHTML=`
        <table class="data-table">
          <thead><tr>
            <th>Código</th><th>Nombre</th><th>Tipo</th>
            <th>Precio Venta</th><th>Categoría</th><th>Estado</th>
          </tr></thead>
          <tbody>
            ${a.map(i=>{const d=i.name&&typeof i.name=="object"?i.name.es_MX||i.name.en_US||Object.values(i.name)[0]||`Producto #${i.id}`:i.name||i.nombre||`Producto #${i.id}`,y=i.type_||i.type||"",x=y==="consu"?"Consumible":y==="service"?"Servicio":y==="product"?"Almacenable":"Consumible",l=y==="service"?"violet":y==="consu"?"sky":"indigo",n=E(parseFloat(i.list_price||i.precio||0)),u=i.active!==!1,m=i.categ_name||i.categoria||"",g=m==="Goods"?"Mercancía":m==="Services"?"Servicios":m||"—";return`
              <tr data-tipo="${y}" data-id="${i.id}" style="cursor:pointer" onclick="window._verProducto(${i.id})" title="Ver detalle">
                <td class="td-mono">${i.default_code||"—"}</td>
                <td class="td-primary">${d}</td>
                <td><span class="badge badge-${l}">${x}</span></td>
                <td class="td-amount" style="font-weight:700">${n}</td>
                <td style="color:var(--text-400);font-size:12px">${g}</td>
                <td><span class="badge badge-${u?"emerald":"gray"}">${u?"Activo":"Inactivo"}</span></td>
              </tr>`}).join("")}
          </tbody>
        </table>
        ${N(q,o,i=>{q=i,lt()})}`);let b;(t=document.getElementById("buscar-prod"))==null||t.addEventListener("input",i=>{clearTimeout(b),b=setTimeout(()=>{z=i.target.value.trim(),q=1,lt()},400)}),(e=document.getElementById("filtro-tipo"))==null||e.addEventListener("change",i=>{const d=i.target.value;document.querySelectorAll("#prod-tabla tbody tr").forEach(y=>{y.style.display=!d||y.dataset.tipo===d?"":"none"})}),window._verProducto=i=>{const d=a.find(m=>m.id===i);if(!d)return;const y=d.name&&typeof d.name=="object"?d.name.es_MX||d.name.en_US||"":d.name||"",x=d.type_||d.type||"",l=x==="consu"?"Consumible":x==="service"?"Servicio":"Almacenable",n=d.categ_name||"",u=n==="Goods"?"Mercancía":n==="Services"?"Servicios":n||"—";H("Detalle de Producto",async()=>d,()=>`
        ${C("Identificación",[p("Nombre",y),p("Código interno",d.default_code||"—"),p("Código de barras",d.barcode||"—"),p("Tipo",l),p("Categoría",u),p("Estado",`<span class="badge badge-${d.active!==!1?"emerald":"gray"}">${d.active!==!1?"Activo":"Inactivo"}</span>`)].join(""))}
        ${C("Precios",[p("Precio de venta",E(parseFloat(d.list_price||0))),p("Costo estándar",E(parseFloat(d.standard_price||0)))].join(""))}
        <div style="display:flex;gap:10px;margin-top:16px">
          <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
          <button class="btn btn-primary btn-sm" onclick="alert('Editar producto — próximamente')">✏️ Editar</button>
        </div>`)}}catch(s){console.error(s),I("Error al cargar productos",s.message,"error")}}let A=1,D="";async function Yt(){M(),L([{label:"Dashboard",href:"dashboard"},{label:"Clientes / Proveedores"}]),A=1,D="",await it()}async function it(){var t,e,s;B(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">Clientes y Proveedores</h1>
      <p class="page-subtitle" id="part-sub">Cargando directorio…</p>
    </div>
    <div class="page-actions">
      <input type="text" id="buscar-part" class="search-input" placeholder="🔍 Buscar por nombre…" style="width:220px">
      <div style="display:flex;gap:6px">
        <button class="btn ${D===""?"btn-primary":"btn-secondary"}" id="btn-todos" onclick="window._filterPart('')">Todos</button>
        <button class="btn ${D==="clientes"?"btn-primary":"btn-secondary"}" id="btn-cli" onclick="window._filterPart('clientes')">👥 Clientes</button>
        <button class="btn ${D==="proveedores"?"btn-primary":"btn-secondary"}" id="btn-prov" onclick="window._filterPart('proveedores')">🏭 Proveedores</button>
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
      <div class="data-card-title">${D==="clientes"?"👥 Clientes":D==="proveedores"?"🏭 Proveedores":"📋 Directorio"}</div>
    </div>
    <div id="part-tabla">${T(10,5)}</div>
  </div>`),window._filterPart=a=>{D=a,A=1,it()},window._nuevoContacto=()=>Ut(()=>it());try{let a;D==="clientes"?a=h.clientes(A):D==="proveedores"?a=h.proveedores(A):a=h.partners(A);const[o,c]=await Promise.allSettled([a,h.partners(1)]),r=o.status==="fulfilled"?((t=o.value)==null?void 0:t.data)||[]:[],b=c.status==="fulfilled"?((e=c.value)==null?void 0:e.data)||[]:r,i=r.length>=20,d=document.getElementById("stats-row");if(d){const l=b.filter(u=>(u.customer_rank||0)>0).length,n=b.filter(u=>(u.supplier_rank||0)>0).length;d.innerHTML=[{label:"Total Contactos",val:b.length,color:"indigo",icon:"📋"},{label:"Clientes",val:l,color:"emerald",icon:"👥"},{label:"Proveedores",val:n,color:"violet",icon:"🏭"}].map(u=>`
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${u.icon} ${u.label}</div>
        <div style="font-size:26px;font-weight:800;color:var(--text-900)">${nt(u.val)}</div>
      </div>`).join("")}const y=document.getElementById("part-sub");y&&(y.textContent=`${r.length} contactos · Página ${A}`);const x=document.getElementById("part-tabla");x&&(r.length===0?x.innerHTML='<p style="text-align:center;padding:32px;color:var(--text-400)">Sin contactos registrados</p>':x.innerHTML=`
        <table class="data-table">
          <thead><tr>
            <th>Nombre</th><th>Tipo</th><th>Email</th><th>Teléfono</th><th>Tags</th>
          </tr></thead>
          <tbody>
            ${r.map(l=>{const n=(l.customer_rank||0)>0,u=(l.supplier_rank||0)>0,m=l.is_company;return`
              <tr style="cursor:pointer" onclick="window._verPartner(${l.id})" title="Ver detalle">
                <td>
                  <div style="display:flex;align-items:center;gap:8px">
                    <div style="width:32px;height:32px;border-radius:50%;background:linear-gradient(135deg,hsl(${l.id*37%360},60%,55%),hsl(${l.id*71%360},70%,45%));display:flex;align-items:center;justify-content:center;color:white;font-size:13px;font-weight:700;flex-shrink:0">
                      ${(l.name||l.nombre||"?")[0].toUpperCase()}
                    </div>
                    <div>
                      <div class="td-primary">${l.name||l.nombre||"—"}</div>
                      ${m?'<div style="font-size:11px;color:var(--text-400)">Empresa</div>':""}
                    </div>
                  </div>
                </td>
                <td>
                  ${n?'<span class="badge badge-emerald">Cliente</span>':""}
                  ${u?'<span class="badge badge-violet" style="margin-left:2px">Proveedor</span>':""}
                  ${!n&&!u?'<span class="badge badge-gray">Contacto</span>':""}
                </td>
                <td style="color:var(--text-500);font-size:12.5px">${l.email||"—"}</td>
                <td style="color:var(--text-500);font-size:12.5px">${l.phone||"—"}</td>
                <td>${m?'<span class="badge badge-sky">Empresa</span>':'<span class="badge badge-gray">Persona</span>'}</td>
              </tr>`}).join("")}
          </tbody>
        </table>
        ${N(A,i,l=>{A=l,it()})}`),(s=document.getElementById("buscar-part"))==null||s.addEventListener("input",l=>{const n=l.target.value.toLowerCase();document.querySelectorAll("#part-tabla tbody tr").forEach(u=>{u.style.display=u.textContent.toLowerCase().includes(n)?"":"none"})}),window._verPartner=l=>{H("Detalle de Contacto",()=>h.partner(l),n=>{const u=(n.customer_rank||0)>0,m=(n.supplier_rank||0)>0;return`
          ${C("Información General",[p("Nombre",n.name),p("Tipo",n.is_company?"Empresa":"Persona física"),p("Rol",[u?"Cliente":"",m?"Proveedor":""].filter(Boolean).join(", ")||"Contacto"),p("RFC",n.vat||"—"),p("Website",n.website||"—")].join(""))}
          ${C("Contacto",[p("Email",n.email?`<a href="mailto:${n.email}" style="color:var(--primary)">${n.email}</a>`:"—"),p("Teléfono",n.phone||"—"),p("Móvil",n.mobile||"—"),p("Ciudad",n.city||"—"),p("País",n.country_name||"—")].join(""))}
          <div style="display:flex;gap:10px;margin-top:16px">
            <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
            <button class="btn btn-primary btn-sm" onclick="alert('Editar contacto — próximamente')">✏️ Editar</button>
          </div>`})}}catch(a){console.error(a),I("Error al cargar contactos",a.message,"error")}}const Wt=["deposit","down payment","downpayment","pago inicial"];let Y=1;async function Qt(){M(),L([{label:"Dashboard",href:"dashboard"},{label:"Inventario"}]),Y=1,await Ct()}async function Ct(){var t,e,s,a,o;B(`
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
  </div>`);try{const[c,r,b]=await Promise.allSettled([h.stockKpis(),h.stock(Y),h.stockBajo()]),i=c.status==="fulfilled"?(t=c.value)==null?void 0:t.data:null,d=document.getElementById("kpi-row");d&&i&&(d.innerHTML=[{label:"Con stock",val:i.total_productos_con_stock||0,tipo:"num",color:"emerald",icon:"✅"},{label:"Sin stock",val:i.total_sin_stock||0,tipo:"num",color:"red",icon:"❌"},{label:"Valor Inventario",val:i.valor_inventario||0,tipo:"mxn",color:"indigo",icon:"💰"},{label:"Alertas Bajo",val:i.alertas_stock_bajo||0,tipo:"num",color:"amber",icon:"⚠️"}].map(v=>`
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${v.icon} ${v.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${v.tipo==="mxn"?E(parseFloat(v.val)):Number(v.val).toLocaleString("es-MX")}
        </div>
      </div>`).join(""));const y=r.status==="fulfilled"?((e=r.value)==null?void 0:e.data)||[]:[],x=y.filter(v=>{const w=(v.product_name||"").toLowerCase();return!Wt.some(f=>w.includes(f))}),l=y.length>=20,n=document.getElementById("stock-sub");n&&(n.textContent=`${x.length} productos · Página ${Y}`);const u=document.getElementById("stock-tabla");u&&(x.length===0?u.innerHTML='<p style="text-align:center;padding:32px;color:var(--text-400)">Sin datos de stock</p>':u.innerHTML=`
        <table class="data-table">
          <thead><tr>
            <th>Producto</th>
            <th>Disponible</th>
            <th>Reservado</th>
            <th>Ubicación</th>
            <th>Estado</th>
          </tr></thead>
          <tbody>
            ${x.map(v=>{const w=parseFloat(v.cantidad_disponible||0),f=parseFloat(v.cantidad_reservada||0),$=w<=0?"red":w<10?"amber":"emerald",k=w<=0?"❌ Sin stock":w<10?"⚠️ Stock bajo":"✅ Normal";return`
              <tr data-alerta="${w<10?"bajo":"ok"}" style="cursor:pointer" onclick="window._verStock(${v.product_id})" title="Ver detalle">
                <td class="td-primary">${v.product_name||`Producto #${v.product_id}`}</td>
                <td><span class="badge badge-${$}">${G(w,0)}</span></td>
                <td style="color:var(--text-400)">${G(f,0)}</td>
                <td class="td-mono" style="font-size:11px">${v.ubicacion||"—"}</td>
                <td><span class="badge badge-${$}">${k}</span></td>
              </tr>`}).join("")}
          </tbody>
        </table>
        ${N(Y,l,v=>{Y=v,Ct()})}`);const m=b.status==="fulfilled"?((s=b.value)==null?void 0:s.data)||[]:[],g=document.getElementById("stock-bajo-lista");g&&(m.length===0?g.innerHTML='<p style="color:var(--emerald);font-size:13px;text-align:center;padding:16px">✅ Todo en niveles normales</p>':g.innerHTML=m.map(v=>{const w=parseFloat(v.cantidad_disponible||0),f=w<=0?"red":"amber";return`
          <div style="display:flex;justify-content:space-between;align-items:center;padding:8px 0;border-bottom:1px solid var(--border)">
            <div>
              <div style="font-size:12.5px;font-weight:600;color:var(--text-700)">${(v.product_name||`#${v.product_id}`).substring(0,28)}</div>
            </div>
            <span class="badge badge-${f}">${w}</span>
          </div>`}).join("")),(a=document.getElementById("buscar-stock"))==null||a.addEventListener("input",v=>{const w=v.target.value.toLowerCase();document.querySelectorAll("#stock-tabla tbody tr").forEach(f=>{f.style.display=f.textContent.toLowerCase().includes(w)?"":"none"})}),(o=document.getElementById("filtro-stock"))==null||o.addEventListener("change",v=>{const w=v.target.value;document.querySelectorAll("#stock-tabla tbody tr").forEach(f=>{if(w==="todos"){f.style.display="";return}const $=f.dataset.alerta;f.style.display=$===w?"":"none"})}),window._verStock=v=>{H("Detalle de Stock",()=>h.stockProducto(v),w=>{const f=Array.isArray(w)?w:[w],$=f[0]||{},k=parseFloat($.cantidad_disponible||0),R=parseFloat($.cantidad_reservada||0),P=k<=0?"var(--red)":k<10?"var(--warning)":"var(--success)";return`
          ${C("Producto",[p("Nombre",$.product_name||`#${v}`),p("Cantidad disponible",`<strong style="color:${P}">${G(k,2)}</strong>`),p("Cantidad reservada",G(R,2)),p("Cantidad neta",G(k-R,2))].join(""))}
          ${f.length>1?C("Por ubicación",f.map(V=>p(V.ubicacion||"Sin ubicación",G(parseFloat(V.cantidad_disponible||0),2))).join("")):C("Ubicación",[p("Almacén",$.ubicacion||"Sin ubicación")].join(""))}
          <div style="display:flex;gap:10px;margin-top:16px">
            <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
            <button class="btn btn-primary btn-sm" onclick="alert('Ajuste de inventario — próximamente')">📋 Ajustar</button>
          </div>`})}}catch(c){console.error(c),I("Error al cargar inventario",c.message,"error")}}let W=1,at="historial";async function Zt(){M(),L([{label:"Dashboard",href:"dashboard"},{label:"CFDI 4.0"}]),W=1,await It()}async function It(){B(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">🔏 CFDI 4.0</h1>
      <p class="page-subtitle" id="cfdi-sub">Comprobantes Fiscales Digitales</p>
    </div>
    <div class="page-actions">
      <button class="btn ${at==="historial"?"btn-primary":"btn-secondary"}"
        onclick="window._cfdiTab('historial')">📋 Historial</button>
      <button class="btn ${at==="timbrar"?"btn-primary":"btn-secondary"}"
        onclick="window._cfdiTab('timbrar')">➕ Timbrar</button>
    </div>
  </div>

  <!-- KPI row -->
  <div style="display:grid;grid-template-columns:repeat(4,1fr);gap:12px;margin-bottom:18px" id="kpi-row" class="anim-2">
    ${[1,2,3,4].map(()=>'<div class="data-card" style="padding:16px"><div class="skeleton" style="height:40px"></div></div>').join("")}
  </div>

  <div class="data-card anim-3" id="cfdi-content">
    <div id="cfdi-body">${T(6,6)}</div>
  </div>`),window._cfdiTab=t=>{at=t,It()};try{const t=await h.cfdiKpis().catch(()=>null),e=t==null?void 0:t.data,s=document.getElementById("kpi-row");s&&(s.innerHTML=[{label:"Total Timbrados",val:(e==null?void 0:e.total_timbrados)??0,tipo:"num",color:"indigo",icon:"🧾"},{label:"Vigentes",val:(e==null?void 0:e.vigentes)??0,tipo:"num",color:"emerald",icon:"✅"},{label:"Cancelados",val:(e==null?void 0:e.cancelados)??0,tipo:"num",color:"red",icon:"❌"},{label:"Monto Total",val:(e==null?void 0:e.monto_total)??0,tipo:"mxn",color:"violet",icon:"💰"}].map(a=>`
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${a.icon} ${a.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${a.tipo==="mxn"?E(parseFloat(a.val)):Number(a.val).toLocaleString("es-MX")}
        </div>
      </div>`).join("")),at==="historial"?await dt():te()}catch(t){console.error(t),I("Error CFDI",t.message,"error")}}async function dt(){const t=document.getElementById("cfdi-body");t&&(t.innerHTML=T(6,6));const e=await h.cfdiTimbrados(W).catch(()=>({data:[],total:0})),s=(e==null?void 0:e.data)||[],a=(e==null?void 0:e.total)??s.length,o=s.length>=20,c=document.getElementById("cfdi-sub");if(c&&(c.textContent=`${a} CFDIs timbrados · Página ${W}`),!!t){if(s.length===0){t.innerHTML=`
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
      ${s.map(r=>{const b=r.estado==="vigente"?"emerald":r.estado==="cancelado"?"red":"gray";return`
        <tr style="cursor:pointer" onclick="window._verCfdi('${r.uuid}')" title="Ver detalle">
          <td class="td-mono" style="font-size:11px">${r.uuid.substring(0,18)}…</td>
          <td class="td-mono">${r.serie||""}${r.folio||"—"}</td>
          <td class="td-primary">${r.nombre_receptor||r.rfc_receptor}</td>
          <td class="td-amount" style="font-weight:700">${E(parseFloat(r.total||0))}</td>
          <td><span class="badge badge-sky">${r.tipo_cfdi==="I"?"Ingreso":r.tipo_cfdi==="E"?"Egreso":r.tipo_cfdi||"—"}</span></td>
          <td><span class="badge badge-${b}">${r.estado||"—"}</span></td>
          <td style="font-size:12px">${F(r.fecha_timbrado||r.created_at)}</td>
        </tr>`}).join("")}
    </tbody>
  </table>
  ${N(W,o,r=>{W=r,dt()})}`,window._verCfdi=r=>{H("Detalle CFDI",()=>h.cfdiTimbrado(r),b=>`
      ${C("Comprobante",[p("UUID",`<span style="font-family:monospace;font-size:11px">${b.uuid}</span>`),p("Serie / Folio",`${b.serie||""}${b.folio||"—"}`),p("Tipo",b.tipo_cfdi==="I"?"Ingreso":b.tipo_cfdi==="E"?"Egreso":b.tipo_cfdi),p("Estado",`<span class="badge badge-${b.estado==="vigente"?"emerald":"red"}">${b.estado}</span>`),p("Fecha emisión",F(b.fecha_emision)),p("Fecha timbrado",F(b.fecha_timbrado))].join(""))}
      ${C("Partes",[p("RFC Emisor",b.rfc_emisor),p("Emisor",b.nombre_emisor||"—"),p("RFC Receptor",b.rfc_receptor),p("Receptor",b.nombre_receptor||"—")].join(""))}
      ${C("Importes",[p("Total",`<strong>${E(parseFloat(b.total||0))}</strong>`,{color:"var(--primary)"})].join(""))}
      <div style="display:flex;gap:10px;margin-top:16px">
        <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
        ${b.estado==="vigente"?`<button class="btn btn-danger btn-sm" onclick="window._cancelarCfdi('${b.uuid}')">❌ Cancelar</button>`:""}
      </div>`)},window._cancelarCfdi=async r=>{if(confirm(`¿Cancelar el CFDI ${r.substring(0,18)}…?`))try{await h.cancelarCfdi({uuid:r,rfc_emisor:"",motivo:"02"}),I("CFDI cancelado",r,"success"),window.__closeModal(),dt()}catch(b){I("Error al cancelar",b.message,"error")}}}}function te(){var e;const t=document.getElementById("cfdi-body");t&&(t.innerHTML=`
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
  </div>`,(e=document.getElementById("f-subtotal"))==null||e.addEventListener("input",s=>{const a=parseFloat(s.target.value)||0,o=a*.16;document.getElementById("f-iva").value=o.toFixed(2),document.getElementById("f-total").value=(a+o).toFixed(2)}),window._timbrar=async()=>{var o,c,r,b,i,d,y,x,l,n,u,m,g,v;const s=document.getElementById("btn-timbrar");s.textContent="⏳ Timbrando…",s.disabled=!0;const a=document.getElementById("cfdi-resultado");try{const w=(o=document.getElementById("f-cer"))==null?void 0:o.files[0],f=(c=document.getElementById("f-key"))==null?void 0:c.files[0],$=mt=>new Promise((ut,Bt)=>{if(!mt){ut("");return}const st=new FileReader;st.onload=Lt=>ut(Lt.target.result.split(",")[1]||""),st.onerror=Bt,st.readAsDataURL(mt)}),[k,R]=await Promise.all([$(w),$(f)]),P=parseFloat((r=document.getElementById("f-subtotal"))==null?void 0:r.value)||0,V=P*.16,Mt={cfdi:{serie:((b=document.getElementById("f-serie"))==null?void 0:b.value)||"A",folio:((i=document.getElementById("f-folio"))==null?void 0:i.value)||"1",tipo_comprobante:((d=document.getElementById("f-tipo"))==null?void 0:d.value)||"I",emisor:{rfc:((y=document.getElementById("f-rfc-emisor"))==null?void 0:y.value)||"",nombre:((x=document.getElementById("f-nombre-emisor"))==null?void 0:x.value)||"",regimen_fiscal:((l=document.getElementById("f-regimen"))==null?void 0:l.value)||"601"},receptor:{rfc:((n=document.getElementById("f-rfc-receptor"))==null?void 0:n.value)||"",nombre:((u=document.getElementById("f-nombre-receptor"))==null?void 0:u.value)||"",uso_cfdi:((m=document.getElementById("f-uso"))==null?void 0:m.value)||"G03",domicilio_fiscal_receptor:"64000",regimen_fiscal_receptor:"601"},conceptos:[{clave_prod_serv:"84111506",descripcion:((g=document.getElementById("f-concepto"))==null?void 0:g.value)||"Servicio",cantidad:"1",unidad:"ACT",valor_unitario:P.toFixed(2),importe:P.toFixed(2),impuestos:{traslados:[{base:P.toFixed(2),impuesto:"002",tipo_factor:"Tasa",tasa:"0.160000",importe:V.toFixed(2)}]}}],subtotal:P.toFixed(2),total:(P+V).toFixed(2),moneda:"MXN",lugar_expedicion:"64000"},cert_b64:k,key_b64:R,password:((v=document.getElementById("f-pwd"))==null?void 0:v.value)||""},j=await h.timbrar(Mt);j!=null&&j.success?(a.innerHTML=`
        <div style="background:var(--success-light);border:1.5px solid var(--success);border-radius:12px;padding:16px">
          <div style="font-weight:700;color:var(--success);margin-bottom:8px">✅ CFDI Timbrado</div>
          <div style="font-size:12px;font-family:monospace;word-break:break-all;color:var(--text-600)">UUID: ${j.uuid}</div>
          <div style="font-size:12px;color:var(--text-500);margin-top:4px">Fecha: ${F(j.fecha_timbrado)}</div>
        </div>`,I("CFDI timbrado",`UUID: ${j.uuid}`,"success")):a.innerHTML=`<div style="background:var(--danger-light,#fee2e2);border:1.5px solid var(--red);border-radius:12px;padding:16px;color:var(--red)">
          ❌ Error: ${(j==null?void 0:j.error)||"Error desconocido"}</div>`}catch(w){a.innerHTML=`<div style="background:#fee2e2;border:1.5px solid var(--red);border-radius:12px;padding:16px;color:var(--red)">❌ ${w.message}</div>`}finally{s.textContent="🔏 Timbrar CFDI",s.disabled=!1}})}let Q=1;async function ee(){M(),L([{label:"Dashboard",href:"dashboard"},{label:"Nómina IMSS"}]),Q=1,await Tt()}async function Tt(){var t,e,s,a;B(`
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
  </div>`);try{const[o,c]=await Promise.allSettled([h.nominaKpis(),h.nomina(Q)]),r=o.status==="fulfilled"?(t=o.value)==null?void 0:t.data:null,b=document.getElementById("kpi-row");b&&(b.innerHTML=[{label:"Total Empleados",val:(r==null?void 0:r.total_empleados)??0,tipo:"num",color:"indigo",icon:"👥"},{label:"Activos",val:(r==null?void 0:r.activos)??0,tipo:"num",color:"emerald",icon:"✅"},{label:"Departamentos",val:(r==null?void 0:r.departamentos)??0,tipo:"num",color:"violet",icon:"🏢"},{label:"Nómina Mensual",val:(r==null?void 0:r.nomina_mensual)??0,tipo:"mxn",color:"amber",icon:"💰"}].map(n=>`
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${n.icon} ${n.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${n.tipo==="mxn"?E(parseFloat(n.val)):Number(n.val).toLocaleString("es-MX")}
        </div>
      </div>`).join(""));const i=c.status==="fulfilled"?((e=c.value)==null?void 0:e.data)||[]:[],d=((s=c.value)==null?void 0:s.total)??i.length,y=i.length>=20,x=document.getElementById("nom-sub");x&&(x.textContent=`${d} empleados · Página ${Q}`);const l=document.getElementById("nom-tabla");l&&(i.length===0?l.innerHTML=`
        <div style="text-align:center;padding:60px 24px">
          <div style="font-size:48px;margin-bottom:16px">👔</div>
          <div style="font-size:16px;font-weight:700;color:var(--text-700);margin-bottom:8px">Sin empleados registrados</div>
          <div style="font-size:13px;color:var(--text-400)">Agrega empleados para gestionar tu nómina</div>
        </div>`:l.innerHTML=`
        <table class="data-table">
          <thead><tr>
            <th>Empleado</th><th>Puesto</th><th>Departamento</th>
            <th>IMSS</th><th>Estado</th>
          </tr></thead>
          <tbody>
            ${i.map(n=>{const u=n.active!==!1,m=(n.name||"?").split(" ").map(g=>g[0]).slice(0,2).join("");return`
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
                <td><span class="badge badge-${u?"emerald":"gray"}">${u?"Activo":"Baja"}</span></td>
              </tr>`}).join("")}
          </tbody>
        </table>
        ${N(Q,y,n=>{Q=n,Tt()})}`),(a=document.getElementById("buscar-nom"))==null||a.addEventListener("input",n=>{const u=n.target.value.toLowerCase();document.querySelectorAll("#nom-tabla tbody tr").forEach(m=>{m.style.display=m.textContent.toLowerCase().includes(u)?"":"none"})}),window._verEmpleado=n=>{const u=i.find(m=>m.id===n);u&&H("Detalle de Empleado",async()=>u,m=>`
        ${C("Información",[p("Nombre completo",m.name),p("Puesto",m.job_title||m.job_id_name||"—"),p("Departamento",m.department_name||m.department_id_name||"—"),p("Estado",`<span class="badge badge-${m.active!==!1?"emerald":"gray"}">${m.active!==!1?"Activo":"Baja"}</span>`)].join(""))}
        ${C("IMSS / Fiscal",[p("N° IMSS",m.ssnid||m.imss||"—"),p("RFC",m.rfc||"—"),p("CURP",m.curp||"—")].join(""))}
        ${C("Contacto",[p("Email",m.work_email||m.email||"—"),p("Teléfono",m.work_phone||m.mobile_phone||"—")].join(""))}
        <div style="display:flex;gap:10px;margin-top:16px">
          <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
          <button class="btn btn-primary btn-sm" onclick="alert('Recibo de nómina — próximamente')">📄 Ver recibo</button>
        </div>`)}}catch(o){console.error(o),I("Error al cargar nómina",o.message,"error")}}const wt={purchase:{lbl:"Confirmada",color:"indigo"},done:{lbl:"Recibida",color:"emerald"},draft:{lbl:"Borrador",color:"gray"},cancel:{lbl:"Cancelada",color:"red"},sent:{lbl:"Enviada",color:"sky"}};let Z=1;async function ae(){M(),L([{label:"Dashboard",href:"dashboard"},{label:"Compras"}]),Z=1,await St()}async function St(){var t,e,s,a;B(`
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
  </div>`);try{const[o,c]=await Promise.allSettled([h.comprasKpis(),h.compras(Z)]),r=o.status==="fulfilled"?(t=o.value)==null?void 0:t.data:null,b=document.getElementById("kpi-row");b&&(b.innerHTML=[{label:"Total OC",val:(r==null?void 0:r.total)??0,tipo:"num",color:"indigo",icon:"📋"},{label:"Confirmadas",val:(r==null?void 0:r.confirmadas)??0,tipo:"num",color:"emerald",icon:"✅"},{label:"Monto Total",val:(r==null?void 0:r.monto_total)??0,tipo:"mxn",color:"violet",icon:"💰"},{label:"Completadas",val:(r==null?void 0:r.completadas)??0,tipo:"num",color:"amber",icon:"📦"}].map(n=>`
      <div class="data-card" style="padding:16px">
        <div style="font-size:11px;color:var(--text-400);font-weight:600;margin-bottom:4px">${n.icon} ${n.label}</div>
        <div style="font-size:22px;font-weight:800;color:var(--text-900)">
          ${n.tipo==="mxn"?E(parseFloat(n.val)):Number(n.val).toLocaleString("es-MX")}
        </div>
      </div>`).join(""));const i=c.status==="fulfilled"?((e=c.value)==null?void 0:e.data)||[]:[],d=((s=c.value)==null?void 0:s.total)??i.length,y=i.length>=20,x=document.getElementById("comp-sub");x&&(x.textContent=`${d} órdenes · Página ${Z}`);const l=document.getElementById("comp-tabla");l&&(i.length===0?l.innerHTML='<div style="text-align:center;padding:60px;color:var(--text-400)">Sin órdenes de compra registradas</div>':l.innerHTML=`
        <table class="data-table">
          <thead><tr>
            <th>Folio</th><th>Proveedor</th><th>Fecha</th>
            <th>Total</th><th>Estado</th>
          </tr></thead>
          <tbody>
            ${i.map(n=>{const u=wt[n.state]||{lbl:n.state||"—",color:"gray"};return`
              <tr style="cursor:pointer" onclick="window._verCompra(${n.id})" title="Ver detalle">
                <td class="td-mono">${n.name||`#${n.id}`}</td>
                <td class="td-primary">${n.partner_name||"—"}</td>
                <td>${F(n.date_order)}</td>
                <td class="td-amount" style="font-weight:700">${E(parseFloat(n.amount_total||0))}</td>
                <td>${U(n.state,u.lbl)}</td>
              </tr>`}).join("")}
          </tbody>
        </table>
        ${N(Z,y,n=>{Z=n,St()})}`),(a=document.getElementById("buscar-comp"))==null||a.addEventListener("input",n=>{const u=n.target.value.toLowerCase();document.querySelectorAll("#comp-tabla tbody tr").forEach(m=>{m.style.display=m.textContent.toLowerCase().includes(u)?"":"none"})}),window._verCompra=n=>{const u=i.find(m=>m.id===n);u&&H("Detalle Orden de Compra",async()=>u,m=>{const g=wt[m.state]||{lbl:m.state,color:"gray"};return`
          ${C("Orden",[p("Folio",m.name),p("Estado",U(m.state,g.lbl)),p("Proveedor",m.partner_name||"—"),p("Fecha",F(m.date_order)),p("Fecha entrega",F(m.date_planned))].join(""))}
          ${C("Importes",[p("Subtotal",E(parseFloat(m.amount_untaxed||0))),p("IVA",E(parseFloat(m.amount_tax||0))),p("Total",`<strong>${E(parseFloat(m.amount_total||0))}</strong>`,{color:"var(--primary)"})].join(""))}
          <div style="display:flex;gap:10px;margin-top:16px">
            <button class="btn btn-secondary btn-sm" onclick="window.__closeModal()">Cerrar</button>
            <button class="btn btn-primary btn-sm" onclick="alert('Recibir mercancía — próximamente')">📦 Recibir</button>
          </div>`})}}catch(o){console.error(o),I("Error al cargar compras",o.message,"error")}}let rt=null;async function oe(){M(),L([{label:"Dashboard",href:"dashboard"},{label:"NexusSearch"}]),await ie()}async function ie(){var e,s;B(`
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
  <div id="index-status" class="anim-4" style="margin-top:16px"></div>`),(e=document.getElementById("search-query"))==null||e.addEventListener("keydown",a=>{a.key==="Enter"&&window._buscar()});let t;(s=document.getElementById("search-query"))==null||s.addEventListener("input",a=>{clearTimeout(t),!(a.target.value.length<2)&&(t=setTimeout(()=>window._buscar(),500))}),await _t(),window._buscar=ne,window._checkStatus=_t,window._syncSearch=se}async function ne(){var s,a;const t=(a=(s=document.getElementById("search-query"))==null?void 0:s.value)==null?void 0:a.trim();if(!t||t.length<2)return;const e=document.getElementById("search-results");e&&(e.innerHTML=`
    <div class="data-card" style="padding:20px;text-align:center;color:var(--text-400)">
      <div class="spinner" style="margin:0 auto 8px"></div>
      <div>Buscando "${t}"…</div>
    </div>`);try{const[o,c,r]=await Promise.allSettled([h.ventas(1).then(i=>((i==null?void 0:i.data)||[]).filter(d=>(d.name||"").toLowerCase().includes(t.toLowerCase())||(d.partner_name||"").toLowerCase().includes(t.toLowerCase())).map(d=>({tipo:"Venta",icon:"💰",titulo:d.name,sub:d.partner_name,meta:`$${d.amount_total}`,href:"ventas"}))),h.productos(1,t).then(i=>((i==null?void 0:i.data)||[]).map(d=>{var y,x;return{tipo:"Producto",icon:"📦",titulo:typeof d.name=="object"?((y=d.name)==null?void 0:y.es_MX)||((x=d.name)==null?void 0:x.en_US)||"":d.name||"",sub:d.categ_name||"",meta:"",href:"productos"}})),h.partners(1).then(i=>((i==null?void 0:i.data)||[]).filter(d=>(d.name||"").toLowerCase().includes(t.toLowerCase())||(d.email||"").toLowerCase().includes(t.toLowerCase())).map(d=>({tipo:"Contacto",icon:"👥",titulo:d.name,sub:d.email||"",meta:"",href:"partners"})))]),b=[...o.status==="fulfilled"?o.value:[],...c.status==="fulfilled"?c.value:[],...r.status==="fulfilled"?r.value:[]];if(!e)return;if(b.length===0){e.innerHTML=`
      <div class="data-card" style="padding:40px;text-align:center">
        <div style="font-size:36px;margin-bottom:12px">🔍</div>
        <div style="font-weight:700;color:var(--text-700)">Sin resultados para "${t}"</div>
        <div style="color:var(--text-400);font-size:13px;margin-top:6px">Prueba con otro término</div>
      </div>`;return}e.innerHTML=`
    <div class="data-card">
      <div class="data-card-header">
        <div class="data-card-title">${b.length} resultados para "${t}"</div>
      </div>
      <div style="padding:0 4px">
        ${b.slice(0,30).map(i=>`
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
    </div>`}catch(o){console.error(o),e&&(e.innerHTML=`<p style="color:var(--red);padding:20px">Error: ${o.message}</p>`)}}async function _t(){const t=document.getElementById("index-status");try{const e=await h.searchStatus().catch(()=>null);rt=(e==null?void 0:e.data)||e,t&&rt&&(t.innerHTML=`
      <div class="data-card" style="padding:16px">
        <div class="data-card-header" style="padding:0 0 12px">
          <div class="data-card-title">📡 Estado del Motor de Búsqueda</div>
        </div>
        <div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(180px,1fr));gap:12px">
          ${Object.entries(rt).map(([s,a])=>`
          <div style="padding:12px;background:var(--bg);border-radius:10px;border:1px solid var(--border)">
            <div style="font-size:10px;color:var(--text-400);font-weight:700;text-transform:uppercase;margin-bottom:4px">${s}</div>
            <div style="font-size:13px;font-weight:600;color:var(--text-800)">${JSON.stringify(a)}</div>
          </div>`).join("")}
        </div>
      </div>`)}catch{t&&(t.innerHTML="")}}async function se(){const t=document.getElementById("btn-sync");t&&(t.textContent="⏳ Sincronizando…",t.disabled=!0);try{const e=await h.searchSync();I("Sincronización iniciada",(e==null?void 0:e.message)||"Los índices se están actualizando","success")}catch(e){I("Error de sincronización",e.message,"error")}finally{t&&(t.textContent="⚡ Sincronizar Índices",t.disabled=!1)}}async function re(){M(),L([{label:"Dashboard",href:"dashboard"},{label:"Reportes"}]),await le()}async function le(){B(`
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
  </div>`),window._verReporte=t=>{I("Reporte seleccionado",`Generando reporte de ${t}…`,"info"),de(t)},window._exportReporte=()=>{I("Exportar","Función de exportación CSV/PDF — próximamente","info")},await Ft()}async function Ft(){var e,s,a,o;const t=document.getElementById("rep-fecha");t&&(t.textContent=new Date().toLocaleDateString("es-MX",{day:"2-digit",month:"long",year:"numeric"}));try{const[c,r,b,i]=await Promise.allSettled([h.ventaKpis(),h.factKpis(),h.stockKpis(),h.comprasKpis()]),d=((e=c.value)==null?void 0:e.data)||{},y=((s=r.value)==null?void 0:s.data)||{},x=((a=b.value)==null?void 0:a.data)||{},l=((o=i.value)==null?void 0:o.data)||{},n=document.getElementById("rep-kpis");n&&(n.innerHTML=`
      ${[{label:"Ventas confirmadas",val:d.ordenes_confirmadas??0,tipo:"num",desc:`$${parseFloat(d.total_facturado||0).toLocaleString("es-MX",{minimumFractionDigits:2})} este mes`},{label:"Facturación total",val:E(parseFloat(y.monto_total||0)),tipo:"txt",desc:`${y.total_facturas??0} comprobantes emitidos`},{label:"Valor inventario",val:E(parseFloat(x.valor_inventario||0)),tipo:"txt",desc:`${x.alertas_stock_bajo??0} alertas de stock bajo`}].map(u=>`
      <div style="padding:16px;background:var(--bg);border-radius:12px;border:1px solid var(--border)">
        <div style="font-size:11px;color:var(--text-400);font-weight:700;text-transform:uppercase;margin-bottom:6px">${u.label}</div>
        <div style="font-size:24px;font-weight:800;color:var(--text-900);margin-bottom:4px">
          ${u.tipo==="num"?Number(u.val).toLocaleString("es-MX"):u.val}
        </div>
        <div style="font-size:11px;color:var(--text-500)">${u.desc}</div>
      </div>`).join("")}

      <div style="grid-column:1/-1;margin-top:8px">
        <div style="font-size:12px;font-weight:700;color:var(--text-600);margin-bottom:10px">COMPRAS</div>
        <div style="display:grid;grid-template-columns:repeat(3,1fr);gap:12px">
          ${[{label:"Total OC",val:l.total??0},{label:"Confirmadas",val:l.confirmadas??0},{label:"Monto compras",val:E(parseFloat(l.monto_total||0))}].map(u=>`
          <div style="padding:12px;background:var(--bg);border-radius:10px;border:1px solid var(--border)">
            <div style="font-size:10px;color:var(--text-400);font-weight:700;text-transform:uppercase;margin-bottom:4px">${u.label}</div>
            <div style="font-size:18px;font-weight:800;color:var(--text-900)">${u.val}</div>
          </div>`).join("")}
        </div>
      </div>`)}catch(c){console.error(c)}}async function de(t){const e=document.getElementById("rep-kpis"),s=document.querySelector(".data-card-title");if(s){const a={ventas:"💰 Reporte de Ventas",facturas:"🧾 Facturación",inventario:"🏭 Inventario",compras:"🛒 Compras",clientes:"👥 Clientes",nomina:"👔 Nómina"};s.textContent=a[t]||"Reporte"}e&&(e.innerHTML='<div class="skeleton" style="height:120px;border-radius:12px;grid-column:1/-1"></div>'),await Ft()}function ce(t,e,s,a){M(),L([{label:"Dashboard",href:"dashboard"},{label:e}]),B(`
  <div class="page-header anim-1">
    <div>
      <h1 class="page-title">${a} ${e}</h1>
      <p class="page-subtitle">${s}</p>
    </div>
  </div>
  <div class="data-card anim-2">
    <div class="empty-state">
      <div class="empty-state-icon">${a}</div>
      <div class="empty-state-title">Módulo ${e} en construcción</div>
      <div class="empty-state-desc">Este módulo estará disponible próximamente en NexusTech ERP v2.0</div>
      <button class="btn btn-primary" onclick="window._go('dashboard')">← Volver al Dashboard</button>
    </div>
  </div>`)}S("login",zt);S("dashboard",$t);S("ventas",Xt);S("facturas",Kt);S("productos",Jt);S("partners",Yt);S("stock",Qt);S("cfdi",Zt);S("nomina",ee);S("compras",ae);S("search",oe);S("reportes",re);S("404",()=>ce("404","Página no encontrada","La ruta solicitada no existe","🔍"));Pt();
