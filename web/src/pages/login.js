import { api } from '../api.js'
import { auth } from '../auth.js'
import { go } from '../router.js'

export function renderLogin() {
  const shell = document.getElementById('__shell')
  if (shell) shell.remove()

  document.getElementById('app').innerHTML = `
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
  </div>`

  const btn  = document.getElementById('lbtn')
  const user = document.getElementById('lu')
  const pass = document.getElementById('lp')
  const err  = document.getElementById('lerr')

  async function doLogin() {
    if (btn.disabled) return
    const login = user.value.trim()
    const password = pass.value

    if (!login || !password) {
      err.textContent = 'Ingresa usuario y contraseña'
      err.classList.add('show')
      return
    }

    btn.disabled = true
    btn.textContent = 'Iniciando...'
    err.classList.remove('show')

    try {
      const res = await api.login(login, password)
      const data = res?.data || res
      const token = data?.access_token || data?.token

      if (token) {
        auth.setSession(token, {
          nombre: data.email || login,
          email: data.email || login,
          user_id: data.user_id,
          company_id: data.company_id,
        })
        document.getElementById('app').innerHTML = ''
        go('dashboard')
        return
      }

      err.textContent = 'Error inesperado. Intenta de nuevo.'
      err.classList.add('show')
    } catch (e) {
      err.textContent = e?.status === 401
        ? 'Credenciales incorrectas.'
        : `Error de conexión: ${e?.message || 'Fallo de red'}`
      err.classList.add('show')
    }

    btn.disabled = false
    btn.textContent = 'Iniciar sesión'
  }

  btn.addEventListener('click', doLogin)
  pass.addEventListener('keydown', e => e.key === 'Enter' && doLogin())
  user.addEventListener('keydown', e => e.key === 'Enter' && pass.focus())
  setTimeout(() => user.focus(), 100)
}
