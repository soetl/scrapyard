import { defineStore } from "pinia";

export const useUserStore = defineStore("user", {
  state: () => ({
    name: "",
  }),
  getters: {
    loggedIn: (state) => state.name !== "",
  },
  actions: {
    logIn(name, password) {
      this.name = name;
    },
    logOut() {
      this.name = "";
    },
  },
});
