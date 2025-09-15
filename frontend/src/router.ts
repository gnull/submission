import { createRouter, createWebHistory } from 'vue-router'
import Home from './views/Home.vue'
import TeacherView from './views/TeacherView.vue'
import StudentView from './views/StudentView.vue'
import ProblemDetail from './views/ProblemDetail.vue'
import SubmissionDetail from './views/SubmissionDetail.vue'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home
  },
  {
    path: '/teacher',
    name: 'Teacher',
    component: TeacherView
  },
  {
    path: '/student',
    name: 'Student',
    component: StudentView
  },
  {
    path: '/problem/:id',
    name: 'ProblemDetail',
    component: ProblemDetail,
    props: true
  },
  {
    path: '/submission/:id',
    name: 'SubmissionDetail',
    component: SubmissionDetail,
    props: true
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
