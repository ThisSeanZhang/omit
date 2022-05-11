import { createRouter, createWebHashHistory, RouteRecordRaw } from 'vue-router';
import TerminalWorkView from '@/views/TerminalWorkView.vue';
import SnapshotManageView from '@/views/SnapshotManageView.vue';
import CommandCreateView from '@/views/CommandCreateView.vue';
import CommandManageView from '@/views/CommandManageView.vue';
import TermRoute from './TermRoute';
import CmdRoute from './CmdRoute';

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'TerminalWorkView',
    component: TerminalWorkView,
  }, {
    path: '/snap-manage',
    name: 'SnapshotManageView',
    component: SnapshotManageView,
  }, {
    // path: '/cmd-manage',
    path: '/CommandCreateView',
    name: 'CommandCreateView',
    component: CommandCreateView,
  }, {
    path: '/CommandManageView',
    name: 'CommandManageView',
    component: CommandManageView,
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
