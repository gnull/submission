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
import Toast from "primevue/toast";
import ToastService from "primevue/toastservice";
import Panel from "primevue/panel";
import Chip from "primevue/chip";
import Badge from "primevue/badge";
import Divider from "primevue/divider";

// PrimeVue CSS
import "primeicons/primeicons.css";

const app = createApp(App);

app.use(router);
app.use(PrimeVue, {
  theme: {
    preset: Aura,
  },
});
app.use(ToastService);

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
app.component("Toast", Toast);
app.component("Panel", Panel);
app.component("Chip", Chip);
app.component("Badge", Badge);
app.component("Divider", Divider);

app.mount("#app");
