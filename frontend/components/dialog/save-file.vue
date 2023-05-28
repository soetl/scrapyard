<template>
  <v-dialog v-model="dialog" persistent width="512">
    <v-card>
      <v-card-title>Save File: {{ props.context.name }}</v-card-title>
      <v-card-text>
        <v-text-field v-model="filename" label="Filename" dense />
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
        <v-btn text @click="saveFile">Save</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { usePastesStore } from "~/stores/pastes";
import { useUserStore } from "~/stores/user";

const expirationList = ref(["1 day", "1 week", "1 month", "1 year"]);
const usePassword = ref(false);
const showPassword = ref(false);

const filename = ref("");
const password = ref("");
const expiration = ref("1 day");

const userStore = useUserStore();
const pastesStore = usePastesStore();

const props = defineProps({
  context: {
    type: Object,
    required: true,
  },
});
const dialog = inject("saveFileDialog");
const logInDialog = inject("logInDialog");

function saveFile() {
  if (!userStore.loggedIn) {
    logInDialog.value = true;
    return;
  }

  if (!filename.value) {
    filename.value = props.context.name;
  }

  const file = {
    name: filename.value,
    type: "file",
    owner: userStore.name,
    content: props.context,
    password: usePassword.value ? password.value : null,
    expiration: expiration.value,
  };
  
  pastesStore.addPaste(file);

  console.log(pastesStore.pastes);

  clearForm();
  dialog.value = false;
}

function clearForm() {
  filename.value = "";
  password.value = "";
  expiration.value = "1 day";
  usePassword.value = false;
}
</script>
