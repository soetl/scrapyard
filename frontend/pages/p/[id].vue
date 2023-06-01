<template>
  <v-main class="fill-height d-flex justify-center">
    <floating-menu />
    <v-container>
      <v-sheet rounded="lg" class="mt-4 elevation-2">
        <v-container>
          <div class="text-h6 text-truncate d-flex justify-space-between mb-4">
            <div class="d-flex">
              <span class="title text-truncate">{{ route.params.id }}</span>
              <span class="plain mx-2">by</span>
              <nuxt-link :to="'/u/' + author">{{ author }} </nuxt-link>
            </div>

            <div class="">
              <v-btn variant="outlined" class="ml-4">Raw</v-btn>
              <v-btn variant="outlined" class="ml-4">Download</v-btn>
              <v-btn variant="outlined" class="ml-4">Delete</v-btn>
            </div>
          </div>
          <code-viewer />
        </v-container>
      </v-sheet>
    </v-container>
  </v-main>
</template>

<script setup>
const route = useRoute();
const author = "soetl";
</script>

<style scoped>
.plain {
  color: #a1a1a1;
}

.title {
  max-width: 15vw;
}

a {
  text-decoration: none;
  color: rgb(var(--v-theme-text));
}
</style>
