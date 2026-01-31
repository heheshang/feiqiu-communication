import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: {
    port: 1420,
  },
  build: {
    sourcemap: false,
    minify: 'esbuild',
    rollupOptions: {
      output: {
        manualChunks: {
          'react-vendor': ['react', 'react-dom'],
          'tauri-vendor': ['@tauri-apps/api'],
        },
      },
    },
    target: 'es2020',
  },
  optimizeDeps: {
    include: ['react', 'react-dom', '@tauri-apps/api', 'zustand'],
  },
});
