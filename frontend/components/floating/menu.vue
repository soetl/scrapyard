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
      />
      <v-scroll-y-reverse-transition origin="top right">
        <div v-if="expand" class="d-flex flex-column">
          <v-btn
            v-show="homeButton"
            class="mb-2"
            icon="mdi-home"
            size="40"
            to="/"
          />
          <v-btn
            v-show="userStore.loggedIn"
            class="mb-2"
            icon="mdi-account"
            size="40"
          />
          <v-btn
            class="mb-2"
            :icon="
              theme === 'light' ? 'mdi-weather-sunny' : 'mdi-weather-night'
            "
            @click="toggleTheme"
            size="40"
          />
          <v-btn
            v-show="!userStore.loggedIn"
            class="mb-2"
            icon="mdi-login-variant"
            size="40"
            @click="logInDialog = true"
          />
          <v-btn
            v-show="userStore.loggedIn"
            icon="mdi-logout-variant"
            size="40"
            @click="logOutDialog = true"
          />
        </div>
      </v-scroll-y-reverse-transition>
    </div>
  </teleport>
</template>

<script setup>
import { useUserStore } from "~/stores/user";

const expand = ref(false);

const userStore = useUserStore();

const props = defineProps({
  homeButton: {
    type: Boolean,
    default: true,
  },
});

const theme = inject("theme");
const toggleTheme = inject("toggleTheme");
const logInDialog = inject("logInDialog");
const logOutDialog = inject("logOutDialog");
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
