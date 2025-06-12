import tailwindcss from '@tailwindcss/vite';

// https://nuxt.com/docs/api/configuration/nuxt-config
const host = process.env.TAURI_DEV_HOST;

export default defineNuxtConfig({
  compatibilityDate: "2025-05-15",
  devtools: { enabled: true },
  ssr: false,
  devServer: { host: host || "localhost", port: 4000 },
  telemetry: false,
  css: ['~/assets/css/main.css'],

  vite: {
    clearScreen: true,
    envPrefix: ["VITE_", "TAURI_"],
    server: {
      strictPort: true,
      hmr: host
        ? {
          protocol: "ws",
          host,
          port: 1421,
        }
        : undefined,
      watch: {
        ignored: ["**/src-tauri/**"],
      },
    },
    plugins: [
      tailwindcss(),
    ]
  },

  modules: [
    "shadcn-nuxt",
    "nuxt-typed-router",
    "@nuxtjs/i18n",
    "@nuxt/eslint"
  ],
  i18n: {
    lazy: true,
    defaultLocale: 'en',
    compilation: {
      strictMessage: false,
      escapeHtml: false,
    },
    locales: [{ code: 'en', language: 'en-GB', file: 'en_GB.json' }],
  },
  shadcn: {
    prefix: '',
    componentDir: 'components/ui'
  }
});