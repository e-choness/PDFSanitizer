import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

export default defineConfig({
  plugins: [svelte()],
  build: {
    target: 'esnext',
    minify: false,
    outDir: 'src-tauri/tauri-dist',
    emptyOutDir: true,
  },
})
