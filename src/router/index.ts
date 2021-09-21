import { createRouter, createWebHashHistory, RouteRecordRaw } from 'vue-router';
import TermRoute from './TermRoute';
import Welcome from '../views/Welcome.vue';

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'Welcome',
    component: Welcome,
  },
];

routes.push(...TermRoute);

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;
