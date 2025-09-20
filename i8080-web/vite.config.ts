import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import wasm from 'vite-plugin-wasm';
import fs from 'fs';
import type { Plugin } from 'vite';
import path from 'path';
import tailwindcss from 'tailwindcss';

const hexLoader: Plugin = {
  name: 'hex-loader',
  transform(_code, id) {
    const [path, query] = id.split('?');
    if (query !== 'raw-hex') return null;

    const data = fs.readFileSync(path);
    const hexString = data.toString('hex');

    // Convert hex string to an array of hex values
    const hexArray = [];
    for (let i = 0; i < hexString.length; i += 2) {
      hexArray.push(`0x${hexString.substr(i, 2)}`);
    }

    return `export default ${JSON.stringify(hexArray)};`;
  }
};

// https://vitejs.dev/config/
export default defineConfig({
  build:{
    target: 'esnext',
  },
  plugins: [react(), wasm(), hexLoader],
  css: {
    postcss: {
      plugins: [tailwindcss()],
    },
  },
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },

})

