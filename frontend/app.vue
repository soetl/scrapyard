<template>
  <v-app :theme="stored_theme">
    <nuxt-layout>
      <nuxt-page />
    </nuxt-layout>
  </v-app>
</template>

<script setup>
const stored_theme = useCookie("theme", { default: () => "light" });

function toggleTheme() {
  stored_theme.value = stored_theme.value === "light" ? "dark" : "light";
}

provide("theme", stored_theme)
provide("toggleTheme", toggleTheme)
</script>
