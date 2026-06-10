import { auth } from './auth.js'

const routes = {}

export function on(hash, fn) { routes[hash] = fn }

export function go(hash) { window.location.hash = hash }

export function start() {
  window.addEventListener('hashchange', dispatch)
  dispatch()
}

function dispatch() {
  const hash = window.location.hash.replace('#', '') || 'home'
  if (!auth.isLoggedIn() && hash !== 'login') { go('login'); return }
  if (auth.isLoggedIn() && hash === 'login')  { go('home'); return }
  const fn = routes[hash]
  if (fn) fn()
  else if (routes['404']) routes['404']()
}
