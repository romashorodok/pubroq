import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";


export default defineConfig({
  server: {
    https: {
      key: "../certificate.key",
      cert: "../certificate.pem",
    }
  },
  plugins: [
    react(),
    wasm(),
    topLevelAwait()
  ],
})
