import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig(({ mode }) => {
  return {
    plugins: [react()],
    server: {
      proxy:
        mode === "development"
          ? {
              "/api": {
                target: "http://localhost:2333", // 后端服务器地址
                changeOrigin: true, // 支持跨域
              },
            }
          : {},
    },
    build: {
      minify: mode === "development" ? false: true
    }
  };
});
