export const auth = {
  isLoggedIn: () => !!localStorage.getItem('nx_token'),
  getUser: () => {
    try { return JSON.parse(localStorage.getItem('nx_user') || '{}') }
    catch { return {} }
  },
  setSession(token, user) {
    localStorage.setItem('nx_token', token)
    localStorage.setItem('nx_user', JSON.stringify(user))
  },
  clear() {
    localStorage.removeItem('nx_token')
    localStorage.removeItem('nx_user')
  }
}
