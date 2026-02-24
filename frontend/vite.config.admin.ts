import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  root: ".",
  base: "/admin/",
  build: {
    outDir: "../public/admin",
    emptyOutDir: true,
    rollupOptions: {
      input: "admin.html",
    },
  },
  experimental: {
    renderBuiltUrl(filename, { hostType }) {
      if (hostType === "html") return filename;
      return "/admin/" + filename;
    },
  },
  server: {
    port: 5174,
    proxy: {
      "/api": "http://localhost:3000",
    },
  },
});
