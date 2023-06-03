<template>
  <v-dialog v-model="dialog" width="256">
    <v-card>
      <v-card-title>
        <span class="text-h6">Are you sure?</span>
      </v-card-title>
      <v-card-text>{{ prop.text }}</v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="primary" @click="dialog = false">Cancel</v-btn>
        <v-btn color="primary" @click="confirm">Yes</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
const prop = defineProps(["text"]);
const emit = defineEmits(["confirm"]);

const dialog = inject("confirmDialog");

const confirm = () => {
  dialog.value = false;
  emit("confirm");
};
</script>
