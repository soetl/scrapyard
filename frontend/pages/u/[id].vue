<template>
  <v-main>
    <floating-menu />
    <v-container width="1280">
      <div rounded="lg" elevation="2" class="d-flex">
        <v-container
          width="512"
          class="d-flex flex-column align-center justify-center text-truncate"
        >
          <div class="text-center">
            <v-avatar
              image="https://cdn.discordapp.com/avatars/270176025713967104/96dc0b71496b75e7431316c2eacb20ba.webp"
              size="128"
            ></v-avatar>
            <div>
              <span class="nickname text-h6 text-truncate"
                >asasdadsdasdassdaasddasasdklasdlkklsdaklasdlkasdlklaskdlkasdklkdaskadskkdaslklkdaslkdsad</span
              >
            </div>
          </div>
        </v-container>
      </div>
      <v-container>
        <v-row>
          <v-col v-for="n in 24" :key="n" cols="3">
            <v-card height="400"></v-card>
          </v-col>
        </v-row>
      </v-container>
    </v-container>
  </v-main>
</template>

<script setup>
const route = useRoute();

useSeoMeta({
  title: `${route.params.id}`,
  ogTitle: `${route.params.id}`,
});
</script>

<style scoped>
.nickname {
  max-width: 256px;
  display: inline-block;
}
</style>
