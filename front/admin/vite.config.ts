import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    proxy: {
      // 与后端联调时，/api 转发到 Axum，供 ALTCHA 拉取 challenge
      '/api': { target: 'http://127.0.0.1:8000', changeOrigin: true },
    },
  },
})
