<template>
  <teleport to="body">
    <v-btn
      class="save"
      color="success"
      icon="mdi-content-save"
      size="40"
    ></v-btn>
  </teleport>
</template>

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
