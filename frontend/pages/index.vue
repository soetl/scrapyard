<template>
  <div ref="window">
    <v-overlay
      v-model="isOverDropZone"
      class="align-center justify-center"
      contained
    >
      <span>Drop your file here.</span>
    </v-overlay>
    <floating-menu />
    <code-editor :placeholder="placeholder" />
    <floating-save />
    <dialog-save-file :context="saveFileContext" />
    <dialog-save-text />
  </div>
</template>

<script setup>
import { useUserStore } from '~/stores/user';

const window = ref(null);

const code = ref("");
const placeholder = ref("You can save your files by dragging them here.");
const { isOverDropZone } = useDropZone(window, onDrop);

const saveFileDialog = inject("saveFileDialog");
const saveFileContext = ref(null);

const userStore = useUserStore();

const logInDialog = inject("logInDialog");

function onDrop(files) {
  if (!userStore.loggedIn) {
    logInDialog.value = true;
    return;
  }
  
  saveFileContext.value = files[0];
  saveFileDialog.value = true;
}

provide("code", code);
</script>
