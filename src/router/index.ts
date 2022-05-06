import { createRouter, createWebHashHistory, RouteRecordRaw } from 'vue-router';
import TermRoute from './TermRoute';
import CmdRoute from './CmdRoute';
import TerminalWorkView from '../views/TerminalWorkView.vue';
import SnapshotManageView from '../views/SnapshotManageView.vue';

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'TerminalWorkView',
    component: TerminalWorkView,
  }, {
    path: '/snap-manage',
    name: 'SnapshotManageView',
    component: SnapshotManageView,
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
