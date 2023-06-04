<template>
  <teleport to="body">
    <v-btn
      class="save"
      color="success"
      icon="mdi-content-save"
      size="40"
      @click="saveText"
    />
  </teleport>
</template>

<script setup>
import { useUserStore } from "~/stores/user";

const code = inject("code");
const saveTextDialog = inject("saveTextDialog");
const logInDialog = inject("logInDialog");

const userStore = useUserStore();

function saveText() {
  if (code.value === "") {
    return;
  }

  if (!userStore.loggedIn) {
    logInDialog.value = true;
    return;
  }

  saveTextDialog.value = true;
}
</script>

<style scoped>
.save {
  position: fixed;
  bottom: 0.5rem;
  right: 0.5rem;
  z-index: 1000;
  opacity: 0.6;
  transition: opacity 0.3s ease-in-out;
}

.save:hover {
  opacity: 1;
}
</style>
