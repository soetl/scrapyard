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

const props = defineProps(["text"]);
const code = shallowRef(props.text);
const placeholder = ref("Loading...");

const theme = inject("theme");

const extensions = computed(() => {
  return theme.value === "light" ? [] : [oneDark];
});
</script>
