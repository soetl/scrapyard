<template>
  <v-main>
    <floating-menu />
    <v-container width="1280">
      <div rounded="lg" elevation="2" class="d-flex">
        <v-container
          width="512"
          class="d-flex flex-column align-center justify-center text-truncate"
        >
          <div class="text-center">
            <div class="avatar ma-auto">
              <v-hover v-if="itsMe" v-slot="{ isHovering, props }">
                <v-avatar size="200px" v-bind="props">
                  <v-img
                    :src="profileImage"
                    class="image"
                    :aspect-ratio="1 / 1"
                    cover
                  >
                    <template v-slot:error v-slot:placeholder>
                      <v-icon
                        size="200px"
                        icon="mdi-account-circle-outline avatar-background"
                        color="background"
                      ></v-icon>
                    </template>
                  </v-img>
                  <v-overlay
                    :model-value="isHovering"
                    contained
                    class="align-center justify-center"
                  >
                    <v-btn @click="changeAvatarDialog = true">Change</v-btn>
                  </v-overlay>
                </v-avatar>
              </v-hover>
              <v-avatar v-else size="200px">
                <v-img :src="profileImage">
                  <template v-slot:error>
                    <v-icon
                      size="200px"
                      icon="mdi-account-circle-outline"
                      color="background"
                    ></v-icon>
                  </template>
                </v-img>
              </v-avatar>
            </div>

            <div>
              <span class="nickname text-h4 text-truncate">{{
                profile.username
              }}</span>
            </div>
          </div>
        </v-container>
      </div>
      <v-container>
        <v-row>
          <v-col v-for="item in pastes" :key="item.link" cols="3">
            <paste-card
              :item="item"
              :its-me="itsMe"
              @delete="deletePasteEmited"
              @download="downloadPasteEmited"
            />
          </v-col>
        </v-row>
        <v-card v-if="enableScroll" v-intersect="nextPage"></v-card>
      </v-container>
    </v-container>
  </v-main>
  <dialog-change-avatar @changeAvatar="(img) => (profileImage = img)" />
  <dialog-confirmation
    :text="'Are you sure you want to delete this?'"
    @confirm="deletePasteConfirm"
  />
</template>

<script setup>
import { getPastes, getProfile, deletePaste } from "~/api";
import { useUserStore } from "~/stores/user";

const profileImage = ref("http://localhost/bad/request");
const baseLimit = ref(8);
const offset = ref(0);
const loads = ref(0);
const pastes = ref([]);
const enableScroll = ref(true);
const deleteLink = ref("");

const route = useRoute();
const userStorage = useUserStore();

const itsMe = computed(() => {
  return userStorage.getUsername === route.params.id;
});

const changeAvatarDialog = inject("changeAvatarDialog");
const confirmDialog = inject("confirmDialog");

const { profile, error } = await getProfile(route.params.id);

if (error) {
  navigateTo("/404");
}

if (profile.image === null || profile.image === "") {
  profileImage.value = "http://localhost/bad/request";
} else {
  profileImage.value = profile.image;
}

await nextPage();

async function deletePasteEmited(link) {
  confirmDialog.value = true;
  deleteLink.value = link;
}

async function deletePasteConfirm() {
  await deletePaste(userStorage.getToken, deleteLink.value);
  pastes.value = pastes.value.filter((paste) => paste.link !== deleteLink.value);
  deleteLink.value = "";
}

function downloadPasteEmited() {

}

async function nextPage() {
  const { pastes: raw_pastes, error: pasteError } = await getPastes(
    route.params.id,
    baseLimit.value,
    offset.value
  );

  if (raw_pastes.pastes[1] === 0) {
    enableScroll.value = false;
  }

  pastes.value = pastes.value.concat(raw_pastes.pastes[0]);

  offset.value += baseLimit.value;
  loads.value += 1;
}
</script>

<style scoped>
.avatar {
  border-radius: 50%;
  background-color: rgb(var(--v-theme-primary));
}

.nickname {
  max-width: 256px;
  display: inline-block;
}
</style>
