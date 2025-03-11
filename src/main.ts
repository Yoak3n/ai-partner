import { createApp } from "vue";
import App from "./App.vue";
import "./assets/style/index.css";
const app = createApp(App)

import router from "./router";
app.use(router)

import pinia from './store'
app.use(pinia)
app.mount("#app");

