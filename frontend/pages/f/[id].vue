<template>
  <v-main class="fill-height">
    <floating-menu />
    <v-container class="d-flex align-center justify-center fill-height">
      <v-sheet rounded="lg" class="elevation-2" max-width="1024px">
        <v-container class="d-flex">
          <div>
            <v-img
              height="320px"
              aspect-ratio="1/1"
              width="320px"
              rounded="lg"
              class="image mr-4"
              src="https://cdn.discordapp.com/avatars/270176025713967104/96dc0b71496b75e7431316c2eacb20ba.webp"
            >
              <template v-slot:error>
                <div class="d-flex align-center justify-center fill-height">
                  <v-icon
                    size="128px"
                    icon="mdi-file-document-outline"
                  ></v-icon>
                </div>
              </template>
            </v-img>
          </div>
          <div class="text-truncate d-flex flex-column justify-space-between">
            <div class="text-truncate">
              <span class="text-h5">{{ route.params.id }}</span>
              <br />
              <span class="plain mr-2">by</span>
              <nuxt-link :to="'/u/' + author">{{ author }}</nuxt-link>
            </div>
            <div class="d-flex justify-end">
              <v-btn variant="outlined">Download</v-btn>
              <v-btn variant="outlined" class="ml-4">Delete</v-btn>
            </div>
          </div>
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

.image {
  border-radius: 8px;
}

a {
  text-decoration: none;
  color: rgb(var(--v-theme-text));
}
</style>
