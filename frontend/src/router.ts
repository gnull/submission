import { createRouter, createWebHistory } from "vue-router";
import Home from "./views/Home.vue";
import TeacherView from "./views/TeacherView.vue";
import StudentView from "./views/StudentView.vue";

const routes = [
  {
    path: "/",
    name: "Home",
    component: Home,
  },
  {
    path: "/teacher",
    name: "Teacher",
    component: TeacherView,
  },
  {
    path: "/student",
    name: "Student",
    component: StudentView,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
