import { api } from '../api.js'
import { auth } from '../auth.js'
import { go } from '../router.js'

export function renderLogin() {
  // Remove existing shell if present
  const shell = document.getElementById('__shell')
  if (shell) shell.remove()

  document.getElementById('app').innerHTML = `
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
          <label class="form-label" for="lu">Correo electrónico</label>
          <input class="form-control" id="lu" type="email" placeholder="admin@empresa.mx" autocomplete="username">
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
          <strong>Demo:</strong> Usa cualquier email y contraseña para acceder
        </div>
      </div>
    </div>
  </div>`

  const btn  = document.getElementById('lbtn')
  const user = document.getElementById('lu')
  const pass = document.getElementById('lp')
  const err  = document.getElementById('lerr')

  async function doLogin() {
    if (btn.disabled) return
    btn.disabled = true
    btn.textContent = 'Verificando...'
    err.classList.remove('show')

    try {
      // Try real API first
      const res = await api.login(user.value.trim(), pass.value)
      if (res?.token || res?.access_token) {
        auth.setSession(res.token || res.access_token, res.usuario || res.user || { nombre: user.value })
        document.getElementById('app').innerHTML = ''
        go('dashboard')
        return
      }
    } catch {
      // fallback: demo login
    }

    // Demo mode: accept any credentials
    if (user.value.trim()) {
      const nombre = user.value.split('@')[0].replace('.', ' ')
      auth.setSession('demo-token-' + Date.now(), {
        nombre: nombre.charAt(0).toUpperCase() + nombre.slice(1),
        email: user.value.trim()
      })
      document.getElementById('app').innerHTML = ''
      go('dashboard')
    } else {
      err.classList.add('show')
      btn.disabled = false
      btn.textContent = 'Acceder al sistema'
    }
  }

  btn.addEventListener('click', doLogin)
  pass.addEventListener('keydown', e => e.key === 'Enter' && doLogin())
  user.addEventListener('keydown', e => e.key === 'Enter' && pass.focus())
  setTimeout(() => user.focus(), 100)
}
