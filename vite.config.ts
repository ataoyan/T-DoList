import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { readFileSync } from 'fs';
import { resolve } from 'path';

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// 读取版本信息
const versionInfo = JSON.parse(readFileSync(resolve(__dirname, 'version.json'), 'utf-8'));

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [vue()],
  
  // 定义环境变量
  define: {
    'import.meta.env.VITE_APP_VERSION': JSON.stringify(versionInfo.version),
    'import.meta.env.VITE_APP_NAME': JSON.stringify(versionInfo.name),
    'import.meta.env.VITE_APP_DESCRIPTION': JSON.stringify(versionInfo.description),
    'import.meta.env.VITE_APP_AUTHOR': JSON.stringify(versionInfo.author),
    'import.meta.env.VITE_APP_REPOSITORY_URL': JSON.stringify(versionInfo.repository.url),
    'import.meta.env.VITE_APP_LICENSE': JSON.stringify(versionInfo.license),
    'import.meta.env.VITE_APP_BUILD_DATE': JSON.stringify(versionInfo.build.date),
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1421,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1422,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
