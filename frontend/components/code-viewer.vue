<template>
  <v-main>
    <codemirror
      disabled
      v-model="code"
      :placeholder="placeholder"
      :extensions="extensions"
    />
  </v-main>
</template>

<script setup>
import { oneDark } from "@codemirror/theme-one-dark";

const theme = inject("theme");

const code = ref(`#include <stdio.h>\n\nint main() {\n  printf("Hello World!");\n  return 0;\n}`);

const extensions = computed(() => {
  return theme.value === "light" ? [] : [oneDark];
});
</script>
