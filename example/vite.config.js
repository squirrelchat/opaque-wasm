import wasm from 'vite-plugin-wasm'
export default {
  plugins: [ wasm() ],
  server: {
    proxy: {
      '/registration': 'http://localhost:1337',
      '/login': 'http://localhost:1337',
    }
  }
}
