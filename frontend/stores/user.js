import { defineStore } from "pinia";
import { getCurrentUser } from "~/api";

export const useUserStore = defineStore("user", {
  state: () => ({
    username: "",
    token: "",
  }),
  getters: {
    loggedIn: (state) => state.username != "",
    getUsername: (state) => state.username,
    getToken: (state) => state.token,
  },
  actions: {
    logIn(username, token) {
      this.username = username;
      this.token = token;
    },

    logOut() {
      this.username = "";
      this.token = "";
    },

    async fetchUser() {
      if (this.token === "" || this.token === undefined || this.token === null) {
        this.logOut();
        return;
      }

      const { user, error } = await getCurrentUser(this.token);

      if (error) {
        this.logOut();
      }

      this.logIn(user.username, this.token);
    },
  },
  persist: {
    storage: persistedState.cookiesWithOptions({
      sameSite: "strict",
    }),
  },
});
