<template>
  <floating-menu @toggleTheme="toggleTheme" />
  <code-editor :placeholder="placeholder" />
  <floating-save />
</template>

<script setup>
import { oneDark } from "@codemirror/theme-one-dark";

//const stored_theme = useCookie("theme", { default: () => "light" });

const code = ref("");
const placeholder = ref("You can save your files by dragging them here.");

provide("code", code);

//function toggleTheme() {
//  stored_theme.value = stored_theme.value === "light" ? "dark" : "light";
//}
</script>
