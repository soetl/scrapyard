import "vuetify/styles";
import { createVuetify } from "vuetify";
import * as components from "vuetify/components";
import * as directives from "vuetify/directives";

export default defineNuxtPlugin((nuxtApp) => {
  const dark = {
    dark: true,
    colors: {
      background: "#1d2025",
      surface: "#282c34",
    },
  };

  const light = {
    dark: false,
    colors: {
      background: "#eeeff1",
      surface: "#fafafa",
    },
  };

  const vuetify = createVuetify({
    ssr: true,
    components,
    directives,
    theme: {
      defaultTheme: "dark",
      themes: { dark, light },
    },
  });

  nuxtApp.vueApp.use(vuetify);
});
