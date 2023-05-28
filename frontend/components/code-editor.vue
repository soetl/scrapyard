<template>
  <v-main>
    <codemirror
      v-model="code"
      :placeholder="placeholder"
      :extensions="extensions"
      :style="{ height: '100vh' }"
    />
  </v-main>
</template>

<script setup>
import { oneDark } from "@codemirror/theme-one-dark";

const props = defineProps(["placeholder"]);

const code = inject("code");
const theme = inject("theme");

const extensions = computed(() => {
  return theme.value === "light" ? [] : [oneDark];
});
</script>
