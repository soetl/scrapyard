// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  css: ["@mdi/font/css/materialdesignicons.css"],
  build: {
    transpile: ["vuetify"],
  },
  modules: ["@vueuse/nuxt", "@pinia/nuxt", "@pinia-plugin-persistedstate/nuxt"],
  runtimeConfig: {
    public: {
      baseURL: process.env.BASE_URL || "http://localhost:8000",
    },
  },
  ssr: false,
});
