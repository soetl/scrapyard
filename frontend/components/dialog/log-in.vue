<template>
  <v-dialog v-model="dialog" persistent width="512">
    <v-card v-show="isLogIn">
      <v-card-title>
        <span class="text-h5">Log In</span>
      </v-card-title>
      <v-card-text>
        <v-form ref="logInForm" class="pa-0">
          <v-row>
            <v-col cols="12" class="py-0">
              <v-text-field
                label="Username"
                v-model="username"
                :rules="[
                  (v) => !!v || 'Username is required',
                  (v) =>
                    v.length >= 4 || 'Username must be at least 4 characters',
                  (v) =>
                    v.length <= 16 || 'Username must be at most 16 characters',
                ]"
                required
              />
            </v-col>
            <v-col cols="12" class="py-0">
              <v-text-field
                label="Password"
                v-model="password"
                type="password"
                :rules="[(v) => !!v || 'Password is required']"
                required
              />
            </v-col>
          </v-row>
        </v-form>
        <span class="status" v-if="status.length > 0">{{ status }}</span>
      </v-card-text>
      <v-card-actions>
        <v-spacer />
        <v-btn color="primary" variant="text" @click="isLogIn = !isLogIn">
          To Sign Up dialog
        </v-btn>
        <v-btn color="primary" variant="text" @click="dialog = false"
          >Close</v-btn
        >
        <v-btn color="primary" variant="outlined" @click="logIn">Log In</v-btn>
      </v-card-actions>
    </v-card>
    <v-card v-show="!isLogIn">
      <v-card-title>
        <span class="text-h5">Sign Up</span>
      </v-card-title>
      <v-card-text>
        <v-form ref="signUpForm" class="pa-0">
          <v-row>
            <v-col cols="12" class="py-0">
              <v-text-field
                label="Username"
                hint="Your username"
                v-model="username"
                :rules="[
                  (v) => !!v || 'username is required',
                  (v) =>
                    v.length >= 4 || 'Username must be at least 4 characters',
                  (v) =>
                    v.length <= 16 || 'Username must be at most 16 characters',
                ]"
                counter="32"
                required
              />
            </v-col>
            <v-col cols="12" class="py-0">
              <v-text-field
                label="Password"
                hint="A strong password must contain mixed case letters and symbols"
                v-model="password"
                type="password"
                :rules="[
                  (v) => !!v || 'Password is required',
                  (v) =>
                    v.length >= 8 || 'Password must be at least 8 characters',
                  (v) =>
                    v.length <= 32 || 'Password must be at most 24 characters',
                ]"
                counter="24"
                required
              />
              <v-text-field
                label="Repeat Password"
                hint="A strong password must contain mixed case letters and symbols"
                type="password"
                :rules="[
                  (v) => !!v || 'Repeat password is required',
                  (v) => v === password || 'Password does not match',
                ]"
                required
              />
            </v-col>
          </v-row>
        </v-form>
        <span class="status" v-if="status.length > 0">{{ status }}</span>
      </v-card-text>
      <v-card-actions>
        <v-spacer />
        <v-btn color="primary" variant="text" @click="isLogIn = !isLogIn"
          >To Log In dialog</v-btn
        >
        <v-btn
          color="primary"
          variant="text"
          @click="
            dialog = false;
            isLogIn = !isLogIn;
          "
          >Close</v-btn
        >
        <v-btn color="primary" variant="outlined" @click="signUp"
          >Sign Up</v-btn
        >
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { useUserStore } from "~/stores/user";
import { createUser, loginUser } from "~/api";

const dialog = inject("logInDialog");
const isLogIn = ref(true);

const username = ref("");
const password = ref("");
const status = ref("");

const signUpForm = ref(null);
const logInForm = ref(null);

const userStore = useUserStore();

async function logIn() {
  status.value = "";
  const { valid } = await logInForm.value.validate();

  if (valid) {
    const { user, error } = await loginUser(username.value, password.value);

    if (error) {
      switch (error) {
        case "username_or_password_invalid":
          status.value = "Username or password is invalid.";
          break;
        default:
          status.value = "Unknown error. Try again later.";
      }

      return;
    }

    userStore.logIn(user.username, user.token);
    dialog.value = false;
  }
}

async function signUp() {
  status.value = "";
  const { valid } = await signUpForm.value.validate();

  if (valid) {
    const { user, error } = await createUser(username.value, password.value);

    if (error) {
      switch (error) {
        case "username_taken":
          status.value = "Username already taken.";
          break;
        case "username_length":
          status.value =
            "Username must be at least 4 characters and at most 16 characters.";
          break;
        case "password_length":
          status.value =
            "Password must be at least 8 characters and at most 24 characters.";
          break;
        default:
          status.value = "Unknown error. Try again later.";
      }

      return;
    }

    userStore.logIn(user.username, user.token);
    dialog.value = false;
  }
}

watch(
  () => dialog.value,
  (value) => {
    clearForm();
    isLogIn.value = true;
  }
);

watch(
  () => isLogIn.value,
  (value) => {
    clearForm();
    status.value = "";
  }
);

function clearForm() {
  username.value = null;
  password.value = null;
}
</script>

<style scoped>
.status {
  color: rgb(var(--v-theme-error));
}
</style>
