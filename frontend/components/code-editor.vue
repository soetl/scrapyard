<template>
  <v-main>
    <codemirror
      v-model="code"
      :placeholder="placeholder"
      :extensions="extensions"
      :style="{ height: '100vh' }"
    ></codemirror>
  </v-main>
</template>

<script setup>
import { oneDark } from "@codemirror/theme-one-dark";

const props = defineProps(["placeholder", "global_theme"]);

const code = inject("code");

const extensions = computed(() => {
  return props.global_theme === "light" ? [] : [oneDark];
});
</script>
