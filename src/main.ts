import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import VueMathjax from "vue-mathjax-next";

createApp(App).use(VueMathjax).mount("#app");
