<template>
  <v-main class="fill-height">
    <floating-menu />
    <v-container class="d-flex align-center justify-center fill-height">
      <v-sheet rounded="lg" class="elevation-2" max-width="1024px">
        <v-container class="d-flex">
          <div>
            <v-img
              height="320px"
              :aspect-ratio="1 / 1"
              width="320px"
              rounded="lg"
              :src="imageSrc"
              cover
              class="image mr-4 border"
            >
              <template v-slot:error v-slot:placeholder>
                <div class="d-flex align-center justify-center fill-height">
                  <v-icon
                    size="128px"
                    icon="mdi-file-document-outline"
                  ></v-icon>
                </div>
              </template>
            </v-img>
          </div>
          <div class="text-truncate d-flex flex-column justify-space-between">
            <div class="text-truncate">
              <span class="text-h5">{{ paste.filename }}</span>
              <br />
              <span class="plain mr-2">by</span>
              <nuxt-link :to="'/u/' + paste.owner">{{ paste.owner }}</nuxt-link>
            </div>
            <div class="d-flex justify-end">
              <v-btn variant="text" @click="downloadButton">Download</v-btn>
              <v-btn
                v-if="paste.owner === userStore.getUsername"
                class="ml-4"
                variant="text"
                @click="confirmDialog = true"
              >
                Delete
              </v-btn>
            </div>
          </div>
        </v-container>
      </v-sheet>
    </v-container>
  </v-main>
  <dialog-confirmation
    :text="'Are you sure you want to delete the file?'"
    @confirm="deleteConfirm"
  />
</template>

<script setup>
import { getPaste, downloadPaste, deletePaste } from "~/api";
import { useUserStore } from "~/stores/user";

const imageSrc = ref("http://localhost/bad/request.png");

const route = useRoute();

const userStore = useUserStore();

const confirmDialog = inject("confirmDialog");

const { paste: raw_paste, error: error } = await getPaste(route.params.id);

if (error) {
  navigateTo("/404");
}

const paste = {
  filename: raw_paste.value.paste.filename,
  owner: raw_paste.value.paste.owner.username,
  mime: raw_paste.value.paste.mime,
};

const { paste: pasteBinary, error: errorBinary } = await downloadPaste(
  route.params.id,
  paste.filename
);

if (!errorBinary) {
  imageSrc.value = await blobToImage(pasteBinary.value, paste.mime);
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
</script>

<style scoped>
.plain {
  color: rgb(var(--v-theme-textplain));
}

.title {
  max-width: 15vw;
}

.image {
  border-radius: 8px;
}

a {
  text-decoration: none;
  color: rgb(var(--v-theme-text));
}
</style>
