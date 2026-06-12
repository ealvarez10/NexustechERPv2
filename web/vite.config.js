import { defineConfig } from 'vite'
export default defineConfig({
  server: {
    port: 3001,
    strictPort: true,
    host: '0.0.0.0',
    proxy: { '/api': { target: 'http://localhost:8090', changeOrigin: true } }
  },
  build: { outDir: 'dist', minify: 'esbuild' }
})
