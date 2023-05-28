import { defineStore } from "pinia";

export const usePastesStore = defineStore("files", {
  state: () => ({
    pastes: [],
  }),
  getters: {
    getUserPastes: (state) => (user) =>
      state.pastes.filter((paste) => paste.owner === user),
    findByUUID: (state) => (uuid) =>
      state.pastes.find((paste) => paste.uuid === uuid),
  },
  actions: {
    addPaste(paste) {
      paste.uuid = uuidGen();
      this.pastes.push(paste);
    },
    deleteFile(paste) {
      this.pastes = this.pastes.filter((f) => f.uuid !== paste.uuid);
    },
  },
});

function uuidGen(a) {
  return a
    ? (a ^ ((Math.random() * 16) >> (a / 4))).toString(16)
    : ([1e7] + -1e3 + -4e3 + -8e3 + -1e11).replace(/[018]/g, uuidGen);
}
