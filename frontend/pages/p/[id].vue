<template>
  <v-main class="fill-height d-flex justify-center">
    <floating-menu />
    <v-container>
      <v-sheet rounded="lg" class="mt-4 elevation-2">
        <v-container>
          <div class="text-h6 text-truncate d-flex justify-space-between mb-4">
            <div class="d-flex">
              <span class="title text-truncate">{{ paste.filename }}</span>
              <span class="plain mx-2">by</span>
              <nuxt-link :to="'/u/' + paste.owner"
                >{{ paste.owner }}
              </nuxt-link>
            </div>

            <div class="">
              <v-btn variant="outlined" class="ml-4">Copy</v-btn>
              <v-btn variant="outlined" class="ml-4" @click="downloadButton">
                Download
              </v-btn>
              <v-btn
                v-if="paste.owner === userStore.getUsername"
                variant="outlined"
                class="ml-4"
                @click="confirmDialog = true"
              >
                Delete
              </v-btn>
            </div>
          </div>
          <code-viewer :text="text" />
        </v-container>
      </v-sheet>
    </v-container>
  </v-main>
  <dialog-confirmation
    :text="'Are you sure you want to delete the paste?'"
    @confirm="deleteConfirm"
  />
</template>

<script setup>
import { downloadPaste, getPaste } from "~/api";
import { useUserStore } from "~/stores/user";
import { useClipboard } from "@vueuse/core";

const text = ref("asd");

const route = useRoute();

const { paste: raw_paste, error: error } = await getPaste(route.params.id);

const userStore = useUserStore();

const confirmDialog = inject("confirmDialog");

if (error) {
  navigateTo("/404");
}

const paste = {
  filename: raw_paste.value.paste.filename,
  owner: raw_paste.value.paste.owner.username,
};

const { paste: pasteBinary, error: errorBinary } = await downloadPaste(
  route.params.id,
  paste.filename
);

if (!errorBinary) {
  text.value = await new Response(pasteBinary.value).text();
}

function downloadButton() {
  let url = URL.createObjectURL(pasteBinary.value);
  let link = document.createElement("a");
  link.href = url;
  link.download = paste.filename;
  link.click();
}

async function deleteConfirm() {
  const { error } = await deletePaste(userStore.getToken, route.params.id);

  if (!error) {
    navigateTo("/");
  } else {
    console.log(error);
    return;
  }
}

function copyToClipboard() {
  const { copy } = useClipboard();
  copy(text.value);
}
</script>

<style scoped>
.plain {
  color: rgb(var(--v-theme-textplain));
}

.title {
  max-width: 15vw;
}

a {
  text-decoration: none;
  color: rgb(var(--v-theme-text));
}
</style>
