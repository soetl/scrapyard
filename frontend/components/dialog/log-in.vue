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
                    v.length <= 32 || 'Username must be at most 16 characters',
                ]"
                required
              />
            </v-col>
            <v-col cols="12" class="py-0">
              <v-text-field
                label="Password"
                v-model="password"
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
                    v.length <= 32 || 'Username must be at most 16 characters',
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
                :rules="[
                  (v) => !!v || 'Password is required',
                  (v) =>
                    v.length >= 8 || 'Password must be at least 8 characters',
                  (v) =>
                    v.length <= 24 || 'Password must be at most 24 characters',
                ]"
                counter="24"
                required
              />
              <v-text-field
                label="Repeat Password"
                hint="A strong password must contain mixed case letters and symbols"
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

const dialog = inject("logInDialog");
const isLogIn = ref(true);

const username = ref("");
const password = ref("");
const status = ref("");

const signUpForm = ref(null);
const logInForm = ref(null);

const userStore = useUserStore();
const cookie = useCookie();

async function logIn() {
  status.value = "";
  const { valid } = await logInForm.value.validate();

  if (valid) {
    logInProceed("/api/v1/users/login");
  }
}

async function signUp() {
  status.value = "";
  const { valid } = await signUpForm.value.validate();

  if (valid) {
    logInProceed("/api/v1/users", true);
  }
}

function logInProceed(path, isSignUp = false) {
  apiFetch(path, {
    method: "POST",
    body: {
      user: {
        username: username.value,
        password: password.value,
      },
    },
  })
    .catch((error) => {
      if (error.status === 422) {
        status.value = isSignUp
          ? "Username already taken."
          : "Invalid username or password.";
      } else {
        console.log(error);
      }
    })
    .then((response) => {
      if (response !== undefined) {
        const user = response.user;
        userStore.logIn(user.username, user.token);
        dialog.value = false;
      }
    });
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
  color: #ff5252;
}
</style>
