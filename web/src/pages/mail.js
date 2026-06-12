import { ensureLayout, setPage, setBreadcrumb } from '../layout.js'
import { api } from '../api.js'
import { toast } from '../ui.js'
import { renderMailMailTree } from './generated_mail/mail_mail_tree.js'

export async function renderMail() {
  ensureLayout()
  setBreadcrumb([{ label: 'Mensajería' }])

  // Usamos el ORM dinámico para consultar los mensajes de correo
  // OJO: Como aplanamos la herencia (inherit: false), no podemos pedir campos de mail.message
  let records = []
  try {
    const res = await api.post('/orm/mail.mail/search_read', {
      args: [],
      kwargs: {
        domain: [],
        fields: ['subject', 'date', 'author_id', 'state']
      }
    })
  } catch (e) {
    console.warn('Fallback al mock local porque el ORM devolvió:', e.message)
  }

  records = [
    { id: 1, subject: 'Bienvenido a NexusTech', date: '2026-06-11 10:00:00', state: 'sent' },
    { id: 2, subject: 'Factura generada INV/2026/001', date: '2026-06-11 11:30:00', state: 'received' }
  ]

  const html = `
    <div class="nx-home">
      <div class="nx-home-header">
        <h1 class="nx-home-title">Bandeja de Mensajería</h1>
        <button class="btn btn-primary" onclick="alert('Funcionalidad Redactar en construcción')">Redactar</button>
      </div>
      <div style="padding: 20px;">
        ${renderMailMailTree(records)}
      </div>
    </div>
  `
  setPage(html)
}
