import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  root: ".",
  base: "/",
  build: {
    outDir: "../public",
    emptyOutDir: false,
    rollupOptions: {
      input: "user.html",
    },
  },
  // Rename user.html → index.html in the output so the Rust SPA
  // fallback (which looks for public/index.html) works unchanged.
  experimental: {
    renderBuiltUrl(filename, { hostType }) {
      if (hostType === "html") return filename;
      return "/" + filename;
    },
  },
  server: {
    port: 5173,
    proxy: {
      "/api": "http://localhost:3000",
    },
  },
});
