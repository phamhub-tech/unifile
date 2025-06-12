// https://nuxt.com/docs/api/configuration/nuxt-config
const host = process.env.TAURI_DEV_HOST;

export default defineNuxtConfig({
  compatibilityDate: "2025-05-15",
  devtools: { enabled: true },
  ssr: false,
  devServer: { host: host || "localhost", port: 4000 },
  telemetry: false,
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
  },
});
