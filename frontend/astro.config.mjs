import { defineConfig } from "astro/config";
import svelte from "@astrojs/svelte";
import tailwind from "@astrojs/tailwind";

export default defineConfig({
  integrations: [svelte(), tailwind()],
  output: "static",
  i18n: {
    defaultLocale: "fr",
    locales: ["fr", "ar", "en"],
  },
  vite: {
    resolve: {
      alias: {
        "@": "/src",
      },
    },
    server: {
      watch: {
        usePolling: true,
        interval: 1000,
      },
    },
  },
});
