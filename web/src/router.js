import { auth } from './auth.js'

const routes = {}

export function on(hash, fn) { routes[hash] = fn }

export function go(hash) {
  // Si el hash ya es el mismo, hashchange no se dispara — forzamos dispatch manualmente
  const current = window.location.hash.replace('#', '') || 'home'
  if (current === hash) {
    dispatch()
  } else {
    window.location.hash = hash
  }
}

export function start() {
  window.addEventListener('hashchange', dispatch)
  dispatch()
}

function dispatch() {
  const full = window.location.hash.replace('#', '') || 'home'
  const [routeKey, queryStr] = full.split('?')
  const params = {}
  if (queryStr) {
    queryStr.split('&').forEach(p => {
      const [k, v] = p.split('=')
      if (k) params[decodeURIComponent(k)] = decodeURIComponent(v || '')
    })
  }
  if (!auth.isLoggedIn() && routeKey !== 'login') { go('login'); return }
  if (auth.isLoggedIn()  && routeKey === 'login')  { go('home');  return }
  const fn = routes[routeKey]
  if (fn) fn(params)
  else if (routes['404']) routes['404'](params)
}
