<template>
  <teleport to="body">
    <div class="menu">
      <v-btn
        class="menu_btn mb-2"
        :class="{ active_btn: expand }"
        color="primary"
        icon="mdi-dots-vertical"
        @click="expand = !expand"
        size="40"
      ></v-btn>
      <v-scroll-y-reverse-transition origin="top right">
        <div v-if="expand" class="d-flex flex-column">
          <v-btn class="mb-2" icon="mdi-account" size="40"></v-btn>
          <v-btn
            class="mb-2"
            :icon="
              props.theme === 'light'
                ? 'mdi-weather-sunny'
                : 'mdi-weather-night'
            "
            @click="this.$emit('toggleTheme')"
            size="40"
          ></v-btn>
          <v-btn class="mb-2" icon="mdi-login-variant" size="40" >
            <v-icon icon="mdi-login-variant"></v-icon>
            <dialog-log-in />
          </v-btn>
          <v-btn icon="mdi-logout-variant" size="40">
            <v-icon icon="mdi-logout-variant"></v-icon>
            <dialog-log-out />
          </v-btn>
        </div>
      </v-scroll-y-reverse-transition>
    </div>
  </teleport>
</template>

<script setup>
const expand = ref(false);

const props = defineProps({
  theme: String,
});
</script>

<style scoped>
.menu {
  position: fixed;
  top: 0.5rem;
  right: 0.5rem;
  z-index: 1000;
}

.menu_btn {
  opacity: 0.6;
  transition: opacity 0.4s ease-in-out;
}

.menu_btn:hover {
  opacity: 1;
}

.active_btn {
  opacity: 1;
}
</style>
