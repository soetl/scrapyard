<template>
  <v-dialog v-model="dialog" width="256">
    <v-card>
      <v-card-title>
        <span class="text-h6">Log Out</span>
      </v-card-title>
      <v-card-text>Are you sure you want to log out?</v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="primary" @click="dialog = false">Cancel</v-btn>
        <v-btn color="primary" @click="logOut">Log out</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { useUserStore } from "~/stores/user";

const dialog = inject("logOutDialog");

const userStore = useUserStore();

function logOut() {
  userStore.logOut();
  dialog.value = false;
}
</script>
