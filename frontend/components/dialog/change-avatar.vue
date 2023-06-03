<template>
  <v-dialog v-model="dialog" width="512">
    <v-card>
      <v-card-title>Change avatar</v-card-title>
      <v-card-text>
        <v-text-field
          label="Image URL"
          hint="Put image URL"
          v-model="imageUrl"
          required
          clearable
        />
      </v-card-text>
      <v-card-actions class="justify-end">
        <v-btn variant="outlined" @click="dialog = false">Cancel</v-btn>
        <v-btn variant="outlined" class="ml-4" @click="saveAvatar">Save</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { updateUser } from "~/api";
import { useUserStore } from "~/stores/user";

const imageUrl = ref("");

const dialog = inject("changeAvatarDialog");

const userStorage = useUserStore();

const emit = defineEmits(["changeAvatar"]);

async function saveAvatar() {
  if (imageUrl.value === "") {
    return;
  }

  await updateUser(userStorage.getToken, imageUrl.value);
  emit("changeAvatar", imageUrl.value);
  dialog.value = false;
}

watch(dialog, (value) => {
  imageUrl.value = "";
});
</script>
