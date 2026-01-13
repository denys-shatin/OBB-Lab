import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// Для Tauri используем относительные пути, для GitHub Pages - /OBB-Lab/
const isTauri = process.env.TAURI_ENV_PLATFORM !== undefined

export default defineConfig({
  plugins: [svelte()],
  base: isTauri ? './' : '/OBB-Lab/',
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true
  }
})
