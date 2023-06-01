import { defineStore } from "pinia";

export const useUserStore = defineStore("user", {
  state: () => ({
    username: "",
    token: "",
  }),
  getters: {
    loggedIn: (state) => state.username !== "",
  },
  actions: {
    logIn(name, token) {
      this.name = name;
      this.token = token;
    },

    logOut() {
      this.name = "";
      this.token = "";
    },

    fetchUser() {
      if (this.token === "" || this.token === undefined || this.token === null) {
        return;
      }

      apiFetch("/api/v1/user", {
        headers: {
          Authorization: `Token ${this.token}`,
        },
      })
        .catch(() => {
          this.logOut();
        })
        .then((response) => {
          console.log(response);
          if (response !== undefined) {
            this.username = response.username;
          }
        });
    },
  },
  persist: {
    storage: persistedState.cookiesWithOptions({
      sameSite: "strict",
    }),
  },
});
