<template>
  <v-dialog v-model="dialog" width="512">
    <v-card>
      <v-card-title>Save Text</v-card-title>
      <v-card-text>
        <v-text-field v-model="title" label="Title" dense />
        <v-select
          v-model="expiration"
          label="Expiration"
          :items="expirationList"
        />
        <v-switch v-model="usePassword" label="Use password" />
        <v-text-field
          v-if="usePassword"
          v-model="password"
          label="Password"
          dense
          :append-icon="showPassword ? 'mdi-eye' : 'mdi-eye-off'"
          :type="showPassword ? 'text' : 'password'"
          @click:append="showPassword = !showPassword"
        />
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
import { usePastesStore } from "~/stores/pastes";
import { useUserStore } from "~/stores/user";

const dialog = inject("saveTextDialog");

const expirationList = ref(["1 day", "1 week", "1 month", "1 year"]);
const usePassword = ref(false);
const showPassword = ref(false);

const title = ref("");
const password = ref("");
const expiration = ref("1 day");

const userStore = useUserStore();
const pastesStore = usePastesStore();

const code = inject("code");

function savePaste() {
  if (!userStore.loggedIn) {
    logInDialog.value = true;
    return;
  }

  const paste = {
    name: title.value,
    type: "text",
    owner: userStore.name,
    content: code.value,
    password: usePassword.value ? password.value : null,
    expiration: expiration.value,
  };

  pastesStore.addPaste(paste);
  console.log(pastesStore.pastes);

  clearForm();
  dialog.value = false;
}

function clearForm() {
  title.value = "";
  password.value = "";
  expiration.value = "1 day";
  usePassword.value = false;
}
</script>
