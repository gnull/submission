import { createApp } from "vue";
import PrimeVue from "primevue/config";
import Aura from "@primeuix/themes/aura";
import router from "./router";
import App from "./App.vue";

// PrimeVue Components
import Card from "primevue/card";
import Message from "primevue/message";
import ProgressSpinner from "primevue/progressspinner";
import Button from "primevue/button";
import Dialog from "primevue/dialog";
import Tag from "primevue/tag";
import DataView from "primevue/dataview";
import InputText from "primevue/inputtext";
import Textarea from "primevue/textarea";
import FloatLabel from "primevue/floatlabel";

// PrimeVue CSS
import "primeicons/primeicons.css";

const app = createApp(App);

app.use(router);
app.use(PrimeVue, {
  theme: {
    preset: Aura,
  },
});

// Register PrimeVue components globally
app.component("Card", Card);
app.component("Message", Message);
app.component("ProgressSpinner", ProgressSpinner);
app.component("Button", Button);
app.component("Dialog", Dialog);
app.component("Tag", Tag);
app.component("DataView", DataView);
app.component("InputText", InputText);
app.component("Textarea", Textarea);
app.component("FloatLabel", FloatLabel);

app.mount("#app");
