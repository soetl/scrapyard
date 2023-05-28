<template>
  <floating-menu />
  <v-sheet class="mt-5 h-auto"
    ><span>{{ route.params.id }}</span></v-sheet
  >
</template>

<script setup>
const route = useRoute();

useSeoMeta({
  title: `${route.params.id}`,
  ogTitle: `${route.params.id}`,
});
</script>
