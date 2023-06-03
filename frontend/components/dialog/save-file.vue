<template>
  <v-dialog v-model="dialog" persistent width="512">
    <v-card>
      <v-card-title>Save File</v-card-title>
      <v-card-text>
        <v-form ref="saveForm">
          <v-text-field
            v-model="filename"
            label="Filename"
            :rules="[
              (v) => v.length >= 1 || 'Filename must be at least 1 character',
              (v) => v.length <= 32 || 'Filename must be at most 32 characters',
            ]"
            dense
          />
        </v-form>
      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn text @click="dialog = false">Cancel</v-btn>
        <v-btn text @click="saveFile">Save</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { uploadPaste } from "~/api";
import { useUserStore } from "~/stores/user";

const props = defineProps({
  context: {
    required: true,
  },
});

const filename = ref("");
const saveForm = ref(null);

const userStore = useUserStore();

const dialog = inject("saveFileDialog");
const logInDialog = inject("logInDialog");

watch(dialog, async (value, _) => {
  if (value) {
    clearForm();
    setTimeout(() => {
      filename.value = props.context.name;
    }, 100);
  }
});

async function saveFile() {
  if (!userStore.loggedIn) {
    logInDialog.value = true;
    return;
  }

  const { valid } = await saveForm.value.validate();

  if (!valid) {
    return;
  }

  const { paste, error } = await uploadPaste(
    userStore.token,
    filename.value,
    "file",
    props.context
  );

  if (error) {
    console.error(error);

    return;
  }

  dialog.value = false;
  clearForm();

  navigateTo(`/f/${paste.paste.link}`);
}

function clearForm() {
  filename.value = "";
}
</script>

<style scoped>
.status {
  color: rgb(var(--v-theme-error));
}
</style>
