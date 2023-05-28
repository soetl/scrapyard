<template>
  <v-sheet class="mt-5 h-screen"
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
