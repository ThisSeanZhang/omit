import { createRouter, createWebHashHistory, RouteRecordRaw } from 'vue-router';
import TermRoute from './TermRoute';
import CmdRoute from './CmdRoute';
import Welcome from '../views/Welcome.vue';
import TerminalWorkView from '../views/TerminalWorkView.vue';
import VTerminal from '../views/VTerminal.vue';

const routes: Array<RouteRecordRaw> = [
  {
    path: '/:sessionName?',
    name: 'TerminalWorkView',
    component: TerminalWorkView,
  },
  // {
  //   path: '/:sessionName?',
  //   name: 'Welcome',
  //   component: Welcome,
  // },
];

routes.push(...TermRoute);
routes.push(...CmdRoute);

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;
