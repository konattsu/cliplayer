import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "path";
// @ts-expect-error 型定義エラーを無視
import eslint from "vite-plugin-eslint";

export default defineConfig({
  plugins: [
    react(),
    eslint({
      include: ["src/**/*.ts", "src/**/*.tsx"],
      emitWarning: true,
      emitError: true,
    }),
  ],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "src"),
    },
  },
  build: {
    outDir: "dist",
    sourcemap: true,
    target: "esnext",
  },
});
