import { basicSetup } from 'codemirror'
import VueCodemirror from 'vue-codemirror'

export default defineNuxtPlugin((nuxtApp) => {
    nuxtApp.vueApp.use(VueCodemirror, {
        extensions: [basicSetup],
    })
})