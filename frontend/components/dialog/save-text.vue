<template>
  <v-dialog v-model="dialog" width="512">
    <v-card>
      <v-card-title>Save Text</v-card-title>
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
        <v-btn text @click="savePaste">Save</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { uploadPaste } from "~/api";
import { useUserStore } from "~/stores/user";

const filename = ref("");
const saveForm = ref(null);

const userStore = useUserStore();

const dialog = inject("saveTextDialog");
const code = inject("code");

async function savePaste() {
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
    "text",
    new Blob([code.value], { type: "text/plain" })
  );

  if (error) {
    console.error(error);

    return;
  }

  clearForm();
  dialog.value = false;

  navigateTo(`/p/${paste.paste.link}`);
}

function clearForm() {
  filename.value = "";
}
</script>
