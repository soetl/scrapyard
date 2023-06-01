<template>
  <VApp :theme="stored_theme">
    <NuxtLayout>
      <NuxtPage />
      <DialogLogIn />
      <DialogLogOut />
    </NuxtLayout>
  </VApp>
</template>

<script setup>
import { useUserStore } from "~/stores/user";

const stored_theme = useCookie("theme", { default: () => "light" });
const logInDialog = ref(false);
const logOutDialog = ref(false);
const saveFileDialog = ref(false);
const saveTextDialog = ref(false);

function toggleTheme() {
  stored_theme.value = stored_theme.value === "light" ? "dark" : "light";
}

provide("theme", stored_theme);
provide("toggleTheme", toggleTheme);
provide("logInDialog", logInDialog);
provide("logOutDialog", logOutDialog);
provide("saveFileDialog", saveFileDialog);
provide("saveTextDialog", saveTextDialog);

onMounted(() => {
  const userStore = useUserStore();

  userStore.fetchUser();
});
</script>
